use std::sync::{RwLock, Mutex};

use rusqlite::{Connection, named_params, params, Row};
use serde::Deserialize;
use tauri::State;
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};

use crate::{encryption::{generate_ephemeral, kdf, Key, RootKey, Otherkey}, message::{InitialData, self, Message}, keybundle::{IdentityKey, StoredKey, ManagedKey, Prekey, SignedKey, Onetime}, store::DatabaseState, user::{UserState, User}, with_state};

fn row_to_chain(row: &Row, chain_name: &str) -> rusqlite::Result<Chain> {
    let input_key: Vec<u8> = row.get(format!("{}_input_bytes", &chain_name).as_ref())?;
    let chain = Chain {
        input_key: input_key.try_into().unwrap(),
        id: row.get(format!("{}_id", &chain_name).as_ref())?
    };
    Ok(chain)
} 

#[derive(Debug, Clone, Copy)]
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
    pub receiverUsedKeys: Option<InitialData>,
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
            receiverUsedKeys: None
        }
    }
    pub fn initial_sender(receiver_identity: &PublicKey, receiver_prekey: &PublicKey, receiver_onetime: Option<PublicKey>, conn: &Connection, prekey_id: u32, onetime_key_id: Option<u32>, user: &User) -> Self {
        let identity_key = IdentityKey::fetch(None, conn, user).unwrap();
        let ephemeral = IdentityKey::generate();
        let vec = Self::dh_sender(&ephemeral.get_keypair(), &identity_key.get_keypair(), &receiver_identity, &receiver_prekey, receiver_onetime);
        let mut new_self = Self::new(identity_key, receiver_identity, vec);
        new_self.receiverUsedKeys = Some(InitialData { onetime_key_id, ephemeral: ephemeral.get_public_key(), prekey_id });
        new_self
    }
    pub fn initial_receiver(initialData: &InitialData, rachet: &PublicKey, conn: &Connection, sender_identity: PublicKey, user: &User) -> Self {
        let identity_key = IdentityKey::fetch(None, conn, user).unwrap();
        // TODO: Add ids
        let prekey = SignedKey::fetch(None, conn, user).unwrap();
        // TODO: Handle, may be a common case?
        let onetime_key = Onetime::fetch(initialData.onetime_key_id, conn, user).ok();
        let sender_ephemeral = initialData.ephemeral;

        let vec = Self::dh_receiver(&sender_ephemeral, &identity_key.get_keypair(), &sender_identity, &prekey.get_keypair(), onetime_key);
        Self::new(identity_key, &rachet, vec)
    }
    pub fn is_initial(&self) -> bool {
        self.rootChain.id == 0u32 && self.senderChain.id == 0u32
    }
    pub fn move_sender(&mut self) -> (PublicKey, Otherkey, u32) {
        let key = self.senderChain.step(None);
        (PublicKey::from(&self.rachet.our_keypair), key, self.senderChain.id)
    }
    pub fn move_receiver(&mut self, new_public_key: PublicKey) -> Otherkey {
        let receiver_dh = self.rachet.step(new_public_key);
        let receiver_key = self.rootChain.step(Some(&receiver_dh));
        self.receiverChain.set_key(receiver_key);
        let sender_key = self.rootChain.step(Some(&self.rachet.calculate_dh()));
        self.senderChain.set_key(sender_key);
        self.receiverChain.step(None)
    }
    pub fn save(&self, user: &User, connection: &Connection, chat_id: &str) -> rusqlite::Result<usize> {
        let dh_public = self.rachet.their_public.as_bytes();
        let dh_private = self.rachet.our_keypair.to_bytes();
        let root_input = &self.rootChain.input_key;
        let sender_input = &self.receiverChain.input_key;
        let receiver_input = &self.senderChain.input_key;
        connection.execute("INSERT INTO
            rachet_state(chat_id, user_id, diffie_public_bytes, diffie_private_bytes,
            root_input_bytes, sender_input_bytes, receiver_input_bytes, root_id, sender_id, receiver_id)
            VALUES (:chat_id, :user_id, :dh_public, :dh_private, :root_input, :sender_input, :receiver_input, :root_id, :sender_id, :receiver_id)
            ON CONFLICT(chat_id, user_id) DO UPDATE SET
                diffie_public_bytes = :dh_public,
                diffie_private_bytes = :dh_private,
                root_input_bytes = :root_input,
                sender_input_bytes = :sender_input,
                receiver_input_bytes = :receiver_input,
                root_id = :root_id,
                sender_id = :sender_id,
                receiver_id = :receiver_id",
        named_params! {
            ":dh_public": dh_public,
            ":dh_private": dh_private,
            ":chat_id": chat_id,
            ":user_id": user.user_id,
            ":root_input": root_input,
            ":sender_input": sender_input,
            ":receiver_input": receiver_input,
            ":root_id": self.rootChain.id,
            ":sender_id": self.senderChain.id,
            ":receiver_id": self.receiverChain.id
        })
    }
    pub fn load(user: &User, connection: &Connection, chat_id: &str) -> rusqlite::Result<Self> {
        connection.query_row("SELECT * FROM rachet_state WHERE chat_id = ? AND user_id = ? LIMIT 1", params![chat_id, user.user_id],
            |row| {
                let dh_public: Vec<u8> = row.get("diffie_public_bytes")?;
                let dh_private: Vec<u8> = row.get("diffie_private_bytes")?;
                let dh_public_bytes: [u8; 32] = dh_public.try_into().unwrap();
                let dh_private_bytes: [u8; 32] = dh_private.try_into().unwrap();
                let rachet = DHRachet {
                    their_public: PublicKey::from(dh_public_bytes),
                    our_keypair: Key::from(dh_private_bytes)
                };
                Ok(Self {
                    rachet,
                    receiverChain: row_to_chain(row, "receiver")?,
                    receiverUsedKeys: None,
                    senderChain: row_to_chain(row, "sender")?,
                    rootChain: row_to_chain(row, "root")?
                })
            })
    }
}


pub struct WrappedChatState(pub Mutex<Option<ChatState>>);

#[derive(Deserialize)]
pub struct ReceiverBundle {
    pub receiver_identity: PublicKey,
    pub receiver_prekey: PublicKey,
    pub receiver_onetime: Option<PublicKey>,
    pub receiver_onetime_id: Option<u32>,
    pub receiver_prekey_id: u32,
}

#[tauri::command]
pub fn enter_chat(chat_id: String, sender_identity: Option<PublicKey>, received_message: Option<Message>, receiver_keys: Option<ReceiverBundle>, state: State<WrappedChatState>, db_state: State<DatabaseState>, user_state: State<UserState>) {
    let user = user_state.0.lock().unwrap();
    let conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection();
    let mut chat = state.0.lock().unwrap();
    *chat = if let Some(message) = received_message {
        Some(ChatState::initial_receiver(&message.header.initial.unwrap(), &message.header.rachet_key, conn, sender_identity.unwrap(), &user))
    } else if let Some(receiver_keys) = receiver_keys {
        Some(ChatState::initial_sender(&receiver_keys.receiver_identity,&receiver_keys.receiver_prekey, receiver_keys.receiver_onetime, conn, receiver_keys.receiver_prekey_id, receiver_keys.receiver_onetime_id, &user))
    } else {
        None
    };
}

#[tauri::command]
pub fn reenter_chat(chat_id: String, state: State<WrappedChatState>, db_state: State<DatabaseState>, user_state: State<UserState>) -> Option<bool> {
    let user = user_state.0.lock().unwrap();
    let conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection();
    let mut chat = state.0.lock().unwrap();
    if let Ok(stored_chat) = ChatState::load(&user, conn, &chat_id) {
        *chat = Some(stored_chat);
        Some(true)
    } else {
        error!("CRITICAL! Failed to restore the rachet");
        None
    }
}