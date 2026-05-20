import { describe, expect, it } from 'vitest';
import { renderNomorSurat } from './nomorSurat';

describe('renderNomorSurat', () => {
  it('renders padded sequence and roman month', () => {
    expect(
      renderNomorSurat('{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}', {
        seq: 42,
        kode: 'F-1.02',
        kodeDesa: 'DESA-X',
        tanggal: new Date('2026-05-20T00:00:00Z'),
      }),
    ).toBe('0042/F-1.02/DESA-X/V/2026');
  });
});
