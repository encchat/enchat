use crate::{keybundle::{IdentityKey, StoredKey, Onetime, Prekey, ManagedKey, SignedKey}, encryption::{generate_ephemeral, PublicKey, Otherkey}};

use super::{ChatState, DHRachet};

#[test]
fn bob_alice_should_share_same_initial_key() {
    let alice_id = IdentityKey::generate();
    let alice_ephemeral = IdentityKey::generate();
    let bob_onetime = Onetime::generate();
    let bob_id = IdentityKey::generate();
    let bob_prekey = SignedKey::generate();
    let alice_dh = ChatState::dh_sender(alice_ephemeral.get_keypair(), &alice_id.get_keypair(), &bob_id.get_public_key(), &bob_prekey.get_public_key(), Some(bob_onetime.get_public_key()));
    let bob_dh = ChatState::dh_receiver(&alice_ephemeral.get_public_key(), bob_id.get_keypair(), &alice_id.get_public_key(), bob_prekey.get_keypair(), Some(bob_onetime));
    assert_eq!(alice_dh, bob_dh);
}

#[test]
fn bob_should_be_able_to_receive_alice() {
    let psk = vec![0x02u8; 32];
    let bob_keypair = IdentityKey::generate();
    let mut initial_alice_chat = ChatState::new_sender(&bob_keypair.get_public_key(), psk.clone());
    let (alice_rachet_key, alice_message_key, _) = initial_alice_chat.move_sender();
    let mut initial_bob_chat = ChatState::new_receiver(bob_keypair.get_keypair().clone(), &alice_rachet_key, psk);
    let bob_message_key = initial_bob_chat.move_receiver(alice_rachet_key);
    assert_eq!(alice_message_key, bob_message_key);
}
#[test]
fn bob_should_be_able_to_receive_alice_multiple() {
    let psk = vec![0x02u8; 32];
    let bob_keypair = IdentityKey::generate();
    let mut initial_alice_chat = ChatState::new_sender(&bob_keypair.get_public_key(), psk.clone());
    let mut vec: Vec<(PublicKey, Otherkey)> = Vec::new();
    for _ in 0..4 {
        let (alice_rachet_key, alice_message_key, _) = initial_alice_chat.move_sender();
        vec.push((alice_rachet_key, alice_message_key));
    }
    let mut initial_bob_chat = ChatState::new_receiver(bob_keypair.get_keypair().clone(), &vec[0].0, psk.clone());
    for message in vec {
        let bob_message_key = initial_bob_chat.move_receiver(message.0);
        assert_eq!(message.1, bob_message_key);
        debug!("Valid");
    }
}
#[test]
fn bob_alice_should_pingpong_multiple_times() {
    let psk = vec![0x02u8; 32];
    let bob_keypair = IdentityKey::generate();
    let mut initial_alice_chat = ChatState::new_sender(&bob_keypair.get_public_key(), psk.clone());
    let mut initial_bob_chat: Option<ChatState> = None;
    for x in 0..5 {
        let (alice_rachet_key, alice_message_key, _) = initial_alice_chat.move_sender();
        if x == 0 {
            initial_bob_chat = Some(ChatState::new_receiver(bob_keypair.get_keypair().clone(), &alice_rachet_key, psk.clone()));
        }
        assert_eq!(initial_bob_chat.as_mut().unwrap().move_receiver(alice_rachet_key), alice_message_key);
        let (bob_rachet_key, bob_message_key, _) = initial_bob_chat.as_mut().unwrap().move_sender();
        assert_eq!(initial_alice_chat.move_receiver(bob_rachet_key), bob_message_key);
    }
}