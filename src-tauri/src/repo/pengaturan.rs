use crate::error::AppResult;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PengaturanDesa {
    pub nama_desa: String,
    pub kecamatan: String,
    pub kabupaten: String,
    pub provinsi: String,
    pub kode_wilayah: Option<String>,
    pub kode_desa: Option<String>,
    pub alamat_kantor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PejabatRecord {
    pub id: i64,
    pub nama: String,
    pub jabatan: String,
    pub nipd: Option<String>,
    pub is_default: bool,
    pub aktif: bool,
}

/// Get pengaturan desa (singleton row)
pub fn get_pengaturan_desa(conn: &Connection) -> AppResult<Option<PengaturanDesa>> {
    let result = conn
        .query_row(
            "SELECT nama_desa, kecamatan, kabupaten, provinsi, kode_wilayah, kode_desa, alamat_kantor
             FROM pengaturan_desa WHERE id = 1",
            [],
            |row| {
                Ok(PengaturanDesa {
                    nama_desa: row.get(0)?,
                    kecamatan: row.get(1)?,
                    kabupaten: row.get(2)?,
                    provinsi: row.get(3)?,
                    kode_wilayah: row.get(4)?,
                    kode_desa: row.get(5)?,
                    alamat_kantor: row.get(6)?,
                })
            },
        )
        .ok();
    Ok(result)
}

/// Save pengaturan desa (upsert)
pub fn save_pengaturan_desa(conn: &Connection, data: &PengaturanDesa) -> AppResult<()> {
    conn.execute(
        "INSERT INTO pengaturan_desa (id, nama_desa, kecamatan, kabupaten, provinsi, kode_wilayah, kode_desa, alamat_kantor)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(id) DO UPDATE SET
           nama_desa = excluded.nama_desa,
           kecamatan = excluded.kecamatan,
           kabupaten = excluded.kabupaten,
           provinsi = excluded.provinsi,
           kode_wilayah = excluded.kode_wilayah,
           kode_desa = excluded.kode_desa,
           alamat_kantor = excluded.alamat_kantor,
           updated_at = datetime('now')",
        params![
            data.nama_desa,
            data.kecamatan,
            data.kabupaten,
            data.provinsi,
            data.kode_wilayah,
            data.kode_desa,
            data.alamat_kantor,
        ],
    )?;
    Ok(())
}

/// List all pejabat (active only by default)
pub fn list_pejabat(conn: &Connection) -> AppResult<Vec<PejabatRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, nama, jabatan, nipd, is_default, aktif FROM pejabat WHERE aktif = 1 ORDER BY is_default DESC, nama",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(PejabatRecord {
                id: row.get(0)?,
                nama: row.get(1)?,
                jabatan: row.get(2)?,
                nipd: row.get(3)?,
                is_default: row.get::<_, i32>(4)? != 0,
                aktif: row.get::<_, i32>(5)? != 0,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(items)
}

/// Save pejabat (insert or update)
pub fn save_pejabat(conn: &Connection, data: &PejabatRecord) -> AppResult<i64> {
    if data.id > 0 {
        conn.execute(
            "UPDATE pejabat SET nama = ?2, jabatan = ?3, nipd = ?4, is_default = ?5, aktif = ?6, updated_at = datetime('now')
             WHERE id = ?1",
            params![
                data.id,
                data.nama,
                data.jabatan,
                data.nipd,
                data.is_default as i32,
                data.aktif as i32,
            ],
        )?;
        Ok(data.id)
    } else {
        conn.execute(
            "INSERT INTO pejabat (nama, jabatan, nipd, is_default, aktif) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                data.nama,
                data.jabatan,
                data.nipd,
                data.is_default as i32,
                data.aktif as i32,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
