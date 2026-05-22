'use client';

import Link from 'next/link';
import { useEffect, useState } from 'react';
import { listFormulirDef } from '@/lib/tauri';

interface FormulirItem {
  kode: string;
  nama: string;
  kategori: string;
  deskripsi?: string | null;
}

export default function FormulirPage() {
  const [formulirs, setFormulirs] = useState<FormulirItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState('');

  useEffect(() => {
    listFormulirDef()
      .then((items) => {
        setFormulirs(
          items.map((f) => ({
            kode: f.kode,
            nama: f.nama,
            kategori: f.kategori,
            deskripsi: f.deskripsi,
          })),
        );
        setLoading(false);
      })
      .catch(() => {
        // Fallback for non-Tauri context (SSG build)
        setFormulirs([]);
        setLoading(false);
      });
  }, []);

  const filteredFormulirs = formulirs.filter(
    (f) =>
      f.nama.toLowerCase().includes(filter.toLowerCase()) ||
      f.kode.toLowerCase().includes(filter.toLowerCase()),
  );

  // Group by kategori
  const grouped = filteredFormulirs.reduce(
    (acc, f) => {
      if (!acc[f.kategori]) acc[f.kategori] = [];
      acc[f.kategori].push(f);
      return acc;
    },
    {} as Record<string, FormulirItem[]>,
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
          aria-label="Cari formulir"
        />
      </div>

      {formulirs.length === 0 && !loading && (
        <div className="alert alert-info">
          <span>Belum ada formulir tersedia. Pastikan aplikasi berjalan dalam konteks Tauri.</span>
        </div>
      )}

      {Object.entries(grouped).map(([kategori, items]) => (
        <div key={kategori} className="mb-6">
          <h2 className="text-xl font-semibold mb-3 text-primary">{kategori}</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {items.map((formulir) => (
              <Link
                key={formulir.kode}
                href={`/formulir/${formulir.kode}/buat`}
                className="card bg-base-200 hover:bg-base-300 transition-colors"
              >
                <div className="card-body">
                  <h3 className="card-title text-base">{formulir.nama}</h3>
                  <p className="text-sm text-base-content/70">Kode: {formulir.kode}</p>
                  {formulir.deskripsi && <p className="text-sm">{formulir.deskripsi}</p>}
                  <div className="card-actions justify-end mt-2">
                    <span className="badge badge-primary badge-sm">{formulir.kategori}</span>
                  </div>
                </div>
              </Link>
            ))}
          </div>
        </div>
      ))}

      {filteredFormulirs.length === 0 && formulirs.length > 0 && (
        <div className="text-center py-12">
          <p className="text-base-content/70">
            Tidak ada formulir ditemukan untuk &quot;{filter}&quot;
          </p>
        </div>
      )}
    </div>
  );
}
