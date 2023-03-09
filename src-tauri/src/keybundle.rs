
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

pub fn to_base58<K: AsRef<[u8]>>(bytes: K) -> String {
    bs58::encode(bytes).into_string()
}


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
    extern crate serde_test;
    use super::*;
    #[test]
    fn convert_to_base58() {
        let bytes: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
        assert_eq!(to_base58(&bytes), "6h8cQN");
    }
}