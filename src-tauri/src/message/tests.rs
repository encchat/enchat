use std::sync::Mutex;

use tauri::State;

use crate::{chat::{ChatState, WrappedChatState}, keybundle::{IdentityKey, StoredKey, ManagedKey}, message::{send, send_inner, receive_inner, Message}, encryption::PublicKey, user::User, helpers::prepare_database};

fn mock_alice_state(bob_keypair: PublicKey) -> (ChatState, User) {
    let psk = vec![0x02u8; 32];
    let mut initial_alice_chat = ChatState::new_sender(&bob_keypair, psk);
    (initial_alice_chat, User {user_id: Some("1".to_owned())})
}

fn mock_bob_state(bob_keypair: IdentityKey, alice_public: &PublicKey) -> (ChatState, User) {

    let psk = vec![0x02u8; 32];
    let mut initial_alice_chat = ChatState::new_receiver(bob_keypair.get_keypair().clone(), alice_public, psk);
    (initial_alice_chat, User {user_id: Some("2".to_owned())})
}

#[test]
fn bob_should_decrypt_alice_sent_message() {
    let db = prepare_database();
    let bob = IdentityKey::generate();
    let (mut alice_state, alice_user) = mock_alice_state(bob.get_public_key());
    let message = send_inner("1".to_owned(), "test".to_owned(), &mut alice_state, &alice_user, &db); 
    assert!(message.is_some());
    let (mut bob_state, bob_user) = mock_bob_state(bob, &message.as_ref().unwrap().header.rachet_key);
    let decrypted = receive_inner("1".to_owned(), message.unwrap(), &mut bob_state, &bob_user, &db);
    assert!(decrypted.is_some());
    assert!(decrypted.unwrap() == "test".as_bytes());
}
#[test]
fn bob_should_decrypt_alice_sent_multiple_message() {
    let db = prepare_database();
    let bob = IdentityKey::generate();
    let (mut alice_state, alice_user) = mock_alice_state(bob.get_public_key());
    let messages: Vec<Message> = (0..10).map(|_| {
        send_inner("1".to_owned(), "test".to_owned(), &mut alice_state, &alice_user, &db).unwrap()
    }).collect();
    let (mut bob_state, bob_user) = mock_bob_state(bob, &messages[0].header.rachet_key);
    for message in messages {
        let decrypted = receive_inner("1".to_owned(), message, &mut bob_state, &bob_user, &db);
        assert!(decrypted.is_some());
        assert!(decrypted.unwrap() == "test".as_bytes());
    }
}

#[test]
fn bob_should_decrypt_alice_sent_messages_out_of_order() {
    let db = prepare_database();
    let bob = IdentityKey::generate();
    let (mut alice_state, alice_user) = mock_alice_state(bob.get_public_key());
    let messages: Vec<Message> = (0..10).map(|_| {
        send_inner("1".to_owned(), "test".to_owned(), &mut alice_state, &alice_user, &db).unwrap()
    }).collect();
    let (mut bob_state, bob_user) = mock_bob_state(bob, &messages[0].header.rachet_key);
    let decrypted = receive_inner("1".to_owned(), messages[9].clone(), &mut bob_state, &bob_user, &db);
    assert!(decrypted.is_some());
    assert!(decrypted.unwrap() == "test".as_bytes());
}
