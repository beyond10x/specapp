import {execFileSync} from 'node:child_process';
import {defineConfig} from '@playwright/test';

// The UX lane of specapp: the docs sites of both fixture packs (build/<pack>, written by
// `task ux`), each served once by `docusaurus serve` on a free port of 127.0.0.1. Every page is
// checked at 1440x900 and 1280x800 in the light and the dark theme: axe-core finds no serious or
// critical violation and the page does not scroll sideways. A test is killed at 60 s and the run
// at 120 s; the ten slowest tests are printed.

const packs = ['harbor', 'orchard'];

// The workers load this file again; the ports are chosen once, in the runner, and passed on in
// the environment.
function freePort(): string {
  return execFileSync(process.execPath, [
    '-e',
    "const s=require('net').createServer();s.listen(0,'127.0.0.1',()=>{process.stdout.write(String(s.address().port));s.close()})",
  ])
    .toString()
    .trim();
}
for (const pack of packs) {
  const key = `SPECAPP_UX_PORT_${pack.toUpperCase()}`;
  if (!process.env[key]) process.env[key] = freePort();
}
const origin = (pack: string) => `http://127.0.0.1:${process.env[`SPECAPP_UX_PORT_${pack.toUpperCase()}`]}`;

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
  use: {browserName: 'chromium', headless: true},
  webServer: packs.map((pack) => ({
    command: `npx docusaurus serve --dir build/${pack} --host 127.0.0.1 --port ${process.env[`SPECAPP_UX_PORT_${pack.toUpperCase()}`]} --no-open`,
    url: `${origin(pack)}/`,
    reuseExistingServer: false,
    timeout: 30_000,
    stdout: 'ignore' as const,
    stderr: 'pipe' as const,
  })),
  projects: packs.map((pack) => ({name: pack, use: {baseURL: `${origin(pack)}/`}, metadata: {pack}})),
});
