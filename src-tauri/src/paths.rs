use std::path::PathBuf;

/// Application paths structure
#[derive(Debug, Clone)]
pub struct AppPaths {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub backups_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub exports_dir: PathBuf,
    pub template_override_dir: PathBuf,
}

impl AppPaths {
    /// Create AppPaths from a data directory
    pub fn from_data_dir(data_dir: PathBuf) -> Self {
        let db_path = data_dir.join("nawala.db");
        let backups_dir = data_dir.join("backups");
        let assets_dir = data_dir.join("assets");
        let exports_dir = data_dir.join("exports");
        let template_override_dir = data_dir.join("templates").join("override");

        Self {
            data_dir,
            db_path,
            backups_dir,
            assets_dir,
            exports_dir,
            template_override_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_expected_paths() {
        let data_dir = PathBuf::from("/tmp/Nawala");
        let paths = AppPaths::from_data_dir(data_dir.clone());

        assert_eq!(paths.data_dir, PathBuf::from("/tmp/Nawala"));
        assert_eq!(paths.db_path, PathBuf::from("/tmp/Nawala/nawala.db"));
        assert_eq!(paths.backups_dir, PathBuf::from("/tmp/Nawala/backups"));
        assert_eq!(
            paths.template_override_dir,
            PathBuf::from("/tmp/Nawala/templates/override")
        );
    }
}
