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
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
