'use client';

import Link from 'next/link';
import { useCallback, useEffect, useState } from 'react';
import {
  getPengaturanDesa,
  listPejabat,
  type PejabatRecord,
  type PengaturanDesa,
  savePejabat,
  savePengaturanDesa,
} from '@/lib/tauri';

export default function PengaturanPage() {
  const [activeTab, setActiveTab] = useState<'desa' | 'pejabat'>('desa');
  const [desa, setDesa] = useState<PengaturanDesa>({
    nama_desa: '',
    kecamatan: '',
    kabupaten: '',
    provinsi: '',
    kode_wilayah: null,
    kode_desa: null,
    alamat_kantor: null,
  });
  const [pejabatList, setPejabatList] = useState<PejabatRecord[]>([]);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState('');
  const [error, setError] = useState('');

  // New pejabat form
  const [newPejabat, setNewPejabat] = useState({ nama: '', jabatan: '', nipd: '' });

  const loadData = useCallback(async () => {
    try {
      const desaData = await getPengaturanDesa();
      if (desaData) setDesa(desaData);
      const pejabats = await listPejabat();
      setPejabatList(pejabats);
    } catch {
      // Not in Tauri context
    }
  }, []);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleSaveDesa = async () => {
    if (!desa.nama_desa || !desa.kecamatan || !desa.kabupaten || !desa.provinsi) {
      setError('Nama desa, kecamatan, kabupaten, dan provinsi wajib diisi');
      return;
    }
    setSaving(true);
    setError('');
    setMessage('');
    try {
      await savePengaturanDesa(desa);
      setMessage('Pengaturan desa berhasil disimpan');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Gagal menyimpan');
    } finally {
      setSaving(false);
    }
  };

  const handleAddPejabat = async () => {
    if (!newPejabat.nama || !newPejabat.jabatan) {
      setError('Nama dan jabatan pejabat wajib diisi');
      return;
    }
    setSaving(true);
    setError('');
    setMessage('');
    try {
      await savePejabat({
        id: 0,
        nama: newPejabat.nama,
        jabatan: newPejabat.jabatan,
        nipd: newPejabat.nipd || null,
        is_default: pejabatList.length === 0,
        aktif: true,
      });
      setNewPejabat({ nama: '', jabatan: '', nipd: '' });
      setMessage('Pejabat berhasil ditambahkan');
      const pejabats = await listPejabat();
      setPejabatList(pejabats);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Gagal menyimpan pejabat');
    } finally {
      setSaving(false);
    }
  };

  return (
    <main className="min-h-screen bg-base-200 p-6">
      <section className="mx-auto max-w-4xl space-y-6">
        <div className="rounded-box bg-base-100 p-8 shadow-sm">
          <p className="text-sm uppercase tracking-[0.3em] text-primary">Pengaturan</p>
          <h1 className="mt-3 text-4xl font-bold">Pusat Kendali Nawala</h1>
          <p className="mt-2 max-w-2xl text-base-content/70">
            Kelola konfigurasi desa dan pejabat penanda tangan.
          </p>
        </div>

        {message && (
          <div className="alert alert-success">
            <span>{message}</span>
          </div>
        )}
        {error && (
          <div className="alert alert-error">
            <span>{error}</span>
          </div>
        )}

        <div role="tablist" className="tabs tabs-boxed">
          <button
            type="button"
            role="tab"
            className={`tab ${activeTab === 'desa' ? 'tab-active' : ''}`}
            onClick={() => setActiveTab('desa')}
          >
            Identitas Desa
          </button>
          <button
            type="button"
            role="tab"
            className={`tab ${activeTab === 'pejabat' ? 'tab-active' : ''}`}
            onClick={() => setActiveTab('pejabat')}
          >
            Pejabat
          </button>
        </div>

        {activeTab === 'desa' && (
          <div className="card bg-base-100 shadow-sm">
            <div className="card-body">
              <h2 className="card-title">Identitas Desa</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
                <div className="form-control">
                  <label className="label" htmlFor="nama_desa">
                    <span className="label-text">Nama Desa *</span>
                  </label>
                  <input
                    id="nama_desa"
                    type="text"
                    className="input input-bordered"
                    value={desa.nama_desa}
                    onChange={(e) => setDesa({ ...desa, nama_desa: e.target.value })}
                  />
                </div>
                <div className="form-control">
                  <label className="label" htmlFor="kecamatan">
                    <span className="label-text">Kecamatan *</span>
                  </label>
                  <input
                    id="kecamatan"
                    type="text"
                    className="input input-bordered"
                    value={desa.kecamatan}
                    onChange={(e) => setDesa({ ...desa, kecamatan: e.target.value })}
                  />
                </div>
                <div className="form-control">
                  <label className="label" htmlFor="kabupaten">
                    <span className="label-text">Kabupaten *</span>
                  </label>
                  <input
                    id="kabupaten"
                    type="text"
                    className="input input-bordered"
                    value={desa.kabupaten}
                    onChange={(e) => setDesa({ ...desa, kabupaten: e.target.value })}
                  />
                </div>
                <div className="form-control">
                  <label className="label" htmlFor="provinsi">
                    <span className="label-text">Provinsi *</span>
                  </label>
                  <input
                    id="provinsi"
                    type="text"
                    className="input input-bordered"
                    value={desa.provinsi}
                    onChange={(e) => setDesa({ ...desa, provinsi: e.target.value })}
                  />
                </div>
                <div className="form-control">
                  <label className="label" htmlFor="kode_desa">
                    <span className="label-text">Kode Desa</span>
                  </label>
                  <input
                    id="kode_desa"
                    type="text"
                    className="input input-bordered"
                    value={desa.kode_desa || ''}
                    onChange={(e) => setDesa({ ...desa, kode_desa: e.target.value || null })}
                  />
                </div>
                <div className="form-control">
                  <label className="label" htmlFor="kode_wilayah">
                    <span className="label-text">Kode Wilayah</span>
                  </label>
                  <input
                    id="kode_wilayah"
                    type="text"
                    className="input input-bordered"
                    value={desa.kode_wilayah || ''}
                    onChange={(e) => setDesa({ ...desa, kode_wilayah: e.target.value || null })}
                  />
                </div>
                <div className="form-control md:col-span-2">
                  <label className="label" htmlFor="alamat_kantor">
                    <span className="label-text">Alamat Kantor</span>
                  </label>
                  <input
                    id="alamat_kantor"
                    type="text"
                    className="input input-bordered"
                    value={desa.alamat_kantor || ''}
                    onChange={(e) => setDesa({ ...desa, alamat_kantor: e.target.value || null })}
                  />
                </div>
              </div>
              <div className="card-actions justify-end mt-4">
                <button
                  type="button"
                  className="btn btn-primary"
                  onClick={handleSaveDesa}
                  disabled={saving}
                >
                  {saving ? <span className="loading loading-spinner loading-sm" /> : 'Simpan'}
                </button>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'pejabat' && (
          <div className="space-y-4">
            <div className="card bg-base-100 shadow-sm">
              <div className="card-body">
                <h2 className="card-title">Tambah Pejabat</h2>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                  <div className="form-control">
                    <label className="label" htmlFor="pejabat_nama">
                      <span className="label-text">Nama *</span>
                    </label>
                    <input
                      id="pejabat_nama"
                      type="text"
                      className="input input-bordered"
                      value={newPejabat.nama}
                      onChange={(e) => setNewPejabat({ ...newPejabat, nama: e.target.value })}
                    />
                  </div>
                  <div className="form-control">
                    <label className="label" htmlFor="pejabat_jabatan">
                      <span className="label-text">Jabatan *</span>
                    </label>
                    <input
                      id="pejabat_jabatan"
                      type="text"
                      className="input input-bordered"
                      placeholder="Kepala Desa"
                      value={newPejabat.jabatan}
                      onChange={(e) => setNewPejabat({ ...newPejabat, jabatan: e.target.value })}
                    />
                  </div>
                  <div className="form-control">
                    <label className="label" htmlFor="pejabat_nipd">
                      <span className="label-text">NIPD</span>
                    </label>
                    <input
                      id="pejabat_nipd"
                      type="text"
                      className="input input-bordered"
                      value={newPejabat.nipd}
                      onChange={(e) => setNewPejabat({ ...newPejabat, nipd: e.target.value })}
                    />
                  </div>
                </div>
                <div className="card-actions justify-end mt-4">
                  <button
                    type="button"
                    className="btn btn-primary"
                    onClick={handleAddPejabat}
                    disabled={saving}
                  >
                    Tambah Pejabat
                  </button>
                </div>
              </div>
            </div>

            <div className="card bg-base-100 shadow-sm">
              <div className="card-body">
                <h2 className="card-title">Daftar Pejabat</h2>
                {pejabatList.length === 0 ? (
                  <p className="text-base-content/70">Belum ada pejabat terdaftar.</p>
                ) : (
                  <div className="overflow-x-auto">
                    <table className="table">
                      <thead>
                        <tr>
                          <th>Nama</th>
                          <th>Jabatan</th>
                          <th>NIPD</th>
                          <th>Default</th>
                        </tr>
                      </thead>
                      <tbody>
                        {pejabatList.map((p) => (
                          <tr key={p.id}>
                            <td>{p.nama}</td>
                            <td>{p.jabatan}</td>
                            <td>{p.nipd || '-'}</td>
                            <td>{p.is_default ? '✓' : ''}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        <div className="flex gap-2">
          <Link href="/pengaturan/tentang" className="btn btn-ghost btn-sm">
            Tentang Aplikasi
          </Link>
          <Link href="/" className="btn btn-ghost btn-sm">
            Kembali ke Dashboard
          </Link>
        </div>
      </section>
    </main>
  );
}
