use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulirDefRecord {
    pub kode: String,
    pub nama: String,
    pub kategori: String,
    pub deskripsi: Option<String>,
    pub ukuran_kertas: String,
    pub schema_json: String,
    pub template_html: String,
    pub aktif: bool,
}

/// Seed formulir_def from embedded schema/template resources.
/// Uses INSERT OR IGNORE so existing customized records are preserved.
pub fn seed_formulir_def(conn: &Connection) -> AppResult<()> {
    // Embedded schemas and templates
    let schemas: &[(&str, &str)] = &[
        ("F-1.01", include_str!("../resources/schemas/F-1.01.json")),
        ("F-1.02", include_str!("../resources/schemas/F-1.02.json")),
        ("F-1.03", include_str!("../resources/schemas/F-1.03.json")),
        ("F-1.06", include_str!("../resources/schemas/F-1.06.json")),
        ("F-1.08", include_str!("../resources/schemas/F-1.08.json")),
        ("F-1.15", include_str!("../resources/schemas/F-1.15.json")),
        ("F-1.16", include_str!("../resources/schemas/F-1.16.json")),
        ("F-1.25", include_str!("../resources/schemas/F-1.25.json")),
        ("F-1.27", include_str!("../resources/schemas/F-1.27.json")),
        (
            "F-2.01",
            include_str!("../resources/schemas/F-2.01.json"),
        ),
        (
            "F-2.01-kelahiran",
            include_str!("../resources/schemas/F-2.01-kelahiran.json"),
        ),
        (
            "F-2.01-kematian",
            include_str!("../resources/schemas/F-2.01-kematian.json"),
        ),
        ("F-2.12", include_str!("../resources/schemas/F-2.12.json")),
        ("F-2.29", include_str!("../resources/schemas/F-2.29.json")),
        ("F-2.30", include_str!("../resources/schemas/F-2.30.json")),
    ];

    let templates: &[(&str, &str)] = &[
        (
            "F-1.01",
            include_str!("../resources/templates/F-1.01.html"),
        ),
        (
            "F-1.02",
            include_str!("../resources/templates/F-1.02.html"),
        ),
        (
            "F-1.03",
            include_str!("../resources/templates/F-1.03.html"),
        ),
        (
            "F-1.06",
            include_str!("../resources/templates/F-1.06.html"),
        ),
        (
            "F-1.08",
            include_str!("../resources/templates/F-1.08.html"),
        ),
        (
            "F-1.15",
            include_str!("../resources/templates/F-1.15.html"),
        ),
        (
            "F-1.16",
            include_str!("../resources/templates/F-1.16.html"),
        ),
        (
            "F-1.25",
            include_str!("../resources/templates/F-1.25.html"),
        ),
        (
            "F-1.27",
            include_str!("../resources/templates/F-1.27.html"),
        ),
        (
            "F-2.01",
            include_str!("../resources/templates/F-2.01.html"),
        ),
        (
            "F-2.01-kelahiran",
            include_str!("../resources/templates/F-2.01-kelahiran.html"),
        ),
        (
            "F-2.01-kematian",
            include_str!("../resources/templates/F-2.01-kematian.html"),
        ),
        (
            "F-2.12",
            include_str!("../resources/templates/F-2.12.html"),
        ),
        (
            "F-2.29",
            include_str!("../resources/templates/F-2.29.html"),
        ),
        (
            "F-2.30",
            include_str!("../resources/templates/F-2.30.html"),
        ),
    ];

    for (kode, schema_json) in schemas {
        // Parse schema to extract nama and kategori
        let schema: serde_json::Value = serde_json::from_str(schema_json)
            .unwrap_or_else(|_| serde_json::json!({}));
        let nama = schema["nama"].as_str().unwrap_or(kode);
        let kategori = schema["kategori"].as_str().unwrap_or("Umum");
        let ukuran_kertas = schema["ukuran_kertas"].as_str().unwrap_or("F4");
        let versi_regulasi = schema["versi_regulasi"].as_str().unwrap_or("Permendagri 73/2022");

        // Find matching template
        let template_html = templates
            .iter()
            .find(|(k, _)| k == kode)
            .map(|(_, t)| *t)
            .unwrap_or("");

        conn.execute(
            "INSERT OR IGNORE INTO formulir_def (kode, nama, kategori, ukuran_kertas, template_html, schema_json, versi_regulasi)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![kode, nama, kategori, ukuran_kertas, template_html, *schema_json, versi_regulasi],
        )?;
    }

    Ok(())
}

/// List all active formulir definitions
pub fn list_formulir_def(conn: &Connection) -> AppResult<Vec<FormulirDefRecord>> {
    let mut stmt = conn.prepare(
        "SELECT kode, nama, kategori, deskripsi, ukuran_kertas, schema_json, template_html, aktif
         FROM formulir_def WHERE aktif = 1 ORDER BY urutan, kode",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(FormulirDefRecord {
                kode: row.get(0)?,
                nama: row.get(1)?,
                kategori: row.get(2)?,
                deskripsi: row.get(3)?,
                ukuran_kertas: row.get(4)?,
                schema_json: row.get(5)?,
                template_html: row.get(6)?,
                aktif: row.get::<_, i32>(7)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(items)
}

/// Get a single formulir definition by kode
pub fn get_formulir_def(conn: &Connection, kode: &str) -> AppResult<Option<FormulirDefRecord>> {
    let mut stmt = conn.prepare(
        "SELECT kode, nama, kategori, deskripsi, ukuran_kertas, schema_json, template_html, aktif
         FROM formulir_def WHERE kode = ?1",
    )?;

    let result = stmt
        .query_row(params![kode], |row| {
            Ok(FormulirDefRecord {
                kode: row.get(0)?,
                nama: row.get(1)?,
                kategori: row.get(2)?,
                deskripsi: row.get(3)?,
                ukuran_kertas: row.get(4)?,
                schema_json: row.get(5)?,
                template_html: row.get(6)?,
                aktif: row.get::<_, i32>(7)? != 0,
            })
        })
        .ok();

    Ok(result)
}
