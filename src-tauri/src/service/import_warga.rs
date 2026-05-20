use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ImportedPenduduk {
    pub no_kk: String,
    pub nik: String,
    pub nama_lengkap: String,
    pub jenis_kelamin: String,
    pub tempat_lahir: String,
    pub tanggal_lahir: String,
    pub agama: String,
    pub status_perkawinan: String,
    pub hubungan_keluarga: String,
    pub pendidikan: String,
    pub pekerjaan: String,
    pub nama_ibu: String,
    pub nama_ayah: String,
    pub alamat: String,
    pub rt: String,
    pub rw: String,
    pub keterangan: String,
}

#[allow(dead_code)]
pub fn parse_buku_induk_csv(input: &str) -> AppResult<Vec<ImportedPenduduk>> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(input.as_bytes());
    let headers = reader
        .headers()
        .map_err(|err| AppError::Validation(format!("Invalid CSV header: {err}")))?
        .clone();

    // Find column indices
    let idx_no_kk = find_column(&headers, "NOMOR KK")?;
    let idx_nik = find_column(&headers, "NIK")?;
    let idx_nama = find_column(&headers, "NAMA")?;
    let idx_jk = find_column(&headers, "JENIS KELAMIN")?;
    let idx_tempat_lahir = find_column(&headers, "TEMPAT LAHIR")?;
    let idx_tanggal_lahir = find_column(&headers, "TANGGAL LAHIR")?;
    let idx_agama = find_column(&headers, "AGAMA")?;
    let idx_status = find_column(&headers, "STATUS")?;
    let idx_hubungan = find_column(&headers, "HUBUNGAN KELUARGA")?;
    let idx_pendidikan = find_column(&headers, "PENDIDIKAN")?;
    let idx_pekerjaan = find_column(&headers, "PEKERJAAN")?;
    let idx_nama_ibu = find_column(&headers, "NAMA IBU")?;
    let idx_nama_ayah = find_column(&headers, "NAMA AYAH")?;
    let idx_alamat = find_column(&headers, "ALAMAT")?;
    let idx_rt = find_column(&headers, "RT")?;
    let idx_rw = find_column(&headers, "RW")?;
    let idx_ket = find_column(&headers, "KET")?;

    let mut results = Vec::new();

    for record in reader.records() {
        let fields =
            record.map_err(|err| AppError::Validation(format!("Invalid CSV record: {err}")))?;
        let no_kk = get_field(&fields, idx_no_kk)?;
        let nik = get_field(&fields, idx_nik)?;

        // Validate NIK and No KK as 16 ASCII digits
        validate_16_digits(nik, "NIK")?;
        validate_16_digits(no_kk, "No KK")?;

        results.push(ImportedPenduduk {
            no_kk: no_kk.to_string(),
            nik: nik.to_string(),
            nama_lengkap: get_field(&fields, idx_nama)?.to_string(),
            jenis_kelamin: get_field(&fields, idx_jk)?.to_string(),
            tempat_lahir: get_field(&fields, idx_tempat_lahir)?.to_string(),
            tanggal_lahir: get_field(&fields, idx_tanggal_lahir)?.to_string(),
            agama: get_field(&fields, idx_agama)?.to_string(),
            status_perkawinan: get_field(&fields, idx_status)?.to_string(),
            hubungan_keluarga: get_field(&fields, idx_hubungan)?.to_string(),
            pendidikan: get_field(&fields, idx_pendidikan)?.to_string(),
            pekerjaan: get_field(&fields, idx_pekerjaan)?.to_string(),
            nama_ibu: get_field(&fields, idx_nama_ibu)?.to_string(),
            nama_ayah: get_field(&fields, idx_nama_ayah)?.to_string(),
            alamat: get_field(&fields, idx_alamat)?.to_string(),
            rt: get_field(&fields, idx_rt)?.to_string(),
            rw: get_field(&fields, idx_rw)?.to_string(),
            keterangan: get_field(&fields, idx_ket)?.to_string(),
        });
    }

    Ok(results)
}

fn find_column(headers: &csv::StringRecord, name: &str) -> AppResult<usize> {
    headers
        .iter()
        .position(|h| h.trim() == name)
        .ok_or_else(|| AppError::Validation(format!("Column '{}' not found", name)))
}

fn get_field(fields: &csv::StringRecord, index: usize) -> AppResult<&str> {
    fields
        .get(index)
        .ok_or_else(|| AppError::Validation(format!("Missing field at index {}", index)))
        .map(str::trim)
}

fn validate_16_digits(value: &str, field_name: &str) -> AppResult<()> {
    if value.len() != 16 {
        return Err(AppError::Validation(format!(
            "{} must be exactly 16 digits, got {} characters",
            field_name,
            value.len()
        )));
    }

    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::Validation(format!(
            "{} must contain only digits",
            field_name
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_buku_induk_fixture() {
        let csv = include_str!("../../../tests/fixtures/buku_induk_minimal.csv");
        let rows = parse_buku_induk_csv(csv).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].nama_lengkap, "SENA");
        assert_eq!(rows[2].nama_ayah, "SENA");
    }
}
