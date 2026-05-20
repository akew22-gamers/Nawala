'use client';

import { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';

export default function LoginPage() {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const { setAuthenticated } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    // TODO: Implement actual authentication logic
    if (password) {
      setAuthenticated(true);
    } else {
      setError('Password tidak boleh kosong');
    }
  };

  return (
    <main className="grid min-h-screen place-items-center bg-base-200 p-4">
      <section className="card w-full max-w-md bg-base-100 shadow-xl">
        <div className="card-body">
          <h1 className="card-title text-3xl">Nawala</h1>
          <p className="text-base-content/70">Masuk untuk mengelola surat adminduk desa.</p>

          <form onSubmit={handleSubmit} className="mt-4 grid gap-3">
            <label className="form-control" htmlFor="password">
              <span className="label-text">Password</span>
              <input
                id="password"
                name="password"
                type="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="input input-bordered mt-1"
                placeholder="PIN atau password"
              />
            </label>

            {error && <p className="text-sm text-error">{error}</p>}

            <button type="submit" className="btn btn-primary mt-2">
              Masuk
            </button>
          </form>

          <p className="mt-6 text-center text-xs text-base-content/50">
            EAS Creative Studio - https://eas.biz.id
          </p>
        </div>
      </section>
    </main>
  );
}
