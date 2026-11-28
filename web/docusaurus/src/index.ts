/**
 * @beyond10x/specapp-docusaurus: adds the pages `specapp docs build` writes to a Docusaurus site
 * built on @beyond10x/docs-system. `withSpecappSite(config, {out})` adds one docs plugin instance
 * over `<out>/docs` with the generated sidebar, and a plugin that loads the styles of
 * @beyond10x/specapp-ui and the pack's palette and tells the components how the site links. The
 * site wraps the result with docs-system's `withProductSite`, which draws every page's header.
 */
import {mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import path from 'node:path';
import type {LoadContext, Plugin} from '@docusaurus/types';

const require = createRequire(import.meta.url);

export interface SpecappSiteOptions {
  /** The output directory of `specapp docs build`, absolute or relative to `siteDir`. */
  out: string;
  /** The directory of the site, where `docusaurus.config.ts` is. */
  siteDir: string;
  /** The route of the pack's pages below `baseUrl`. Default `docs`. */
  routeBasePath?: string;
  /** The id of the docs plugin instance. Default `specapp`. */
  id?: string;
}

/** The parts of site.json (specapp-site/1) the preset reads. */
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

export const SITE_FORMAT = 'specapp-site/1';

/** Reads and checks `<out>/site.json`. */
export function readSite(out: string): SiteFile {
  const file = path.join(out, 'site.json');
  let site: SiteFile;
  try {
    site = JSON.parse(readFileSync(file, 'utf8')) as SiteFile;
  } catch (error) {
    throw new Error(`specapp-docusaurus: ${file} cannot be read (${(error as Error).message}); run specapp docs build first`);
  }
  if (site.format !== SITE_FORMAT) {
    throw new Error(`specapp-docusaurus: ${file} has format ${site.format}, not ${SITE_FORMAT}`);
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
 * import @beyond10x/specapp-ui.
 */
export function specappPlugin(context: LoadContext, options: PluginOptions): Plugin {
  const out = path.resolve(context.siteDir, options.out);
  const dir = path.join(context.generatedFilesDir, 'specapp');
  mkdirSync(dir, {recursive: true});
  const client = path.join(dir, 'client.js');
  const base = `/${options.routeBasePath.replace(/^\/+|\/+$/g, '')}`;
  writeFileSync(
    client,
    [
      '// Written by @beyond10x/specapp-docusaurus. Do not edit.',
      "import {createElement} from 'react';",
      "import Link from '@docusaurus/Link';",
      "import {configure} from '@beyond10x/specapp-ui';",
      'function SiteLink(props) {',
      '  return createElement(Link, props);',
      '}',
      `configure({Link: SiteLink, base: ${JSON.stringify(base === '/' ? '' : base)}});`,
      '',
    ].join('\n'),
  );
  const ui = path.dirname(require.resolve('@beyond10x/specapp-ui/package.json'));
  return {
    name: 'specapp-docusaurus',
    getClientModules() {
      return [path.join(ui, 'dist/styles.css'), path.join(out, 'palette.css'), client];
    },
    getPathsToWatch() {
      return [path.join(out, '**/*')];
    },
    configureWebpack() {
      return {resolve: {alias: {'@beyond10x/specapp-ui$': path.join(ui, 'dist/index.js')}}};
    },
  };
}

/**
 * Adds the pack's pages to a Docusaurus config. A pack that is not public is refused unless the
 * site's `url` is this machine (localhost or 127.0.0.1): its pages are not published.
 */
export function withSpecappSite<T extends Record<string, unknown>>(config: T, options: SpecappSiteOptions): T {
  const out = path.resolve(options.siteDir, options.out);
  const site = readSite(out);
  const url = String(config.url ?? '');
  if (site.visibility !== 'public' && !isLoopback(url)) {
    throw new Error(
      `specapp-docusaurus: the pack "${site.title}" is ${site.visibility}; its site is served only from this machine (url http://127.0.0.1 or http://localhost), not ${url || 'an unset url'}`,
    );
  }
  const routeBasePath = options.routeBasePath ?? 'docs';
  const id = options.id ?? 'specapp';
  const generated = path.join(options.siteDir, '.docusaurus', 'specapp');
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
    [specappPlugin, {out: path.relative(options.siteDir, out), routeBasePath} satisfies PluginOptions],
  );
  return {...config, plugins};
}

export default withSpecappSite;
