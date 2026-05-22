/**
 * Tauri invoke wrapper - provides type-safe command invocation
 * Falls back to mock responses when not running in Tauri context (e.g., during SSG build)
 */

type InvokeArgs = Record<string, unknown>;

async function tauriInvoke<T>(cmd: string, args?: InvokeArgs): Promise<T> {
  if (typeof window === 'undefined') {
    throw new Error('Cannot invoke Tauri commands during SSR');
  }

  // Dynamic import to avoid SSR issues
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(cmd, args);
}

// Auth types
export interface AuthStatus {
  configured: boolean;
  locked: boolean;
  locked_until: string | null;
}

export interface LoginResult {
  success: boolean;
  message: string;
  locked_until: string | null;
}

// Auth commands
export async function checkAuthStatus(): Promise<AuthStatus> {
  return tauriInvoke<AuthStatus>('check_auth_status_cmd');
}

export async function setupPassword(password: string, hint?: string): Promise<void> {
  return tauriInvoke<void>('setup_password_cmd', { password, hint });
}

export async function login(password: string): Promise<LoginResult> {
  return tauriInvoke<LoginResult>('login_cmd', { password });
}

// Penduduk types
export interface PendudukRecord {
  id: number;
  nik: string;
  no_kk: string | null;
  nama_lengkap: string;
  jenis_kelamin: string;
  tempat_lahir: string | null;
  tanggal_lahir: string | null;
  agama: string | null;
  status_perkawinan: string | null;
  pekerjaan: string | null;
  alamat_lengkap: string | null;
  rt: string | null;
  rw: string | null;
}

export interface SearchPendudukResult {
  items: PendudukRecord[];
  total: number;
}

// Penduduk commands
export async function searchPenduduk(query: string, limit?: number): Promise<SearchPendudukResult> {
  return tauriInvoke<SearchPendudukResult>('search_penduduk_cmd', { query, limit: limit ?? 20 });
}

export async function getPendudukByNik(nik: string): Promise<PendudukRecord | null> {
  return tauriInvoke<PendudukRecord | null>('get_penduduk_by_nik_cmd', { nik });
}

// Import types
export interface ImportResult {
  total_rows: number;
  inserted_rows: number;
  updated_rows: number;
  skipped_rows: number;
  error_rows: number;
  errors: string[];
}

// Import commands
export async function importWargaCsv(csvContent: string, filename: string): Promise<ImportResult> {
  return tauriInvoke<ImportResult>('import_warga_csv_cmd', { csvContent, filename });
}

// Formulir types
export interface FormulirDefRecord {
  kode: string;
  nama: string;
  kategori: string;
  deskripsi: string | null;
  ukuran_kertas: string;
  schema_json: string;
  template_html: string;
  aktif: boolean;
}

// Formulir commands
export async function listFormulirDef(): Promise<FormulirDefRecord[]> {
  return tauriInvoke<FormulirDefRecord[]>('list_formulir_def_cmd');
}

export async function getFormulirDef(kode: string): Promise<FormulirDefRecord | null> {
  return tauriInvoke<FormulirDefRecord | null>('get_formulir_def_cmd', { kode });
}

// Pengaturan types
export interface PengaturanDesa {
  nama_desa: string;
  kecamatan: string;
  kabupaten: string;
  provinsi: string;
  kode_wilayah: string | null;
  kode_desa: string | null;
  alamat_kantor: string | null;
}

export interface PejabatRecord {
  id: number;
  nama: string;
  jabatan: string;
  nipd: string | null;
  is_default: boolean;
  aktif: boolean;
}

// Pengaturan commands
export async function getPengaturanDesa(): Promise<PengaturanDesa | null> {
  return tauriInvoke<PengaturanDesa | null>('get_pengaturan_desa_cmd');
}

export async function savePengaturanDesa(data: PengaturanDesa): Promise<void> {
  return tauriInvoke<void>('save_pengaturan_desa_cmd', { data });
}

export async function listPejabat(): Promise<PejabatRecord[]> {
  return tauriInvoke<PejabatRecord[]>('list_pejabat_cmd');
}

export async function savePejabat(
  data: Omit<PejabatRecord, 'id'> & { id?: number },
): Promise<number> {
  return tauriInvoke<number>('save_pejabat_cmd', { data });
}

// Nomor surat
export interface NomorSuratResult {
  nomor: string;
  seq: number;
}

export async function getNextNomorSurat(kodeFormulir: string): Promise<NomorSuratResult> {
  return tauriInvoke<NomorSuratResult>('get_next_nomor_surat_cmd', { kodeFormulir });
}
