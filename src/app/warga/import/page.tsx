'use client';

import ImportReview from '@/components/warga/ImportReview';
import type { ImportPreviewRow } from '@/types/warga';

export default function ImportWargaPage() {
  // Sample data for demonstration
  const sampleRows: ImportPreviewRow[] = [
    {
      rowNumber: 1,
      nik: '3210221506570081',
      noKk: '3210221601060011',
      namaLengkap: 'SENA',
      status: 'valid',
    },
    {
      rowNumber: 2,
      nik: '3210224801590001',
      noKk: '3210221601060011',
      namaLengkap: 'OMIH',
      status: 'valid',
    },
    {
      rowNumber: 3,
      nik: '3210222503910001',
      noKk: '3210221601060011',
      namaLengkap: 'DEDE ALIYUDIN',
      status: 'valid',
    },
  ];

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">Import Data Warga</h1>
        <p className="text-gray-600 mt-2">Upload file Buku Induk untuk mengimpor data warga</p>
      </div>

      <div className="card bg-base-100 shadow-xl mb-6">
        <div className="card-body">
          <h2 className="card-title">Upload File</h2>
          <label className="form-control w-full max-w-xs">
            <span className="label-text">File Buku Induk CSV</span>
            <input type="file" accept=".csv" className="file-input file-input-bordered w-full" />
          </label>
          <div className="card-actions justify-end mt-4">
            <button type="button" className="btn btn-primary">
              Proses Import
            </button>
          </div>
        </div>
      </div>

      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          <h2 className="card-title">Preview Import</h2>
          {sampleRows.length > 0 ? (
            <ImportReview rows={sampleRows} />
          ) : (
            <div className="text-center py-8 text-gray-500">
              Belum ada data untuk dipreview. Upload file untuk memulai.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
