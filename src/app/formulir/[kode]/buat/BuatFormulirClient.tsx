'use client';

import { useParams, useRouter } from 'next/navigation';
import { useCallback, useEffect, useState } from 'react';
import { FormRenderer, SubjectSelector } from '@/components/formulir/FormRenderer';
import { PreviewPane } from '@/components/formulir/PreviewPane';
import {
  getFormulirDef,
  getNextNomorSurat,
  type PendudukRecord,
  searchPenduduk,
} from '@/lib/tauri';
import type { FormSchema } from '@/types/formulir';

type WizardStep = 1 | 2 | 3;

interface SubjectData {
  nik: string;
  nama: string;
}

export default function BuatFormulirClient() {
  const params = useParams();
  const router = useRouter();
  const kode = params.kode as string;

  const [step, setStep] = useState<WizardStep>(1);
  const [schema, setSchema] = useState<FormSchema | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState('');

  // Form data
  const [subjects, setSubjects] = useState<Record<string, SubjectData>>({});
  const [fieldValues, setFieldValues] = useState<Record<string, string>>({});
  const [previewHtml, setPreviewHtml] = useState<string>('');
  const [nomorSurat, setNomorSurat] = useState<string>('');

  // Search
  const [searchResults, setSearchResults] = useState<PendudukRecord[]>([]);
  const [searching, setSearching] = useState(false);

  // Validation errors
  const [errors, setErrors] = useState<Record<string, string>>({});

  // Submission
  const [submitting, setSubmitting] = useState(false);

  const loadSchema = useCallback(async () => {
    try {
      const def = await getFormulirDef(kode);
      if (def) {
        const parsed: FormSchema = JSON.parse(def.schema_json);
        setSchema(parsed);
      } else {
        setLoadError(`Formulir ${kode} tidak ditemukan`);
      }
    } catch {
      // Fallback for non-Tauri context
      setSchema({
        kode: kode.toUpperCase(),
        nama: `Formulir ${kode.toUpperCase()}`,
        kategori: 'Umum',
        ukuran_kertas: 'F4',
        orientasi: 'portrait',
        versi_regulasi: 'Permendagri 73/2022',
        versi_template: 1,
        subjek: [{ kode: 'individu', label: 'Pemohon', wajib: true, sumber: 'penduduk' }],
        field: [
          { kode: 'keperluan', label: 'Keperluan', tipe: 'textarea', wajib: true },
          { kode: 'tanggal_terbit', label: 'Tanggal Terbit', tipe: 'date', wajib: true },
        ],
        nomor_surat: { pakai: true, pola: '{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}' },
        tanda_tangan: { pejabat_default: 'Kepala Desa' },
      });
    } finally {
      setLoading(false);
    }
  }, [kode]);

  useEffect(() => {
    loadSchema();
  }, [loadSchema]);

  const handleSearchPenduduk = useCallback(async (query: string) => {
    if (query.length < 2) {
      setSearchResults([]);
      return;
    }
    setSearching(true);
    try {
      const result = await searchPenduduk(query, 10);
      setSearchResults(result.items);
    } catch {
      setSearchResults([]);
    } finally {
      setSearching(false);
    }
  }, []);

  const validateStep = (currentStep: WizardStep): boolean => {
    const newErrors: Record<string, string> = {};

    if (currentStep === 1) {
      schema?.subjek.forEach((subject) => {
        if (subject.wajib && !subjects[subject.kode]) {
          newErrors[`subject_${subject.kode}`] = `${subject.label} wajib diisi`;
        }
      });
    }

    if (currentStep === 2) {
      schema?.field.forEach((field) => {
        if (field.wajib && !fieldValues[field.kode]) {
          newErrors[field.kode] = `${field.label} wajib diisi`;
        }
      });
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = async () => {
    if (validateStep(step)) {
      if (step === 2) {
        await generatePreview();
      }
      setStep((prev) => Math.min(3, prev + 1) as WizardStep);
    }
  };

  const handlePrev = () => {
    setStep((prev) => Math.max(1, prev - 1) as WizardStep);
  };

  const generatePreview = async () => {
    // Get nomor surat
    try {
      const nomorResult = await getNextNomorSurat(kode);
      setNomorSurat(nomorResult.nomor);
    } catch {
      setNomorSurat('(nomor surat tidak tersedia)');
    }

    // Build preview HTML from field values
    const subjectInfo = Object.entries(subjects)
      .map(([k, v]) => `<p><strong>${k}:</strong> ${v.nama} (${v.nik})</p>`)
      .join('');
    const fieldInfo = Object.entries(fieldValues)
      .map(([k, v]) => `<p><strong>${k}:</strong> ${v}</p>`)
      .join('');

    setPreviewHtml(`
      <div style="font-family: sans-serif; padding: 1rem;">
        <h2 style="text-align: center;">Preview ${schema?.nama || kode}</h2>
        <hr/>
        <h3>Subjek</h3>
        ${subjectInfo || '<p>-</p>'}
        <h3>Data</h3>
        ${fieldInfo || '<p>-</p>'}
        <p><strong>Nomor Surat:</strong> ${nomorSurat || '(akan digenerate)'}</p>
      </div>
    `);
  };

  const handleSubmit = async () => {
    setSubmitting(true);
    try {
      const { invoke } = await import('@tauri-apps/api/core');

      // Build payload for commit_riwayat_formulir
      const payload = {
        kode_formulir: kode,
        versi_template: schema?.versi_template || 1,
        nomor_surat: nomorSurat || null,
        tanggal_terbit: fieldValues.tanggal_terbit || new Date().toISOString().split('T')[0],
        pejabat_snapshot: JSON.stringify({
          jabatan: schema?.tanda_tangan?.pejabat_default || 'Kepala Desa',
        }),
        data_snapshot: JSON.stringify({ subjects, fields: fieldValues }),
        template_snapshot: `<p>${schema?.nama}</p>`,
        subjek_niks: Object.values(subjects).map((s) => ({
          nik: s.nik,
          peran: 'pemohon',
        })),
      };

      await invoke('commit_riwayat_formulir_cmd', { payload });
      router.push('/formulir');
    } catch (err) {
      setErrors({ submit: err instanceof Error ? err.message : 'Gagal menyimpan' });
    } finally {
      setSubmitting(false);
    }
  };

  const clearError = (key: string) => {
    setErrors((current) => {
      const next = { ...current };
      delete next[key];
      return next;
    });
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <span className="loading loading-spinner loading-lg" />
      </div>
    );
  }

  if (loadError) {
    return (
      <div className="container mx-auto p-6">
        <div className="alert alert-error">
          <span>{loadError}</span>
        </div>
      </div>
    );
  }

  if (!schema) return null;

  return (
    <div className="container mx-auto p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold">{schema.nama}</h1>
        <p className="text-base-content/70">Kode: {schema.kode}</p>
      </div>

      {/* Steps indicator */}
      <ul className="steps steps-horizontal w-full mb-6">
        <li className={`step ${step >= 1 ? 'step-primary' : ''}`}>Pilih Subjek</li>
        <li className={`step ${step >= 2 ? 'step-primary' : ''}`}>Isi Data</li>
        <li className={`step ${step >= 3 ? 'step-primary' : ''}`}>Preview & Cetak</li>
      </ul>

      {errors.submit && (
        <div className="alert alert-error mb-4">
          <span>{errors.submit}</span>
        </div>
      )}

      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          {step === 1 && (
            <div>
              <h2 className="card-title mb-4">Langkah 1: Pilih Subjek</h2>

              {/* Search box */}
              <div className="form-control mb-4">
                <label className="label" htmlFor="search-penduduk">
                  <span className="label-text">Cari warga (NIK atau nama)</span>
                </label>
                <input
                  id="search-penduduk"
                  type="text"
                  className="input input-bordered"
                  placeholder="Ketik NIK atau nama..."
                  onChange={(e) => handleSearchPenduduk(e.target.value)}
                />
                {searching && <span className="loading loading-dots loading-sm mt-2" />}
                {searchResults.length > 0 && (
                  <div className="mt-2 max-h-48 overflow-y-auto border rounded-lg">
                    {searchResults.map((p) => (
                      <button
                        key={p.nik}
                        type="button"
                        className="w-full text-left p-2 hover:bg-base-200 border-b last:border-b-0"
                        onClick={() => {
                          const firstSubject = schema.subjek[0]?.kode || 'individu';
                          setSubjects({
                            ...subjects,
                            [firstSubject]: { nik: p.nik, nama: p.nama_lengkap },
                          });
                          setSearchResults([]);
                          clearError(`subject_${firstSubject}`);
                        }}
                      >
                        <span className="font-mono text-sm">{p.nik}</span> - {p.nama_lengkap}
                      </button>
                    ))}
                  </div>
                )}
              </div>

              <SubjectSelector
                subjects={schema.subjek}
                selectedSubjects={subjects}
                errors={errors}
                onSubjectChange={(kode, subject) => {
                  if (subject) {
                    setSubjects({ ...subjects, [kode]: subject });
                    clearError(`subject_${kode}`);
                  } else {
                    const next = { ...subjects };
                    delete next[kode];
                    setSubjects(next);
                  }
                }}
              />
            </div>
          )}

          {step === 2 && (
            <div>
              <h2 className="card-title mb-4">Langkah 2: Isi Data Tambahan</h2>
              <FormRenderer
                fields={schema.field}
                subjects={schema.subjek}
                values={fieldValues}
                onChange={(values) => {
                  for (const key of Object.keys(values)) {
                    if (values[key] !== fieldValues[key]) {
                      clearError(key);
                    }
                  }
                  setFieldValues(values);
                }}
                errors={errors}
              />
            </div>
          )}

          {step === 3 && (
            <div>
              <h2 className="card-title mb-4">Langkah 3: Preview & Cetak</h2>
              {nomorSurat && (
                <div className="alert alert-info mb-4">
                  <span>
                    Nomor Surat: <strong>{nomorSurat}</strong>
                  </span>
                </div>
              )}
              <PreviewPane html={previewHtml} />
            </div>
          )}

          {/* Navigation Buttons */}
          <div className="card-actions justify-between mt-6">
            <button
              type="button"
              className="btn btn-outline"
              onClick={handlePrev}
              disabled={step === 1}
            >
              Kembali
            </button>

            {step < 3 ? (
              <button type="button" className="btn btn-primary" onClick={handleNext}>
                Lanjut
              </button>
            ) : (
              <button
                type="button"
                className="btn btn-success"
                onClick={handleSubmit}
                disabled={submitting}
              >
                {submitting ? (
                  <span className="loading loading-spinner loading-sm" />
                ) : (
                  'Simpan & Cetak'
                )}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
