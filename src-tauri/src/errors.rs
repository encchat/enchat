use std::{io, error};

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("Error parsing binary structure: {0}")]
    BinarySerializationError(#[from] bincode::Error),
    #[error("Error encrypting or decrypting message")]
    MessageCipherError(#[from] aead::Error),
    #[error("Unexpected error")]
    Other(#[from] anyhow::Error)
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;