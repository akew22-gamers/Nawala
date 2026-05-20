use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::audit::{list_audit_logs, AuditLogRecord};
use tauri::State;

#[tauri::command]
pub fn list_audit_logs_cmd(
    state: State<AppState>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> AppResult<Vec<AuditLogRecord>> {
    let conn = state.db.lock();
    list_audit_logs(&conn, limit.unwrap_or(50), offset.unwrap_or(0))
}
