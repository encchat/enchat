#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::{keybundle::{request_onetime_keys, request_prekey, request_identity_key}, encryption::calculate_psk};

mod encryption;
mod keybundle;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


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
        .invoke_handler(tauri::generate_handler![greet, request_onetime_keys, request_identity_key, request_prekey, calculate_psk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
