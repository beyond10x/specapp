/**
 * @beyond10x/metaspec-docusaurus: adds the pages `metaspec docs build` writes to a Docusaurus site
 * built on @beyond10x/docs-system. `withMetaspecSite(config, {out})` adds one docs plugin instance
 * over `<out>/docs` with the generated sidebar, and a plugin that loads the styles of
 * @beyond10x/metaspec-ui and the pack's palette and tells the components how the site links. The
 * site wraps the result with docs-system's `withProductSite`, which draws every page's header.
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
  /** The route of the pack's pages below `baseUrl`. Default `docs`. */
  routeBasePath?: string;
  /** The id of the docs plugin instance. Default `metaspec`. */
  id?: string;
}

/** The parts of site.json (metaspec-site/1) the preset reads. */
export interface SiteFile {
  format: string;
  title: string;
  summary: string;
  visibility: 'public' | 'private';
  source_url: string | null;
  counts: Record<string, number>;
  sidebar: unknown[];
  pages: {id: string; kind: string; slug: string; title: string}[];
}

export const SITE_FORMAT = 'metaspec-site/1';

/** Reads and checks `<out>/site.json`. */
export function readSite(out: string): SiteFile {
  const file = path.join(out, 'site.json');
  let site: SiteFile;
  try {
    site = JSON.parse(readFileSync(file, 'utf8')) as SiteFile;
  } catch (error) {
    throw new Error(`metaspec-docusaurus: ${file} cannot be read (${(error as Error).message}); run metaspec docs build first`);
  }
  if (site.format !== SITE_FORMAT) {
    throw new Error(`metaspec-docusaurus: ${file} has format ${site.format}, not ${SITE_FORMAT}`);
  }
  return site;
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

interface PluginOptions {
  out: string;
  routeBasePath: string;
}

/**
 * The plugin of the preset: the client modules (the components' styles, the pack's palette and
 * the configuration of the links) and the alias that lets generated pages outside the site
 * import @beyond10x/metaspec-ui.
 */
export function metaspecPlugin(context: LoadContext, options: PluginOptions): Plugin {
  const out = path.resolve(context.siteDir, options.out);
  const dir = path.join(context.generatedFilesDir, 'metaspec');
  mkdirSync(dir, {recursive: true});
  const client = path.join(dir, 'client.js');
  const base = `/${options.routeBasePath.replace(/^\/+|\/+$/g, '')}`;
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
      `configure({Link: SiteLink, base: ${JSON.stringify(base === '/' ? '' : base)}});`,
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
 * Adds the pack's pages to a Docusaurus config. A pack that is not public is refused unless the
 * site's `url` is this machine (localhost or 127.0.0.1): its pages are not published.
 */
export function withMetaspecSite<T extends Record<string, unknown>>(config: T, options: MetaspecSiteOptions): T {
  const out = path.resolve(options.siteDir, options.out);
  const site = readSite(out);
  const url = String(config.url ?? '');
  if (site.visibility !== 'public' && !isLoopback(url)) {
    throw new Error(
      `metaspec-docusaurus: the pack "${site.title}" is ${site.visibility}; its site is served only from this machine (url http://127.0.0.1 or http://localhost), not ${url || 'an unset url'}`,
    );
  }
  const routeBasePath = options.routeBasePath ?? 'docs';
  const id = options.id ?? 'metaspec';
  const generated = path.join(options.siteDir, '.docusaurus', 'metaspec');
  mkdirSync(generated, {recursive: true});
  const sidebarPath = path.join(generated, `sidebars-${id}.json`);
  writeFileSync(sidebarPath, `${JSON.stringify({[id]: site.sidebar}, null, 2)}\n`);
  const plugins = Array.isArray(config.plugins) ? [...(config.plugins as unknown[])] : [];
  plugins.push(
    [
      '@docusaurus/plugin-content-docs',
      {
        id,
        path: path.relative(options.siteDir, path.join(out, 'docs')),
        routeBasePath,
        sidebarPath: path.relative(options.siteDir, sidebarPath),
        numberPrefixParser: false,
        showLastUpdateTime: false,
        showLastUpdateAuthor: false,
      },
    ],
    [metaspecPlugin, {out: path.relative(options.siteDir, out), routeBasePath} satisfies PluginOptions],
  );
  return {...config, plugins};
}

export default withMetaspecSite;
