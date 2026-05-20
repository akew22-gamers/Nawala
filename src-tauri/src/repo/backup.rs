use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BackupLogRecord {
    pub id: i64,
    pub ts: String,
    pub tipe: String,
    pub path: String,
    pub size_bytes: Option<i64>,
    pub hash_sha256: Option<String>,
    pub catatan: Option<String>,
}

pub fn insert_backup_log(
    conn: &Connection,
    tipe: &str,
    path: &str,
    size_bytes: u64,
    hash_sha256: &str,
    catatan: Option<&str>,
) -> AppResult<i64> {
    conn.execute(
        "INSERT INTO backup_log (tipe, path, size_bytes, hash_sha256, catatan) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![tipe, path, size_bytes as i64, hash_sha256, catatan],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn list_backup_logs(
    conn: &Connection,
    limit: i32,
    offset: i32,
) -> AppResult<Vec<BackupLogRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, ts, tipe, path, size_bytes, hash_sha256, catatan
         FROM backup_log
         ORDER BY ts DESC, id DESC
         LIMIT ?1 OFFSET ?2",
    )?;

    let rows = stmt.query_map(params![limit, offset], |row| {
        Ok(BackupLogRecord {
            id: row.get(0)?,
            ts: row.get(1)?,
            tipe: row.get(2)?,
            path: row.get(3)?,
            size_bytes: row.get(4)?,
            hash_sha256: row.get(5)?,
            catatan: row.get(6)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }

    Ok(records)
}
