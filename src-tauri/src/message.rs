use serde::{Deserialize, Serialize};
use tauri::State;
use x25519_dalek::{PublicKey};

use crate::{chat::{WrappedChatState}, encryption::{encrypt, decrypt}, store::{DatabaseState}, user::{UserState}, with_state, keybundle::{save_message_key, MessageKeyType, read_message_key}};


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
pub fn send(chat_id: String, message: String, state: State<WrappedChatState>, db_state: State<DatabaseState>, user_state: State<UserState>) -> Option<Message> {
    // FIXME: DRY!! maybe macro?
    with_state!(state, user_state, db_state, |chat, user, conn| {
        let (rachet_key, message_key, id) = chat.move_sender();
        chat.save(&user, &conn, &chat_id).expect("Failed to save double rachet state");
        save_message_key(MessageKeyType::Sending, id, &message_key, &user, &chat_id, &conn);
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
    })
}

#[tauri::command]
pub fn receive(chat_id: String, message: Message, state: State<WrappedChatState>, db_state: State<DatabaseState>, user_state: State<UserState>) -> Option<Vec<u8>> {
    with_state!(state, user_state, db_state, |chat, user, conn| {
        let message_key = chat.move_receiver(message.header.rachet_key);
        chat.save(&user, &conn, &chat_id).expect("Failed to save double rachet state");
        // Is it okay to relay on the message for ids?
        save_message_key(MessageKeyType::Receiving, message.header.id, &message_key, &user, &chat_id, &conn);
        let ad = bincode::serialize(&message.header).unwrap();
        let decoded = decrypt(&message_key, &message.ciphertext, &ad);
        decoded.ok()
    })
}

#[tauri::command]
pub fn try_decrypt(chat_id: String, received: bool, message: Message, state: State<WrappedChatState>, db_state: State<DatabaseState>, user_state: State<UserState>) -> Option<Vec<u8>> {
    with_state!(state, user_state, db_state, |_chat, user, conn| {
        let message_key = read_message_key(if received { MessageKeyType::Receiving} else { MessageKeyType::Sending }, message.header.id, &chat_id, &user, &conn)?;
        let ad = bincode::serialize(&message.header).unwrap();
        let decoded = decrypt(&message_key, &message.ciphertext, &ad);
        decoded.ok()
    })
}