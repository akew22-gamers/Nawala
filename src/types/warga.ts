export type ImportRowStatus = 'valid' | 'error' | 'conflict';

export type ImportPreviewRow = {
  rowNumber: number;
  nik: string;
  noKk: string;
  namaLengkap: string;
  status: ImportRowStatus;
  message?: string;
};
