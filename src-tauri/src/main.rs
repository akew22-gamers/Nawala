mod app;
mod auth;
mod commands;
mod db;
mod error;
mod paths;
mod repo;
mod service;

use app::AppState;
use db::Db;
use paths::AppPaths;

fn main() {
    let data_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join(".nawala");
    let paths = AppPaths::from_data_dir(data_dir);
    let db = Db::open(&paths.db_path).expect("failed to open Nawala database");

    // Seed formulir definitions from embedded resources
    {
        let conn = db.lock();
        repo::formulir::seed_formulir_def(&conn).expect("failed to seed formulir definitions");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db, paths })
        .invoke_handler(tauri::generate_handler![
            commands::auth::check_auth_status_cmd,
            commands::auth::setup_password_cmd,
            commands::auth::login_cmd,
            commands::audit::list_audit_logs_cmd,
            commands::backup::create_manual_backup_cmd,
            commands::backup::restore_backup_cmd,
            commands::backup::list_backup_logs_cmd,
            commands::formulir::list_formulir_def_cmd,
            commands::formulir::get_formulir_def_cmd,
            commands::formulir::commit_riwayat_formulir_cmd,
            commands::formulir::list_riwayat_cmd,
            commands::formulir::get_riwayat_by_id_cmd,
            commands::formulir::export_pdf_cmd,
            commands::import::import_warga_csv_cmd,
            commands::nomor_surat::get_next_nomor_surat_cmd,
            commands::penduduk::search_penduduk_cmd,
            commands::penduduk::get_penduduk_by_nik_cmd,
            commands::pengaturan::get_pengaturan_desa_cmd,
            commands::pengaturan::save_pengaturan_desa_cmd,
            commands::pengaturan::list_pejabat_cmd,
            commands::pengaturan::save_pejabat_cmd
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
