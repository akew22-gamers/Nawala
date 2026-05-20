import Link from 'next/link';
import { appRoutes } from '@/lib/navigation';

export default function DashboardPage() {
  return (
    <main className="min-h-screen bg-base-200 p-6">
      <section className="mx-auto max-w-6xl rounded-box bg-base-100 p-8 shadow-sm">
        <div className="flex flex-col gap-6 lg:flex-row lg:items-end lg:justify-between">
          <div>
            <p className="text-sm uppercase tracking-[0.3em] text-primary">EAS Creative Studio</p>
            <h1 className="mt-3 text-4xl font-bold">Nawala</h1>
            <p className="mt-2 text-base-content/70">Aplikasi Surat Adminduk Desa</p>
          </div>
          <Link className="btn btn-primary" href="/login">
            Mulai / Masuk
          </Link>
        </div>

        <div className="mt-8 grid gap-4 md:grid-cols-2 xl:grid-cols-3">
          {appRoutes.map((route) => (
            <Link
              className="card bg-base-200 transition-colors hover:bg-base-300"
              href={route.href}
              key={route.href}
            >
              <div className="card-body">
                <h2 className="card-title">{route.label}</h2>
                <p className="text-sm text-base-content/70">{route.description}</p>
              </div>
            </Link>
          ))}
        </div>
      </section>
    </main>
  );
}
