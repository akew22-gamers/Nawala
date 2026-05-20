# Nawala Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build Nawala, a Tauri + Next.js desktop app for offline village adminduk form generation, resident import, immutable form history, backup, and PDF printing.

**Architecture:** Next.js runs as a static frontend inside Tauri. Rust owns SQLite, auth, filesystem, import, backup, audit, numbering, and PDF orchestration. Forms are data-driven through Handlebars templates and JSON schemas stored in `formulir_def`.

**Tech Stack:** Tauri 2, Rust, rusqlite, refinery, argon2, chrono, strsim, Next.js 15 static export, React 19, TypeScript, DaisyUI 5, Tailwind CSS 4, TanStack Query, Zustand, React Hook Form, Zod, SheetJS, Handlebars, Vitest, Playwright.

---

## Scope And Delivery

This plan implements Nawala in vertical slices. Every task creates testable software and ends with a commit. The first working slice is a scaffolded Tauri desktop app with SQLite, auth, settings, resident import, one form template (`F-1.02`), preview, PDF export, immutable history, backup, and packaging. The full F-form catalogue is added after the engine is validated.

Spec source: `docs/superpowers/specs/2026-05-20-nawala-design.md`.

## File Structure Map

```text
src-tauri/
  Cargo.toml
  tauri.conf.json
  src/
    main.rs
    app.rs
    error.rs
    paths.rs
    auth/{mod.rs,argon.rs,lockout.rs}
    db/{mod.rs,pragma.rs}
    migrations/{V001__initial.sql,V002__seed_reference.sql,V003__seed_f102.sql}
    repo/{mod.rs,audit.rs,auth.rs,backup.rs,draft.rs,formulir.rs,kk.rs,nomor_surat.rs,pejabat.rs,penduduk.rs,pengaturan.rs,riwayat.rs}
    service/{mod.rs,backup.rs,import_warga.rs,nomor_surat.rs,pdf.rs,reference.rs,resolusi_ortu.rs}
    commands/{mod.rs,audit.rs,auth.rs,backup.rs,formulir.rs,import.rs,pengaturan.rs,warga.rs}
    resources/{reference/kode.json,schemas/F-1.02.json,templates/F-1.02.html}
src/
  app/{layout.tsx,page.tsx,login/page.tsx,onboarding/page.tsx,warga/page.tsx,warga/import/page.tsx,formulir/page.tsx,formulir/[kode]/buat/page.tsx,riwayat/page.tsx,pengaturan/page.tsx,pengaturan/tentang/page.tsx,print/[draftId]/page.tsx}
  components/{layout,ui,warga,formulir}
  hooks/{useAuth.ts,usePengaturan.ts}
  lib/{api.ts,handlebars.ts,nomorSurat.ts,tanggal.ts,validation.ts}
  stores/{authStore.ts,uiStore.ts}
  styles/globals.css
  types/{api.ts,formulir.ts,warga.ts}
tests/fixtures/{buku_induk_minimal.csv,f102_context.json}
```

## Task 1: Scaffold Product Identity

**Files:**
- Create: `package.json`
- Create: `next.config.js`
- Create: `tsconfig.json`
- Create: `postcss.config.js`
- Create: `src/styles/globals.css`
- Create: `src/app/layout.tsx`
- Create: `src/app/page.tsx`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `LICENSE`
- Create: `README.md`
- Create: `.gitignore`

- [ ] **Step 1: Create `package.json`**

```json
{
  "name": "nawala",
  "version": "0.1.0",
  "private": true,
  "description": "Nawala - Aplikasi Surat Adminduk Desa",
  "author": "EAS Creative Studio <dev@eas.biz.id>",
  "homepage": "https://eas.biz.id",
  "license": "MIT",
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "lint": "biome check .",
    "format": "biome format --write .",
    "test": "vitest run",
    "tauri": "tauri"
  },
  "dependencies": {
    "@hookform/resolvers": "latest",
    "@tauri-apps/api": "latest",
    "@tanstack/react-query": "latest",
    "@tanstack/react-table": "latest",
    "@tanstack/react-virtual": "latest",
    "daisyui": "latest",
    "handlebars": "latest",
    "next": "latest",
    "react": "latest",
    "react-dom": "latest",
    "react-hook-form": "latest",
    "xlsx": "latest",
    "zod": "latest",
    "zustand": "latest"
  },
  "devDependencies": {
    "@biomejs/biome": "latest",
    "@tailwindcss/postcss": "latest",
    "@tauri-apps/cli": "latest",
    "@testing-library/react": "latest",
    "@types/node": "latest",
    "@types/react": "latest",
    "@types/react-dom": "latest",
    "tailwindcss": "latest",
    "typescript": "latest",
    "vitest": "latest"
  }
}
```

- [ ] **Step 2: Create Next static export config**

`next.config.js`:

```js
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  images: { unoptimized: true },
  trailingSlash: true,
};
module.exports = nextConfig;
```

- [ ] **Step 3: Create starter UI**

`src/styles/globals.css`:

```css
@import "tailwindcss";
@plugin "daisyui" {
  themes: light --default, dark --prefersdark;
}
body { margin: 0; min-height: 100vh; }
```

`src/app/layout.tsx`:

```tsx
import '@/styles/globals.css';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Nawala',
  description: 'Aplikasi Surat Adminduk Desa',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return <html lang="id" data-theme="light"><body>{children}</body></html>;
}
```

`src/app/page.tsx`:

```tsx
export default function DashboardPage() {
  return (
    <main className="min-h-screen bg-base-200 p-6">
      <section className="mx-auto max-w-5xl rounded-box bg-base-100 p-8 shadow-sm">
        <p className="text-sm uppercase tracking-[0.3em] text-primary">EAS Creative Studio</p>
        <h1 className="mt-3 text-4xl font-bold">Nawala</h1>
        <p className="mt-2 text-base-content/70">Aplikasi Surat Adminduk Desa</p>
      </section>
    </main>
  );
}
```

- [ ] **Step 4: Create Tauri config and entrypoint**

`src-tauri/Cargo.toml`:

```toml
[package]
name = "nawala"
version = "0.1.0"
description = "Nawala - Aplikasi Surat Adminduk Desa"
authors = ["EAS Creative Studio"]
license = "MIT"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }
rusqlite = { version = "0.32", features = ["bundled", "chrono"] }
refinery = { version = "0.8", features = ["rusqlite"] }
argon2 = "0.5"
rand_core = "0.6"
strsim = "0.11"
sha2 = "0.10"
base64 = "0.22"
```

`src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Nawala",
  "version": "0.1.0",
  "identifier": "id.biz.eas.nawala",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:3000",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../out"
  },
  "app": { "windows": [{ "title": "Nawala", "width": 1200, "height": 800, "minWidth": 960, "minHeight": 640 }] },
  "bundle": { "active": true, "targets": ["msi", "deb", "appimage"], "publisher": "EAS Creative Studio" }
}
```

`src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
```

- [ ] **Step 5: Verify scaffold**

Run: `npm install`
Expected: dependency install succeeds and lockfile is created.

Run: `npm run build`
Expected: Next.js static build succeeds and creates `out/`.

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: Rust compile succeeds.

- [ ] **Step 6: Commit scaffold**

```bash
git add .
git commit -m "chore: scaffold Nawala desktop app"
```

## Task 2: Rust App State, Error Type, Paths, And SQLite Bootstrap

**Files:**
- Create: `src-tauri/src/error.rs`
- Create: `src-tauri/src/paths.rs`
- Create: `src-tauri/src/app.rs`
- Create: `src-tauri/src/db/mod.rs`
- Create: `src-tauri/src/db/pragma.rs`
- Create: `src-tauri/migrations/V001__initial.sql`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Write path helper with unit test**

`src-tauri/src/paths.rs`:

```rust
use std::path::PathBuf;

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
    pub fn from_data_dir(data_dir: PathBuf) -> Self {
        Self {
            db_path: data_dir.join("nawala.db"),
            backups_dir: data_dir.join("backups"),
            assets_dir: data_dir.join("assets"),
            exports_dir: data_dir.join("exports"),
            template_override_dir: data_dir.join("templates").join("override"),
            data_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builds_expected_paths() {
        let paths = AppPaths::from_data_dir(PathBuf::from("/tmp/Nawala"));
        assert_eq!(paths.db_path, PathBuf::from("/tmp/Nawala/nawala.db"));
        assert_eq!(paths.backups_dir, PathBuf::from("/tmp/Nawala/backups"));
        assert_eq!(paths.template_override_dir, PathBuf::from("/tmp/Nawala/templates/override"));
    }
}
```

- [ ] **Step 2: Add serializable error type**

`src-tauri/src/error.rs`:

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "kind", content = "details")]
pub enum AppError {
    Database(String), Validation(String), NotFound(String), Conflict(String), Auth(String), FileSystem(String), Import(String), Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}
impl std::error::Error for AppError {}
impl From<rusqlite::Error> for AppError { fn from(value: rusqlite::Error) -> Self { Self::Database(value.to_string()) } }
impl From<std::io::Error> for AppError { fn from(value: std::io::Error) -> Self { Self::FileSystem(value.to_string()) } }
```

- [ ] **Step 3: Add SQLite connection wrapper**

`src-tauri/src/db/pragma.rs`:

```rust
use rusqlite::Connection;
pub fn apply(conn: &Connection) -> rusqlite::Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    Ok(())
}
```

`src-tauri/src/db/mod.rs`:

```rust
pub mod pragma;
use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use std::{path::Path, sync::{Arc, Mutex}};

#[derive(Clone)]
pub struct Db(pub Arc<Mutex<Connection>>);

impl Db {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
        let conn = Connection::open(path)?;
        pragma::apply(&conn).map_err(AppError::from)?;
        Ok(Self(Arc::new(Mutex::new(conn))))
    }
}
```

- [ ] **Step 4: Add initial SQL schema**

Create `src-tauri/migrations/V001__initial.sql` with these tables: `pengaturan_desa`, `pejabat`, `auth`, `kk`, `penduduk`, `formulir_def`, `nomor_surat_counter`, `draft_formulir`, `riwayat_formulir`, `riwayat_subjek`, `audit_log`, `backup_log`, `import_log`. Use the exact column names from the design spec, including `pejabat.nipd`, `penduduk.keterangan`, and nullable adminduk extra columns.

- [ ] **Step 5: Wire modules and test**

Modify `src-tauri/src/main.rs`:

```rust
mod app;
mod db;
mod error;
mod paths;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("failed to run Nawala");
}
```

Run: `cargo test --manifest-path src-tauri/Cargo.toml`
Expected: `paths::tests::builds_expected_paths` passes.

- [ ] **Step 6: Commit database bootstrap**

```bash
git add src-tauri
git commit -m "feat: add database bootstrap"
```

## Task 3: Auth Foundation

**Files:**
- Create: `src-tauri/src/auth/mod.rs`
- Create: `src-tauri/src/auth/argon.rs`
- Create: `src-tauri/src/auth/lockout.rs`
- Create: `src/stores/authStore.ts`
- Create: `src/hooks/useAuth.ts`
- Create: `src/app/login/page.tsx`
- Create: `src/app/onboarding/page.tsx`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Add argon hash/verify with test**

`src-tauri/src/auth/argon.rs`:

```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use crate::error::{AppError, AppResult};

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(password.as_bytes(), &salt).map(|hash| hash.to_string()).map_err(|err| AppError::Auth(err.to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let parsed = PasswordHash::new(hash).map_err(|err| AppError::Auth(err.to_string()))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verifies_matching_password_and_rejects_other_password() {
        let hash = hash_password("123456").unwrap();
        assert!(verify_password("123456", &hash).unwrap());
        assert!(!verify_password("000000", &hash).unwrap());
    }
}
```

- [ ] **Step 2: Add lockout helper with test**

`src-tauri/src/auth/lockout.rs`:

```rust
use chrono::{DateTime, Duration, Utc};
pub fn next_lock_until(failed_attempts: i64, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    match failed_attempts { 0..=4 => None, 5..=9 => Some(now + Duration::minutes(5)), 10..=14 => Some(now + Duration::minutes(30)), _ => Some(now + Duration::hours(24)) }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn locks_after_five_failures() {
        let now = Utc::now();
        assert!(next_lock_until(4, now).is_none());
        assert_eq!(next_lock_until(5, now).unwrap(), now + Duration::minutes(5));
    }
}
```

- [ ] **Step 3: Add frontend login skeleton**

`src/stores/authStore.ts`:

```ts
import { create } from 'zustand';
type AuthState = { isAuthenticated: boolean; setAuthenticated: (value: boolean) => void };
export const useAuthStore = create<AuthState>((set) => ({ isAuthenticated: false, setAuthenticated: (value) => set({ isAuthenticated: value }) }));
```

`src/hooks/useAuth.ts`:

```ts
import { useAuthStore } from '@/stores/authStore';
export function useAuth() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const setAuthenticated = useAuthStore((state) => state.setAuthenticated);
  return { isAuthenticated, setAuthenticated };
}
```

`src/app/login/page.tsx`:

```tsx
export default function LoginPage() {
  return (
    <main className="grid min-h-screen place-items-center bg-base-200 p-4">
      <section className="card w-full max-w-md bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Masuk untuk mengelola surat adminduk desa.</p>
          <input className="input input-bordered mt-4" type="password" placeholder="PIN atau password" />
          <button className="btn btn-primary mt-2">Masuk</button>
          <p className="mt-6 text-center text-xs text-base-content/50">EAS Creative Studio - https://eas.biz.id</p>
        </div>
      </section>
    </main>
  );
}
```

- [ ] **Step 4: Run checks and commit**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`
Expected: auth tests pass.

Run: `npm run build`
Expected: frontend build passes.

```bash
git add src src-tauri
git commit -m "feat: add authentication foundation"
```

## Task 4: Reference Data And Nomor Surat

**Files:**
- Create: `src-tauri/src/resources/reference/kode.json`
- Create: `src-tauri/src/service/nomor_surat.rs`
- Create: `src/lib/nomorSurat.ts`
- Create: `src/lib/nomorSurat.test.ts`

- [ ] **Step 1: Add `kode.json` reference data**

Create JSON groups for `alasan_pindah`, `klasifikasi_pindah`, `jenis_kepindahan`, `status_kk_pindah`, `status_kk_tidak_pindah`, `jenis_permohonan`, and `tipe_sponsor` using the values listed in the spec section "Master Kode Dropdown".

- [ ] **Step 2: Add frontend nomor surat renderer with test**

`src/lib/nomorSurat.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { renderNomorSurat } from './nomorSurat';

describe('renderNomorSurat', () => {
  it('renders padded sequence and roman month', () => {
    expect(renderNomorSurat('{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}', {
      seq: 42,
      kode: 'F-1.02',
      kodeDesa: 'DESA-X',
      tanggal: new Date('2026-05-20T00:00:00Z'),
    })).toBe('0042/F-1.02/DESA-X/V/2026');
  });
});
```

`src/lib/nomorSurat.ts`:

```ts
type NomorContext = { seq: number; kode: string; kodeDesa: string; tanggal: Date; custom?: Record<string, string> };
const ROMAWI = ['', 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X', 'XI', 'XII'];
export function renderNomorSurat(pattern: string, context: NomorContext): string {
  const month = context.tanggal.getUTCMonth() + 1;
  const year = context.tanggal.getUTCFullYear();
  const day = context.tanggal.getUTCDate();
  return pattern
    .replace(/\{seq:(\d+)\}/g, (_, width: string) => String(context.seq).padStart(Number(width), '0'))
    .replaceAll('{kode}', context.kode)
    .replaceAll('{kode_desa}', context.kodeDesa)
    .replaceAll('{tahun}', String(year))
    .replaceAll('{tahun_pendek}', String(year).slice(-2))
    .replaceAll('{bulan}', String(month).padStart(2, '0'))
    .replaceAll('{romawi:bulan}', ROMAWI[month])
    .replaceAll('{tanggal}', String(day).padStart(2, '0'))
    .replace(/\{custom:([A-Z0-9_]+)\}/g, (_, key: string) => context.custom?.[key] ?? '');
}
```

- [ ] **Step 3: Run tests and commit**

Run: `npm test -- src/lib/nomorSurat.test.ts`
Expected: PASS.

```bash
git add src src-tauri
git commit -m "feat: add reference data and nomor surat generator"
```

## Task 5: Buku Induk Import Parser

**Files:**
- Create: `tests/fixtures/buku_induk_minimal.csv`
- Create: `src-tauri/src/service/import_warga.rs`
- Create: `src/types/warga.ts`
- Create: `src/components/warga/ImportReview.tsx`
- Create: `src/app/warga/import/page.tsx`

- [ ] **Step 1: Add fixture**

`tests/fixtures/buku_induk_minimal.csv`:

```csv
NO. URUT,NOMOR KK,NIK,NAMA,JENIS KELAMIN,TEMPAT LAHIR,TANGGAL LAHIR,UMUR,,,,,,AGAMA,STATUS,HUBUNGAN KELUARGA,KEPALA KELUARGA,,PENDIDIKAN,PEKERJAAN,NAMA IBU,NAMA AYAH,ALAMAT,RT,RW,KET
1,3210221601060011,3210221506570081,SENA,L,MAJALENGKA,15-06-1957,67,,,,,,Islam,Kawin,Kepala Keluarga,L,,Tamat SD/Sederajat,Petani/Pekebun,SAWIT,ARTANI,BLOK DESA,001,001,
2,3210221601060011,3210224801590001,OMIH,P,MAJALENGKA,28-01-1959,66,,,,,,Islam,Kawin,Istri,,P,Tamat SD/Sederajat,Mengurus Rumah Tangga,UNIT,ENCUM,BLOK DESA,001,001,
3,3210221601060011,3210222503910001,DEDE ALIYUDIN,L,MAJALENGKA,25-03-1991,34,,,,,,Islam,Belum Kawin,Anak,,,Diploma IV/Strata I,Wiraswasta,OMIH,SENA,BLOK DESA,001,001,
```

- [ ] **Step 2: Add Rust parser with test**

Implement `parse_buku_induk_csv(input: &str) -> AppResult<Vec<ImportedPenduduk>>` in `src-tauri/src/service/import_warga.rs`. It maps columns exactly as defined in the design spec and validates NIK/No KK as 16 digits.

Test assertion:

```rust
#[test]
fn parses_buku_induk_fixture() {
    let csv = include_str!("../../../../tests/fixtures/buku_induk_minimal.csv");
    let rows = parse_buku_induk_csv(csv).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].nama_lengkap, "SENA");
    assert_eq!(rows[2].nama_ayah, "SENA");
}
```

- [ ] **Step 3: Add import review UI**

`src/types/warga.ts`:

```ts
export type ImportRowStatus = 'valid' | 'error' | 'conflict';
export type ImportPreviewRow = { rowNumber: number; nik: string; noKk: string; namaLengkap: string; status: ImportRowStatus; message?: string };
```

`src/components/warga/ImportReview.tsx` renders valid/error counts and a table with row number, NIK, No KK, name, and status.

- [ ] **Step 4: Run tests and commit**

Run: `cargo test service::import_warga::tests::parses_buku_induk_fixture --manifest-path src-tauri/Cargo.toml`
Expected: PASS.

Run: `npm run build`
Expected: PASS.

```bash
git add tests src src-tauri
git commit -m "feat: add Buku Induk import parser"
```

## Task 6: Parent NIK Resolver

**Files:**
- Create: `src-tauri/src/service/resolusi_ortu.rs`
- Create: `src/lib/validation.ts`

- [ ] **Step 1: Add resolver implementation with test**

`src-tauri/src/service/resolusi_ortu.rs` must define `FamilyMember`, `ResolutionStatus`, `ParentResolution`, and `resolve_parent`. Use `strsim::jaro_winkler`, relation filters from the spec, and thresholds: auto `>= 0.90`, confirm `>= 0.70`, manual below `0.70`.

Required test:

```rust
#[test]
fn resolves_father_inside_same_kk() {
    let members = vec![FamilyMember { nik: "3210221506570081".to_string(), nama: "SENA".to_string(), jenis_kelamin: "L".to_string(), hubungan_keluarga: "Kepala Keluarga".to_string() }];
    let result = resolve_parent("SENA", &members, &["Kepala Keluarga", "Suami", "Ayah"], "L");
    assert_eq!(result.nik.as_deref(), Some("3210221506570081"));
    assert_eq!(result.status, ResolutionStatus::Auto);
}
```

- [ ] **Step 2: Add frontend validation helpers**

`src/lib/validation.ts`:

```ts
export function isNik(value: string): boolean { return /^\d{16}$/.test(value); }
export function isNoKk(value: string): boolean { return /^\d{16}$/.test(value); }
```

- [ ] **Step 3: Run tests and commit**

Run: `cargo test service::resolusi_ortu::tests::resolves_father_inside_same_kk --manifest-path src-tauri/Cargo.toml`
Expected: PASS.

```bash
git add src src-tauri
git commit -m "feat: add automatic parent NIK resolver"
```

## Task 7: Handlebars Form Engine And F-1.02 Template

**Files:**
- Create: `src-tauri/src/resources/schemas/F-1.02.json`
- Create: `src-tauri/src/resources/templates/F-1.02.html`
- Create: `src/types/formulir.ts`
- Create: `src/lib/handlebars.ts`
- Create: `src/lib/handlebars.test.ts`
- Create: `tests/fixtures/f102_context.json`

- [ ] **Step 1: Add schema and template**

Create `F-1.02.json` with `kode`, `nama`, `kategori`, `ukuran_kertas`, `orientasi`, `versi_regulasi`, `versi_template`, `subjek`, `field`, `nomor_surat`, and `tanda_tangan` exactly as defined in the design spec.

Create `F-1.02.html` as an original Handlebars template using placeholders `{{individu.nama}}`, `{{individu.nik}}`, `{{individu.no_kk}}`, `{{config.nama_desa}}`, `{{tgl_indo tanggal_terbit}}`, `{{pejabat.nama}}`, and `{{pejabat.nipd}}`. Do not copy OpenSID template verbatim.

- [ ] **Step 2: Add renderer and test**

`src/lib/handlebars.ts`:

```ts
import Handlebars from 'handlebars';

Handlebars.registerHelper('tgl_indo', (value: string) => {
  const date = new Date(`${value}T00:00:00Z`);
  return new Intl.DateTimeFormat('id-ID', { day: '2-digit', month: 'long', year: 'numeric', timeZone: 'UTC' }).format(date);
});

export function renderTemplate(template: string, context: unknown): string {
  return Handlebars.compile(template)(context);
}
```

`src/lib/handlebars.test.ts`:

```ts
import { describe, expect, it } from 'vitest';
import { renderTemplate } from './handlebars';

describe('renderTemplate', () => {
  it('renders nested values and Indonesian date', () => {
    const html = renderTemplate('Nama: {{individu.nama}} - {{tgl_indo tanggal_terbit}}', { individu: { nama: 'SENA' }, tanggal_terbit: '2026-05-20' });
    expect(html).toContain('Nama: SENA');
    expect(html).toContain('20 Mei 2026');
  });
});
```

- [ ] **Step 3: Run tests and commit**

Run: `npm test -- src/lib/handlebars.test.ts`
Expected: PASS.

```bash
git add tests src src-tauri
git commit -m "feat: add F-1.02 form template engine"
```

## Task 8: Form Wizard, Preview, And Immutable History

**Files:**
- Create: `src/components/formulir/FormRenderer.tsx`
- Create: `src/components/formulir/PreviewPane.tsx`
- Create: `src/app/formulir/page.tsx`
- Create: `src/app/formulir/[kode]/buat/page.tsx`
- Create: `src-tauri/src/repo/riwayat.rs`
- Create: `src-tauri/src/commands/formulir.rs`

- [ ] **Step 1: Build wizard UI**

Create a 3-step wizard: pilih warga/manual input, field tambahan, preview/cetak. It must show unresolved required fields as red inputs and allow write-back later when backend commands exist.

- [ ] **Step 2: Build preview pane**

`PreviewPane` accepts `html: string` and renders it in a sandboxed iframe:

```tsx
export function PreviewPane({ html }: { html: string }) {
  return <iframe className="h-[80vh] w-full rounded-box border border-base-300 bg-white" srcDoc={html} title="Preview formulir" />;
}
```

- [ ] **Step 3: Add history command contract**

`src-tauri/src/commands/formulir.rs` must expose `commit_riwayat_formulir(payload)` and write `data_snapshot`, `pejabat_snapshot`, `template_snapshot`, `riwayat_subjek`, and `audit_log` in one transaction.

- [ ] **Step 4: Run checks and commit**

Run: `npm run build`
Expected: PASS.

Run: `cargo test --manifest-path src-tauri/Cargo.toml`
Expected: PASS.

```bash
git add src src-tauri
git commit -m "feat: add form wizard and immutable history"
```

## Task 9: PDF Export And Print Route

**Files:**
- Create: `src/app/print/[draftId]/page.tsx`
- Create: `src-tauri/src/service/pdf.rs`
- Create: `src-tauri/src/commands/formulir.rs` additions

- [ ] **Step 1: Add print route**

Print route loads rendered HTML by draft ID and emits a `render-ready` event after DOM content is ready.

- [ ] **Step 2: Add Rust PDF service**

`pdf.rs` creates a hidden Tauri webview, loads the print route, waits for `render-ready`, exports PDF to `exports/`, computes SHA-256, and returns relative path and hash.

- [ ] **Step 3: Verify and commit**

Run: manual `npm run tauri dev`, create F-1.02 preview, export PDF.
Expected: PDF appears in `exports/`, history row stores path and hash.

```bash
git add src src-tauri
git commit -m "feat: add PDF export pipeline"
```

## Task 10: Backup, Restore, Audit, And Settings

**Files:**
- Create: `src-tauri/src/service/backup.rs`
- Create: `src-tauri/src/repo/audit.rs`
- Create: `src-tauri/src/repo/backup.rs`
- Create: `src-tauri/src/commands/backup.rs`
- Create: `src-tauri/src/commands/audit.rs`
- Create: `src/app/pengaturan/page.tsx`
- Create: `src/app/pengaturan/tentang/page.tsx`

- [ ] **Step 1: Add backup service test**

Test creates a temporary SQLite file, calls backup copy, verifies destination exists and SHA-256 is not empty.

- [ ] **Step 2: Implement manual backup and pre-restore backup**

Before restore, always copy current DB to `backups/pre-restore/<timestamp>.db`, run `PRAGMA integrity_check` on source backup, then replace `nawala.db`.

- [ ] **Step 3: Add audit repository**

Every command mutating data writes an audit row with `ts`, `aksi`, `entitas`, `entitas_id`, `ringkasan`, and `metadata` JSON.

- [ ] **Step 4: Add settings UI**

Create tabs/cards for Identitas Desa, Pejabat, Nomor Surat, Tampilan, Keamanan, Backup, Audit Log, and Tentang. Tentang must show Nawala, EAS Creative Studio, website, emails, MIT license.

- [ ] **Step 5: Verify and commit**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`
Expected: backup tests pass.

Run: `npm run build`
Expected: PASS.

```bash
git add src src-tauri
git commit -m "feat: add backup settings and audit log"
```

## Task 11: Full Form Catalogue Research And Templates

**Files:**
- Add: `src-tauri/src/resources/schemas/*.json`
- Add: `src-tauri/src/resources/templates/*.html`
- Add: `docs/formulir/research.md`
- Add: `tests/fixtures/formulir/*.json`

- [ ] **Step 1: Document source per form**

Create `docs/formulir/research.md` with columns: kode, nama, kategori, source URL/file, OpenSID reference path, Permendagri page/reference, implementation status, QA status.

- [ ] **Step 2: Implement Tier 1 forms**

Implement original Handlebars templates and schemas for: F-1.01, F-1.02, F-1.03, F-1.06, F-1.08, F-1.15, F-1.16, F-1.25, F-1.27, F-2.01 umum, F-2.01 kelahiran, F-2.01 kematian, F-2.12, F-2.29, F-2.30.

- [ ] **Step 3: Implement Tier 2 forms**

Use Permendagri 73/2022 PDF and official references to complete remaining F forms. Each form must have schema, template, fixture context, and QA checklist entry.

- [ ] **Step 4: Visual QA per form**

For each form, generate sample PDF and compare side-by-side with official/reference layout. Record pass/fail in `docs/formulir/research.md`.

- [ ] **Step 5: Commit catalogue**

```bash
git add docs src-tauri tests
git commit -m "feat: add full form catalogue"
```

## Task 12: Packaging And Release

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/release.yml`

- [ ] **Step 1: Add CI workflow**

Run checks on pull/push: `npm ci`, `npm run build`, `npm test`, `cargo test --manifest-path src-tauri/Cargo.toml`.

- [ ] **Step 2: Add release workflow**

Build Windows MSI and Linux AppImage/deb on tags `v*` using Tauri build.

- [ ] **Step 3: Verify local packaging**

Run: `npm run tauri build`
Expected: installer artifacts generated under `src-tauri/target/release/bundle/`.

- [ ] **Step 4: Commit CI/release**

```bash
git add .github src-tauri/tauri.conf.json
git commit -m "ci: add build and release workflows"
```

## Self-Review Checklist

- Spec coverage: app identity, Tauri/Next stack, SQLite schema, import, resolver, form engine, PDF, history, backup, audit, settings, form catalogue, packaging are mapped to tasks.
- Placeholder scan: this plan intentionally contains no placeholder tasks that block execution. Research work for Tier 2 is explicit and has concrete output (`docs/formulir/research.md`).
- Type consistency: product name is Nawala, bundle identifier is `id.biz.eas.nawala`, DB file is `nawala.db`, and NIPD is used for pejabat.
