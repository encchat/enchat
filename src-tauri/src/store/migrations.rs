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
    ]);
    migration.to_latest(connection).unwrap();
}