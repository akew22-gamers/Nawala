import BuatFormulirClient from './BuatFormulirClient';

export function generateStaticParams() {
  return [{ kode: 'F-1.02' }];
}

export default function BuatFormulirPage() {
  return <BuatFormulirClient />;
}
