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

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db, paths })
        .invoke_handler(tauri::generate_handler![
            commands::audit::list_audit_logs_cmd,
            commands::backup::create_manual_backup_cmd,
            commands::backup::restore_backup_cmd,
            commands::backup::list_backup_logs_cmd,
            commands::formulir::commit_riwayat_formulir_cmd,
            commands::formulir::list_riwayat_cmd,
            commands::formulir::get_riwayat_by_id_cmd,
            commands::formulir::export_pdf_cmd
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
