// TODO: Fetch from API every generation
const ONETIME_KEY_NUMBER: usize = 10;

#[tauri::command]
async fn setup() -> Result<(), String> {
  Ok(())
}

use ed25519_dalek::{Keypair, Signature, PublicKey};
use crate::{keybundle::{generate_identity_key, generate_prekey, generate_onetime_keys, Keybundle, store_key, to_base58}};

#[tauri::command]
pub async fn generate_keys<R: tauri::Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<Keybundle<PublicKey>, String> {
  let id_key = generate_identity_key();
  let prekey = generate_prekey(&id_key);
  let onetime_bundle = generate_onetime_keys(ONETIME_KEY_NUMBER);
  let bundle: Keybundle<PublicKey> = Keybundle {
    identity: id_key.public,
    prekey: prekey.0.public,
    onetime: onetime_bundle.iter().map(|x| x.public).collect(),
    signature: prekey.1
  };
  // I think we should leave unwrap as those are errors are worth panicking over
  // as keys are crucial for the app to work
  // And I don't know a way to handle them gracefully
  store_key("identity",  &to_base58(&id_key.secret), None).unwrap();
  store_key("prekey", &to_base58(&prekey.0.secret), None).unwrap();
  for (i, key) in onetime_bundle.iter().enumerate() {
    store_key("onetime", &to_base58(&key.secret), Some(i)).unwrap();
  }
  Ok(bundle)
}