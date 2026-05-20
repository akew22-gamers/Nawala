use crate::db::Db;
use crate::paths::AppPaths;

/// Application state shared across Tauri commands
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub paths: AppPaths,
}
