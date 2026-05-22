'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { checkAuthStatus, setupPassword } from '@/lib/tauri';

export default function OnboardingPage() {
  const router = useRouter();
  const [password, setPassword] = useState('');
  const [confirm, setConfirm] = useState('');
  const [hint, setHint] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [checking, setChecking] = useState(true);

  useEffect(() => {
    checkAuthStatus()
      .then((status) => {
        if (status.configured) {
          router.replace('/login');
          return;
        }
        setChecking(false);
      })
      .catch(() => {
        // Not in Tauri context
        setChecking(false);
      });
  }, [router]);

  const handleSetup = async () => {
    setError('');

    if (password.length < 4) {
      setError('Password minimal 4 karakter');
      return;
    }
    if (password !== confirm) {
      setError('Konfirmasi password tidak cocok');
      return;
    }

    setLoading(true);
    try {
      await setupPassword(password, hint || undefined);
      router.push('/');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Setup gagal');
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSetup();
    }
  };

  if (checking) {
    return (
      <main className="grid min-h-screen place-items-center bg-base-200 p-4">
        <span className="loading loading-spinner loading-lg" />
      </main>
    );
  }

  return (
    <main className="grid min-h-screen place-items-center bg-base-200 p-4">
      <section className="card w-full max-w-lg bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Setup awal aplikasi surat adminduk desa.</p>

          {error && (
            <div className="alert alert-error mt-2">
              <span>{error}</span>
            </div>
          )}

          <input
            className="input input-bordered mt-4"
            type="password"
            placeholder="Password baru (min. 4 karakter)"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            onKeyDown={handleKeyDown}
            aria-label="Password baru"
          />
          <input
            className="input input-bordered"
            type="password"
            placeholder="Konfirmasi password"
            value={confirm}
            onChange={(e) => setConfirm(e.target.value)}
            onKeyDown={handleKeyDown}
            aria-label="Konfirmasi password"
          />
          <input
            className="input input-bordered"
            type="text"
            placeholder="Hint password (opsional)"
            value={hint}
            onChange={(e) => setHint(e.target.value)}
            aria-label="Hint password"
          />

          <div className="alert alert-info mt-2 text-sm">
            Profil desa dapat dilengkapi setelah masuk melalui menu Pengaturan.
          </div>

          <button
            type="button"
            className="btn btn-primary mt-2"
            onClick={handleSetup}
            disabled={loading}
          >
            {loading ? <span className="loading loading-spinner loading-sm" /> : 'Simpan & Mulai'}
          </button>

          <Link className="btn btn-ghost btn-sm mt-4" href="/">
            Kembali ke Dashboard
          </Link>

          <p className="mt-6 text-center text-xs text-base-content/50">
            EAS Creative Studio - https://eas.biz.id
          </p>
        </div>
      </section>
    </main>
  );
}
