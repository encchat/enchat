
use rusqlite::{Connection, params};
use serde::{Serialize, ser::SerializeStruct};
use x25519_dalek::StaticSecret;

use crate::{encryption::{PublicKey, Key, generate_ephemeral, get_rng, Otherkey}, user::User};

use super::{to_base58, signature::{calculate_signature, Signature}};

// Trait for user-managed keys stored in database
pub trait StoredKey<'a> {
    fn fetch(id: Option<u32>, connection: &Connection, user: &User) -> rusqlite::Result<Self> where Self: Sized;
    fn store(&self, connection: &Connection, user: &User) -> rusqlite::Result<usize>;
    fn generate() -> Self;
}
pub trait ManagedKey {
    fn get_public_key(& self) -> PublicKey;
    // return Some() only if we have a private key
    fn get_keypair(& self) -> & Key;
}

#[derive(Clone)]
pub struct IdentityKey(Key);


impl <'a> StoredKey<'a> for IdentityKey {
    fn fetch(_id: Option<u32>, connection: &Connection, user: &User) -> rusqlite::Result<Self> {
        // TODO: Should we store multiple identites? I'm not sure
        connection.query_row("SELECT key FROM identity WHERE user_id = ? LIMIT 1", params![user.user_id], |row| {
            let blob: Vec<u8> = row.get(0)?;
            // TODO Handle gracefully
            let blob_keyed: [u8; 32] = blob.try_into().unwrap();
            Ok(Self(Key::from(blob_keyed)))
        })
    }

    fn store(&self, connection: &Connection, user: &User) -> rusqlite::Result<usize>{
        connection.execute("INSERT INTO identity(key, user_id) VALUES (?, ?)", params![self.0.to_bytes(), user.user_id.as_ref().unwrap()])
    }

    fn generate() -> Self {
        Self(generate_ephemeral())
    }

}
impl ManagedKey for IdentityKey {
    fn get_public_key(& self) -> PublicKey {
        PublicKey::from(&self.0)
    }

    fn get_keypair(& self) -> &Key {
        &self.0
    }
}

pub struct SignedKey(Key);


impl SignedKey {
    pub fn signature(&self, secret: &StaticSecret) -> Signature {
        let mut rng = get_rng();
        calculate_signature(secret, &mut rng, self.get_public_key().as_bytes())
    }
}


// TODO: DRY
impl <'a> StoredKey<'a> for SignedKey {
    fn fetch(_id: Option<u32>, connection: &Connection, user: &User) -> rusqlite::Result<Self> {
        // TODO: Handle renewal
        connection.query_row("SELECT key FROM signed WHERE user_id = ? LIMIT 1", params![user.user_id], |row| {
            let blob: Vec<u8> = row.get(0)?;
            // TODO Handle gracefully
            let blob_keyed: [u8; 32] = blob.try_into().unwrap();
            Ok(Self(Key::from(blob_keyed)))
        })
    }

    fn store(&self, connection: &Connection, user: &User) -> rusqlite::Result<usize>{
        connection.execute("INSERT INTO signed(key, user_id) VALUES (?, ?)", params![self.0.to_bytes(), user.user_id.as_ref().unwrap()])
    }

    fn generate() -> Self {
        Self(generate_ephemeral())
    }
}

impl ManagedKey for SignedKey {
    fn get_public_key(& self) -> PublicKey {
        PublicKey::from(&self.0)
    }

    fn get_keypair(& self) -> &Key {
        &self.0
    }
}

pub struct Onetime {
    key: Key,
    pub id: usize
}

impl Onetime {
    pub fn generate_id(id: usize) -> Self {
        let mut s = Self::generate();
        s.id = id;
        s
    }
}

impl <'a> StoredKey<'a> for Onetime {
    fn fetch(id: Option<u32>, connection: &Connection, user: &User) -> rusqlite::Result<Self> {
        // TODO: Handle renewal
        connection.query_row("SELECT key, id FROM onetime WHERE id = ? AND user_id = ? LIMIT 1", params![id, user.user_id], |row| {
            debug!("found onetime key");
            let blob: Vec<u8> = row.get(0)?;
            let id: usize = row.get(1)?;
            // TODO Handle gracefully
            let blob_keyed: [u8; 32] = blob.try_into().unwrap();
            Ok(Self {
                key: Key::from(blob_keyed), 
                id
            })
        })
    }

    fn store(&self, connection: &Connection, user: &User) -> rusqlite::Result<usize> {
        connection.execute("INSERT INTO onetime(key, user_id, id) VALUES (?, ?, ?)", params![self.key.to_bytes(), user.user_id.as_ref().unwrap(), self.id])
    }

    fn generate() -> Self {
        Self {
            key: generate_ephemeral(),
            id: 0
        }
    }
}

impl ManagedKey for Onetime {
    fn get_public_key(& self) -> PublicKey {
        PublicKey::from(&self.key)
    }

    fn get_keypair(& self) -> &Key {
        &self.key
    }
}

// TOOD: Figure out DRY, tried dyn ManagedKey but didn't work
impl Serialize for IdentityKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(&to_base58(&self.get_public_key().as_bytes()))
    }
}

impl Serialize for SignedKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            serializer.serialize_str(&to_base58(&self.get_public_key().as_bytes()))
    }
}

impl Serialize for Onetime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("onetime", 2)?;
        s.serialize_field("key", &to_base58(&self.get_public_key().as_bytes()))?;
        s.serialize_field("id", &self.id)?;
        s.end()
    }
}


pub enum MessageKeyType {
    Receiving,
    Sending
}
impl MessageKeyType {
    pub fn from_receiving(receiving: bool) -> Self {
        match receiving {
            true => MessageKeyType::Receiving,
            false => MessageKeyType::Sending
        }
    }
}

pub fn save_message_key(message_key_type: MessageKeyType, message_id: u32, key: &Otherkey, user: &User, chat_id: &str, connection: &Connection) {
    let received = matches!(message_key_type, MessageKeyType::Receiving);
    connection.execute("INSERT INTO message_key(key, chat_id, user_id, received, local_id) VALUES (?, ?, ?, ?, ?)", params![
        key,
        chat_id,
        user.user_id,
        received,
        message_id
    ]).unwrap();
}

pub fn read_message_key(message_key_type: MessageKeyType, message_id: u32, chat_id: &str, user: &User, connection: &Connection) -> Option<Otherkey> {
    let received = matches!(message_key_type, MessageKeyType::Receiving);
    connection.query_row("SELECT key FROM message_key WHERE received = ? AND local_id = ? AND user_id = ? AND chat_id = ? LIMIT 1", params![
        received, message_id, user.user_id, chat_id
    ], |row| {
        let key_vec: Vec<u8> = row.get(0)?;
        Ok(key_vec.try_into().unwrap())
    }).ok()
}

#[cfg(test)]
mod test {

    use crate::{user::User, keybundle::StoredKey, helpers::prepare_database};

    #[inline]
    fn test_user() -> User {
        User {
            user_id: Some("AAAA-GGGG".to_owned())
        }
    }

    #[test]
    fn identity_key_should_be_stored_and_fetched() {
        let mut connection = prepare_database();
        let user = test_user();
        let key = super::IdentityKey::generate();
        key.store(&mut connection, &user).unwrap();
        let key2 = super::IdentityKey::fetch(None, &mut connection, &user).unwrap();
        assert_eq!(key.0.to_bytes(), key2.0.to_bytes());
    }
    #[test]
    fn signed_key_should_be_stored_and_fetch() {
        let mut connection = prepare_database();
        let user = test_user();
        let key = super::SignedKey::generate();
        key.store(&mut connection, &user).unwrap();
        let key2 = super::SignedKey::fetch(None, &mut connection, &user).unwrap();
        assert_eq!(key.0.to_bytes(), key2.0.to_bytes());
    }
    #[test]
    fn onetime_key_should_be_stored_and_fetch() {
        let mut connection = prepare_database();
        let user = test_user();
        let key = super::Onetime::generate();
        key.store(&mut connection, &user).unwrap();
        let key2 = super::Onetime::fetch(Some(0), &mut connection, &user).unwrap();
        assert_eq!(key.key.to_bytes(), key2.key.to_bytes());
    }
    #[test]
    fn message_key_should_be_stored_and_fetch() {
        let mut connection = prepare_database();
        let user = test_user();
        let key = [0x02u8; 32];
        super::save_message_key(super::MessageKeyType::Receiving, 0, &key, &user, "AAAA", &mut connection);
        let key2 = super::read_message_key(super::MessageKeyType::Receiving, 0, "AAAA", &user, &mut connection).unwrap();
        assert_eq!(key, key2);
    }
}