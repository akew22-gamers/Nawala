import { useAuthStore } from '@/stores/authStore';

export function useAuth() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const setAuthenticated = useAuthStore((state) => state.setAuthenticated);

  return {
    isAuthenticated,
    setAuthenticated,
  };
}
