use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogPayload {
    pub aksi: String,
    pub entitas: Option<String>,
    pub entitas_id: Option<String>,
    pub ringkasan: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogRecord {
    pub id: i64,
    pub ts: String,
    pub aksi: String,
    pub entitas: Option<String>,
    pub entitas_id: Option<String>,
    pub ringkasan: Option<String>,
    pub metadata: Option<String>,
}

pub fn insert_audit_log(conn: &Connection, payload: AuditLogPayload) -> AppResult<i64> {
    let metadata = payload.metadata.map(|value| value.to_string());

    conn.execute(
        "INSERT INTO audit_log (aksi, entitas, entitas_id, ringkasan, metadata) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            payload.aksi,
            payload.entitas,
            payload.entitas_id,
            payload.ringkasan,
            metadata,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn list_audit_logs(
    conn: &Connection,
    limit: i32,
    offset: i32,
) -> AppResult<Vec<AuditLogRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, ts, aksi, entitas, entitas_id, ringkasan, metadata
         FROM audit_log
         ORDER BY ts DESC, id DESC
         LIMIT ?1 OFFSET ?2",
    )?;

    let rows = stmt.query_map(params![limit, offset], |row| {
        Ok(AuditLogRecord {
            id: row.get(0)?,
            ts: row.get(1)?,
            aksi: row.get(2)?,
            entitas: row.get(3)?,
            entitas_id: row.get(4)?,
            ringkasan: row.get(5)?,
            metadata: row.get(6)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }

    Ok(records)
}
