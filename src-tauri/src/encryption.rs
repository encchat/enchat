extern crate ed25519_dalek;


use hkdf::Hkdf;
use rand::rngs::OsRng;
use ed25519_dalek::{Signature, SigningKey, Signer, PUBLIC_KEY_LENGTH};
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret, SharedSecret};

use crate::keybundle::{get_key};

pub fn get_rng() -> OsRng {
    OsRng{}
}

pub fn generate_key() -> SigningKey {
    let mut rng = get_rng();
    SigningKey::generate(&mut rng)
}

pub fn generate_ephemeral() -> StaticSecret {
    let rng = get_rng();
    StaticSecret::new(rng)
}

pub fn sign(keypair: &SigningKey, message: &[u8]) -> Signature {
    keypair.sign(message)
}
pub fn x_key_from_b58<K: From<[u8; PUBLIC_KEY_LENGTH]>>(b58:&str) -> K {
    let mut output: [u8; PUBLIC_KEY_LENGTH] = [0x0; PUBLIC_KEY_LENGTH];
    bs58::decode(b58).into(&mut output).unwrap();
    K::from(output)
}

pub fn kdf(secrets: Vec<SharedSecret>) -> [u8; 32] {
    let salt: [u8; 32] = [0x0; 32];
    let mut message: Vec<u8> = vec![0xFF, 32];
    let input_iter = secrets.iter().flat_map(|x| x.to_bytes());
    message.extend(input_iter);
    let hk = Hkdf::<Sha256>::new(Some(&salt), &message);
    let mut output = [0u8; 32];
    let info = [101u8, 110, 99, 104, 97, 116 ]; // "enchat" in ASCII
    hk.expand(&info, &mut output).expect("HKDF Failed");
    output
}

#[tauri::command]
pub fn calculate_psk(receiver_identity: String, receiver_prekey: String, receiver_onetime: Option<String>) -> [u8; 32] {
    // TODO: Handle gracefully
    let identity_b58 = get_key("identity", None).unwrap();
    let identity_key = x_key_from_b58::<StaticSecret>(&identity_b58);
    let ephemeral = generate_ephemeral();
    let receiver_identity_public = x_key_from_b58::<PublicKey>(&receiver_identity);
    let receiver_prekey_public = x_key_from_b58::<PublicKey>(&receiver_prekey);
    let dh1 = identity_key.diffie_hellman(&receiver_prekey_public);
    let dh2 = ephemeral.diffie_hellman(&receiver_identity_public);
    let dh3 = ephemeral.diffie_hellman(&receiver_prekey_public);
    if let Some(receiver_onetime_str) = receiver_onetime {
        let receiver_onetime_public = x_key_from_b58::<PublicKey>(&receiver_onetime_str);
        let dh4 = ephemeral.diffie_hellman(&receiver_onetime_public);
        return kdf(vec![dh1, dh2, dh3, dh4])
    }
    kdf(vec![dh1, dh2, dh3])

}