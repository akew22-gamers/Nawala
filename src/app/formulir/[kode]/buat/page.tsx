/**
 * Form Wizard Page - 3-step wizard for creating forms
 */

"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import type { FormSchema } from "@/types/formulir";
import { FormRenderer, SubjectSelector } from "@/components/formulir/FormRenderer";
import { PreviewPane } from "@/components/formulir/PreviewPane";

type WizardStep = 1 | 2 | 3;

interface SubjectData {
  nik: string;
  nama: string;
}

export default function BuatFormulirPage() {
  const params = useParams();
  const kode = params.kode as string;

  const [step, setStep] = useState<WizardStep>(1);
  const [schema, setSchema] = useState<FormSchema | null>(null);
  const [loading, setLoading] = useState(true);

  // Form data
  const [subjects, setSubjects] = useState<Record<string, SubjectData>>({});
  const [fieldValues, setFieldValues] = useState<Record<string, string>>({});
  const [previewHtml, setPreviewHtml] = useState<string>("");

  // Validation errors
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    // TODO: Load schema from Tauri command
    // Placeholder schema
    setSchema({
      kode: kode.toUpperCase(),
      nama: `Formulir ${kode.toUpperCase()}`,
      kategori: "Keterangan",
      ukuran_kertas: "F4",
      orientasi: "portrait",
      versi_regulasi: "2024",
      versi_template: 1,
      subjek: [
        {
          kode: "pemohon",
          label: "Pemohon",
          wajib: true,
          sumber: "warga",
        },
      ],
      field: [
        {
          kode: "keperluan",
          label: "Keperluan",
          tipe: "textarea",
          wajib: true,
        },
        {
          kode: "tanggal_terbit",
          label: "Tanggal Terbit",
          tipe: "date",
          wajib: true,
        },
      ],
      nomor_surat: {
        pakai: true,
        pola: "{seq}/SURAT/{tahun}",
      },
      tanda_tangan: {
        pejabat_default: "kepala_desa",
      },
    });
    setLoading(false);
  }, [kode]);

  const validateStep = (currentStep: WizardStep): boolean => {
    const newErrors: Record<string, string> = {};

    if (currentStep === 1) {
      // Validate subjects
      schema?.subjek.forEach((subject) => {
        if (subject.wajib && !subjects[subject.kode]) {
          newErrors[`subject_${subject.kode}`] = `${subject.label} wajib diisi`;
        }
      });
    }

    if (currentStep === 2) {
      // Validate fields
      schema?.field.forEach((field) => {
        if (field.wajib && !fieldValues[field.kode]) {
          newErrors[field.kode] = `${field.label} wajib diisi`;
        }
      });
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(step)) {
      if (step === 2) {
        // Generate preview
        generatePreview();
      }
      setStep((prev) => Math.min(3, prev + 1) as WizardStep);
    }
  };

  const handlePrev = () => {
    setStep((prev) => Math.max(1, prev - 1) as WizardStep);
  };

  const generatePreview = () => {
    // TODO: Call Tauri command to render template
    // Placeholder HTML
    const html = `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="UTF-8">
        <style>
          body { font-family: Arial, sans-serif; padding: 2cm; }
          h1 { text-align: center; }
          .content { margin-top: 2cm; }
        </style>
      </head>
      <body>
        <h1>SURAT ${schema?.nama.toUpperCase()}</h1>
        <div class="content">
          <p>Yang bertanda tangan di bawah ini:</p>
          <p><strong>Pemohon:</strong> ${subjects.pemohon?.nama || "-"} (${subjects.pemohon?.nik || "-"})</p>
          <p><strong>Keperluan:</strong> ${fieldValues.keperluan || "-"}</p>
          <p><strong>Tanggal:</strong> ${fieldValues.tanggal_terbit || "-"}</p>
        </div>
      </body>
      </html>
    `;
    setPreviewHtml(html);
  };

  const handleSubmit = async () => {
    // TODO: Call Tauri command to commit to riwayat_formulir
    const payload = {
      kode_formulir: schema?.kode,
      versi_template: schema?.versi_template,
      nomor_surat: null, // Will be generated
      tanggal_terbit: fieldValues.tanggal_terbit,
      pejabat_id: null,
      pejabat_snapshot: JSON.stringify({ nama: "Kepala Desa" }),
      data_snapshot: JSON.stringify({ ...fieldValues, subjects }),
      template_snapshot: JSON.stringify(schema),
      pdf_path: null,
      hash_dokumen: null,
      catatan: null,
      dibuat_oleh: "admin",
      subjek: Object.entries(subjects).map(([peran, data]) => ({
        nik: data.nik,
        peran,
      })),
    };

    console.log("Would submit:", payload);
    alert("Form submitted (backend not wired yet)");
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <span className="loading loading-spinner loading-lg" />
      </div>
    );
  }

  if (!schema) {
    return (
      <div className="container mx-auto p-6">
        <div className="alert alert-error">
          <span>Schema formulir tidak ditemukan</span>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6 max-w-4xl">
      <div className="mb-6">
        <h1 className="text-3xl font-bold mb-2">{schema.nama}</h1>
        <p className="text-base-content/70">Kode: {schema.kode}</p>
      </div>

      {/* Progress Steps */}
      <ul className="steps steps-horizontal w-full mb-8">
        <li className={`step ${step >= 1 ? "step-primary" : ""}`}>
          Pilih Warga
        </li>
        <li className={`step ${step >= 2 ? "step-primary" : ""}`}>
          Isi Data
        </li>
        <li className={`step ${step >= 3 ? "step-primary" : ""}`}>
          Preview & Cetak
        </li>
      </ul>

      {/* Step Content */}
      <div className="card bg-base-100 shadow-xl">
        <div className="card-body">
          {step === 1 && (
            <div>
              <h2 className="card-title mb-4">Langkah 1: Pilih Warga/Subjek</h2>
              <SubjectSelector
                subjects={schema.subjek}
                selectedSubjects={subjects}
                onSubjectChange={(kode, subject) => {
                  if (subject) {
                    setSubjects({ ...subjects, [kode]: subject });
                  } else {
                    const newSubjects = { ...subjects };
                    delete newSubjects[kode];
                    setSubjects(newSubjects);
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
                onChange={setFieldValues}
                errors={errors}
              />
            </div>
          )}

          {step === 3 && (
            <div>
              <h2 className="card-title mb-4">Langkah 3: Preview & Cetak</h2>
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
              <button
                type="button"
                className="btn btn-primary"
                onClick={handleNext}
              >
                Lanjut
              </button>
            ) : (
              <button
                type="button"
                className="btn btn-success"
                onClick={handleSubmit}
              >
                Simpan & Cetak
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
