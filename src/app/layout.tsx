import type { Metadata } from 'next';
import '@/styles/globals.css';

export const metadata: Metadata = {
  title: 'Nawala',
  description: 'Aplikasi Surat Adminduk Desa',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="id" data-theme="light">
      <body>{children}</body>
    </html>
  );
}
