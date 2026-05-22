'use client';

import { useState } from 'react';
import ImportReview from '@/components/warga/ImportReview';
import { importWargaCsv } from '@/lib/tauri';
import type { ImportPreviewRow } from '@/types/warga';

interface ImportResultData {
  total_rows: number;
  inserted_rows: number;
  updated_rows: number;
  skipped_rows: number;
  error_rows: number;
  errors: string[];
}

export default function ImportWargaPage() {
  const [previewRows, setPreviewRows] = useState<ImportPreviewRow[]>([]);
  const [csvContent, setCsvContent] = useState<string>('');
  const [filename, setFilename] = useState<string>('');
  const [importing, setImporting] = useState(false);
  const [result, setResult] = useState<ImportResultData | null>(null);
  const [error, setError] = useState('');

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setFilename(file.name);
    setResult(null);
    setError('');

    const reader = new FileReader();
    reader.onload = (ev) => {
      const text = ev.target?.result as string;
      setCsvContent(text);

      // Parse preview
      const lines = text.split('\n').filter((l) => l.trim());
      if (lines.length < 2) {
        setError('File CSV kosong atau tidak valid');
        setPreviewRows([]);
        return;
      }

      const headers = lines[0].split(',').map((h) => h.trim().replace(/"/g, ''));
      const nikIdx = headers.indexOf('NIK');
      const noKkIdx = headers.indexOf('NOMOR KK');
      const namaIdx = headers.indexOf('NAMA');

      if (nikIdx === -1 || noKkIdx === -1 || namaIdx === -1) {
        setError('Header CSV tidak valid. Harus ada kolom NIK, NOMOR KK, dan NAMA.');
        setPreviewRows([]);
        return;
      }

      const rows: ImportPreviewRow[] = [];
      for (let i = 1; i < Math.min(lines.length, 51); i++) {
        const cols = lines[i].split(',').map((c) => c.trim().replace(/"/g, ''));
        const nik = cols[nikIdx] || '';
        const noKk = cols[noKkIdx] || '';
        const nama = cols[namaIdx] || '';

        const isNikValid = /^\d{16}$/.test(nik);
        const isKkValid = /^\d{16}$/.test(noKk);

        rows.push({
          rowNumber: i,
          nik,
          noKk,
          namaLengkap: nama,
          status: isNikValid && isKkValid ? 'valid' : 'error',
          message: !isNikValid
            ? 'NIK tidak valid (harus 16 digit)'
            : !isKkValid
              ? 'No KK tidak valid (harus 16 digit)'
              : undefined,
        });
      }
      setPreviewRows(rows);
    };
    reader.readAsText(file);
  };

  const handleImport = async () => {
    if (!csvContent) {
      setError('Pilih file CSV terlebih dahulu');
      return;
    }

    setImporting(true);
    setError('');
    try {
      const res = await importWargaCsv(csvContent, filename);
      setResult(res);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setImporting(false);
    }
  };

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">Import Data Warga</h1>
        <p className="text-base-content/70 mt-2">
          Upload file Buku Induk CSV untuk mengimpor data warga ke database
        </p>
      </div>

      {error && (
        <div className="alert alert-error mb-4">
          <span>{error}</span>
        </div>
      )}

      {result && (
        <div className="alert alert-success mb-4">
          <div>
            <p className="font-bold">Import selesai</p>
            <p>
              Total: {result.total_rows} | Baru: {result.inserted_rows} | Update:{' '}
              {result.updated_rows} | Error: {result.error_rows}
            </p>
            {result.errors.length > 0 && (
              <details className="mt-2">
                <summary className="cursor-pointer">Lihat error ({result.errors.length})</summary>
                <ul className="list-disc pl-4 mt-1 text-sm">
                  {result.errors.map((e) => (
                    <li key={e}>{e}</li>
                  ))}
                </ul>
              </details>
            )}
          </div>
        </div>
      )}

      <div className="card bg-base-100 shadow-xl mb-6">
        <div className="card-body">
          <h2 className="card-title">Upload File</h2>
          <p className="text-sm text-base-content/70 mb-2">
            Format: CSV dengan header NOMOR KK, NIK, NAMA, JENIS KELAMIN, TEMPAT LAHIR, TANGGAL
            LAHIR, AGAMA, STATUS, HUBUNGAN KELUARGA, PENDIDIKAN, PEKERJAAN, NAMA IBU, NAMA AYAH,
            ALAMAT, RT, RW, KET
          </p>
          <label className="form-control w-full max-w-md">
            <span className="label-text sr-only">File Buku Induk CSV</span>
            <input
              type="file"
              accept=".csv"
              className="file-input file-input-bordered w-full"
              onChange={handleFileChange}
              aria-label="Pilih file CSV"
            />
          </label>
          <div className="card-actions justify-end mt-4">
            <button
              type="button"
              className="btn btn-primary"
              onClick={handleImport}
              disabled={!csvContent || importing}
            >
              {importing ? (
                <span className="loading loading-spinner loading-sm" />
              ) : (
                'Proses Import'
              )}
            </button>
          </div>
        </div>
      </div>

      {previewRows.length > 0 && (
        <div className="card bg-base-100 shadow-xl">
          <div className="card-body">
            <h2 className="card-title">
              Preview Import ({previewRows.length} baris{previewRows.length >= 50 ? '+' : ''})
            </h2>
            <ImportReview rows={previewRows} />
          </div>
        </div>
      )}
    </div>
  );
}
