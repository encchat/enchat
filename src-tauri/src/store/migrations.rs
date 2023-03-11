use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};


pub fn make_migrations(connection: &mut Connection) {
    let migration = Migrations::new(vec![
        M::up("
        CREATE TABLE identity(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key BLOB NOT NULL
        );"),
        M::up("
        CREATE TABLE signed(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key BLOB NOT NULL
        );
        "),
        M::up("
        CREATE TABLE onetime(
            id INTEGER NOT NULL,
            user_id TEXT NOT NULL,
            key BLOB NOT NULL,
            PRIMARY KEY (id, user_id)
        );
        "),
        M::up("
        CREATE TABLE message_key(
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            chat_id TEXT NOT NULL,
            key BLOB NOT NULL,
            received BOOL DEFAULT FALSE NOT NULL
        );
        "),
        M::up("ALTER TABLE identity ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX identity_user ON identity(user_id);"),
        M::up("ALTER TABLE signed ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX signed_user ON signed(user_id);"),
        M::up("ALTER TABLE message_key ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX message_key_user ON message_key(user_id);"),
        M::up("ALTER TABLE message_key ADD COLUMN local_id INTEGER NOT NULL; CREATE INDEX message_key_local_id ON message_key(local_id);"),
        M::up("CREATE INDEX message_key_chat_index ON message_key(chat_id)"),
        M::up("CREATE TABLE rachet_state(
            chat_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            diffie_public_bytes BLOB NOT NULL,
            diffie_private_bytes BLOB NOT NULL,
            root_input_bytes BLOB NOT NULL,
            root_id INTEGER NOT NULL,
            sender_input_bytes BLOB NOT NULL,
            sender_id INTEGER NOT NULL,
            receiver_input_bytes BLOB NOT NULL,
            receiver_id INTEGER NOT NULL,
            PRIMARY KEY (chat_id, user_id)
        );")
    ]);
    migration.to_latest(connection).unwrap();
}