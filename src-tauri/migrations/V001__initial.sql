-- V001__initial.sql
-- Initial database schema for Nawala

-- Pengaturan Desa
CREATE TABLE pengaturan_desa (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    nama_desa TEXT NOT NULL,
    kode_desa TEXT NOT NULL,
    kecamatan TEXT NOT NULL,
    kabupaten TEXT NOT NULL,
    provinsi TEXT NOT NULL,
    kode_pos TEXT,
    telepon TEXT,
    email TEXT,
    website TEXT,
    logo_path TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Pejabat
CREATE TABLE pejabat (
    nipd TEXT PRIMARY KEY,
    nama TEXT NOT NULL,
    jabatan TEXT NOT NULL,
    nip TEXT,
    pangkat_golongan TEXT,
    aktif INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_pejabat_aktif ON pejabat(aktif);
CREATE INDEX idx_pejabat_jabatan ON pejabat(jabatan);

-- Auth
CREATE TABLE auth (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'operator', 'viewer')),
    nama_lengkap TEXT NOT NULL,
    aktif INTEGER NOT NULL DEFAULT 1,
    last_login TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_auth_username ON auth(username);
CREATE INDEX idx_auth_aktif ON auth(aktif);

-- Kartu Keluarga
CREATE TABLE kk (
    no_kk TEXT PRIMARY KEY,
    kepala_keluarga_nik TEXT,
    alamat TEXT NOT NULL,
    rt TEXT,
    rw TEXT,
    kode_pos TEXT,
    dusun TEXT,
    kelurahan_desa TEXT,
    kecamatan TEXT,
    kabupaten_kota TEXT,
    provinsi TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_kk_kepala ON kk(kepala_keluarga_nik);
CREATE INDEX idx_kk_rt_rw ON kk(rt, rw);

-- Penduduk
CREATE TABLE penduduk (
    nik TEXT PRIMARY KEY,
    no_kk TEXT,
    nama_lengkap TEXT NOT NULL,
    jenis_kelamin TEXT NOT NULL CHECK (jenis_kelamin IN ('L', 'P')),
    tempat_lahir TEXT,
    tanggal_lahir TEXT,
    agama TEXT,
    pendidikan TEXT,
    jenis_pekerjaan TEXT,
    status_perkawinan TEXT,
    shdk TEXT,
    nama_ayah TEXT,
    nama_ibu TEXT,
    golongan_darah TEXT,
    kewarganegaraan TEXT DEFAULT 'WNI',
    no_paspor TEXT,
    no_akta_lahir TEXT,
    no_akta_kawin TEXT,
    tanggal_kawin TEXT,
    no_akta_cerai TEXT,
    tanggal_cerai TEXT,
    keterangan TEXT,
    -- Adminduk extra columns (nullable)
    telepon TEXT,
    email TEXT,
    status_tinggal TEXT,
    status_kependudukan TEXT,
    tanggal_pindah TEXT,
    alamat_pindah TEXT,
    tanggal_meninggal TEXT,
    no_akta_meninggal TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (no_kk) REFERENCES kk(no_kk) ON DELETE SET NULL
);

CREATE INDEX idx_penduduk_kk ON penduduk(no_kk);
CREATE INDEX idx_penduduk_nama ON penduduk(nama_lengkap);
CREATE INDEX idx_penduduk_jk ON penduduk(jenis_kelamin);
CREATE INDEX idx_penduduk_status_kependudukan ON penduduk(status_kependudukan);

-- Formulir Definition
CREATE TABLE formulir_def (
    kode TEXT PRIMARY KEY,
    nama TEXT NOT NULL,
    kategori TEXT NOT NULL,
    template_path TEXT NOT NULL,
    schema_json TEXT NOT NULL,
    aktif INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_formulir_def_kategori ON formulir_def(kategori);
CREATE INDEX idx_formulir_def_aktif ON formulir_def(aktif);

-- Nomor Surat Counter
CREATE TABLE nomor_surat_counter (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kode_formulir TEXT NOT NULL,
    tahun INTEGER NOT NULL,
    bulan INTEGER NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(kode_formulir, tahun, bulan),
    FOREIGN KEY (kode_formulir) REFERENCES formulir_def(kode) ON DELETE CASCADE
);

CREATE INDEX idx_nomor_counter_lookup ON nomor_surat_counter(kode_formulir, tahun, bulan);

-- Draft Formulir
CREATE TABLE draft_formulir (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kode_formulir TEXT NOT NULL,
    nomor_surat TEXT,
    tanggal_surat TEXT NOT NULL,
    data_json TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'final')),
    created_by INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (kode_formulir) REFERENCES formulir_def(kode) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES auth(id) ON DELETE SET NULL
);

CREATE INDEX idx_draft_kode ON draft_formulir(kode_formulir);
CREATE INDEX idx_draft_status ON draft_formulir(status);
CREATE INDEX idx_draft_tanggal ON draft_formulir(tanggal_surat);
CREATE INDEX idx_draft_nomor ON draft_formulir(nomor_surat);

-- Riwayat Formulir
CREATE TABLE riwayat_formulir (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kode_formulir TEXT NOT NULL,
    nomor_surat TEXT NOT NULL,
    tanggal_surat TEXT NOT NULL,
    data_json TEXT NOT NULL,
    created_by INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (kode_formulir) REFERENCES formulir_def(kode) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES auth(id) ON DELETE SET NULL
);

CREATE INDEX idx_riwayat_kode ON riwayat_formulir(kode_formulir);
CREATE INDEX idx_riwayat_nomor ON riwayat_formulir(nomor_surat);
CREATE INDEX idx_riwayat_tanggal ON riwayat_formulir(tanggal_surat);

-- Riwayat Subjek
CREATE TABLE riwayat_subjek (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    riwayat_formulir_id INTEGER NOT NULL,
    nik TEXT,
    no_kk TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (riwayat_formulir_id) REFERENCES riwayat_formulir(id) ON DELETE CASCADE,
    FOREIGN KEY (nik) REFERENCES penduduk(nik) ON DELETE SET NULL,
    FOREIGN KEY (no_kk) REFERENCES kk(no_kk) ON DELETE SET NULL
);

CREATE INDEX idx_riwayat_subjek_formulir ON riwayat_subjek(riwayat_formulir_id);
CREATE INDEX idx_riwayat_subjek_nik ON riwayat_subjek(nik);
CREATE INDEX idx_riwayat_subjek_kk ON riwayat_subjek(no_kk);

-- Audit Log
CREATE TABLE audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    details_json TEXT,
    ip_address TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES auth(id) ON DELETE SET NULL
);

CREATE INDEX idx_audit_user ON audit_log(user_id);
CREATE INDEX idx_audit_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_created ON audit_log(created_at);

-- Backup Log
CREATE TABLE backup_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('success', 'failed')),
    error_message TEXT,
    created_by INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (created_by) REFERENCES auth(id) ON DELETE SET NULL
);

CREATE INDEX idx_backup_status ON backup_log(status);
CREATE INDEX idx_backup_created ON backup_log(created_at);

-- Import Log
CREATE TABLE import_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    import_type TEXT NOT NULL,
    filename TEXT NOT NULL,
    total_records INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    error_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('processing', 'completed', 'failed')),
    error_details_json TEXT,
    created_by INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    FOREIGN KEY (created_by) REFERENCES auth(id) ON DELETE SET NULL
);

CREATE INDEX idx_import_type ON import_log(import_type);
CREATE INDEX idx_import_status ON import_log(status);
CREATE INDEX idx_import_created ON import_log(created_at);
