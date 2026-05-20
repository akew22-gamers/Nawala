/**
 * Type definitions for form schemas and rendering context
 */

export interface FormSubject {
  kode: string;
  label: string;
  wajib: boolean;
  sumber: string;
}

export interface FormField {
  kode: string;
  label: string;
  tipe: string;
  wajib: boolean;
  opsi_ref?: string;
}

export interface FormNumbering {
  pakai: boolean;
  pola: string;
}

export interface FormSignature {
  pejabat_default: string;
}

export interface FormSchema {
  kode: string;
  nama: string;
  kategori: string;
  ukuran_kertas: string;
  orientasi: string;
  versi_regulasi: string;
  versi_template: number;
  subjek: FormSubject[];
  field: FormField[];
  nomor_surat: FormNumbering;
  tanda_tangan: FormSignature;
}

export interface IndividuData {
  nama: string;
  nik: string;
  no_kk: string;
}

export interface ConfigData {
  nama_desa: string;
}

export interface PejabatData {
  nama: string;
  nipd: string;
}

export interface FormRenderContext {
  individu: IndividuData;
  config: ConfigData;
  tanggal_terbit: string;
  pejabat: PejabatData;
  jenis_permohonan: string;
  nomor_surat?: string;
}
