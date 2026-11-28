/**
 * @beyond10x/metaspec-docusaurus: adds the pages `metaspec docs build` writes to a Docusaurus site
 * built on @beyond10x/docs-system. The build writes one site over one or more packs:
 * `withMetaspecSite(config, {out})` adds one docs plugin instance per pack over
 * `<out>/<pack>/docs`, routed at `/<pack>` with the pack's own sidebar, a pages plugin instance for
 * the site's page `/` (`<out>/_index/index.mdx`), and a plugin that loads the styles of
 * @beyond10x/metaspec-ui and the palette and tells the components how the site links.
 * `packNavbarItems(index)` gives one navbar item per pack. The site wraps the result with
 * docs-system's `withProductSite`, which draws every page's header.
 */
import {mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import path from 'node:path';
import type {LoadContext, Plugin} from '@docusaurus/types';

const require = createRequire(import.meta.url);

export interface MetaspecSiteOptions {
  /** The output directory of `metaspec docs build`, absolute or relative to `siteDir`. */
  out: string;
  /** The directory of the site, where `docusaurus.config.ts` is. */
  siteDir: string;
}

/** The parts of `<pack>/site.json` (metaspec-site/1) the preset reads. */
export interface SiteFile {
  format: string;
  pack: string;
  title: string;
  summary: string | null;
  visibility: 'public' | 'private';
  source_url: string | null;
  counts: Record<string, number>;
  sidebar: unknown[];
  pages: {id: string; kind: string; slug: string; title: string}[];
}

/** One pack of `index.json` (metaspec-index/1). */
export interface IndexEntry {
  pack: string;
  title: string;
  summary: string | null;
  visibility: 'public' | 'private';
  source_url: string | null;
  level: string;
  route: string;
  site: string;
  counts: Record<string, number>;
  check: {status: 'valid' | 'refused' | 'not checked'; valid: number; refused: number; not_checked: number};
}

/** `index.json` (metaspec-index/1): the site's title and its packs. */
export interface IndexFile {
  format: string;
  title: string;
  packs: IndexEntry[];
}

export const SITE_FORMAT = 'metaspec-site/1';
export const INDEX_FORMAT = 'metaspec-index/1';

function readJson<T extends {format: string}>(file: string, format: string): T {
  let value: T;
  try {
    value = JSON.parse(readFileSync(file, 'utf8')) as T;
  } catch (error) {
    throw new Error(`metaspec-docusaurus: ${file} cannot be read (${(error as Error).message}); run metaspec docs build first`);
  }
  if (value.format !== format) {
    throw new Error(`metaspec-docusaurus: ${file} has format ${value.format}, not ${format}`);
  }
  return value;
}

/** Reads and checks `<out>/index.json`. */
export function readIndex(out: string): IndexFile {
  return readJson<IndexFile>(path.join(out, 'index.json'), INDEX_FORMAT);
}

/** Reads and checks the part of one pack of the site, `<out>/<pack>/site.json`. */
export function readSite(out: string, pack: string): SiteFile {
  return readJson<SiteFile>(path.join(out, pack, 'site.json'), SITE_FORMAT);
}

/** Whether a site address is this machine only. */
export function isLoopback(url: string): boolean {
  try {
    const host = new URL(url).hostname;
    return host === 'localhost' || host === '127.0.0.1' || host === '[::1]' || host === '::1';
  } catch {
    return false;
  }
}

/** The navbar items of the packs: one per pack, its title, opening its own sidebar. */
export function packNavbarItems(index: IndexFile) {
  return index.packs.map((p) => ({
    type: 'docSidebar' as const,
    docsPluginId: p.pack,
    sidebarId: p.pack,
    label: p.title,
    position: 'left' as const,
  }));
}

interface PluginOptions {
  out: string;
}

/**
 * The plugin of the preset: the client modules (the components' styles, the palette and the
 * configuration of the links) and the alias that lets generated pages outside the site import
 * @beyond10x/metaspec-ui. The routes in the pages' data carry the pack's route, so the
 * components link below the site's `baseUrl` with no base of their own.
 */
export function metaspecPlugin(context: LoadContext, options: PluginOptions): Plugin {
  const out = path.resolve(context.siteDir, options.out);
  const dir = path.join(context.generatedFilesDir, 'metaspec');
  mkdirSync(dir, {recursive: true});
  const client = path.join(dir, 'client.js');
  writeFileSync(
    client,
    [
      '// Written by @beyond10x/metaspec-docusaurus. Do not edit.',
      "import {createElement} from 'react';",
      "import Link from '@docusaurus/Link';",
      "import {configure} from '@beyond10x/metaspec-ui';",
      'function SiteLink(props) {',
      '  return createElement(Link, props);',
      '}',
      "configure({Link: SiteLink, base: ''});",
      '',
    ].join('\n'),
  );
  const ui = path.dirname(require.resolve('@beyond10x/metaspec-ui/package.json'));
  return {
    name: 'metaspec-docusaurus',
    getClientModules() {
      return [path.join(ui, 'dist/styles.css'), path.join(out, 'palette.css'), client];
    },
    getPathsToWatch() {
      return [path.join(out, '**/*')];
    },
    configureWebpack() {
      return {resolve: {alias: {'@beyond10x/metaspec-ui$': path.join(ui, 'dist/index.js')}}};
    },
  };
}

/**
 * Adds the site's pages to a Docusaurus config: per pack a docs plugin instance (id and route the
 * pack's name) with its sidebar, and the page `/`. A site with a pack that is not public is
 * refused unless the site's `url` is this machine (localhost or 127.0.0.1): its pages are not
 * published.
 */
export function withMetaspecSite<T extends Record<string, unknown>>(config: T, options: MetaspecSiteOptions): T {
  const out = path.resolve(options.siteDir, options.out);
  const index = readIndex(out);
  const url = String(config.url ?? '');
  const closed = index.packs.filter((p) => p.visibility !== 'public');
  if (closed.length > 0 && !isLoopback(url)) {
    throw new Error(
      `metaspec-docusaurus: the pack "${closed[0].title}" is ${closed[0].visibility}; a site with it is served only from this machine (url http://127.0.0.1 or http://localhost), not ${url || 'an unset url'}`,
    );
  }
  const generated = path.join(options.siteDir, '.docusaurus', 'metaspec');
  mkdirSync(generated, {recursive: true});
  const relative = (p: string) => path.relative(options.siteDir, p);
  const plugins = Array.isArray(config.plugins) ? [...(config.plugins as unknown[])] : [];
  for (const entry of index.packs) {
    const site = readSite(out, entry.pack);
    const sidebarPath = path.join(generated, `sidebars-${entry.pack}.json`);
    writeFileSync(sidebarPath, `${JSON.stringify({[entry.pack]: site.sidebar}, null, 2)}\n`);
    plugins.push([
      '@docusaurus/plugin-content-docs',
      {
        id: entry.pack,
        path: relative(path.join(out, entry.pack, 'docs')),
        routeBasePath: entry.pack,
        sidebarPath: relative(sidebarPath),
        numberPrefixParser: false,
        showLastUpdateTime: false,
        showLastUpdateAuthor: false,
      },
    ]);
  }
  plugins.push(
    [
      '@docusaurus/plugin-content-pages',
      {id: 'metaspec-index', path: relative(path.join(out, '_index')), routeBasePath: '/', include: ['index.mdx'], exclude: []},
    ],
    [metaspecPlugin, {out: relative(out)} satisfies PluginOptions],
  );
  return {...config, plugins};
}

export default withMetaspecSite;
