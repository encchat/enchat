use std::sync::RwLock;

use rusqlite::Connection;
use serde::Deserialize;
use tauri::State;
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};

use crate::{encryption::{x_key_from_b58, generate_ephemeral, kdf, Key, RootKey, Otherkey}, message::{InitialData, self, Message}, keybundle::{IdentityKey, StoredKey, ManagedKey, Prekey, SignedKey, Onetime}, store::DatabaseState};


#[derive(Debug)]
pub struct Chain {
    id: u32,
    input_key: RootKey,
}

pub struct DHRachet {
    our_keypair: Key,
    their_public: PublicKey
}

impl DHRachet {
    pub fn new(their_public: PublicKey) -> Self {
        Self {
            our_keypair: generate_ephemeral(),
            their_public
        }
    }
    pub fn calculate_dh(&self) -> SharedSecret {
        self.our_keypair.diffie_hellman(&self.their_public)
    }
    pub fn step(&mut self, new_public_key: PublicKey) -> SharedSecret {
        self.their_public = new_public_key;
        let dh = self.calculate_dh();
        self.our_keypair = generate_ephemeral();
        dh
    }
}


impl Chain {
    fn step(&mut self, dh_input: Option<&SharedSecret>) -> Otherkey {
        let mut vec: Vec<u8> = Vec::with_capacity(64);
        if let Some(dh) = dh_input {
            vec.extend_from_slice(dh.as_bytes());
        };
        vec.extend_from_slice(&self.input_key);
        let output = kdf(vec);
        self.input_key = output.0;
        self.id+=1;
        output.1
    }
    fn set_key(&mut self, key: RootKey) {
        self.input_key = key;
    }
}
impl Default for Chain {
    fn default() -> Self {
        Self { id: Default::default(), input_key: Default::default() }
    }
}

pub struct ChatState {
    receiverIdentity: PublicKey,
    ourIdentity: IdentityKey,
    rootChain: Chain,
    senderChain: Chain,
    receiverChain: Chain,
    rachet: DHRachet
}


impl ChatState {
    pub fn dh_receiver(sender_ephemeral: &PublicKey, identity_key: &Key, sender_identity: &PublicKey, prekey: &Key, onetime_key: Option<Onetime> ) -> Vec<u8> {
        let mut vec = Vec::with_capacity(32 * 4);
        vec.extend_from_slice(prekey.diffie_hellman(&sender_identity).as_bytes());
        vec.extend_from_slice(identity_key.diffie_hellman(&sender_ephemeral).as_bytes());
        vec.extend_from_slice(prekey.diffie_hellman(&sender_ephemeral).as_bytes());
        if let Some(onetime) = onetime_key {
            vec.extend_from_slice(onetime.get_keypair().diffie_hellman(&sender_ephemeral).as_bytes());
        }
        vec
    }
    pub fn dh_sender(ephemeral: &Key, identity_key: &Key, receiver_identity: &PublicKey, receiver_prekey: &PublicKey, receiver_onetime: Option<PublicKey>) -> Vec<u8> {
        let mut vec = Vec::with_capacity(32 * 4);
        vec.extend_from_slice(identity_key.diffie_hellman(&receiver_prekey).as_bytes());
        vec.extend_from_slice(ephemeral.diffie_hellman(&receiver_identity).as_bytes());
        vec.extend_from_slice(ephemeral.diffie_hellman(&receiver_prekey).as_bytes());
        if let Some(onetime) = receiver_onetime {
            vec.extend_from_slice(ephemeral.diffie_hellman(&onetime).as_bytes());
        };
        vec
    }
    pub fn new(ourIdentity: IdentityKey, initialRachetKey: &PublicKey, initialDH: Vec<u8>) -> Self {
        let output = kdf(initialDH);
        Self {
            rootChain: Chain { id: 0, input_key: output.0 },
            rachet: DHRachet::new(initialRachetKey.clone()),
            senderChain: Default::default(),
            receiverChain: Default::default(),
            receiverIdentity: initialRachetKey.clone(),
            ourIdentity: ourIdentity
        }
    }
    pub fn initial_sender(receiver_identity: &PublicKey, receiver_prekey: &PublicKey, receiver_onetime: Option<PublicKey>, conn: &Connection) -> Self {
        let identity_key = IdentityKey::fetch(None, conn).unwrap();
        let ephemeral = generate_ephemeral();
        let vec = Self::dh_sender(&ephemeral, &identity_key.get_keypair(), &receiver_identity, &receiver_prekey, receiver_onetime);
        Self::new(identity_key, receiver_identity, vec)
    }
    pub fn initial_receiver(initialData: &InitialData, rachet: &PublicKey, conn: &Connection) -> Self {
        let identity_key = IdentityKey::fetch(None, conn).unwrap();
        // TODO: Add ids
        let prekey = SignedKey::fetch(None, conn).unwrap();
        // TODO: Handle, may be a common case?
        let onetime_key = Onetime::fetch(initialData.onetime_key_id, conn).ok();
        let sender_identity = initialData.identity;
        let sender_ephemeral = initialData.ephemeral;

        let vec = Self::dh_receiver(&sender_ephemeral, &identity_key.get_keypair(), &sender_identity, &prekey.get_keypair(), onetime_key);
        Self::new(identity_key, &rachet, vec)
    }
    pub fn move_sender(&mut self) -> (PublicKey, Otherkey) {
        let key = self.senderChain.step(None);
        (PublicKey::from(&self.rachet.our_keypair), key)
    }
    pub fn move_receiver(&mut self, new_public_key: PublicKey) -> Otherkey {
        let receiver_dh = self.rachet.step(new_public_key);
        let receiver_key = self.rootChain.step(Some(&receiver_dh));
        self.receiverChain.set_key(receiver_key);
        let sender_key = self.rootChain.step(Some(&self.rachet.calculate_dh()));
        self.senderChain.set_key(sender_key);
        self.receiverChain.step(None)
    }
}


pub struct WrappedChatState(pub RwLock<Option<ChatState>>);

#[derive(Deserialize)]
pub struct ReceiverBundle {
    pub receiver_identity: PublicKey,
    pub receiver_prekey: PublicKey,
    pub receiver_onetime: Option<PublicKey>
}

#[tauri::command]
pub fn enter_chat(chat_id: String, received_message: Option<Message>, receiver_keys: Option<ReceiverBundle>, state: State<WrappedChatState>, db_state: State<DatabaseState>) {
    let conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection();
    let mut chat = state.0.write().unwrap();
    *chat = if let Some(message) = received_message {
        Some(ChatState::initial_receiver(&message.initial, &message.rachet_key, conn))
    } else if let Some(receiver_keys) = receiver_keys {
        Some(ChatState::initial_sender(&receiver_keys.receiver_identity,&receiver_keys.receiver_prekey, receiver_keys.receiver_onetime, conn))
    } else {
        None
    };
}