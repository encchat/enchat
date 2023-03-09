
use ed25519_dalek::SigningKey;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use x25519_dalek::StaticSecret;

use crate::encryption::{PublicKey, Key, generate_ephemeral, get_rng};

use super::{to_base58, signature::{calculate_signature, Signature}};

// Trait for user-managed keys stored in database
pub trait StoredKey<'a> {
    fn fetch(id: Option<u32>, connection: &Connection) -> rusqlite::Result<Self> where Self: Sized;
    fn store(&self, connection: &Connection) -> rusqlite::Result<usize>;
    fn generate() -> Self;
}
pub trait ManagedKey {
    fn get_public_key(& self) -> PublicKey;
    // return Some() only if we have a private key
    fn get_keypair(& self) -> & Key;
}

pub struct IdentityKey(Key);


impl <'a> StoredKey<'a> for IdentityKey {
    fn fetch(id: Option<u32>, connection: &Connection) -> rusqlite::Result<Self> {
        // TODO: Should we store multiple identites? I'm not sure
        connection.query_row("SELECT key FROM identity LIMIT 1", params![], |row| {
            let blob: Vec<u8> = row.get(0)?;
            // TODO Handle gracefully
            let blob_keyed: [u8; 32] = blob.try_into().unwrap();
            Ok(Self(Key::from(blob_keyed)))
        })
    }

    fn store(&self, connection: &Connection) -> rusqlite::Result<usize>{
        connection.execute("INSERT INTO identity(key) VALUES (?1)", params![self.0.to_bytes()])
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
    fn fetch(id: Option<u32>, connection: &Connection) -> rusqlite::Result<Self> {
        // TODO: Handle renewal
        connection.query_row("SELECT key FROM signed LIMIT 1", params![], |row| {
            let blob: Vec<u8> = row.get(0)?;
            // TODO Handle gracefully
            let blob_keyed: [u8; 32] = blob.try_into().unwrap();
            Ok(Self(Key::from(blob_keyed)))
        })
    }

    fn store(&self, connection: &Connection) -> rusqlite::Result<usize>{
        connection.execute("INSERT INTO signed(key) VALUES (?1)", params![self.0.to_bytes()])
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
    fn fetch(id: Option<u32>, connection: &Connection) -> rusqlite::Result<Self> {
        // TODO: Handle renewal
        connection.query_row("SELECT key, id FROM onetime WHERE id = ?1 LIMIT 1", params![id], |row| {
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

    fn store(&self, connection: &Connection) -> rusqlite::Result<usize>{
        connection.execute("INSERT INTO signed(key) VALUES (?1)", params![self.key.to_bytes()])
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