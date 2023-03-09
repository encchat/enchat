
use serde::{Serialize, ser::SerializeStruct};
use tauri::State;

mod keys;
mod signature;

use crate::{store::DatabaseState};

pub use self::{keys::{IdentityKey, StoredKey, SignedKey, ManagedKey, Onetime}, signature::Signature};
pub struct Prekey(pub SignedKey, pub Signature);
impl Serialize for Prekey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut s = serializer.serialize_struct("Prekey", 2)?;
            s.serialize_field("prekey", &self.0)?;
            s.serialize_field("signature",&to_base58(&self.1))?;
            s.end()
    }
}

// pub fn generate_onetime_keys(keys: usize) -> Vec<SigningKey> {
//     let mut onetime_keys : Vec<SigningKey> = Vec::with_capacity(keys);
//     for _ in 0..keys {
//         onetime_keys.push(encryption::generate_key());
//     }
//     onetime_keys
// }

pub fn to_base58<K: AsRef<[u8]>>(bytes: K) -> String {
    bs58::encode(bytes).into_string()
}


// fn key_from_b58(b58: &str) -> SigningKey {
//     let mut output: [u8; SECRET_KEY_LENGTH] = [0x0; SECRET_KEY_LENGTH];
//     bs58::decode(b58).into(&mut output).unwrap();
//     return SigningKey::from_bytes(&output)
// }


#[tauri::command]
pub fn request_onetime_keys(keys: usize, last_key: usize, db_state: State<DatabaseState>) -> Result<Vec<Onetime>, ()> {
    let mut conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection_mut();
    let mut onetime_keys: Vec<Onetime> = Vec::with_capacity(keys);
    let tx = conn.transaction().unwrap();
    for i in 0..keys {
        let onetime = Onetime::generate_id(last_key + 1 + i);
        onetime.store(&*tx);
        onetime_keys.push(onetime);
    }
    Ok(onetime_keys)
}

#[tauri::command]
pub fn request_identity_key(db_state: State<DatabaseState>) -> Result<IdentityKey, ()> {
    let conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection();
    let id = IdentityKey::generate();
    id.store(conn).unwrap();
    Ok(id)
}
#[tauri::command]
pub fn request_prekey(db_state: State<DatabaseState>) -> Result<Prekey, &'static str> {
    let conn_mutex = db_state.0.lock().unwrap();
    let conn = conn_mutex.get_connection();
    let identity = IdentityKey::fetch(None, conn).unwrap();
    let prekey = SignedKey::generate();
    prekey.store(conn);
    let signature = prekey.signature(&identity.get_keypair());
    Ok(Prekey(prekey, signature))
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{SigningKey};
    extern crate serde_test;
    use super::*;
    #[test]
    fn convert_to_base58() {
        let bytes: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(to_base58(&bytes), "6h8cQN");
    }
    #[test]
    fn generate_8_onetime_keys() {
        let keys = generate_onetime_keys(8);
        assert_eq!(keys.len(), 8);
    }

    fn test_keypair() -> SigningKey {
        // hardcoded bytes as unit tests shouldn't rely on random values
        let bytes: [u8; 64] = [
            136, 57, 125, 2, 68, 24, 60, 82,
            2, 84, 117, 191, 215, 93, 117, 6,
            236, 239, 35, 121, 63, 204,70,48,
            81, 127, 81, 31, 34, 249, 1, 242,
            28, 99, 43, 104, 255, 37, 232, 196,
            103, 246, 24, 172, 173, 118, 43, 13,
            36, 0, 141, 184, 61, 162, 19, 250,
            129, 114, 199, 206, 50, 132, 234, 146,
        ];
        SigningKey::from_keypair_bytes(&bytes).unwrap()
    }
}