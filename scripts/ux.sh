#!/bin/sh
# ux.sh: the UX lane. Builds the packages of web/, builds the one docs site of both fixture packs
# from its committed pages (fixtures/expected/site) into web/site/build/fixtures, then runs the
# Playwright gates on it: the page / and every page of each pack at 1440x900 and 1280x800, light
# and dark, no serious or critical axe-core violation, no sideways scroll; the navbar title is
# metaspec on every page, each pack is a navbar item, and / lists every pack. A test is killed at
# 60 s, the run at 120 s.
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
rm -rf site/build/fixtures
(cd site && slot --timeout 15m npx docusaurus build --out-dir build/fixtures)
built=$(date +%s)
echo "ux: site built in $((built - start)) s"

(cd site && slot npx playwright test --workers=2)
echo "ux: gates passed in $(($(date +%s) - built)) s"
