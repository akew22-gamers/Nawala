use crate::error::AppResult;
use crate::paths::AppPaths;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExportPdfRequest {
    pub draft_id: String,
    pub output_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExportPdfResponse {
    pub relative_path: String,
    pub hash_sha256: String,
}

pub fn hash_file_sha256(path: impl AsRef<Path>) -> AppResult<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

pub fn export_pdf_placeholder(
    paths: &AppPaths,
    request: ExportPdfRequest,
) -> AppResult<ExportPdfResponse> {
    fs::create_dir_all(&paths.exports_dir)?;

    let output_name = request
        .output_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .map(sanitize_pdf_name)
        .unwrap_or_else(|| sanitize_pdf_name(&format!("{}.pdf", request.draft_id)));
    let output_path = paths.exports_dir.join(&output_name);

    // Placeholder until Tauri/WebKit PDF printing is wired in: deterministic bytes
    // keep export history hashable and testable without relying on webview APIs.
    let content = format!(
        "%PDF-1.4\n% Nawala placeholder PDF\n1 0 obj\n<< /Type /Catalog >>\nendobj\n% draft_id: {}\n%%EOF\n",
        request.draft_id
    );
    fs::write(&output_path, content.as_bytes())?;

    Ok(ExportPdfResponse {
        relative_path: format!("exports/{}", output_name),
        hash_sha256: hash_file_sha256(&output_path)?,
    })
}

pub fn export_pdf(paths: &AppPaths, request: ExportPdfRequest) -> AppResult<ExportPdfResponse> {
    export_pdf_placeholder(paths, request)
}

fn sanitize_pdf_name(name: &str) -> String {
    let mut sanitized: String = name
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => ch,
            _ => '-',
        })
        .collect();

    if !sanitized.ends_with(".pdf") {
        sanitized.push_str(".pdf");
    }

    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::AppPaths;
    use std::fs;
    use std::path::PathBuf;

    fn test_paths(name: &str) -> AppPaths {
        let data_dir =
            std::env::temp_dir().join(format!("nawala-pdf-test-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&data_dir);
        AppPaths::from_data_dir(data_dir)
    }

    #[test]
    fn hash_file_sha256_returns_expected_hex_digest() {
        let paths = test_paths("hash");
        fs::create_dir_all(&paths.data_dir).unwrap();
        let file_path = paths.data_dir.join("input.txt");
        fs::write(&file_path, b"nawala").unwrap();

        let hash = hash_file_sha256(&file_path).unwrap();

        assert_eq!(
            hash,
            "cd39c8125df81bafb33a1448a160d836a4db67e075630272ebe8da26a858325f"
        );
        let _ = fs::remove_dir_all(paths.data_dir);
    }

    #[test]
    fn export_pdf_placeholder_writes_deterministic_file_and_hash() {
        let paths = test_paths("export");
        let request = ExportPdfRequest {
            draft_id: "draft-001".to_string(),
            output_name: Some("surat-test.pdf".to_string()),
        };

        let response = export_pdf_placeholder(&paths, request).unwrap();

        let exported_path = paths.data_dir.join(PathBuf::from(&response.relative_path));
        assert_eq!(response.relative_path, "exports/surat-test.pdf");
        assert_eq!(
            response.hash_sha256,
            hash_file_sha256(&exported_path).unwrap()
        );
        assert!(fs::read(&exported_path).unwrap().starts_with(b"%PDF-1.4\n"));
        let _ = fs::remove_dir_all(paths.data_dir);
    }
}
