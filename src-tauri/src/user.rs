use std::sync::Mutex;

use tauri::State;

use crate::chat::WrappedChatState;

#[derive(Default)]
pub struct User {
    pub user_id: Option<String>
}

pub struct UserState(pub Mutex<User>);

#[tauri::command]
pub fn login(user_id: String, user_state: State<UserState>, chat_state: State<WrappedChatState>) {
    let mut chat = chat_state.0.lock().unwrap();
    let mut user = user_state.0.lock().unwrap();
    *chat = Default::default();
    (*user).user_id = Some(user_id);
}