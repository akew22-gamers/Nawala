import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const formFiles = [
  'F-1.01',
  'F-1.02',
  'F-1.03',
  'F-1.06',
  'F-1.08',
  'F-1.15',
  'F-1.16',
  'F-1.25',
  'F-1.27',
  'F-2.01',
  'F-2.01-kelahiran',
  'F-2.01-kematian',
  'F-2.12',
  'F-2.29',
  'F-2.30',
];

const requiredSchemaKeys = [
  'kode',
  'nama',
  'kategori',
  'ukuran_kertas',
  'orientasi',
  'versi_regulasi',
  'versi_template',
  'subjek',
  'field',
  'nomor_surat',
  'tanda_tangan',
];

describe('Tier 1 form catalogue resources', () => {
  it.each(formFiles)('has schema, template, and fixture for %s', (formFile) => {
    const schemaPath = join(process.cwd(), 'src-tauri/src/resources/schemas', `${formFile}.json`);
    const templatePath = join(
      process.cwd(),
      'src-tauri/src/resources/templates',
      `${formFile}.html`,
    );
    const fixturePath = join(process.cwd(), 'tests/fixtures/formulir', `${formFile}.json`);

    expect(existsSync(schemaPath)).toBe(true);
    expect(existsSync(templatePath)).toBe(true);
    expect(existsSync(fixturePath)).toBe(true);

    const schema = JSON.parse(readFileSync(schemaPath, 'utf8'));
    for (const key of requiredSchemaKeys) {
      expect(schema).toHaveProperty(key);
    }
  });
});
