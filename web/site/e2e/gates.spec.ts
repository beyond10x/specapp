import AxeBuilder from '@axe-core/playwright';
import {expect, test, type Page} from '@playwright/test';
import {readFileSync} from 'node:fs';
import path from 'node:path';

// One test for the page / and one per generated page of each fixture pack, all in the one site of
// both packs: at both viewports and in both themes, axe-core finds no serious or critical
// violation, the page does not scroll sideways, the navbar's title is the site's (metaspec) and
// each pack is a navbar item with its own title, and every component the page names is drawn.
// The page / lists every pack with a link to its pages.

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

interface IndexEntry {
  pack: string;
  title: string;
  route: string;
  counts: Record<string, number>;
  check: {status: string};
}

const site = path.resolve(__dirname, '../../../fixtures/expected/site');
const index = JSON.parse(readFileSync(path.join(site, 'index.json'), 'utf8')) as {title: string; packs: IndexEntry[]};

function pagesOf(pack: string): PageEntry[] {
  return (JSON.parse(readFileSync(path.join(site, pack, 'site.json'), 'utf8')) as {pages: PageEntry[]}).pages;
}

function componentsOf(pack: string, id: string): string[] {
  return (JSON.parse(readFileSync(path.join(site, pack, 'pages', `${id}.json`), 'utf8')) as {components: string[]}).components;
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

/** Opens a route in each theme and viewport and runs the gates every page shares, then `more`. */
async function gate(
  browser: import('@playwright/test').Browser,
  baseURL: string | undefined,
  route: string,
  name: string,
  more: (page: Page, where: string) => Promise<void>,
) {
  // One page load per theme; the second viewport is a resize of the loaded page.
  const context = await browser.newContext({baseURL, viewport: viewports[0]});
  const page = await context.newPage();
  for (const theme of themes) {
    await page.setViewportSize(viewports[0]);
    await page.goto(`${route}?docusaurus-theme=${theme}`, {waitUntil: 'load'});
    await expect(page.locator('html')).toHaveAttribute('data-theme', theme);
    for (const viewport of viewports) {
      await page.setViewportSize(viewport);
      const where = `${name} ${theme} ${viewport.width}x${viewport.height}`;
      await expect(page.locator('.navbar__title'), `${where}: the navbar title`).toHaveText(index.title);
      const items = page.locator('.navbar__items:not(.navbar__items--right) a.navbar__link');
      await expect(items, `${where}: one navbar item per pack`).toHaveText(index.packs.map((p) => p.title));
      for (const entry of index.packs) {
        await expect(items.filter({hasText: entry.title}), `${where}: ${entry.pack} in the navbar`).toHaveAttribute('href', entry.route);
      }
      await more(page, where);
      const sideways = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
      expect(sideways, `${where}: the page scrolls sideways`).toBeLessThanOrEqual(0);
      const result = await new AxeBuilder({page}).withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']).analyze();
      const bad = result.violations
        .filter((v) => v.impact === 'serious' || v.impact === 'critical')
        .map((v) => `${v.id} (${v.impact}): ${v.nodes.map((n) => n.target.join(' ')).slice(0, 3).join(', ')}`);
      expect(bad, where).toEqual([]);
    }
  }
  await context.close();
}

test('site: / lists every pack', async ({browser}, info) => {
  await gate(browser, info.project.use.baseURL, '', '/', async (page, where) => {
    await expect(page.locator('h1'), where).toHaveText(index.title);
    const rows = page.locator('main table tbody tr');
    await expect(rows, `${where}: one row per pack`).toHaveCount(index.packs.length);
    for (const [i, entry] of index.packs.entries()) {
      const row = rows.nth(i);
      await expect(row.locator('a', {hasText: entry.title}), `${where}: ${entry.pack}`).toHaveAttribute('href', entry.route);
      const cells = row.locator('td');
      await expect(cells.nth(2), `${where}: ${entry.pack} blocks`).toHaveText(String(entry.counts.blocks));
      await expect(cells.nth(3), `${where}: ${entry.pack} documents`).toHaveText(String(entry.counts.documents));
      await expect(cells.nth(4), `${where}: ${entry.pack} KPIs`).toHaveText(String(entry.counts.kpis));
      await expect(cells.nth(5), `${where}: ${entry.pack} check`).toContainText(entry.check.status);
    }
  });
});

for (const {pack} of index.packs) {
  for (const entry of pagesOf(pack)) {
    test(`${pack}: ${entry.id}`, async ({browser}, info) => {
      const route = entry.slug.replace(/^\//, '');
      await gate(browser, info.project.use.baseURL, route, `${pack} ${entry.id}`, async (page, where) => {
        for (const name of componentsOf(pack, entry.id)) {
          await expect(page.locator(marker[name]).first(), `${where}: ${name}`).toBeVisible();
        }
      });
    });
  }
}
