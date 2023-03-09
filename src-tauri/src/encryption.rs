extern crate ed25519_dalek;


use hkdf::Hkdf;
use rand::rngs::OsRng;
use ed25519_dalek::{Signature, SigningKey, Signer, PUBLIC_KEY_LENGTH};
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
