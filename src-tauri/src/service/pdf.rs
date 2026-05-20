use crate::error::{AppError, AppResult};
use crate::paths::AppPaths;
use printpdf::ops::PdfFontHandle;
use printpdf::{BuiltinFont, Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, PdfWarnMsg, Point, Pt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExportPdfRequest {
    pub draft_id: String,
    pub output_name: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub rendered_html: Option<String>,
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

pub fn export_pdf(paths: &AppPaths, request: ExportPdfRequest) -> AppResult<ExportPdfResponse> {
    fs::create_dir_all(&paths.exports_dir)?;

    let output_name = request
        .output_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .map(sanitize_pdf_name)
        .unwrap_or_else(|| sanitize_pdf_name(&format!("{}.pdf", request.draft_id)));
    let output_path = paths.exports_dir.join(&output_name);

    let bytes = build_pdf_bytes(&request)?;
    fs::write(&output_path, bytes)?;

    Ok(ExportPdfResponse {
        relative_path: format!("exports/{}", output_name),
        hash_sha256: hash_file_sha256(&output_path)?,
    })
}

fn build_pdf_bytes(request: &ExportPdfRequest) -> AppResult<Vec<u8>> {
    let title = request
        .title
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("Dokumen Surat");
    let body = request
        .body
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or(request.rendered_html.as_deref())
        .unwrap_or("Dokumen PDF resmi dibuat dari aplikasi Nawala.");

    let mut doc = PdfDocument::new(title);
    let font = PdfFontHandle::Builtin(BuiltinFont::Helvetica);
    let bold_font = PdfFontHandle::Builtin(BuiltinFont::HelveticaBold);
    let mut ops = vec![
        Op::StartTextSection,
        Op::SetFont {
            font: bold_font.clone(),
            size: Pt(16.0),
        },
        Op::SetTextCursor {
            pos: Point {
                x: Mm(24.0).into(),
                y: Mm(268.0).into(),
            },
        },
        Op::ShowText {
            items: vec![title.into()],
        },
        Op::SetFont {
            font: font.clone(),
            size: Pt(10.0),
        },
        Op::SetTextCursor {
            pos: Point {
                x: Mm(24.0).into(),
                y: Mm(256.0).into(),
            },
        },
        Op::ShowText {
            items: vec![format!("Nomor Draft: {}", request.draft_id).into()],
        },
        Op::SetFont {
            font,
            size: Pt(11.0),
        },
        Op::SetLineHeight { lh: Pt(15.0) },
        Op::SetTextCursor {
            pos: Point {
                x: Mm(24.0).into(),
                y: Mm(238.0).into(),
            },
        },
    ];

    for line in wrap_text(body, 86).into_iter().take(35) {
        ops.push(Op::ShowText {
            items: vec![line.into()],
        });
        ops.push(Op::AddLineBreak);
    }
    ops.push(Op::EndTextSection);

    doc.pages.push(PdfPage::new(Mm(210.0), Mm(297.0), ops));
    let mut warnings: Vec<PdfWarnMsg> = Vec::new();
    let bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);
    if bytes.is_empty() {
        return Err(AppError::Internal(
            "PDF generation produced an empty file".to_string(),
        ));
    }
    Ok(bytes)
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            if !current.is_empty() && current.len() + word.len() + 1 > max_chars {
                lines.push(current);
                current = String::new();
            }
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
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
    fn export_pdf_writes_real_pdf_file_and_hash() {
        let paths = test_paths("export");
        let request = ExportPdfRequest {
            draft_id: "draft-001".to_string(),
            output_name: Some("surat-test.pdf".to_string()),
            title: Some("Surat Keterangan".to_string()),
            body: Some("Isi dokumen resmi untuk pengujian.".to_string()),
            rendered_html: None,
        };

        let response = export_pdf(&paths, request).unwrap();

        let exported_path = paths.data_dir.join(PathBuf::from(&response.relative_path));
        assert_eq!(response.relative_path, "exports/surat-test.pdf");
        assert_eq!(
            response.hash_sha256,
            hash_file_sha256(&exported_path).unwrap()
        );
        let bytes = fs::read(&exported_path).unwrap();
        assert!(bytes.len() > 100);
        assert!(bytes.starts_with(b"%PDF"));
        let _ = fs::remove_dir_all(paths.data_dir);
    }
}
