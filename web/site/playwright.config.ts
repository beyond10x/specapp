import {execFileSync} from 'node:child_process';
import {defineConfig} from '@playwright/test';

// The UX lane of metaspec: the one docs site of both fixture packs (build/fixtures, written by
// `task ux`), served once by `docusaurus serve` on a free port of 127.0.0.1. The page / and every
// page of each pack are checked at 1440x900 and 1280x800 in the light and the dark theme: axe-core
// finds no serious or critical violation and the page does not scroll sideways. A test is killed
// at 60 s and the run at 120 s; the ten slowest tests are printed.

// The workers load this file again; the port is chosen once, in the runner, and passed on in the
// environment.
function freePort(): string {
  return execFileSync(process.execPath, [
    '-e',
    "const s=require('net').createServer();s.listen(0,'127.0.0.1',()=>{process.stdout.write(String(s.address().port));s.close()})",
  ])
    .toString()
    .trim();
}
if (!process.env.METASPEC_UX_PORT) process.env.METASPEC_UX_PORT = freePort();
const origin = `http://127.0.0.1:${process.env.METASPEC_UX_PORT}`;

export default defineConfig({
  testDir: 'e2e',
  timeout: 60_000,
  globalTimeout: 120_000,
  fullyParallel: true,
  forbidOnly: true,
  retries: 0,
  reporter: [['list'], ['./e2e/slowest.ts']],
  reportSlowTests: null,
  outputDir: '../../target/site-ux/test-results',
  use: {browserName: 'chromium', headless: true, baseURL: `${origin}/`},
  webServer: {
    command: `npx docusaurus serve --dir build/fixtures --host 127.0.0.1 --port ${process.env.METASPEC_UX_PORT} --no-open`,
    url: `${origin}/`,
    reuseExistingServer: false,
    timeout: 30_000,
    stdout: 'ignore',
    stderr: 'pipe',
  },
});
