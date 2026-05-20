import { describe, expect, it } from 'vitest';
import { renderTemplate } from './handlebars';

describe('renderTemplate', () => {
  it('renders nested values and Indonesian date', () => {
    const template = 'Nama: {{individu.nama}} - {{tgl_indo tanggal_terbit}}';
    const context = {
      individu: { nama: 'SENA' },
      tanggal_terbit: '2026-05-20',
    };

    const html = renderTemplate(template, context);

    expect(html).toContain('Nama: SENA');
    expect(html).toContain('20 Mei 2026');
  });

  it('renders complete F-1.02 template with all placeholders', () => {
    const template = `
      <div>
        <p>Nama: {{individu.nama}}</p>
        <p>NIK: {{individu.nik}}</p>
        <p>No KK: {{individu.no_kk}}</p>
        <p>Desa: {{config.nama_desa}}</p>
        <p>Tanggal: {{tgl_indo tanggal_terbit}}</p>
        <p>Pejabat: {{pejabat.nama}}</p>
        <p>NIPD: {{pejabat.nipd}}</p>
        <p>Jenis: {{jenis_permohonan}}</p>
      </div>
    `;
    const context = {
      individu: {
        nama: 'SENA WIJAYA',
        nik: '3201012345678901',
        no_kk: '3201011234567890',
      },
      config: {
        nama_desa: 'Desa Maju Jaya',
      },
      tanggal_terbit: '2026-05-20',
      pejabat: {
        nama: 'BUDI SANTOSO',
        nipd: '196512311988031001',
      },
      jenis_permohonan: 'Pendaftaran Kelahiran',
    };

    const html = renderTemplate(template, context);

    expect(html).toContain('Nama: SENA WIJAYA');
    expect(html).toContain('NIK: 3201012345678901');
    expect(html).toContain('No KK: 3201011234567890');
    expect(html).toContain('Desa: Desa Maju Jaya');
    expect(html).toContain('20 Mei 2026');
    expect(html).toContain('Pejabat: BUDI SANTOSO');
    expect(html).toContain('NIPD: 196512311988031001');
    expect(html).toContain('Jenis: Pendaftaran Kelahiran');
  });

  it('formats Indonesian dates correctly for different months', () => {
    const template = '{{tgl_indo date}}';

    expect(renderTemplate(template, { date: '2026-01-15' })).toContain('15 Januari 2026');
    expect(renderTemplate(template, { date: '2026-02-28' })).toContain('28 Februari 2026');
    expect(renderTemplate(template, { date: '2026-12-31' })).toContain('31 Desember 2026');
  });

  it('renders invalid or missing dates as empty strings', () => {
    const template = 'Tanggal: {{tgl_indo date}}';

    expect(renderTemplate(template, { date: 'tanggal-rusak' })).toBe('Tanggal: ');
    expect(renderTemplate(template, { date: '' })).toBe('Tanggal: ');
    expect(renderTemplate(template, { date: null })).toBe('Tanggal: ');
    expect(renderTemplate(template, {})).toBe('Tanggal: ');
  });
});
