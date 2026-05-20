mod app;
mod auth;
mod commands;
mod db;
mod error;
mod paths;
mod repo;
mod service;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::formulir::commit_riwayat_formulir_cmd,
            commands::formulir::list_riwayat_cmd,
            commands::formulir::get_riwayat_by_id_cmd
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
