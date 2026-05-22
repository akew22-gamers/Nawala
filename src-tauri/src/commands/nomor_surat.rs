use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::nomor_surat::{get_next_nomor_surat, NomorSuratResult};
use tauri::State;

#[tauri::command]
pub fn get_next_nomor_surat_cmd(
    state: State<AppState>,
    kode_formulir: String,
) -> AppResult<NomorSuratResult> {
    let conn = state.db.lock();
    get_next_nomor_surat(&conn, &kode_formulir)
}
