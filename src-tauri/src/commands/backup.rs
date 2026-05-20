use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::audit::{insert_audit_log, AuditLogPayload};
use crate::repo::backup::{insert_backup_log, list_backup_logs, BackupLogRecord};
use crate::service::backup::{copy_backup, restore_backup};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct RestoreBackupRequest {
    pub source_path: String,
}

#[derive(Debug, Serialize)]
pub struct BackupResponse {
    pub path: String,
    pub hash_sha256: String,
    pub size_bytes: u64,
}

#[tauri::command]
pub fn create_manual_backup_cmd(state: State<AppState>) -> AppResult<BackupResponse> {
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let destination = state
        .paths
        .backups_dir
        .join("manual")
        .join(format!("nawala-{}.db", timestamp));
    let result = copy_backup(&state.paths.db_path, &destination)?;
    let path = result.destination.to_string_lossy().to_string();

    let conn = state.db.lock();
    let backup_log_id = insert_backup_log(
        &conn,
        "manual",
        &path,
        result.size_bytes,
        &result.sha256,
        Some("Manual backup"),
    )?;
    insert_audit_log(
        &conn,
        AuditLogPayload {
            aksi: "create_backup".to_string(),
            entitas: Some("backup_log".to_string()),
            entitas_id: Some(backup_log_id.to_string()),
            ringkasan: Some("Manual backup dibuat".to_string()),
            metadata: Some(json!({ "path": path, "hash_sha256": result.sha256 })),
        },
    )?;

    Ok(BackupResponse {
        path,
        hash_sha256: result.sha256,
        size_bytes: result.size_bytes,
    })
}

#[tauri::command]
pub fn restore_backup_cmd(
    state: State<AppState>,
    request: RestoreBackupRequest,
) -> AppResult<BackupResponse> {
    let source = PathBuf::from(request.source_path);
    let pre_restore = restore_backup(
        &source,
        &state.paths.db_path,
        &state.paths.backups_dir.join("pre-restore"),
    )?;
    let path = pre_restore.destination.to_string_lossy().to_string();

    let conn = state.db.lock();
    let backup_log_id = insert_backup_log(
        &conn,
        "pre-restore",
        &path,
        pre_restore.size_bytes,
        &pre_restore.sha256,
        Some("Backup otomatis sebelum restore"),
    )?;
    insert_audit_log(
        &conn,
        AuditLogPayload {
            aksi: "restore_backup".to_string(),
            entitas: Some("backup_log".to_string()),
            entitas_id: Some(backup_log_id.to_string()),
            ringkasan: Some("Database dipulihkan dari backup".to_string()),
            metadata: Some(json!({ "source_path": source, "pre_restore_path": path })),
        },
    )?;

    Ok(BackupResponse {
        path,
        hash_sha256: pre_restore.sha256,
        size_bytes: pre_restore.size_bytes,
    })
}

#[tauri::command]
pub fn list_backup_logs_cmd(
    state: State<AppState>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> AppResult<Vec<BackupLogRecord>> {
    let conn = state.db.lock();
    list_backup_logs(&conn, limit.unwrap_or(50), offset.unwrap_or(0))
}
