export interface NomorContext {
  seq: number;
  kode: string;
  kodeDesa: string;
  tanggal: Date;
  custom?: Record<string, string>;
}

const ROMAWI = ['', 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X', 'XI', 'XII'];

export function renderNomorSurat(pattern: string, context: NomorContext): string {
  let result = pattern;

  // Replace {seq:N} with zero-padded sequence number
  result = result.replace(/\{seq:(\d+)\}/g, (_, width) => {
    return context.seq.toString().padStart(parseInt(width, 10), '0');
  });

  // Replace {kode}
  result = result.replace(/\{kode\}/g, context.kode);

  // Replace {kode_desa}
  result = result.replace(/\{kode_desa\}/g, context.kodeDesa);

  // Replace date-related placeholders
  const year = context.tanggal.getUTCFullYear();
  const month = context.tanggal.getUTCMonth() + 1;
  const date = context.tanggal.getUTCDate();

  result = result.replace(/\{tahun\}/g, year.toString());
  result = result.replace(/\{tahun_pendek\}/g, year.toString().slice(-2));
  result = result.replace(/\{bulan\}/g, month.toString().padStart(2, '0'));
  result = result.replace(/\{romawi:bulan\}/g, ROMAWI[month]);
  result = result.replace(/\{tanggal\}/g, date.toString().padStart(2, '0'));

  // Replace {custom:KEY}
  if (context.custom) {
    result = result.replace(/\{custom:(\w+)\}/g, (_, key) => {
      return context.custom?.[key] ?? '';
    });
  }

  return result;
}
