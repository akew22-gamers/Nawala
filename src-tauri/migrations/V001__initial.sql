-- V001__initial.sql
-- Initial database schema for Nawala.

CREATE TABLE pengaturan_desa (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    nama_desa TEXT NOT NULL,
    kecamatan TEXT NOT NULL,
    kabupaten TEXT NOT NULL,
    provinsi TEXT NOT NULL,
    kode_wilayah TEXT,
    kode_desa TEXT,
    alamat_kantor TEXT,
    kop_logo_path TEXT,
    tema TEXT NOT NULL DEFAULT 'light',
    ukuran_kertas_default TEXT NOT NULL DEFAULT 'F4',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE pejabat (
    id INTEGER PRIMARY KEY,
    nama TEXT NOT NULL,
    jabatan TEXT NOT NULL,
    nipd TEXT,
    ttd_path TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    aktif INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_pejabat_aktif ON pejabat(aktif);
CREATE INDEX idx_pejabat_default ON pejabat(is_default);
CREATE INDEX idx_pejabat_jabatan ON pejabat(jabatan);

CREATE TABLE auth (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    password_hash TEXT NOT NULL,
    hint TEXT,
    failed_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE kk (
    id INTEGER PRIMARY KEY,
    no_kk TEXT NOT NULL UNIQUE,
    alamat TEXT NOT NULL,
    rt TEXT,
    rw TEXT,
    dusun TEXT,
    desa TEXT,
    kecamatan TEXT,
    kabupaten TEXT,
    provinsi TEXT,
    kode_pos TEXT,
    kepala_keluarga_nik TEXT,
    catatan TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_kk_no_kk ON kk(no_kk);
CREATE INDEX idx_kk_rt_rw ON kk(rt, rw);

CREATE TABLE penduduk (
    id INTEGER PRIMARY KEY,
    nik TEXT NOT NULL UNIQUE,
    no_kk TEXT,
    nama_lengkap TEXT NOT NULL,
    jenis_kelamin TEXT NOT NULL CHECK (jenis_kelamin IN ('L', 'P')),
    tempat_lahir TEXT,
    tanggal_lahir TEXT,
    golongan_darah TEXT,
    agama TEXT,
    status_perkawinan TEXT,
    pendidikan TEXT,
    pekerjaan TEXT,
    kewarganegaraan TEXT DEFAULT 'WNI',
    no_paspor TEXT,
    tanggal_akhir_paspor TEXT,
    no_kitas TEXT,
    nama_ayah TEXT,
    nik_ayah TEXT,
    nama_ibu TEXT,
    nik_ibu TEXT,
    hubungan_keluarga TEXT,
    alamat_lengkap TEXT,
    rt TEXT,
    rw TEXT,
    dusun TEXT,
    keterangan TEXT,
    akta_lahir TEXT,
    akta_perkawinan TEXT,
    tanggal_perkawinan TEXT,
    akta_perceraian TEXT,
    tanggal_perceraian TEXT,
    no_kk_sebelumnya TEXT,
    waktu_lahir TEXT,
    tempat_dilahirkan TEXT,
    jenis_kelahiran TEXT,
    kelahiran_anak_ke INTEGER,
    penolong_kelahiran TEXT,
    berat_lahir INTEGER,
    panjang_lahir TEXT,
    cacat TEXT,
    sakit_menahun TEXT,
    no_asuransi TEXT,
    email TEXT,
    telepon TEXT,
    data_extra TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (no_kk) REFERENCES kk(no_kk) ON UPDATE CASCADE ON DELETE SET NULL
);

CREATE INDEX idx_penduduk_no_kk ON penduduk(no_kk);
CREATE INDEX idx_penduduk_nama ON penduduk(nama_lengkap);
CREATE INDEX idx_penduduk_rt_rw ON penduduk(rt, rw);

CREATE TABLE formulir_def (
    kode TEXT PRIMARY KEY,
    nama TEXT NOT NULL,
    kategori TEXT NOT NULL,
    deskripsi TEXT,
    ukuran_kertas TEXT NOT NULL DEFAULT 'F4',
    template_html TEXT NOT NULL,
    schema_json TEXT NOT NULL,
    versi_regulasi TEXT NOT NULL,
    versi_template INTEGER NOT NULL DEFAULT 1,
    aktif INTEGER NOT NULL DEFAULT 1,
    urutan INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_formulir_def_kategori ON formulir_def(kategori);
CREATE INDEX idx_formulir_def_aktif ON formulir_def(aktif);

CREATE TABLE nomor_surat_counter (
    id INTEGER PRIMARY KEY,
    format_kode TEXT NOT NULL,
    tahun INTEGER NOT NULL,
    format_pola TEXT NOT NULL,
    next_seq INTEGER NOT NULL DEFAULT 1,
    UNIQUE (format_kode, tahun)
);

CREATE TABLE draft_formulir (
    id INTEGER PRIMARY KEY,
    kode_formulir TEXT NOT NULL,
    data_json TEXT NOT NULL,
    pejabat_id INTEGER,
    nama_draft TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (kode_formulir) REFERENCES formulir_def(kode) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (pejabat_id) REFERENCES pejabat(id) ON DELETE SET NULL
);

CREATE INDEX idx_draft_kode ON draft_formulir(kode_formulir);
CREATE INDEX idx_draft_updated ON draft_formulir(updated_at);

CREATE TABLE riwayat_formulir (
    id INTEGER PRIMARY KEY,
    kode_formulir TEXT NOT NULL,
    versi_template INTEGER NOT NULL,
    nomor_surat TEXT,
    tanggal_terbit TEXT NOT NULL,
    pejabat_id INTEGER,
    pejabat_snapshot TEXT NOT NULL,
    data_snapshot TEXT NOT NULL,
    template_snapshot TEXT NOT NULL,
    pdf_path TEXT,
    hash_dokumen TEXT,
    catatan TEXT,
    dibuat_oleh TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (kode_formulir) REFERENCES formulir_def(kode) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (pejabat_id) REFERENCES pejabat(id) ON DELETE SET NULL
);

CREATE INDEX idx_riwayat_kode ON riwayat_formulir(kode_formulir, created_at DESC);
CREATE INDEX idx_riwayat_nomor ON riwayat_formulir(nomor_surat);
CREATE INDEX idx_riwayat_tanggal ON riwayat_formulir(tanggal_terbit);

CREATE TABLE riwayat_subjek (
    riwayat_id INTEGER NOT NULL,
    nik TEXT NOT NULL,
    peran TEXT NOT NULL,
    PRIMARY KEY (riwayat_id, nik, peran),
    FOREIGN KEY (riwayat_id) REFERENCES riwayat_formulir(id) ON DELETE CASCADE,
    FOREIGN KEY (nik) REFERENCES penduduk(nik) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE INDEX idx_riwayat_subjek_nik ON riwayat_subjek(nik);

CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY,
    ts TEXT NOT NULL DEFAULT (datetime('now')),
    aksi TEXT NOT NULL,
    entitas TEXT,
    entitas_id TEXT,
    ringkasan TEXT,
    metadata TEXT
);

CREATE INDEX idx_audit_ts ON audit_log(ts DESC);
CREATE INDEX idx_audit_entitas ON audit_log(entitas, entitas_id);

CREATE TABLE backup_log (
    id INTEGER PRIMARY KEY,
    ts TEXT NOT NULL DEFAULT (datetime('now')),
    tipe TEXT NOT NULL,
    path TEXT NOT NULL,
    size_bytes INTEGER,
    hash_sha256 TEXT,
    catatan TEXT
);

CREATE INDEX idx_backup_ts ON backup_log(ts DESC);

CREATE TABLE import_log (
    id INTEGER PRIMARY KEY,
    ts TEXT NOT NULL DEFAULT (datetime('now')),
    mode TEXT NOT NULL,
    filename TEXT NOT NULL,
    total_rows INTEGER NOT NULL DEFAULT 0,
    inserted_rows INTEGER NOT NULL DEFAULT 0,
    updated_rows INTEGER NOT NULL DEFAULT 0,
    skipped_rows INTEGER NOT NULL DEFAULT 0,
    error_rows INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('processing', 'completed', 'failed')),
    error_report_path TEXT,
    metadata TEXT
);

CREATE INDEX idx_import_ts ON import_log(ts DESC);
CREATE INDEX idx_import_status ON import_log(status);
