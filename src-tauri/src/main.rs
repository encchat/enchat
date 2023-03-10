#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::{keybundle::{request_onetime_keys, request_prekey, request_identity_key}, chat::{enter_chat}, message::{send, receive}, user::login};

mod encryption;
mod keybundle;
mod chat;
mod message;
mod store;
mod user;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


use chat::WrappedChatState;
use dotenv;
use store::DatabaseState;
use user::UserState;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .manage(WrappedChatState(Default::default()))
        .manage(DatabaseState(Default::default()))
        .manage(UserState(Default::default()))
        .invoke_handler(tauri::generate_handler![greet, request_onetime_keys, request_identity_key, request_prekey, enter_chat, receive, send, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
