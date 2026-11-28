import AxeBuilder from '@axe-core/playwright';
import {expect, test} from '@playwright/test';
import {readFileSync} from 'node:fs';
import path from 'node:path';

// One test per generated page of each fixture pack: at both viewports and in both themes, axe-core
// finds no serious or critical violation, the page does not scroll sideways, and every component
// the page names is drawn.

const viewports = [
  {width: 1440, height: 900},
  {width: 1280, height: 800},
];
const themes = ['light', 'dark'] as const;

interface PageEntry {
  id: string;
  kind: string;
  slug: string;
  title: string;
}

function pagesOf(pack: string): PageEntry[] {
  const file = path.resolve(__dirname, '../../../fixtures/expected', pack, 'site/site.json');
  return (JSON.parse(readFileSync(file, 'utf8')) as {pages: PageEntry[]}).pages;
}

function componentsOf(pack: string, id: string): string[] {
  const file = path.resolve(__dirname, '../../../fixtures/expected', pack, 'site/pages', `${id}.json`);
  return (JSON.parse(readFileSync(file, 'utf8')) as {components: string[]}).components;
}

const marker: Record<string, string> = {
  LayerMap: '.sa-map',
  BlockHeader: '.sa-head',
  Lifecycle: '.sa-life',
  KpiCatalogue: '.sa-kpis',
  GraphView: '.sa-graph svg.sa-scene',
  FindingsList: '.sa-findings, .sa-findings__none',
  RecordTable: '.sa-record',
  ResultSections: '.sa-result, .sa-result__none',
};

for (const pack of ['harbor', 'orchard']) {
  for (const entry of pagesOf(pack)) {
    test(`${pack}: ${entry.id}`, async ({browser}, info) => {
      test.skip(info.project.name !== pack, 'another pack');
      const route = entry.slug.replace(/^\//, '');
      for (const theme of themes) {
        const context = await browser.newContext({baseURL: info.project.use.baseURL});
        const page = await context.newPage();
        for (const viewport of viewports) {
          await page.setViewportSize(viewport);
          await page.goto(`${route}?docusaurus-theme=${theme}`, {waitUntil: 'load'});
          await expect(page.locator('html')).toHaveAttribute('data-theme', theme);
          const where = `${entry.id} ${theme} ${viewport.width}x${viewport.height}`;
          for (const name of componentsOf(pack, entry.id)) {
            await expect(page.locator(marker[name]).first(), `${where}: ${name}`).toBeVisible();
          }
          const sideways = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
          expect(sideways, `${where}: the page scrolls sideways`).toBeLessThanOrEqual(0);
          const result = await new AxeBuilder({page}).withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']).analyze();
          const bad = result.violations
            .filter((v) => v.impact === 'serious' || v.impact === 'critical')
            .map((v) => `${v.id} (${v.impact}): ${v.nodes.map((n) => n.target.join(' ')).slice(0, 3).join(', ')}`);
          expect(bad, where).toEqual([]);
        }
        await context.close();
      }
    });
  }
}
