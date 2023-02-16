
use ed25519_dalek::{Keypair, Signature};
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
    let signed = sign(identity_key, &prekey.public.to_bytes());
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
        // hardcoded bytes as unit tests shouldn't rely on random values
        let prekey = generate_prekey(&id_keys);
        let signature = prekey.1;
        id_keys.verify(prekey.0.public.as_bytes(), &signature)
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