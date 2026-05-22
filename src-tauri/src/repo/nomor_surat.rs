use crate::error::AppResult;
use crate::repo::pengaturan;
use crate::service::nomor_surat::ROMAWI;
use chrono::Datelike;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NomorSuratResult {
    pub nomor: String,
    pub seq: i64,
}

/// Get next nomor surat for a given formulir kode, auto-incrementing the counter.
pub fn get_next_nomor_surat(conn: &Connection, kode_formulir: &str) -> AppResult<NomorSuratResult> {
    let now = chrono::Local::now();
    let tahun = now.year();
    let bulan = now.month() as usize;

    // Get or create counter
    let existing: Option<i64> = conn
        .query_row(
            "SELECT next_seq FROM nomor_surat_counter WHERE format_kode = ?1 AND tahun = ?2",
            params![kode_formulir, tahun],
            |row| row.get(0),
        )
        .ok();

    let seq = if let Some(next_seq) = existing {
        conn.execute(
            "UPDATE nomor_surat_counter SET next_seq = next_seq + 1 WHERE format_kode = ?1 AND tahun = ?2",
            params![kode_formulir, tahun],
        )?;
        next_seq
    } else {
        conn.execute(
            "INSERT INTO nomor_surat_counter (format_kode, tahun, format_pola, next_seq)
             VALUES (?1, ?2, ?3, 2)",
            params![
                kode_formulir,
                tahun,
                "{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}"
            ],
        )?;
        1
    };

    // Get kode_desa from pengaturan
    let kode_desa = pengaturan::get_pengaturan_desa(conn)?
        .and_then(|d| d.kode_desa)
        .unwrap_or_else(|| "000".to_string());

    // Format nomor surat
    let nomor = format!(
        "{:04}/{}/{}/{}/{}",
        seq,
        kode_formulir,
        kode_desa,
        ROMAWI[bulan],
        tahun
    );

    Ok(NomorSuratResult { nomor, seq })
}
