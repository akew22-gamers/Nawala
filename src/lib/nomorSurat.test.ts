import { describe, expect, it } from 'vitest';
import { renderNomorSurat } from './nomorSurat';

describe('renderNomorSurat', () => {
  const context = {
    seq: 42,
    kode: 'F-1.02',
    kodeDesa: 'DESA-X',
    tanggal: new Date('2026-05-20T00:00:00Z'),
  };

  it('renders padded sequence and roman month', () => {
    expect(renderNomorSurat('{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}', context)).toBe(
      '0042/F-1.02/DESA-X/V/2026',
    );
  });

  it('renders all date placeholders using UTC values', () => {
    expect(renderNomorSurat('{tanggal}-{bulan}-{tahun_pendek}', context)).toBe('20-05-26');
  });

  it('renders first and last month roman numerals', () => {
    expect(
      renderNomorSurat('{romawi:bulan}', { ...context, tanggal: new Date('2026-01-01T00:00:00Z') }),
    ).toBe('I');
    expect(
      renderNomorSurat('{romawi:bulan}', { ...context, tanggal: new Date('2026-12-01T00:00:00Z') }),
    ).toBe('XII');
  });

  it('does not truncate sequences wider than the requested padding', () => {
    expect(renderNomorSurat('{seq:4}', { ...context, seq: 12_345 })).toBe('12345');
  });

  it('renders uppercase custom placeholders and blanks missing keys', () => {
    expect(
      renderNomorSurat('{custom:UNIT}/{custom:MISSING}', { ...context, custom: { UNIT: 'PEM' } }),
    ).toBe('PEM/');
  });

  it('leaves lowercase custom placeholders unchanged', () => {
    expect(renderNomorSurat('{custom:unit}', { ...context, custom: { unit: 'pem' } })).toBe(
      '{custom:unit}',
    );
  });
});
