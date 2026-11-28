import {defineConfig} from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    include: ['test/**/*.test.tsx'],
    testTimeout: 60_000,
    reporters: ['default'],
  },
});
