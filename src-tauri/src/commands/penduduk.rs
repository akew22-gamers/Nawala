use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::penduduk::{self, PendudukRecord, SearchPendudukResult};
use tauri::State;

#[tauri::command]
pub fn search_penduduk_cmd(
    state: State<AppState>,
    query: String,
    limit: Option<i32>,
) -> AppResult<SearchPendudukResult> {
    let conn = state.db.lock();
    penduduk::search_penduduk(&conn, &query, limit.unwrap_or(20))
}

#[tauri::command]
pub fn get_penduduk_by_nik_cmd(
    state: State<AppState>,
    nik: String,
) -> AppResult<Option<PendudukRecord>> {
    let conn = state.db.lock();
    penduduk::get_penduduk_by_nik(&conn, &nik)
}
