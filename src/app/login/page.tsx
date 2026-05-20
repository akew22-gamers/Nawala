export default function LoginPage() {
  return (
    <main className="grid min-h-screen place-items-center bg-base-200 p-4">
      <section className="card w-full max-w-md bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Masuk untuk mengelola surat adminduk desa.</p>
          <input
            className="input input-bordered mt-4"
            type="password"
            placeholder="PIN atau password"
          />
          <button type="button" className="btn btn-primary mt-2">
            Masuk
          </button>
          <p className="mt-6 text-center text-xs text-base-content/50">
            EAS Creative Studio - https://eas.biz.id
          </p>
        </div>
      </section>
    </main>
  );
}
