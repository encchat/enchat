use std::{fmt::Display, sync::Mutex, fs::create_dir, path::PathBuf};

use keyring::Entry;
use rusqlite::{Connection, params};

use crate::{encryption::{KdfOutput, get_rng, kdf}};
use rand::{RngCore};

pub use self::migrations::make_migrations;

mod migrations;

fn ensure_appdata_dir_exists() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("enchat");
    if !path.exists() {
        create_dir(&path).expect("Can't get create an app data directory");
    }
    path
}

#[inline]
fn database_path() -> PathBuf {
    let mut dir = ensure_appdata_dir_exists();
    dir.push("enchat_store.db");
    dir
}


impl Display for KdfOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // folds into one hex string, with formatter result accumulated
        self.0.iter().fold(Ok(()), |result, byte| {
            result.and_then(|_| write!(f, "{:02x}", byte))
        })
    }
}
pub fn get_keyring_entry(key_type: &str) -> Result<Entry, keyring::Error> {
    Entry::new("enchat", key_type)
}

pub fn get_key_keyring(key_type: &str) -> Option<String> {
    match get_keyring_entry(key_type) {
        Ok(entry) => entry.get_password().ok(),
        Err(_) => None,
    }
}

pub fn store_key_keyring(key_type: &str, key: &str) -> Result<(), keyring::Error> {
    get_keyring_entry(key_type)
        .and_then(move |entry| entry.set_password(key))
}

pub struct Database {
    connection: Connection
}

impl Database {
    fn generate_database_key() -> KdfOutput {
        let mut rng = get_rng();
        let mut random_bytes = vec![0x0; 64];
        rng.fill_bytes(&mut random_bytes);
        kdf(random_bytes)
    }
    fn get_database_key() -> String {
        if let Some(key) = get_key_keyring("database") {
            key
        } else {
            let key = Self::generate_database_key().to_string();
            store_key_keyring("database", &key).unwrap();
            key
        }
    }
    fn new() -> rusqlite::Result<Self> {
        let path = database_path();
        let key = Self::get_database_key();
        let mut conn = Connection::open(path)?;
        println!("{}", format!("PRAGMA key = \"x'{}'\"", &key));
        conn.query_row(&format!("PRAGMA key = \"x'{}'\"", &key), params![], |_row| {
            Ok(())
        })?;
        make_migrations(&mut conn);
        Ok(Self {
            connection: conn
        })
    }
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new().expect("Database not found!")
    }
}

pub struct DatabaseState(pub Mutex<Database>);


#[cfg(test)]
mod tests {
    #[test]
    fn database_key_is_64_hex_string() {
        let key = crate::store::Database::generate_database_key().to_string();
        assert_eq!(key.len(), 64);
        for x in key.chars() {
            // should be able to parse into base 16 number if string is really hex
            assert!(x.to_digit(16).is_some())
        }
    }
}

