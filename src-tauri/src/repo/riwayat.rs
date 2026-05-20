/**
 * Repository module for riwayat_formulir operations
 */
use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RiwayatSubjek {
    pub nik: String,
    pub peran: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitRiwayatPayload {
    pub kode_formulir: String,
    pub versi_template: i32,
    pub nomor_surat: Option<String>,
    pub tanggal_terbit: String,
    pub pejabat_id: Option<i32>,
    pub pejabat_snapshot: String,
    pub data_snapshot: String,
    pub template_snapshot: String,
    pub pdf_path: Option<String>,
    pub hash_dokumen: Option<String>,
    pub catatan: Option<String>,
    pub dibuat_oleh: String,
    pub subjek: Vec<RiwayatSubjek>,
}

#[derive(Debug, Serialize)]
pub struct CommitRiwayatResponse {
    pub riwayat_id: i64,
    pub nomor_surat: Option<String>,
}

/// Commit form submission to immutable history
pub fn commit_riwayat_formulir(
    conn: &mut Connection,
    payload: CommitRiwayatPayload,
) -> AppResult<CommitRiwayatResponse> {
    let tx = conn.transaction()?;

    // Insert into riwayat_formulir
    tx.execute(
        "INSERT INTO riwayat_formulir (
            kode_formulir,
            versi_template,
            nomor_surat,
            tanggal_terbit,
            pejabat_id,
            pejabat_snapshot,
            data_snapshot,
            template_snapshot,
            pdf_path,
            hash_dokumen,
            catatan,
            dibuat_oleh
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            payload.kode_formulir,
            payload.versi_template,
            payload.nomor_surat,
            payload.tanggal_terbit,
            payload.pejabat_id,
            payload.pejabat_snapshot,
            payload.data_snapshot,
            payload.template_snapshot,
            payload.pdf_path,
            payload.hash_dokumen,
            payload.catatan,
            payload.dibuat_oleh,
        ],
    )?;

    let riwayat_id = tx.last_insert_rowid();

    // Insert subjects
    for subjek in &payload.subjek {
        tx.execute(
            "INSERT INTO riwayat_subjek (riwayat_id, nik, peran) VALUES (?1, ?2, ?3)",
            params![riwayat_id, subjek.nik, subjek.peran],
        )?;
    }

    // Insert audit log
    let ringkasan = format!(
        "Formulir {} dibuat oleh {}",
        payload.kode_formulir, payload.dibuat_oleh
    );
    tx.execute(
        "INSERT INTO audit_log (aksi, entitas, entitas_id, ringkasan) VALUES (?1, ?2, ?3, ?4)",
        params!["create_formulir", "riwayat_formulir", riwayat_id, ringkasan],
    )?;

    tx.commit()?;

    Ok(CommitRiwayatResponse {
        riwayat_id,
        nomor_surat: payload.nomor_surat,
    })
}

#[derive(Debug, Serialize)]
pub struct RiwayatRecord {
    pub id: i64,
    pub kode_formulir: String,
    pub nomor_surat: Option<String>,
    pub tanggal_terbit: String,
    pub dibuat_oleh: String,
    pub created_at: String,
}

/// List riwayat records with pagination
pub fn list_riwayat(
    conn: &Connection,
    kode_formulir: Option<String>,
    limit: i32,
    offset: i32,
) -> AppResult<Vec<RiwayatRecord>> {
    let (sql, params_vec): (&str, Vec<Box<dyn rusqlite::ToSql>>) =
        if let Some(ref kode) = kode_formulir {
            (
                "SELECT id, kode_formulir, nomor_surat, tanggal_terbit, dibuat_oleh, created_at
             FROM riwayat_formulir
             WHERE kode_formulir = ?1
             ORDER BY created_at DESC
             LIMIT ?2 OFFSET ?3",
                vec![Box::new(kode.clone()), Box::new(limit), Box::new(offset)],
            )
        } else {
            (
                "SELECT id, kode_formulir, nomor_surat, tanggal_terbit, dibuat_oleh, created_at
             FROM riwayat_formulir
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2",
                vec![Box::new(limit), Box::new(offset)],
            )
        };

    let mut stmt = conn.prepare(sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(RiwayatRecord {
            id: row.get(0)?,
            kode_formulir: row.get(1)?,
            nomor_surat: row.get(2)?,
            tanggal_terbit: row.get(3)?,
            dibuat_oleh: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }

    Ok(records)
}

/// Get single riwayat record by ID
pub fn get_riwayat_by_id(conn: &Connection, id: i64) -> AppResult<Option<RiwayatRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, kode_formulir, nomor_surat, tanggal_terbit, dibuat_oleh, created_at
         FROM riwayat_formulir
         WHERE id = ?1",
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(RiwayatRecord {
            id: row.get(0)?,
            kode_formulir: row.get(1)?,
            nomor_surat: row.get(2)?,
            tanggal_terbit: row.get(3)?,
            dibuat_oleh: row.get(4)?,
            created_at: row.get(5)?,
        })
    });

    match result {
        Ok(record) => Ok(Some(record)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        // Create minimal schema for testing
        conn.execute(
            "CREATE TABLE riwayat_formulir (
                id INTEGER PRIMARY KEY,
                kode_formulir TEXT NOT NULL,
                versi_template INTEGER NOT NULL,
                nomor_surat TEXT,
                tanggal_terbit TEXT NOT NULL,
                pejabat_id INTEGER,
                pejabat_snapshot TEXT NOT NULL,
                data_snapshot TEXT NOT NULL,
                template_snapshot TEXT NOT NULL,
                pdf_path TEXT,
                hash_dokumen TEXT,
                catatan TEXT,
                dibuat_oleh TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE riwayat_subjek (
                riwayat_id INTEGER NOT NULL,
                nik TEXT NOT NULL,
                peran TEXT NOT NULL,
                PRIMARY KEY (riwayat_id, nik, peran)
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE audit_log (
                id INTEGER PRIMARY KEY,
                ts TEXT NOT NULL DEFAULT (datetime('now')),
                aksi TEXT NOT NULL,
                entitas TEXT,
                entitas_id TEXT,
                ringkasan TEXT,
                metadata TEXT
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_commit_riwayat_formulir() {
        let mut conn = setup_test_db();

        let payload = CommitRiwayatPayload {
            kode_formulir: "SKCK".to_string(),
            versi_template: 1,
            nomor_surat: Some("001/SKCK/2026".to_string()),
            tanggal_terbit: "2026-05-20".to_string(),
            pejabat_id: None,
            pejabat_snapshot: r#"{"nama":"Kepala Desa"}"#.to_string(),
            data_snapshot: r#"{"keperluan":"Test"}"#.to_string(),
            template_snapshot: r#"{"kode":"SKCK"}"#.to_string(),
            pdf_path: None,
            hash_dokumen: None,
            catatan: None,
            dibuat_oleh: "admin".to_string(),
            subjek: vec![RiwayatSubjek {
                nik: "1234567890123456".to_string(),
                peran: "pemohon".to_string(),
            }],
        };

        let result = commit_riwayat_formulir(&mut conn, payload).unwrap();
        assert_eq!(result.riwayat_id, 1);
        assert_eq!(result.nomor_surat, Some("001/SKCK/2026".to_string()));
    }

    #[test]
    fn test_list_riwayat() {
        let mut conn = setup_test_db();

        // Insert test data
        let payload = CommitRiwayatPayload {
            kode_formulir: "SKCK".to_string(),
            versi_template: 1,
            nomor_surat: None,
            tanggal_terbit: "2026-05-20".to_string(),
            pejabat_id: None,
            pejabat_snapshot: "{}".to_string(),
            data_snapshot: "{}".to_string(),
            template_snapshot: "{}".to_string(),
            pdf_path: None,
            hash_dokumen: None,
            catatan: None,
            dibuat_oleh: "admin".to_string(),
            subjek: vec![],
        };

        commit_riwayat_formulir(&mut conn, payload).unwrap();

        let records = list_riwayat(&conn, None, 10, 0).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kode_formulir, "SKCK");
    }
}
