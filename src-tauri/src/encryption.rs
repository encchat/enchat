extern crate ed25519_dalek;


use rand::rngs::OsRng;
use ed25519_dalek::{Signature, SigningKey, Signer};

pub fn get_rng() -> OsRng {
    OsRng{}
}

pub fn generate_key() -> SigningKey {
    let mut rng = get_rng();
    SigningKey::generate(&mut rng)
}

pub fn sign(keypair: &SigningKey, message: &[u8]) -> Signature {
    keypair.sign(message)
}
