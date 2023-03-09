use serde::{Deserialize, Serialize};
use tauri::State;
use x25519_dalek::{SharedSecret, PublicKey};

use crate::chat::{ChatState, WrappedChatState};


#[derive(Deserialize)]
pub struct InitialData {
    pub onetime_key_id: Option<usize>,
    user_id: String,
    pub ephemeral: PublicKey,
    pub identity: PublicKey,
    prekey_id: String,
}

#[derive(Deserialize)]
pub struct Message {
    // id for receiver chain, so its local to sender
    pub id: u32,
    pub rachet_key: PublicKey,
    pub initial: InitialData,
    pub ciphertext: String,
}

pub fn send(message: String, state: State<WrappedChatState>) {
    let mut chat_option = state.0.write().unwrap();
    if chat_option.is_none(){
        return;
    }
    // let (public, encryption) = (*chat_option).unwrap().move_sender();
}