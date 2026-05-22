use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendudukRecord {
    pub id: i64,
    pub nik: String,
    pub no_kk: Option<String>,
    pub nama_lengkap: String,
    pub jenis_kelamin: String,
    pub tempat_lahir: Option<String>,
    pub tanggal_lahir: Option<String>,
    pub agama: Option<String>,
    pub status_perkawinan: Option<String>,
    pub pekerjaan: Option<String>,
    pub alamat_lengkap: Option<String>,
    pub rt: Option<String>,
    pub rw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPendudukResult {
    pub items: Vec<PendudukRecord>,
    pub total: i64,
}

/// Search penduduk by NIK or nama (LIKE query)
pub fn search_penduduk(
    conn: &Connection,
    query: &str,
    limit: i32,
) -> AppResult<SearchPendudukResult> {
    let pattern = format!("%{}%", query);
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM penduduk WHERE nik LIKE ?1 OR nama_lengkap LIKE ?1",
        params![pattern],
        |row| row.get(0),
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, nik, no_kk, nama_lengkap, jenis_kelamin, tempat_lahir, tanggal_lahir,
                agama, status_perkawinan, pekerjaan, alamat_lengkap, rt, rw
         FROM penduduk
         WHERE nik LIKE ?1 OR nama_lengkap LIKE ?1
         ORDER BY nama_lengkap ASC
         LIMIT ?2",
    )?;

    let items = stmt
        .query_map(params![pattern, limit], |row| {
            Ok(PendudukRecord {
                id: row.get(0)?,
                nik: row.get(1)?,
                no_kk: row.get(2)?,
                nama_lengkap: row.get(3)?,
                jenis_kelamin: row.get(4)?,
                tempat_lahir: row.get(5)?,
                tanggal_lahir: row.get(6)?,
                agama: row.get(7)?,
                status_perkawinan: row.get(8)?,
                pekerjaan: row.get(9)?,
                alamat_lengkap: row.get(10)?,
                rt: row.get(11)?,
                rw: row.get(12)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SearchPendudukResult { items, total })
}

/// Get a single penduduk by NIK
pub fn get_penduduk_by_nik(conn: &Connection, nik: &str) -> AppResult<Option<PendudukRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, nik, no_kk, nama_lengkap, jenis_kelamin, tempat_lahir, tanggal_lahir,
                agama, status_perkawinan, pekerjaan, alamat_lengkap, rt, rw
         FROM penduduk WHERE nik = ?1",
    )?;

    let result = stmt
        .query_row(params![nik], |row| {
            Ok(PendudukRecord {
                id: row.get(0)?,
                nik: row.get(1)?,
                no_kk: row.get(2)?,
                nama_lengkap: row.get(3)?,
                jenis_kelamin: row.get(4)?,
                tempat_lahir: row.get(5)?,
                tanggal_lahir: row.get(6)?,
                agama: row.get(7)?,
                status_perkawinan: row.get(8)?,
                pekerjaan: row.get(9)?,
                alamat_lengkap: row.get(10)?,
                rt: row.get(11)?,
                rw: row.get(12)?,
            })
        })
        .ok();

    Ok(result)
}

/// Insert a penduduk record, upsert on NIK conflict
pub fn upsert_penduduk(
    conn: &Connection,
    nik: &str,
    no_kk: &str,
    nama_lengkap: &str,
    jenis_kelamin: &str,
    tempat_lahir: &str,
    tanggal_lahir: &str,
    agama: &str,
    status_perkawinan: &str,
    hubungan_keluarga: &str,
    pendidikan: &str,
    pekerjaan: &str,
    nama_ibu: &str,
    nama_ayah: &str,
    alamat: &str,
    rt: &str,
    rw: &str,
    keterangan: &str,
) -> AppResult<UpsertResult> {
    // Check if exists
    let existing: Option<i64> = conn
        .query_row("SELECT id FROM penduduk WHERE nik = ?1", params![nik], |row| {
            row.get(0)
        })
        .ok();

    if let Some(_id) = existing {
        conn.execute(
            "UPDATE penduduk SET no_kk = ?2, nama_lengkap = ?3, jenis_kelamin = ?4,
             tempat_lahir = ?5, tanggal_lahir = ?6, agama = ?7, status_perkawinan = ?8,
             hubungan_keluarga = ?9, pendidikan = ?10, pekerjaan = ?11, nama_ibu = ?12,
             nama_ayah = ?13, alamat_lengkap = ?14, rt = ?15, rw = ?16, keterangan = ?17,
             updated_at = datetime('now')
             WHERE nik = ?1",
            params![
                nik,
                no_kk,
                nama_lengkap,
                jenis_kelamin,
                tempat_lahir,
                tanggal_lahir,
                agama,
                status_perkawinan,
                hubungan_keluarga,
                pendidikan,
                pekerjaan,
                nama_ibu,
                nama_ayah,
                alamat,
                rt,
                rw,
                keterangan
            ],
        )?;
        Ok(UpsertResult::Updated)
    } else {
        conn.execute(
            "INSERT INTO penduduk (nik, no_kk, nama_lengkap, jenis_kelamin, tempat_lahir,
             tanggal_lahir, agama, status_perkawinan, hubungan_keluarga, pendidikan, pekerjaan,
             nama_ibu, nama_ayah, alamat_lengkap, rt, rw, keterangan)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                nik,
                no_kk,
                nama_lengkap,
                jenis_kelamin,
                tempat_lahir,
                tanggal_lahir,
                agama,
                status_perkawinan,
                hubungan_keluarga,
                pendidikan,
                pekerjaan,
                nama_ibu,
                nama_ayah,
                alamat,
                rt,
                rw,
                keterangan
            ],
        )?;
        Ok(UpsertResult::Inserted)
    }
}

/// Ensure KK record exists
pub fn ensure_kk(conn: &Connection, no_kk: &str, alamat: &str, rt: &str, rw: &str) -> AppResult<()> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM kk WHERE no_kk = ?1",
            params![no_kk],
            |row| row.get(0),
        )?;

    if !exists {
        conn.execute(
            "INSERT INTO kk (no_kk, alamat, rt, rw) VALUES (?1, ?2, ?3, ?4)",
            params![no_kk, alamat, rt, rw],
        )?;
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum UpsertResult {
    Inserted,
    Updated,
}
