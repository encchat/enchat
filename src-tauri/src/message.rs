use serde::{Deserialize, Serialize};
use tauri::State;
use x25519_dalek::{SharedSecret, PublicKey};

use crate::{chat::{ChatState, WrappedChatState}, encryption::encrypt};


#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct InitialData {
    pub onetime_key_id: Option<u32>,
    pub ephemeral: PublicKey,
    pub prekey_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    // id for receiver chain, so its local to sender
    pub header: MessageHeader,
    pub ciphertext: Vec<u8>,
}
#[derive(Deserialize, Serialize)]

pub struct MessageHeader {
    pub id: u32,
    pub rachet_key: PublicKey,
    pub initial: Option<InitialData>,
}

#[tauri::command]
pub fn send(chat_id: String, message: String, state: State<WrappedChatState>) -> Option<Message> {
    let mut chat_wrapped = state.0.lock().unwrap();
    if let Some(chat) = &mut *chat_wrapped {
        let (rachet_key, message_key, id) = chat.move_sender();
        let message_header = MessageHeader {
            id,
            rachet_key,
            initial: chat.receiverUsedKeys
        };
        let ad = bincode::serialize(&message_header).unwrap();
        let ciphertext = encrypt(&message_key, message.as_bytes(), &ad);
        Some(Message {
            header: message_header,
            ciphertext
        })
    } else {
        todo!()
    }
}

#[tauri::command]
pub fn receive(chat_id: String, message: Message, state: State<WrappedChatState>) -> Option<Vec<u8>> {
    let mut chat_wrapped = state.0.lock().unwrap();
    if let Some(chat) = &mut *chat_wrapped {
        let message_key = chat.move_receiver(message.header.rachet_key);
        let ad = bincode::serialize(&message.header).unwrap();
        let decoded = encrypt(&message_key, &message.ciphertext, &ad);
        Some(decoded)
    } else {
        todo!()
    }
}