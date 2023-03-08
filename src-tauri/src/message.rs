use serde::{Deserialize, Serialize};
use x25519_dalek::{SharedSecret, PublicKey};

use crate::chat::ChatState;


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
