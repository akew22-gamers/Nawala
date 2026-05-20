export default function OnboardingPage() {
  return (
    <main className="grid min-h-screen place-items-center bg-base-200 p-4">
      <section className="card w-full max-w-lg bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Setup awal aplikasi surat adminduk desa.</p>

          <input
            className="input input-bordered mt-4"
            type="password"
            placeholder="Password baru"
          />
          <input
            className="input input-bordered"
            type="password"
            placeholder="Konfirmasi password"
          />

          <div className="alert alert-info mt-2 text-sm">
            Profil desa dapat dilengkapi setelah masuk melalui menu pengaturan.
          </div>

          <button type="button" className="btn btn-primary mt-2">
            Simpan Setup
          </button>

          <p className="mt-6 text-center text-xs text-base-content/50">
            EAS Creative Studio - https://eas.biz.id
          </p>
        </div>
      </section>
    </main>
  );
}
