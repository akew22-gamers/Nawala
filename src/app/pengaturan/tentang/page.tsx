export default function TentangPage() {
  return (
    <main className="min-h-screen bg-base-200 p-6">
      <section className="mx-auto max-w-3xl rounded-box bg-base-100 p-8 shadow-sm">
        <p className="text-sm uppercase tracking-[0.3em] text-primary">Tentang</p>
        <h1 className="mt-3 text-4xl font-bold">Nawala</h1>
        <p className="mt-3 text-base-content/70">
          Aplikasi Surat Adminduk Desa oleh EAS Creative Studio.
        </p>

        <div className="divider" />

        <dl className="grid gap-4 sm:grid-cols-2">
          <div className="rounded-box bg-base-200 p-4">
            <dt className="text-sm font-semibold text-base-content/60">Studio</dt>
            <dd className="mt-1 font-medium">EAS Creative Studio</dd>
          </div>
          <div className="rounded-box bg-base-200 p-4">
            <dt className="text-sm font-semibold text-base-content/60">Lisensi</dt>
            <dd className="mt-1 font-medium">MIT</dd>
          </div>
          <div className="rounded-box bg-base-200 p-4 sm:col-span-2">
            <dt className="text-sm font-semibold text-base-content/60">Website</dt>
            <dd className="mt-1">
              <a className="link link-primary" href="https://eas.biz.id">
                https://eas.biz.id
              </a>
            </dd>
          </div>
          <div className="rounded-box bg-base-200 p-4 sm:col-span-2">
            <dt className="text-sm font-semibold text-base-content/60">Email</dt>
            <dd className="mt-1 flex flex-wrap gap-3">
              <a className="link link-primary" href="mailto:dev@eas.biz.id">
                dev@eas.biz.id
              </a>
              <a className="link link-primary" href="mailto:eas.creative.studio@gmail.com">
                eas.creative.studio@gmail.com
              </a>
            </dd>
          </div>
        </dl>
      </section>
    </main>
  );
}
