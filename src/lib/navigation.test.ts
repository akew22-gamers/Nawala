import { describe, expect, it } from 'vitest';
import { appRoutes } from './navigation';

describe('appRoutes', () => {
  it('contains the routes needed to enter and test the app', () => {
    expect(appRoutes.map((route) => route.href)).toEqual(
      expect.arrayContaining([
        '/login',
        '/onboarding',
        '/formulir',
        '/warga/import',
        '/pengaturan',
      ]),
    );
  });
});
