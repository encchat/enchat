
use ed25519_dalek::{Keypair, Signature};
use keyring::Entry;
use serde::{Serialize, ser::SerializeStruct};

use crate::encryption::{self, sign};


pub struct Prekey(pub Keypair, pub Signature);
#[derive(Debug)]
pub struct Keybundle<K: AsRef<[u8]>> {
    pub identity: K,
    pub prekey: K,
    pub signature: Signature,
    pub onetime: Vec<K>
}

impl <K: AsRef<[u8]>> Serialize for Keybundle<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let identity_b58 = to_base58(&self.identity);
            let prekey_b58 = to_base58(&self.prekey);
            let signature_b58 = to_base58(&self.signature);
            let onetime_b58 = self.onetime.iter().map(|x| to_base58(&x)).collect::<Vec<String>>();
            let mut s = serializer.serialize_struct("Keybundle", 4)?;
            s.serialize_field("identity", &identity_b58)?;
            s.serialize_field("prekey", &prekey_b58)?;
            s.serialize_field("signature", &signature_b58)?;
            s.serialize_field("onetime", &onetime_b58)?;
            s.end()
    }
}

pub fn generate_identity_key() -> Keypair {
    encryption::generate_key()
}

pub fn generate_prekey(identity_key: &Keypair) ->  Prekey {
    let prekey = encryption::generate_key();
    let b58 = to_base58(&prekey.public);
    let signed = sign(identity_key, &b58.as_bytes());
    Prekey(prekey, signed)
}

pub fn generate_onetime_keys(keys: usize) -> Vec<Keypair> {
    let mut onetime_keys : Vec<Keypair> = Vec::with_capacity(keys);
    for x in 0..keys {
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

fn get_key(key_type: &str, id: Option<usize>) -> Option<String> {
    match get_key_entry(key_type, id) {
        Ok(entry) => entry.get_password().ok(),
        Err(_) => None,
    }
}

pub fn store_key(key_type: &str, key: &str, id: Option<usize>) -> Result<(), keyring::Error> {
    get_key_entry(key_type, id)
        .and_then(move |entry| entry.set_password(key))
}

#[tauri::command]
pub fn request_onetime_keys(keys: usize, last_key: usize) -> Result<Vec<String>, ()> {
    let keys = generate_onetime_keys(keys);
    let b58_keys: Vec<String> = keys.iter().map(|x| to_base58(x.public)).collect();
    for (i, key) in keys.iter().enumerate() {
        let id = i + 1 + last_key;
        let result = store_key("onetime", &to_base58(&key.secret), Some(id));
        if result.is_err() {
            debug!("Failed to store onetime key {}", id);
        }
    }
    Ok(b58_keys)
}


#[cfg(test)]
mod tests {
    use ed25519_dalek::{SignatureError, PublicKey, Signer};
    use serde_test::{assert_ser_tokens, Token};
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

    fn test_keypair() -> Keypair {
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
        Keypair::from_bytes(&bytes).unwrap()
    }

    #[test]
    fn prekey_valid_signature() -> Result<(), SignatureError> {
        let id_keys = test_keypair();
        let prekey = generate_prekey(&id_keys);
        let signature = prekey.1;
        let prekey_public_b58 = bs58::encode(&prekey.0.public).into_string();
        id_keys.verify(prekey_public_b58.as_bytes(), &signature)
    }
    #[test]
    fn keybundle_serializion() {
        let keys = test_keypair();
        let bundle: Keybundle<PublicKey> = Keybundle {
            identity: keys.public,
            prekey: keys.public,
            signature: keys.sign(keys.public.as_bytes()),
            onetime: [keys.public, keys.public].to_vec()
        };
        let b58_public = "2up846HMkTuxSmnn5EgF9J96MGoiXrtNtAe453mcnoys";
        let b58_signature = "54URn1aoiwPEmoEAqfNsvnaVRZen6JSjubqDmuSFpGnmMnophiT4k7gT9mRHxa4pgj3mePn7VhiucutZcAFDbvm5";
        assert_ser_tokens(&bundle, &[
            Token::Struct { name: "Keybundle", len: 4},
            Token::Str("identity"),
            Token::String(b58_public),
            Token::Str("prekey"),
            Token::String(b58_public),
            Token::Str("signature"),
            Token::String(b58_signature),
            Token::Str("onetime"),
            Token::Seq { len: Some(bundle.onetime.len()) },
            Token::String(b58_public),
            Token::String(b58_public),
            Token::SeqEnd,
            Token::StructEnd
        ])
    }
}