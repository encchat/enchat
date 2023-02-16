extern crate ed25519_dalek;


use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signature, Signer};

pub fn get_rng() -> OsRng {
    OsRng{}
}

pub fn generate_key() -> Keypair {
    let mut rng = get_rng();
    Keypair::generate(&mut rng)
}

pub fn sign(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}
