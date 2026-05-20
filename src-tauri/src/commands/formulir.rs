/**
 * Tauri commands for formulir operations
 */
use crate::app::AppState;
use crate::error::AppResult;
use crate::repo::riwayat::{
    commit_riwayat_formulir, get_riwayat_by_id, list_riwayat, update_riwayat_pdf_metadata,
    CommitRiwayatPayload, CommitRiwayatResponse, RiwayatRecord,
};
use crate::service::pdf::{export_pdf, ExportPdfRequest, ExportPdfResponse};
use tauri::State;

#[tauri::command]
pub fn commit_riwayat_formulir_cmd(
    state: State<AppState>,
    payload: CommitRiwayatPayload,
) -> AppResult<CommitRiwayatResponse> {
    let mut conn = state.db.lock();
    commit_riwayat_formulir(&mut conn, payload)
}

#[tauri::command]
pub fn list_riwayat_cmd(
    state: State<AppState>,
    kode_formulir: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> AppResult<Vec<RiwayatRecord>> {
    let conn = state.db.lock();
    list_riwayat(
        &conn,
        kode_formulir,
        limit.unwrap_or(50),
        offset.unwrap_or(0),
    )
}

#[tauri::command]
pub fn get_riwayat_by_id_cmd(state: State<AppState>, id: i64) -> AppResult<Option<RiwayatRecord>> {
    let conn = state.db.lock();
    get_riwayat_by_id(&conn, id)
}

#[tauri::command]
pub fn export_pdf_cmd(
    state: State<AppState>,
    request: ExportPdfRequest,
) -> AppResult<ExportPdfResponse> {
    let draft_id = request.draft_id.clone();
    let response = export_pdf(&state.paths, request)?;

    if let Ok(riwayat_id) = draft_id.parse::<i64>() {
        let conn = state.db.lock();
        update_riwayat_pdf_metadata(
            &conn,
            riwayat_id,
            &response.relative_path,
            &response.hash_sha256,
        )?;
    }

    Ok(response)
}
