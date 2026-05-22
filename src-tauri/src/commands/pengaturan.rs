use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::pengaturan::{self, PejabatRecord, PengaturanDesa};
use tauri::State;

#[tauri::command]
pub fn get_pengaturan_desa_cmd(state: State<AppState>) -> AppResult<Option<PengaturanDesa>> {
    let conn = state.db.lock();
    pengaturan::get_pengaturan_desa(&conn)
}

#[tauri::command]
pub fn save_pengaturan_desa_cmd(state: State<AppState>, data: PengaturanDesa) -> AppResult<()> {
    let conn = state.db.lock();
    pengaturan::save_pengaturan_desa(&conn, &data)
}

#[tauri::command]
pub fn list_pejabat_cmd(state: State<AppState>) -> AppResult<Vec<PejabatRecord>> {
    let conn = state.db.lock();
    pengaturan::list_pejabat(&conn)
}

#[tauri::command]
pub fn save_pejabat_cmd(state: State<AppState>, data: PejabatRecord) -> AppResult<i64> {
    let conn = state.db.lock();
    pengaturan::save_pejabat(&conn, &data)
}
