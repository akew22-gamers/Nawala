use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::penduduk::{ensure_kk, upsert_penduduk, UpsertResult};
use crate::service::import_warga::parse_buku_induk_csv;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub total_rows: usize,
    pub inserted_rows: usize,
    pub updated_rows: usize,
    pub skipped_rows: usize,
    pub error_rows: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
pub fn import_warga_csv_cmd(
    state: State<AppState>,
    csv_content: String,
    filename: String,
) -> AppResult<ImportResult> {
    let rows = parse_buku_induk_csv(&csv_content)?;
    let conn = state.db.lock();

    let mut result = ImportResult {
        total_rows: rows.len(),
        inserted_rows: 0,
        updated_rows: 0,
        skipped_rows: 0,
        error_rows: 0,
        errors: Vec::new(),
    };

    for (i, row) in rows.iter().enumerate() {
        // Ensure KK exists
        if let Err(e) = ensure_kk(&conn, &row.no_kk, &row.alamat, &row.rt, &row.rw) {
            result.error_rows += 1;
            result.errors.push(format!("Baris {}: KK error: {}", i + 1, e));
            continue;
        }

        // Upsert penduduk
        match upsert_penduduk(
            &conn,
            &row.nik,
            &row.no_kk,
            &row.nama_lengkap,
            &row.jenis_kelamin,
            &row.tempat_lahir,
            &row.tanggal_lahir,
            &row.agama,
            &row.status_perkawinan,
            &row.hubungan_keluarga,
            &row.pendidikan,
            &row.pekerjaan,
            &row.nama_ibu,
            &row.nama_ayah,
            &row.alamat,
            &row.rt,
            &row.rw,
            &row.keterangan,
        ) {
            Ok(UpsertResult::Inserted) => result.inserted_rows += 1,
            Ok(UpsertResult::Updated) => result.updated_rows += 1,
            Err(e) => {
                result.error_rows += 1;
                result.errors.push(format!("Baris {}: {}", i + 1, e));
            }
        }
    }

    // Log the import
    conn.execute(
        "INSERT INTO import_log (mode, filename, total_rows, inserted_rows, updated_rows, skipped_rows, error_rows, status)
         VALUES ('csv', ?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            filename,
            result.total_rows as i64,
            result.inserted_rows as i64,
            result.updated_rows as i64,
            result.skipped_rows as i64,
            result.error_rows as i64,
            if result.error_rows == result.total_rows { "failed" } else { "completed" }
        ],
    )?;

    Ok(result)
}
