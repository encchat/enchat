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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key BLOB NOT NULL
        );
        "),
        M::up("
        CREATE TABLE message_key(
            id INTEGER PRIMARY KEY NOT NULL,
            chat_id TEXT NOT NULL,
            key BLOB NOT NULL,
            received BOOL DEFAULT FALSE NOT NULL
        );
        "),
        M::up("ALTER TABLE identity ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX identity_user ON identity(user_id);"),
        M::up("ALTER TABLE signed ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX signed_user ON signed(user_id);"),
        M::up("ALTER TABLE onetime ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX onetime_user ON onetime(user_id);"),
        M::up("ALTER TABLE message_key ADD COLUMN user_id TEXT NOT NULL; CREATE INDEX message_key_user ON message_key(user_id);"),
        M::up("ALTER TABLE message_key ADD COLUMN local_id INTEGER NOT NULL; CREATE INDEX message_key_local_id ON message_key(local_id);"),
        M::up("CREATE INDEX message_key_chat_index ON message_key(chat_id)")
    ]);
    migration.to_latest(connection).unwrap();
}