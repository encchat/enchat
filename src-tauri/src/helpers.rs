#[macro_export]
macro_rules! with_state {
    ($state:expr, $user_state:expr, $db_state:expr, |$chat:ident, $user:ident, $conn: ident| $body:block) => {{
        let $user = $user_state.0.lock().unwrap();
        let conn_mutex = $db_state.0.lock().unwrap();
        let $conn = conn_mutex.get_connection();
        let mut chat_wrapped = $state.0.lock().unwrap();
        if let Some($chat) = &mut *chat_wrapped {
            $body
        } else {
            todo!();
        }
    }};
}