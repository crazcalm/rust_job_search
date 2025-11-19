use refinery::{Migration, embed_migrations};
use rusqlite::Connection;

pub enum ConnectionType {
    Path(String),
    InMemory,
}

pub fn get_connection(conn_type: ConnectionType) -> Result<Connection, rusqlite::Error> {
    match conn_type {
        ConnectionType::Path(path) => Ok(Connection::open(path)?),
        ConnectionType::InMemory => Ok(Connection::open_in_memory()?),
    }
}

pub fn run_migrations(conn: &mut Connection) -> Result<(), refinery::Error> {
    refinery::embed_migrations!("migrations");
    migrations::runner().run(conn)?;
    Ok(())
}
