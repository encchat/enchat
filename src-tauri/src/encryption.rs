extern crate ed25519_dalek;


use aes_gcm_siv::{Aes256GcmSiv, KeyInit, aead::{Aead, Payload}, Nonce};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use x25519_dalek::{StaticSecret};

pub type Key = StaticSecret;
pub type PublicKey = x25519_dalek::PublicKey;
pub type RootKey = [u8; 32];
pub type Otherkey = [u8; 32];

pub fn get_rng() -> OsRng {
    OsRng{}
}


pub fn generate_ephemeral() -> StaticSecret {
    let rng = get_rng();
    StaticSecret::new(rng)
}

pub struct KdfOutput(pub RootKey, pub Otherkey);

pub fn kdf(secrets: Vec<u8>) -> KdfOutput {
    let mut message: Vec<u8> = vec![0xFF];
    message.extend(secrets);
    let hk = Hkdf::<Sha256>::new(None, &message);
    let mut output = [0u8; 64];
    hk.expand(b"enchat", &mut output).expect("HKDF Failed");
    let (root_key, other_key) = output.split_at(32);
    KdfOutput(root_key.try_into().expect("Invalid size"), other_key.try_into().expect("Invalid size"))
}

pub fn encrypt(key: &Otherkey, message: &[u8], ad: &[u8]) -> Vec<u8> {
    let output = kdf(key.to_vec());
    let (iv, _) = output.1.split_at(12);
    let aes_key = Aes256GcmSiv::new_from_slice(output.0.as_ref()).unwrap();
    let nonce = Nonce::from_slice(&iv);
    let payload = Payload {
        msg: message,
        aad: ad
    };
    aes_key.encrypt(nonce, payload).expect("Failed to encrypt???")
}

pub fn decrypt(key: &Otherkey, message: &[u8], ad: &[u8]) -> Result<Vec<u8>, aes_gcm_siv::Error> {
    let output = kdf(key.to_vec());
    let (iv, _) = output.1.split_at(12);
    let aes_key = Aes256GcmSiv::new_from_slice(output.0.as_ref()).unwrap();
    let nonce = Nonce::from_slice(&iv);
    let payload = Payload {
        msg: message,
        aad: ad
    };
    aes_key.decrypt(nonce, payload)
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn should_encrypt_with_ad() {
        let key = [0x20u8; 32];
        let message = "Hello World".as_bytes();
        let ad = "AD".as_bytes();
        let encrypted = super::encrypt(&key, message, ad);
        let decrypted = super::decrypt(&key, &encrypted, ad).expect("Failed to decrypt");
        assert_eq!(message, &decrypted[..]);
    }
    #[test]
    pub fn encryption_should_fail_with_tempered_ad() {
        let key = [0x20u8; 32];
        let message = "Hello World".as_bytes();
        let ad = "AD".as_bytes();
        let encrypted = super::encrypt(&key, message, ad);
        let ad = "ADE".as_bytes();
        let decrypted = super::decrypt(&key, &encrypted, ad);
        assert!(decrypted.is_err());
    }
    #[test]
    pub fn decryption_should_fail_with_wrong_key() {
        let key = [0x20u8; 32];
        let message = "Hello World".as_bytes();
        let ad = "AD".as_bytes();
        let encrypted = super::encrypt(&key, message, ad);
        let key = [0x21u8; 32];
        let decrypted = super::decrypt(&key, &encrypted, ad);
        assert!(decrypted.is_err());
    }
    #[test]
    pub fn kdf_should_be_deterministic() {
        let key = [0x20u8; 32];
        let output = super::kdf(key.to_vec());
        let output2 = super::kdf(key.to_vec());
        assert_eq!(output.0, output2.0);
        assert_eq!(output.1, output2.1);
    }
    #[test]
    pub fn kdf_output_should_differ() {
        let key = [0x20u8; 32];
        let key2 = [0x21u8; 32];
        let output = super::kdf(key.to_vec());
        let output2 = super::kdf(key2.to_vec());
        assert_ne!(output.0, output2.0);
        assert_ne!(output.1, output2.1);
    }
    #[test]
    pub fn kdf_output_should_differ_from_input() {
        let key = [0x20u8; 32];
        let output = super::kdf(key.to_vec());
        assert_ne!(output.0, key);
        assert_ne!(output.1, key);
    }

}