use crate::app::AppState;
use crate::auth::argon::{hash_password, verify_password};
use crate::auth::lockout::next_lock_until;
use crate::error::{AppError, AppResult};
use crate::repo::auth as auth_repo;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub configured: bool,
    pub locked: bool,
    pub locked_until: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    pub success: bool,
    pub message: String,
    pub locked_until: Option<String>,
}

#[tauri::command]
pub fn check_auth_status_cmd(state: State<AppState>) -> AppResult<AuthStatus> {
    let conn = state.db.lock();
    let configured = auth_repo::is_auth_configured(&conn)?;

    if !configured {
        return Ok(AuthStatus {
            configured: false,
            locked: false,
            locked_until: None,
        });
    }

    let (_failed_attempts, locked_until) = auth_repo::get_lockout_status(&conn)?;
    let locked = if let Some(ref until) = locked_until {
        // Check if lock is still active
        if let Ok(lock_time) = chrono::DateTime::parse_from_rfc3339(until) {
            lock_time > Utc::now()
        } else {
            // Try parsing as SQLite datetime format
            chrono::NaiveDateTime::parse_from_str(until, "%Y-%m-%d %H:%M:%S")
                .map(|dt| dt.and_utc() > Utc::now())
                .unwrap_or(false)
        }
    } else {
        false
    };

    Ok(AuthStatus {
        configured,
        locked,
        locked_until: if locked { locked_until } else { None },
    })
}

#[tauri::command]
pub fn setup_password_cmd(
    state: State<AppState>,
    password: String,
    hint: Option<String>,
) -> AppResult<()> {
    if password.len() < 4 {
        return Err(AppError::Validation(
            "Password minimal 4 karakter".into(),
        ));
    }

    let password_hash = hash_password(&password)?;
    let conn = state.db.lock();
    auth_repo::setup_password(&conn, &password_hash, hint.as_deref())?;
    Ok(())
}

#[tauri::command]
pub fn login_cmd(state: State<AppState>, password: String) -> AppResult<LoginResult> {
    let conn = state.db.lock();

    // Check if locked
    let (_failed_attempts, locked_until) = auth_repo::get_lockout_status(&conn)?;
    if let Some(ref until) = locked_until {
        let is_locked = chrono::NaiveDateTime::parse_from_str(until, "%Y-%m-%d %H:%M:%S")
            .map(|dt| dt.and_utc() > Utc::now())
            .or_else(|_| {
                chrono::DateTime::parse_from_rfc3339(until).map(|dt| dt > Utc::now())
            })
            .unwrap_or(false);

        if is_locked {
            return Ok(LoginResult {
                success: false,
                message: format!("Akun terkunci hingga {}", until),
                locked_until: Some(until.clone()),
            });
        }
    }

    // Verify password
    let stored_hash = auth_repo::get_password_hash(&conn)?;
    let valid = verify_password(&password, &stored_hash)?;

    if valid {
        auth_repo::reset_failed_attempts(&conn)?;
        Ok(LoginResult {
            success: true,
            message: "Login berhasil".into(),
            locked_until: None,
        })
    } else {
        let new_attempts = auth_repo::record_failed_attempt(&conn)?;
        let now = Utc::now();
        if let Some(lock_time) = next_lock_until(new_attempts, now) {
            let lock_str = lock_time.format("%Y-%m-%d %H:%M:%S").to_string();
            auth_repo::set_locked_until(&conn, Some(&lock_str))?;
            Ok(LoginResult {
                success: false,
                message: format!(
                    "Password salah. {} percobaan gagal. Terkunci hingga {}",
                    new_attempts, lock_str
                ),
                locked_until: Some(lock_str),
            })
        } else {
            Ok(LoginResult {
                success: false,
                message: format!(
                    "Password salah. {} percobaan gagal.",
                    new_attempts
                ),
                locked_until: None,
            })
        }
    }
}
