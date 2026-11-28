/**
 * Renders every component tag of every committed page of both fixture packs (their parts of the
 * one site fixtures/expected/site, written by `metaspec docs build`) with the data and
 * presentation the page's MDX hands it, and checks what each draws against that data.
 */
import {cleanup, fireEvent, render} from '@testing-library/react';
import {existsSync, readdirSync, readFileSync, statSync} from 'node:fs';
import {join, relative, resolve} from 'node:path';
import type {ComponentType} from 'react';
import {afterEach, describe, expect, it} from 'vitest';
import * as ui from '../src/index.js';
import type {
  FindingsData,
  GraphSwitchData,
  KpiCatalogueData,
  LayerMapData,
  Presentation,
  RecordTableData,
  Scene,
} from '../src/index.js';

const expected = resolve(process.cwd(), '../../fixtures/expected/site');
const packs = (JSON.parse(readFileSync(join(expected, 'index.json'), 'utf8')) as {packs: {pack: string}[]}).packs
  .map((p) => p.pack)
  .filter((n) => existsSync(join(expected, n, 'site.json')))
  .sort();
if (packs.length < 2) throw new Error(`fixtures/expected/site holds ${packs.length} packs; metaspec docs build writes them`);

function walk(dir: string): string[] {
  return readdirSync(dir).flatMap((n) => {
    const p = join(dir, n);
    return statSync(p).isDirectory() ? walk(p) : [p];
  });
}

interface Tag {
  name: string;
  data: unknown;
}

const TAG = /<([A-Z][A-Za-z]*) data=\{(?:page\.data\["([^"]+)"\]|page\.data\.tables\[(\d+)\]|(null))\} presentation=\{site\.presentation\} \/>/g;

function tags(mdx: string, page: {data: Record<string, unknown>}): Tag[] {
  return [...mdx.matchAll(TAG)].map((m) => ({
    name: m[1],
    data: m[2] !== undefined ? page.data[m[2]] : m[3] !== undefined ? (page.data.tables as unknown[])[Number(m[3])] : null,
  }));
}

afterEach(cleanup);

describe.each(packs)('pack %s', (pack) => {
  const root = join(expected, pack);
  const site = JSON.parse(readFileSync(join(root, 'site.json'), 'utf8')) as {
    presentation: Presentation;
    counts: Record<string, number>;
  };
  const docs = walk(join(root, 'docs')).filter((p) => p.endsWith('.mdx'));

  it('every page draws each of its components without an error', () => {
    let drawn = 0;
    for (const file of docs) {
      const id = relative(join(root, 'docs'), file).replace(/\.mdx$/, '');
      const page = JSON.parse(readFileSync(join(root, 'pages', `${id}.json`), 'utf8'));
      const mdx = readFileSync(file, 'utf8');
      const found = tags(mdx, page);
      expect(new Set(found.map((t) => t.name)), id).toEqual(new Set(page.components));
      for (const tag of found) {
        const Component = (ui as unknown as Record<string, ComponentType<{data: unknown; presentation: Presentation}>>)[tag.name];
        expect(Component, `${id}: ${tag.name}`).toBeTypeOf('function');
        if (tag.name !== 'ResultSections') expect(tag.data, `${id}: ${tag.name} has data`).toBeTruthy();
        const {container} = render(<Component data={tag.data} presentation={site.presentation} />);
        expect(container.innerHTML.length, `${id}: ${tag.name}`).toBeGreaterThan(0);
        expect(container.innerHTML, `${id}: ${tag.name}`).not.toContain('undefined');
        cleanup();
        drawn += 1;
      }
    }
    expect(drawn).toBeGreaterThan(5);
  });

  it('the layer map draws one card per block, each in its colour class', () => {
    const page = JSON.parse(readFileSync(join(root, 'pages/blocks/index.json'), 'utf8'));
    const map = page.data.map as LayerMapData;
    const {container} = render(<ui.LayerMap data={map} presentation={site.presentation} />);
    const cards = container.querySelectorAll('.sa-card');
    expect(cards.length).toBe(map.blocks.length);
    expect(cards.length).toBe(site.counts.blocks);
    for (const b of map.blocks) {
      const colour = site.presentation.blocks[b.name]?.colour ?? 'slate';
      expect(container.querySelector(`.sa-card.sa-c-${colour}`), b.name).not.toBeNull();
    }
    expect(container.querySelectorAll('.sa-map__edge').length).toBe(map.edges.length);
  });

  it('the KPI catalogue lists every KPI with an anchor', () => {
    const page = JSON.parse(readFileSync(join(root, 'pages/kpis.json'), 'utf8'));
    const data = page.data.catalogue as KpiCatalogueData;
    const {container} = render(<ui.KpiCatalogue data={data} presentation={site.presentation} />);
    expect(data.kpis.length).toBe(site.counts.kpis);
    for (const k of data.kpis) {
      expect(container.querySelector(`#kpi-${k.id}`), k.id).not.toBeNull();
    }
  });

  it('each drawing has its boxes, edges and finding marks, and each mark links to a finding row', () => {
    for (const file of walk(join(root, 'pages'))) {
      const page = JSON.parse(readFileSync(file, 'utf8'));
      const scenes = Object.entries(page.data).filter(([k]) => k.startsWith('scene-'));
      if (scenes.length === 0) continue;
      const findings = page.data.findings as FindingsData | undefined;
      const list = findings ? render(<ui.FindingsList data={findings} presentation={site.presentation} />).container : null;
      const drawings = scenes.flatMap(([key, value]) =>
        'views' in (value as object) ? (value as GraphSwitchData).views.map((s) => [key, s] as const) : [[key, value as Scene] as const],
      );
      for (const [key, scene] of drawings) {
        const {container} = render(<ui.GraphView data={scene} presentation={site.presentation} />);
        const where = `${relative(root, file)} ${key}`;
        expect(container.querySelectorAll('g.sa-scene__badge').length, where).toBe(scene.badges?.length ?? 0);
        expect(container.querySelectorAll('g.sa-scene__box--hub').length, where).toBe(scene.boxes.filter((b) => b.hub).length);
        expect(container.querySelector(`[id="view-${scene.view.id}"]`), where).not.toBeNull();
        expect(container.querySelectorAll('g.sa-scene__box').length, where).toBe(scene.boxes.length);
        expect(container.querySelectorAll('g.sa-scene__edge').length, where).toBe(scene.edges.length);
        const marks = container.querySelectorAll('a[href^="#finding-"]');
        expect(marks.length, where).toBe(scene.marks.length);
        for (const a of marks) {
          const target = a.getAttribute('href')!.slice(1);
          expect(list?.querySelector(`#${target}`), `${where} ${target}`).not.toBeNull();
        }
        expect(container.querySelector('svg.sa-scene')?.getAttribute('width'), where).toBe(String(scene.width));
      }
      cleanup();
    }
  });

  it('a view with hubs and its edges view are one drawing with a switch', () => {
    let switches = 0;
    let tagged = 0;
    for (const file of walk(join(root, 'pages'))) {
      const page = JSON.parse(readFileSync(file, 'utf8'));
      const mdx = join(root, 'docs', relative(join(root, 'pages'), file).replace(/\.json$/, '.mdx'));
      tagged += readFileSync(mdx, 'utf8').split('<GraphSwitch ').length - 1;
      for (const [key, value] of Object.entries(page.data)) {
        if (!key.startsWith('scene-') || !('views' in (value as object))) continue;
        const data = value as GraphSwitchData;
        const [hubs, edges] = data.views;
        const where = `${relative(root, file)} ${key}`;
        expect(edges.view.id, where).toBe(`${hubs.view.id}/edges`);
        const {container} = render(<ui.GraphSwitch data={data} presentation={site.presentation} />);
        const buttons = container.querySelectorAll('button');
        expect(buttons.length, where).toBe(2);
        expect(container.querySelector(`[id="view-${hubs.view.id}"]`), where).not.toBeNull();
        expect(container.querySelector(`[id="view-${edges.view.id}"]`), where).not.toBeNull();
        expect(container.querySelectorAll('g.sa-scene__badge').length, where).toBe(hubs.badges?.length ?? 0);
        expect(container.querySelectorAll('g.sa-scene__edge').length, where).toBe(hubs.edges.length);
        fireEvent.click(buttons[1]);
        expect(buttons[1].getAttribute('aria-pressed'), where).toBe('true');
        expect(container.querySelectorAll('g.sa-scene__badge').length, where).toBe(0);
        expect(container.querySelectorAll('g.sa-scene__edge').length, where).toBe(edges.edges.length);
        expect(edges.edges.length, where).toBe(hubs.edges.length + (hubs.badges?.length ?? 0));
        cleanup();
        switches += 1;
      }
    }
    expect(switches, pack).toBe(tagged);
  });

  it('a record table has one row per record', () => {
    for (const file of walk(join(root, 'pages'))) {
      const page = JSON.parse(readFileSync(file, 'utf8'));
      for (const table of (page.data.tables ?? []) as RecordTableData[]) {
        const {container} = render(<ui.RecordTable data={table} presentation={site.presentation} />);
        expect(container.querySelectorAll('tbody tr').length).toBe(table.rows.length);
        expect(container.querySelectorAll('thead th').length).toBe(table.columns.length);
        cleanup();
      }
    }
  });
});

describe('components know no pack', () => {
  it('a block missing from the presentation table is drawn in slate', () => {
    const presentation: Presentation = {blocks: {}, groups: [], icons: {}, vocabulary: {}};
    const {container} = render(
      <ui.BlockHeader
        data={{name: 'x', label: 'x', layer: null, layer_name: null, planned: true, counts: {}}}
        presentation={presentation}
      />,
    );
    expect(container.querySelector('.sa-c-slate')).not.toBeNull();
  });

  it('a result stub says that no result is committed', () => {
    const {container} = render(<ui.ResultSections data={null} />);
    expect(container.textContent).toContain('No run result');
  });
});
