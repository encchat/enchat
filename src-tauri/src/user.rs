use std::sync::Mutex;

use tauri::State;

#[derive(Default)]
pub struct User {
    pub user_id: Option<String>
}

pub struct UserState(pub Mutex<User>);

#[tauri::command]
pub fn login(user_id: String, user_state: State<UserState>) {
    let mut user = user_state.0.lock().unwrap();
    (*user).user_id = Some(user_id);
}