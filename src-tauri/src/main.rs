#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::{user_setup::generate_keys, auth::{login, get_refresh_token, set_refresh_token, logout}, keybundle::request_onetime_keys};

mod encryption;
mod user_setup;
mod keybundle;
mod auth;

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
        .invoke_handler(tauri::generate_handler![greet, generate_keys, login, get_refresh_token, set_refresh_token, logout, request_onetime_keys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
