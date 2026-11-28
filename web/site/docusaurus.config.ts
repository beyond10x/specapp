import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {withProductSite} from '@beyond10x/docs-system/product-site';
import {packNavbarItems, readIndex, withMetaspecSite} from '@beyond10x/metaspec-docusaurus';

// The one docs site of every pack a docs build was given. By default the site of both fixture
// packs that metaspec docs build wrote and committed under fixtures/expected/site; the UX lane
// builds it into build/fixtures. METASPEC_SITE_OUT names the output directory of any other docs
// build instead (the acceptance builds the site of the real packs so).

const siteDir = path.dirname(fileURLToPath(import.meta.url));
const out = process.env.METASPEC_SITE_OUT
  ? path.resolve(process.env.METASPEC_SITE_OUT)
  : path.resolve(siteDir, '../../fixtures/expected/site');
const index = readIndex(out);

const config: Config = {
  title: index.title,
  tagline: 'The docs of every specification pack of this site.',
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
      title: index.title,
      items: packNavbarItems(index),
    },
    footer: {copyright: `${index.title}: pages written by metaspec docs build.`},
  } satisfies Preset.ThemeConfig,
};

const built = withProductSite(withMetaspecSite(config, {out, siteDir}), {mark: index.title.slice(0, 1)});

export default {
  ...built,
  staticDirectories: built.staticDirectories?.map((directory) =>
    directory && path.isAbsolute(directory) ? path.relative(siteDir, directory) : directory,
  ),
} satisfies Config;
