#!/bin/sh
# ux.sh: the UX lane. Builds the packages of web/, builds the docs site of each fixture pack from
# its committed pages (fixtures/expected/<pack>/site) into web/site/build/<pack>, then runs the
# Playwright gates on both: every page at 1440x900 and 1280x800, light and dark, no serious or
# critical axe-core violation, no sideways scroll. A test is killed at 60 s, the run at 120 s.
#
#   scripts/ux.sh
#
# Needs `npm ci` in web/ and a Chromium for Playwright (npx playwright install chromium).
# Exit 0: green. Otherwise the exit code of the failed step.

set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root/web"
start=$(date +%s)

slot sh -c 'npm run build --workspace ui && npm run build --workspace docusaurus'
for pack in harbor orchard; do
  rm -rf "site/build/$pack"
  (cd site && METASPEC_SITE_PACK=$pack slot --timeout 15m npx docusaurus build --out-dir "build/$pack")
done
built=$(date +%s)
echo "ux: sites built in $((built - start)) s"

(cd site && slot npx playwright test --workers=2)
echo "ux: gates passed in $(($(date +%s) - built)) s"
