export type AppRoute = {
  href: string;
  label: string;
  description: string;
};

export const appRoutes: AppRoute[] = [
  {
    href: '/login',
    label: 'Masuk',
    description: 'Masuk ke aplikasi untuk mulai mengelola surat.',
  },
  {
    href: '/onboarding',
    label: 'Setup Awal',
    description: 'Siapkan password dan profil dasar desa.',
  },
  {
    href: '/formulir',
    label: 'Formulir Surat',
    description: 'Pilih dan buat formulir adminduk.',
  },
  {
    href: '/warga/import',
    label: 'Import Warga',
    description: 'Uji import data Buku Induk warga.',
  },
  {
    href: '/pengaturan',
    label: 'Pengaturan',
    description: 'Kelola identitas desa, backup, audit, dan tentang aplikasi.',
  },
];
