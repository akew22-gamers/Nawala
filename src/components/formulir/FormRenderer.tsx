/**
 * FormRenderer - Reusable form field renderer for wizard
 */

'use client';

import { useState } from 'react';
import type { FormField, FormSubject } from '@/types/formulir';

interface FormRendererProps {
  fields: FormField[];
  subjects: FormSubject[];
  values: Record<string, string>;
  onChange: (values: Record<string, string>) => void;
  errors?: Record<string, string>;
}

export function FormRenderer({ fields, values, onChange, errors = {} }: FormRendererProps) {
  const handleFieldChange = (kode: string, value: string) => {
    onChange({ ...values, [kode]: value });
  };

  const renderField = (field: FormField) => {
    const value = values[field.kode] || '';
    const hasError = errors[field.kode];
    const inputClass = `input input-bordered w-full ${hasError ? 'input-error' : ''}`;

    switch (field.tipe) {
      case 'text':
      case 'email':
      case 'number':
        return (
          <input
            id={field.kode}
            type={field.tipe}
            className={inputClass}
            value={value}
            onChange={(e) => handleFieldChange(field.kode, e.target.value)}
            required={field.wajib}
          />
        );

      case 'textarea':
        return (
          <textarea
            id={field.kode}
            className={`textarea textarea-bordered w-full ${hasError ? 'textarea-error' : ''}`}
            value={value}
            onChange={(e) => handleFieldChange(field.kode, e.target.value)}
            required={field.wajib}
            rows={4}
          />
        );

      case 'date':
        return (
          <input
            id={field.kode}
            type="date"
            className={inputClass}
            value={value}
            onChange={(e) => handleFieldChange(field.kode, e.target.value)}
            required={field.wajib}
          />
        );

      case 'select':
        // Placeholder for select - opsi_ref would be resolved from backend
        return (
          <select
            id={field.kode}
            className={`select select-bordered w-full ${hasError ? 'select-error' : ''}`}
            value={value}
            onChange={(e) => handleFieldChange(field.kode, e.target.value)}
            required={field.wajib}
          >
            <option value="">Pilih {field.label}</option>
            <option value="option1">Option 1</option>
            <option value="option2">Option 2</option>
          </select>
        );

      default:
        return (
          <input
            id={field.kode}
            type="text"
            className={inputClass}
            value={value}
            onChange={(e) => handleFieldChange(field.kode, e.target.value)}
            required={field.wajib}
          />
        );
    }
  };

  return (
    <div className="space-y-4">
      {fields.map((field) => (
        <div key={field.kode} className="form-control">
          <label htmlFor={field.kode} className="label">
            <span className="label-text">
              {field.label}
              {field.wajib && <span className="text-error ml-1">*</span>}
            </span>
          </label>
          {renderField(field)}
          {errors[field.kode] && (
            <div className="label">
              <span className="label-text-alt text-error">{errors[field.kode]}</span>
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

interface SubjectSelectorProps {
  subjects: FormSubject[];
  selectedSubjects: Record<string, { nik: string; nama: string }>;
  errors?: Record<string, string>;
  onSubjectChange: (kode: string, subject: { nik: string; nama: string } | null) => void;
}

export function SubjectSelector({
  subjects,
  selectedSubjects,
  errors = {},
  onSubjectChange,
}: SubjectSelectorProps) {
  const [searchTerms, setSearchTerms] = useState<Record<string, string>>({});

  const handleSearch = (kode: string, term: string) => {
    setSearchTerms({ ...searchTerms, [kode]: term });
    // TODO: Implement actual search via Tauri command
    // For now, allow manual input
  };

  const handleManualInput = (kode: string, nik: string, nama: string) => {
    if (nik && nama) {
      onSubjectChange(kode, { nik, nama });
    } else {
      onSubjectChange(kode, null);
    }
  };

  return (
    <div className="space-y-6">
      {subjects.map((subject) => {
        const selected = selectedSubjects[subject.kode];
        const error = errors[`subject_${subject.kode}`];
        const hasError = Boolean(error) || (subject.wajib && !selected);

        return (
          <div key={subject.kode} className="card bg-base-200">
            <div className="card-body">
              <h3 className="card-title text-base">
                {subject.label}
                {subject.wajib && <span className="text-error ml-1">*</span>}
              </h3>

              {subject.sumber === 'penduduk' && (
                <div className="space-y-2">
                  <input
                    type="text"
                    placeholder="Cari NIK atau nama warga..."
                    className={`input input-bordered w-full ${hasError ? 'input-error' : ''}`}
                    value={searchTerms[subject.kode] || ''}
                    onChange={(e) => handleSearch(subject.kode, e.target.value)}
                  />
                  {/* Search results would appear here */}
                </div>
              )}

              {/* Manual input fallback */}
              <div className="grid grid-cols-2 gap-2 mt-2">
                <input
                  type="text"
                  placeholder="NIK"
                  className={`input input-bordered input-sm ${hasError ? 'input-error' : ''}`}
                  value={selected?.nik || ''}
                  onChange={(e) =>
                    handleManualInput(subject.kode, e.target.value, selected?.nama || '')
                  }
                />
                <input
                  type="text"
                  placeholder="Nama"
                  className={`input input-bordered input-sm ${hasError ? 'input-error' : ''}`}
                  value={selected?.nama || ''}
                  onChange={(e) =>
                    handleManualInput(subject.kode, selected?.nik || '', e.target.value)
                  }
                />
              </div>

              {selected && (
                <div className="alert alert-success mt-2">
                  <span>
                    ✓ {selected.nama} ({selected.nik})
                  </span>
                </div>
              )}

              {hasError && (
                <div className="alert alert-error mt-2">
                  <span>{error || 'Subjek ini wajib diisi'}</span>
                </div>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
