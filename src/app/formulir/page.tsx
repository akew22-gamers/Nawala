/**
 * Formulir List Page - Browse available forms
 */

'use client';

import Link from 'next/link';
import { useEffect, useState } from 'react';

interface FormulirItem {
  kode: string;
  nama: string;
  kategori: string;
  deskripsi?: string;
}

export default function FormulirPage() {
  const [formulirs, setFormulirs] = useState<FormulirItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState('');

  useEffect(() => {
    // TODO: Load from Tauri command
    // Placeholder data
    setFormulirs([
      {
        kode: 'SKCK',
        nama: 'Surat Keterangan Catatan Kepolisian',
        kategori: 'Keterangan',
        deskripsi: 'Surat pengantar SKCK',
      },
      {
        kode: 'SKTM',
        nama: 'Surat Keterangan Tidak Mampu',
        kategori: 'Keterangan',
        deskripsi: 'Surat keterangan ekonomi tidak mampu',
      },
    ]);
    setLoading(false);
  }, []);

  const filteredFormulirs = formulirs.filter(
    (f) =>
      f.nama.toLowerCase().includes(filter.toLowerCase()) ||
      f.kode.toLowerCase().includes(filter.toLowerCase()),
  );

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <span className="loading loading-spinner loading-lg" />
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">Formulir Surat</h1>
        <p className="text-base-content/70">Pilih jenis surat yang akan dibuat</p>
      </div>

      <div className="mb-4">
        <input
          type="text"
          placeholder="Cari formulir..."
          className="input input-bordered w-full max-w-md"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredFormulirs.map((formulir) => (
          <Link
            key={formulir.kode}
            href={`/formulir/${formulir.kode}/buat`}
            className="card bg-base-200 hover:bg-base-300 transition-colors"
          >
            <div className="card-body">
              <h2 className="card-title">{formulir.nama}</h2>
              <p className="text-sm text-base-content/70">Kode: {formulir.kode}</p>
              {formulir.deskripsi && <p className="text-sm">{formulir.deskripsi}</p>}
              <div className="card-actions justify-end mt-2">
                <span className="badge badge-primary">{formulir.kategori}</span>
              </div>
            </div>
          </Link>
        ))}
      </div>

      {filteredFormulirs.length === 0 && (
        <div className="text-center py-12">
          <p className="text-base-content/70">Tidak ada formulir ditemukan</p>
        </div>
      )}
    </div>
  );
}
