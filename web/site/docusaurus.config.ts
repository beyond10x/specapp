import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {withProductSite} from '@beyond10x/docs-system/product-site';
import {readSite, withMetaspecSite} from '@beyond10x/metaspec-docusaurus';

// The docs site of one pack. By default the pages of a fixture pack that metaspec docs build wrote
// and committed under fixtures/expected/<pack>/site: METASPEC_SITE_PACK names the pack (default
// harbor); the UX lane builds each into build/<pack>. METASPEC_SITE_OUT names the output directory
// of any other docs build instead (the acceptance builds the real packs' sites so).

const siteDir = path.dirname(fileURLToPath(import.meta.url));
const pack = process.env.METASPEC_SITE_PACK ?? 'harbor';
const out = process.env.METASPEC_SITE_OUT
  ? path.resolve(process.env.METASPEC_SITE_OUT)
  : path.resolve(siteDir, '../../fixtures/expected', pack, 'site');
const site = readSite(out);

const config: Config = {
  title: site.title,
  tagline: site.summary,
  future: {v4: true},
  url: 'http://127.0.0.1',
  baseUrl: '/',
  trailingSlash: false,
  onBrokenLinks: 'throw',
  onBrokenAnchors: 'throw',
  markdown: {format: 'detect', hooks: {onBrokenMarkdownLinks: 'throw'}},
  i18n: {defaultLocale: 'en', locales: ['en']},
  presets: [
    [
      'classic',
      {
        docs: false,
        blog: false,
        pages: false,
      } satisfies Preset.Options,
    ],
  ],
  themeConfig: {
    navbar: {
      title: site.title,
      items: site.pages
        .filter((p) => ['layer-map', 'kpis', 'class', 'rules'].includes(p.kind))
        .map((p) => ({to: p.slug, label: p.kind === 'layer-map' ? 'Building blocks' : p.title, position: 'left' as const})),
    },
    footer: {copyright: `${site.title}: pages written by metaspec docs build.`},
  } satisfies Preset.ThemeConfig,
};

const built = withProductSite(withMetaspecSite(config, {out, siteDir, routeBasePath: '/'}), {mark: site.title.slice(0, 1)});

export default {
  ...built,
  staticDirectories: built.staticDirectories?.map((directory) =>
    directory && path.isAbsolute(directory) ? path.relative(siteDir, directory) : directory,
  ),
} satisfies Config;
