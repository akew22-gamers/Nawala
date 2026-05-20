use rusqlite::Connection;

use crate::error::AppResult;

/// Apply SQLite pragmas for optimal performance and safety
pub fn apply_pragmas(conn: &Connection) -> AppResult<()> {
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        PRAGMA synchronous = NORMAL;
        ",
    )?;
    Ok(())
}
