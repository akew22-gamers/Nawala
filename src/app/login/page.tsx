'use client';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { checkAuthStatus, login } from '@/lib/tauri';

export default function LoginPage() {
  const router = useRouter();
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [checking, setChecking] = useState(true);
  const [locked, setLocked] = useState(false);
  const [lockedUntil, setLockedUntil] = useState<string | null>(null);

  useEffect(() => {
    checkAuthStatus()
      .then((status) => {
        if (!status.configured) {
          router.replace('/onboarding');
          return;
        }
        if (status.locked) {
          setLocked(true);
          setLockedUntil(status.locked_until);
        }
        setChecking(false);
      })
      .catch(() => {
        // Not in Tauri context (dev/build), show page anyway
        setChecking(false);
      });
  }, [router]);

  const handleLogin = async () => {
    if (!password) {
      setError('Masukkan password');
      return;
    }
    setLoading(true);
    setError('');
    try {
      const result = await login(password);
      if (result.success) {
        router.push('/');
      } else {
        setError(result.message);
        if (result.locked_until) {
          setLocked(true);
          setLockedUntil(result.locked_until);
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Login gagal');
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleLogin();
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
      <section className="card w-full max-w-md bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Masuk untuk mengelola surat adminduk desa.</p>

          {locked && (
            <div className="alert alert-warning mt-2">
              <span>Akun terkunci{lockedUntil ? ` hingga ${lockedUntil}` : ''}</span>
            </div>
          )}

          {error && (
            <div className="alert alert-error mt-2">
              <span>{error}</span>
            </div>
          )}

          <input
            className="input input-bordered mt-4"
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            onKeyDown={handleKeyDown}
            disabled={locked || loading}
            aria-label="Password"
          />
          <button
            type="button"
            className="btn btn-primary mt-2"
            onClick={handleLogin}
            disabled={locked || loading}
          >
            {loading ? <span className="loading loading-spinner loading-sm" /> : 'Masuk'}
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
