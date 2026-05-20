pub mod pragma;

mod embedded {
    refinery::embed_migrations!("migrations");
}

use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error::AppResult;

/// Database wrapper with thread-safe connection
#[derive(Clone)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    /// Open or create a database at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open SQLite connection
        let mut conn = Connection::open(path)?;

        // Apply pragmas
        pragma::apply(&conn)?;

        // Ensure runtime databases have the schema expected by commands.
        embedded::migrations::runner()
            .run(&mut conn)
            .map_err(|err| crate::error::AppError::Internal(err.to_string()))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get a lock on the connection for executing queries
    pub fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}
