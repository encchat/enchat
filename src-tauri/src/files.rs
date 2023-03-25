use std::{path::{PathBuf, Path}, fs::File, io::{Read, Write}, any};


use aead::stream;
use anyhow::Context;
use chacha20poly1305::{XChaCha20Poly1305, KeyInit, Nonce};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{store::DatabaseState, chat::WrappedChatState, user::UserState, with_state, keybundle::{MessageKeyType, read_message_key}, errors::CommandResult, encryption::get_rng};

#[derive(Deserialize, Serialize, Clone)]
pub struct FileInfo {
    pub filename: String,
    pub size: u64,
    pub nonce: Vec<u8>
}

const BUFFER_LEN_ENCRYPT: usize = 500;
const BUFFER_LEN_DECRYPT: usize = 516;

#[inline]
fn get_nonce(message_id: u32) -> Vec<u8> {
    let mut nonce = vec![0u8; 15];
    get_rng().fill_bytes(&mut nonce[0..15]);
    nonce.extend_from_slice(&message_id.to_be_bytes());
    dbg!(&nonce);
    nonce
}

fn decrypt_file(nonce: &[u8], source: &mut File, output: &mut File, message_key: &[u8]) -> anyhow::Result<()> {
    // Based on https://kerkour.com/rust-file-encryption
    let cipher = XChaCha20Poly1305::new(message_key.into());
    let mut decryptor = stream::DecryptorBE32::from_aead(cipher, nonce.into());
    let mut buffer = [0u8; BUFFER_LEN_DECRYPT];
    loop {
        let read_count = source.read(&mut buffer)?;
        if read_count == BUFFER_LEN_DECRYPT {
            output.write(
            &decryptor.decrypt_next(buffer.as_slice())
                    .map_err(|_| anyhow::anyhow!("Decryption failed"))?
            )?;
        } else if read_count == 0 {
            break;
        } else {
            output.write(
                &decryptor.decrypt_last(&buffer[..read_count])
                    .map_err(|_| anyhow::anyhow!("Decryption failed"))?
            )?;
            break;
        }
    }
    Ok(())
}
fn encrypt(nonce: &[u8], source: &mut File, output: &mut File, message_key: &[u8]) -> anyhow::Result<()> {
    let cipher = XChaCha20Poly1305::new(message_key.into());
    let mut encryptor = stream::EncryptorBE32::from_aead(cipher, nonce.into());
    let mut buffer = [0u8; BUFFER_LEN_ENCRYPT];

    loop {
        let read_count = source.read(&mut buffer)?;
        if read_count == BUFFER_LEN_ENCRYPT {
            output.write(
            &encryptor.encrypt_next(buffer.as_slice())
                    .map_err(|_| anyhow::anyhow!("Encryption failed"))?
            )?;
        } else if read_count == 0 {
            break;
        } else {
            output.write(
            &encryptor.encrypt_last(&buffer[..read_count])
                    .map_err(|_| anyhow::anyhow!("Encryption of last segment failed"))?
            )?;
            break;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn decrypt_and_open(info: FileInfo, receiving: bool, message_id: u32, input_path: PathBuf, output_path: PathBuf, chat_id: String,
        db_state: State<DatabaseState>, user_state: State<UserState>, chat_state: State<WrappedChatState>) -> CommandResult<()> {
    let message_key = with_state!(chat_state, user_state, db_state, |_chat, user, conn| {
        read_message_key(MessageKeyType::from_receiving(receiving), message_id, &chat_id, &user, &conn).context("Message key for the file not found, have you received the message?")?
    });
    let mut output = File::create(&output_path)?;
    let mut source = File::open(input_path)?;
    decrypt_file(&info.nonce, &mut source, &mut output, &message_key).unwrap();
    open::that_in_background(&output_path);
    Ok(())
}
#[derive(serde::Serialize)]
pub struct EncryptedFile {
    file_info: FileInfo,
    path: PathBuf
}
#[tauri::command]
pub fn encrypt_file(input_path: PathBuf,  message_id: u32, chat_id: String,
        db_state: State<DatabaseState>, user_state: State<UserState>, chat_state: State<WrappedChatState>) -> CommandResult<EncryptedFile> {
    let message_key = with_state!(chat_state, user_state, db_state, |_chat, user, conn| {
        read_message_key(MessageKeyType::Sending, message_id, &chat_id, &user, &conn).context("Message key for the file not found, have you received the message?")?
    });
    let mut source = File::open(&input_path)?;
    // Is nonce reuse a problem with the double rachet?
    let output_path = std::env::temp_dir().join(input_path.file_name().unwrap());
    let mut output = File::create(&output_path)?;
    let nonce = get_nonce(message_id);
    encrypt(&nonce, &mut source, &mut output, &message_key).unwrap();
    Ok(EncryptedFile {
        file_info: FileInfo {
            filename: input_path.file_name().unwrap().to_str().unwrap().to_string(),
            size: source.metadata()?.len(),
            nonce: nonce,
        },
        path: output_path
    })
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{Write, Read}};

    use rand::RngCore;

    use crate::{encryption::get_rng, files::get_nonce};


    #[test]
    fn can_encrypt_and_decrypt_file() {
        let content = [79u8; 5124];
        let mut key: [u8;32] = [0u8; 32];
        get_rng().fill_bytes(&mut key);
        {
            let mut to_be_encrypted = File::create("/tmp/enchat-test.txt").unwrap();
            to_be_encrypted.write_all(&content).unwrap();
        }
        let nonce = get_nonce(2);
        {
            let mut to_be_encrypted = File::open("/tmp/enchat-test.txt").unwrap();
            let mut encrypted = File::create("/tmp/enchat-test-encrypted").unwrap();
            super::encrypt( &nonce, &mut to_be_encrypted, &mut encrypted, &key).unwrap();
        }
        {
            let mut encrypted = File::open("/tmp/enchat-test-encrypted").unwrap();
            let mut decrypted = File::create("/tmp/enchat-test-decrypted").unwrap();
            super::decrypt_file( &nonce,  &mut encrypted, &mut decrypted, &key).unwrap();
        }
        let mut decrypted = File::open("/tmp/enchat-test-decrypted").unwrap();
        let mut read = [0u8; 5124];
        decrypted.read_exact(&mut read).unwrap();
        assert_eq!(content, read);
    }
}