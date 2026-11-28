#!/bin/sh
# parity.sh: compare the docs site metaspec generates for agenticspec with the pages agenticspec
# writes itself (its docs/ directory), page by page, section by section and component by
# component.
#
#   scripts/parity.sh <agenticspec-dir> [<manifest>]
#
# Generates the site with `metaspec docs build agenticspec --pack <dir>` (the manifest metaspec
# knows, packs/agenticspec.yaml), or with `--manifest <manifest>` when one is given, into
# target/parity/site (the pack's tool, if the manifest names one, trusted under
# target/parity/state only), then checks:
#
#   block pages       docs/blocks/<N-layer>/<block>.md  against  docs/blocks/<N-layer>/<block>.mdx
#   layer map         docs/blocks/index.md and planned.md against the generated ones
#   documents         docs/architectures/<name>.md  against  the generated document pages
#   KPI catalogue     docs/kpis-catalogue.md against kpis.mdx; the rows of kpis/catalogue.yaml
#                     against the rows of the generated catalogue
#   sections          every ## and ### heading of each page agenticspec writes, on the generated
#                     page of the same name (a "Graph" section is a drawn view, GraphView)
#   components        every component agenticspec's pages draw, by its counterpart in
#                     @beyond10x/metaspec-ui
#
# Expected to be missing, and listed as such: the catalog (docs/catalog: downloads, deployment and
# the results of a run of the pack's tool, which the docs build does not run) and the sections
# that show such a result.
#
# Prints one line per item, `green <item> ...` or `missing <item>: ...` or `expected-missing ...`,
# then a summary line. Exit 0: nothing is missing beyond the expected. Exit 1: an item is
# missing. Exit 2: bad usage, or metaspec failed. Not part of `task check`: it needs a checkout
# of agenticspec. Needs jq and cargo.

set -eu

[ $# -eq 1 ] || [ $# -eq 2 ] || { echo "usage: scripts/parity.sh <agenticspec-dir> [<manifest>]" >&2; exit 2; }
pack=$(CDPATH= cd -- "$1" && pwd)
# "$@" becomes the pack's manifest: its name, or --manifest and its absolute path
if [ $# -eq 2 ]; then
  set -- --manifest "$(CDPATH= cd -- "$(dirname -- "$2")" && pwd)/$(basename -- "$2")"
else
  set -- agenticspec
fi

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$root/target/parity
rm -rf "$work"
mkdir -p "$work/state"

(cd "$root" && cargo build --quiet --locked -p metaspec)
metaspec=$root/target/debug/metaspec
XDG_STATE_HOME=$work/state "$metaspec" trust "$@" --pack "$pack" > /dev/null 2>&1 || true
XDG_STATE_HOME=$work/state "$metaspec" docs build "$@" --pack "$pack" --out "$work/site" \
  || { echo "parity: metaspec docs build failed" >&2; exit 2; }
site=$work/site
theirs=$pack/docs

missing=0
expected=0
green=0
say_green() { echo "green $*"; green=$((green + 1)); }
say_missing() { echo "missing $*"; missing=$((missing + 1)); }
say_expected() { echo "expected-missing $*"; expected=$((expected + 1)); }

# compare <item> <their list> <our list>: names in the first and not in the second
compare() {
  sort -u "$2" > "$work/a"
  sort -u "$3" > "$work/b"
  n=$(wc -l < "$work/a" | tr -d ' ')
  gone=$(comm -23 "$work/a" "$work/b" | tr '\n' ' ')
  if [ -z "$gone" ]; then
    say_green "$1 $n/$n"
  else
    say_missing "$1: $gone"
  fi
}

# --- pages ---
(cd "$theirs" && find blocks -mindepth 2 -name '*.md' | sed 's/\.md$//') > "$work/their-blocks"
(cd "$site/docs" && find blocks -mindepth 2 -name '*.mdx' | sed 's/\.mdx$//') > "$work/our-blocks"
compare "block pages" "$work/their-blocks" "$work/our-blocks"

for page in index planned; do
  if [ -f "$theirs/blocks/$page.md" ]; then
    if [ -f "$site/docs/blocks/$page.mdx" ]; then say_green "page blocks/$page"; else say_missing "page blocks/$page"; fi
  fi
done

jq -r '.pages[] | select(.kind == "document" or .kind == "record") | .id' "$site/site.json" > "$work/our-doc-ids"
sed 's#.*/##' "$work/our-doc-ids" > "$work/our-docs"
(cd "$theirs/architectures" && ls -1 *.md | grep -v '^index\.md$' | sed 's/\.md$//') > "$work/their-docs"
compare "document pages" "$work/their-docs" "$work/our-docs"
if [ -f "$theirs/architectures/index.md" ]; then
  if jq -e '.pages[] | select(.kind == "class")' "$site/site.json" > /dev/null; then
    say_green "page architectures/index (class index)"
  else
    say_missing "page architectures/index"
  fi
fi

if [ -f "$theirs/kpis-catalogue.md" ]; then
  if [ -f "$site/docs/kpis.mdx" ]; then say_green "page kpis-catalogue"; else say_missing "page kpis-catalogue"; fi
fi
own=$(grep -c '^  - id: ' "$pack/kpis/catalogue.yaml" || true)
ours=$(jq '.data.catalogue.kpis | length' "$site/pages/kpis.json")
if [ "$own" = "$ours" ]; then say_green "KPI rows $ours/$own"; else say_missing "KPI rows: $ours of $own"; fi

# --- sections ---
# headings <file>: its ## and ### headings, one per line
headings() {
  sed -n 's/^###* \(.*\)$/\1/p' "$1" | sed 's/[[:space:]]*$//'
}
: > "$work/sections-missing"
: > "$work/sections-expected"
nsections=0
check_sections() {
  their=$1
  our=$2
  name=$3
  [ -f "$our" ] || return 0
  headings "$our" > "$work/our-h"
  headings "$their" | while IFS= read -r h; do
    case "$h" in
      Graph)
        grep -q '<GraphView ' "$our" && echo ok || echo "$name: $h" >> "$work/sections-missing" ;;
      'What the optimization changed' | Result | Download | 'Run it' | Deploy | Optimization | 'Search result' | 'KPIs that moved')
        echo "$name: $h" >> "$work/sections-expected" ;;
      *)
        grep -qxF "$h" "$work/our-h" && echo ok || echo "$name: $h" >> "$work/sections-missing" ;;
    esac
  done > /dev/null
  nsections=$((nsections + $(headings "$their" | wc -l)))
}
while IFS= read -r id; do
  check_sections "$theirs/$id.md" "$site/docs/$id.mdx" "$id"
done < "$work/their-blocks"
for page in index planned; do
  [ -f "$theirs/blocks/$page.md" ] && check_sections "$theirs/blocks/$page.md" "$site/docs/blocks/$page.mdx" "blocks/$page"
done
while IFS= read -r id; do
  stem=${id##*/}
  [ -f "$theirs/architectures/$stem.md" ] && check_sections "$theirs/architectures/$stem.md" "$site/docs/$id.mdx" "architectures/$stem"
done < "$work/our-doc-ids"
[ -f "$theirs/kpis-catalogue.md" ] && check_sections "$theirs/kpis-catalogue.md" "$site/docs/kpis.mdx" "kpis-catalogue"
if [ -s "$work/sections-missing" ]; then
  say_missing "sections ($(wc -l < "$work/sections-missing" | tr -d ' ') of $nsections): $(sort "$work/sections-missing" | awk -F': ' '{ c[$2]++ } END { for (h in c) printf "%s (%d pages); ", h, c[h] }')"
else
  say_green "sections $nsections/$nsections"
fi

# --- components ---
# Their component -> ours. A component of the catalog is expected to be missing.
grep -rhoE '^<[A-Z][A-Za-z]*' "$theirs/blocks" "$theirs/architectures" "$theirs/kpis-catalogue.md" 2>/dev/null \
  | sed 's/^<//' | sort -u > "$work/their-components"
jq -r '.components[]' "$site"/pages/*.json "$site"/pages/*/*.json "$site"/pages/*/*/*.json 2>/dev/null | sort -u > "$work/our-components"
while IFS= read -r c; do
  case "$c" in
    ArchitectureFrame) mine=GraphView ;;
    BlockMap) mine=LayerMap ;;
    *) mine=$c ;;
  esac
  if grep -qxF "$mine" "$work/our-components"; then
    say_green "component $c -> $mine"
  else
    say_missing "component $c (no $mine on any generated page)"
  fi
done < "$work/their-components"

# --- expected to be missing ---
if [ -d "$theirs/catalog" ]; then
  n=$(find "$theirs/catalog" -name '*.md' | wc -l | tr -d ' ')
  say_expected "catalog: $n pages (downloads, deployment, results of a run; the docs build runs no simulation)"
fi
if [ -s "$work/sections-expected" ]; then
  say_expected "sections of a run result: $(cut -d: -f2- "$work/sections-expected" | sort | uniq -c | awk '{ $1 = $1 "x"; print }' | tr '\n' ';')"
fi

echo "parity: $green green, $missing missing, $expected expected missing (site: target/parity/site)"
[ "$missing" = 0 ]
