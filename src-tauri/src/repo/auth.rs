use crate::error::{AppError, AppResult};
use rusqlite::Connection;

/// Check if auth has been set up (password exists)
pub fn is_auth_configured(conn: &Connection) -> AppResult<bool> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM auth WHERE id = 1", [], |row| {
        row.get(0)
    })?;
    Ok(count > 0)
}

/// Store password hash during onboarding
pub fn setup_password(conn: &Connection, password_hash: &str, hint: Option<&str>) -> AppResult<()> {
    // Only allow setup if not already configured
    if is_auth_configured(conn)? {
        return Err(AppError::Conflict(
            "Password sudah dikonfigurasi. Gunakan fitur ubah password.".into(),
        ));
    }
    conn.execute(
        "INSERT INTO auth (id, password_hash, hint) VALUES (1, ?1, ?2)",
        rusqlite::params![password_hash, hint],
    )?;
    Ok(())
}

/// Get stored password hash for verification
pub fn get_password_hash(conn: &Connection) -> AppResult<String> {
    conn.query_row("SELECT password_hash FROM auth WHERE id = 1", [], |row| {
        row.get(0)
    })
    .map_err(|_| AppError::NotFound("Password belum dikonfigurasi".into()))
}

/// Record a failed login attempt
pub fn record_failed_attempt(conn: &Connection) -> AppResult<i64> {
    conn.execute(
        "UPDATE auth SET failed_attempts = failed_attempts + 1, updated_at = datetime('now') WHERE id = 1",
        [],
    )?;
    let attempts: i64 = conn.query_row(
        "SELECT failed_attempts FROM auth WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(attempts)
}

/// Set lockout time
pub fn set_locked_until(conn: &Connection, locked_until: Option<&str>) -> AppResult<()> {
    conn.execute(
        "UPDATE auth SET locked_until = ?1, updated_at = datetime('now') WHERE id = 1",
        rusqlite::params![locked_until],
    )?;
    Ok(())
}

/// Reset failed attempts on successful login
pub fn reset_failed_attempts(conn: &Connection) -> AppResult<()> {
    conn.execute(
        "UPDATE auth SET failed_attempts = 0, locked_until = NULL, updated_at = datetime('now') WHERE id = 1",
        [],
    )?;
    Ok(())
}

/// Get current lockout status
pub fn get_lockout_status(conn: &Connection) -> AppResult<(i64, Option<String>)> {
    conn.query_row(
        "SELECT failed_attempts, locked_until FROM auth WHERE id = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )
    .map_err(|_| AppError::NotFound("Auth belum dikonfigurasi".into()))
}
