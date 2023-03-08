#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::{keybundle::{request_onetime_keys, request_prekey, request_identity_key}, chat::{enter_chat}};

mod encryption;
mod keybundle;
mod chat;
mod message;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


use chat::WrappedChatState;
use dotenv;

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
        .invoke_handler(tauri::generate_handler![greet, request_onetime_keys, request_identity_key, request_prekey, enter_chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
