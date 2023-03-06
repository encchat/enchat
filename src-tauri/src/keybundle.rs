
use ed25519_dalek::{Signature, SigningKey, SECRET_KEY_LENGTH};
use keyring::Entry;
use serde::{Serialize, ser::SerializeStruct};

use crate::encryption::{self, sign};

pub struct Prekey(pub SigningKey, pub Signature);
impl Serialize for Prekey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let public_prekey_b58 = to_base58(&self.0.verifying_key());
            let b58_signature = to_base58(&self.1.to_bytes());
            let mut s = serializer.serialize_struct("Prekey", 2)?;
            s.serialize_field("prekey", &public_prekey_b58)?;
            s.serialize_field("signature",&b58_signature)?;
            s.end()
    }
}

pub fn generate_onetime_keys(keys: usize) -> Vec<SigningKey> {
    let mut onetime_keys : Vec<SigningKey> = Vec::with_capacity(keys);
    for _ in 0..keys {
        onetime_keys.push(encryption::generate_key());
    }
    onetime_keys
}

pub fn to_base58<K: AsRef<[u8]>>(bytes: K) -> String {
    bs58::encode(bytes).into_string()
}


fn get_key_entry(key_type: &str, id: Option<usize>) -> Result<Entry, keyring::Error> {
    if let Some(x) = id {
        Entry::new_with_target("enchat-keyring","enchat",&format!("{}-{}", key_type, x))
    } else {
        Entry::new_with_target("enchat-keyring","enchat", key_type)
    }
}

pub fn get_key(key_type: &str, id: Option<usize>) -> Option<String> {
    match get_key_entry(key_type, id) {
        Ok(entry) => entry.get_password().ok(),
        Err(_) => None,
    }
}

pub fn store_key(key_type: &str, key: &str, id: Option<usize>) -> Result<(), keyring::Error> {
    get_key_entry(key_type, id)
        .and_then(move |entry| entry.set_password(key))
}

fn key_from_b58(b58: &str) -> SigningKey {
    let mut output: [u8; SECRET_KEY_LENGTH] = [0x0; SECRET_KEY_LENGTH];
    bs58::decode(b58).into(&mut output).unwrap();
    return SigningKey::from_bytes(&output)
}

#[tauri::command]
pub fn request_onetime_keys(keys: usize, last_key: usize) -> Result<Vec<String>, ()> {
    let keys = generate_onetime_keys(keys);
    let b58_keys: Vec<String> = keys.iter().map(|x| to_base58(&x.verifying_key())).collect();
    for (i, key) in keys.iter().enumerate() {
        let id = i + 1 + last_key;
        let result = store_key("onetime", &to_base58(&key.to_bytes()), Some(id));
        if result.is_err() {
            debug!("Failed to store onetime key {}", id);
        }
    }
    Ok(b58_keys)
}

#[tauri::command]
pub fn request_identity_key() -> Result<String, ()> {
    let id = encryption::generate_key();
    store_key("identity", &to_base58(&id.to_bytes()), None).unwrap();
    Ok(to_base58(&id.verifying_key()))
}
#[tauri::command]
pub fn request_prekey() -> Result<Prekey, &'static str> {
    let identity_b58 = get_key("identity", None).ok_or("Identity key not found in the keyring")?;
    let identity_key = key_from_b58(&identity_b58);
    let prekey = encryption::generate_key();
    let b58 = to_base58(&prekey.verifying_key());
    let signed = sign(&identity_key, &b58.as_bytes());
    store_key("prekey", &to_base58(&prekey.to_bytes()), None).unwrap();
    Ok(Prekey(prekey, signed))
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