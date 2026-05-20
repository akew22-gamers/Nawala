use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct BackupCopyResult {
    pub destination: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
}

pub fn copy_backup(source: &Path, destination: &Path) -> AppResult<BackupCopyResult> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source, destination)?;
    let metadata = fs::metadata(destination)?;

    Ok(BackupCopyResult {
        destination: destination.to_path_buf(),
        sha256: hash_file(destination)?,
        size_bytes: metadata.len(),
    })
}

pub fn restore_backup(
    source_backup: &Path,
    current_db: &Path,
    pre_restore_dir: &Path,
) -> AppResult<BackupCopyResult> {
    verify_sqlite_integrity(source_backup)?;

    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let pre_restore_path = pre_restore_dir.join(format!("{}.db", timestamp));
    let pre_restore = copy_backup(current_db, &pre_restore_path)?;

    fs::copy(source_backup, current_db)?;

    Ok(pre_restore)
}

fn verify_sqlite_integrity(path: &Path) -> AppResult<()> {
    let conn = Connection::open(path)?;
    let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;

    if result == "ok" {
        Ok(())
    } else {
        Err(AppError::Validation(format!(
            "Backup database failed integrity check: {}",
            result
        )))
    }
}

fn hash_file(path: &Path) -> AppResult<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn copies_sqlite_file_and_returns_sha256() {
        let temp_dir = std::env::temp_dir().join(format!(
            "nawala-backup-test-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap()
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        let source = temp_dir.join("nawala.db");
        let destination = temp_dir.join("backup.db");
        {
            let conn = Connection::open(&source).unwrap();
            conn.execute("CREATE TABLE sample (id INTEGER PRIMARY KEY)", [])
                .unwrap();
            conn.execute("INSERT INTO sample (id) VALUES (1)", [])
                .unwrap();
        }

        let result = copy_backup(&source, &destination).unwrap();

        assert!(destination.exists());
        assert_eq!(result.destination, destination);
        assert!(!result.sha256.is_empty());

        let copied = Connection::open(temp_dir.join("backup.db")).unwrap();
        let integrity: String = copied
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .unwrap();
        assert_eq!(integrity, "ok");

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
