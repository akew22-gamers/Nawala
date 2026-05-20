const sections = [
  {
    title: 'Identitas Desa',
    description: 'Kelola nama desa, alamat, kontak, dan atribut resmi kop surat.',
  },
  {
    title: 'Pejabat',
    description: 'Atur pejabat penanda tangan dan snapshot jabatan untuk dokumen.',
  },
  {
    title: 'Nomor Surat',
    description: 'Konfigurasi pola penomoran, reset periode, dan urutan surat.',
  },
  {
    title: 'Tampilan',
    description: 'Sesuaikan preferensi tema, ukuran tampilan, dan pengalaman pengguna.',
  },
  {
    title: 'Keamanan',
    description: 'Pantau kebijakan akses, kunci aplikasi, dan proteksi data lokal.',
  },
  {
    title: 'Backup',
    description: 'Buat backup manual, pulihkan database, dan lihat riwayat backup.',
  },
  {
    title: 'Audit Log',
    description: 'Telusuri aktivitas penting seperti backup, restore, dan perubahan data.',
  },
  {
    title: 'Tentang',
    description: 'Informasi aplikasi, lisensi, dan kontak EAS Creative Studio.',
    href: '/pengaturan/tentang',
  },
];

export default function PengaturanPage() {
  return (
    <main className="min-h-screen bg-base-200 p-6">
      <section className="mx-auto max-w-6xl space-y-6">
        <div className="rounded-box bg-base-100 p-8 shadow-sm">
          <p className="text-sm uppercase tracking-[0.3em] text-primary">Pengaturan</p>
          <h1 className="mt-3 text-4xl font-bold">Pusat Kendali Nawala</h1>
          <p className="mt-2 max-w-2xl text-base-content/70">
            Kelola konfigurasi desa, operasional surat, keamanan, backup, dan audit aplikasi.
          </p>
        </div>

        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
          {sections.map((section) => {
            const card = (
              <article className="card h-full bg-base-100 shadow-sm transition hover:-translate-y-1 hover:shadow-md">
                <div className="card-body">
                  <h2 className="card-title">{section.title}</h2>
                  <p className="text-sm text-base-content/70">{section.description}</p>
                  <div className="card-actions justify-end pt-4">
                    <span className="btn btn-primary btn-sm">Buka</span>
                  </div>
                </div>
              </article>
            );

            if (section.href) {
              return (
                <a key={section.title} href={section.href} className="block">
                  {card}
                </a>
              );
            }

            return <div key={section.title}>{card}</div>;
          })}
        </div>
      </section>
    </main>
  );
}
