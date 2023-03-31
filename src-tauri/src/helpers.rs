use crate::chat::ChatState;
use crate::encryption::PublicKey;
use crate::keybundle::{IdentityKey, ManagedKey};
use crate::user::User;
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

#[cfg(test)]
pub fn prepare_database() -> rusqlite::Connection {
    use rusqlite::Connection;

    use crate::store::make_migrations;

    let mut connection = Connection::open_in_memory().unwrap();
    make_migrations(&mut connection);
    connection
}

#[cfg(test)]
pub fn mock_alice_state(bob_keypair: PublicKey) -> (ChatState, User) {
    let psk = vec![0x02u8; 32];
    let initial_alice_chat = ChatState::new_sender(&bob_keypair, psk);
    (initial_alice_chat, User {user_id: Some("1".to_owned())})
}

#[cfg(test)]
pub fn mock_bob_state(bob_keypair: IdentityKey, alice_public: &PublicKey) -> (ChatState, User) {

    let psk = vec![0x02u8; 32];
    let initial_alice_chat = ChatState::new_receiver(bob_keypair.get_keypair().clone(), alice_public, psk);
    (initial_alice_chat, User {user_id: Some("2".to_owned())})
}
