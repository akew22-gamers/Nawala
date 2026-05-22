import BuatFormulirClient from './BuatFormulirClient';

export function generateStaticParams() {
  return [
    { kode: 'F-1.01' },
    { kode: 'F-1.02' },
    { kode: 'F-1.03' },
    { kode: 'F-1.06' },
    { kode: 'F-1.08' },
    { kode: 'F-1.15' },
    { kode: 'F-1.16' },
    { kode: 'F-1.25' },
    { kode: 'F-1.27' },
    { kode: 'F-2.01' },
    { kode: 'F-2.01-kelahiran' },
    { kode: 'F-2.01-kematian' },
    { kode: 'F-2.12' },
    { kode: 'F-2.29' },
    { kode: 'F-2.30' },
  ];
}

export default function BuatFormulirPage() {
  return <BuatFormulirClient />;
}
