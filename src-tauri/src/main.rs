#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::{keybundle::{request_onetime_keys, request_prekey, request_identity_key}, chat::{enter_chat, reenter_chat}, message::{send, receive, try_decrypt}, user::login, files::{decrypt_and_open, encrypt_file}};

mod encryption;
mod keybundle;
mod chat;
mod message;
mod store;
mod user;
mod files;
mod errors;
#[macro_use]
mod helpers;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


use chat::WrappedChatState;
use dotenv;
use store::DatabaseState;
use user::UserState;

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .manage(WrappedChatState(Default::default()))
        .manage(DatabaseState(Default::default()))
        .manage(UserState(Default::default()))
        .invoke_handler(tauri::generate_handler![request_onetime_keys, request_identity_key, request_prekey, enter_chat, reenter_chat, receive, send, login, try_decrypt, decrypt_and_open, encrypt_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
