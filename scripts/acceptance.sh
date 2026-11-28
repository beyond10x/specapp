#!/bin/sh
# acceptance.sh: the acceptance of waves 1 ("Contract") and 2 ("Graph") on both fixture packs and
# both real packs.
#
#   scripts/acceptance.sh <orgspec-dir> <agenticspec-dir>
#
# specapp knows the packs; a pack knows nothing of specapp. The real packs are read with their
# manifests under packs/, by name (`specapp pack check orgspec --pack <dir>`); the fixture packs
# with fixtures/manifests/<name>.yaml. First scripts/check-pack-dirs.sh: no pack directory given
# here holds a file named like a manifest of specapp (the fixture packs: nothing of specapp at
# all).
#
# Runs `specapp pack check --json` on each pack and compares its counts with what the pack's
# own tooling prints:
#
#   fixture packs  blocks: the domains `ess specify compile` prints; layers: the numbered
#                  directories under blocks/; KPIs: the rows of the catalogue; documents: the
#                  files each class's glob lists (ls).
#   orgspec        blocks: scripts/check-layers.sh; layers: the layer numbers of
#                  scripts/check-layers.sh --graph; KPIs: scripts/check-kpis.sh; organizations:
#                  `orgspec check --json organizations/*.yaml`; types:
#                  `orgspec type check --json types/*.yaml`.
#   agenticspec    blocks: scripts/check-layers.sh; layers: the layer numbers of
#                  scripts/check-layers.sh --graph and the rows of its generated
#                  website/src/generated/blocks.json; KPIs: scripts/check-kpis.sh;
#                  architectures: the pages its generator wrote, docs/architectures/*.md
#                  without index.md.
#
# Wave 2: `specapp render` draws every parts-links/1 document of every pack, with the tools
# trusted in a state directory of this run (target/acceptance/state): one page per document
# (fixture packs: the drawn documents of pack check; orgspec: `orgspec check --json`;
# agenticspec: architectures/*.yaml). For every broken design of orgspec
# (organizations/broken/*.yaml, through a manifest of this run made from packs/orgspec.yaml)
# and every example, the rule ids in the page data equal those `orgspec check --json` reports,
# and every finding about a part or link is marked in a scene. The findings come from
# `orgspec check --json` itself, mapped by the adapter in packs/orgspec.yaml. Pages are under
# target/acceptance/pages-<pack>/.
#
# Then runs scripts/check-vocabulary.sh with all four packs.
#
# Not part of `task check` and not run in CI: it needs local checkouts of the real packs.
# The pack scripts write into their pack's target/ directory. Needs ess, jq, yq (for the
# packs' KPI scripts) and cargo.
#
# Exit 0: every count is equal and no word of a pack is in the code. Exit 1: a count differs,
# a word was found or a pack directory holds a manifest of specapp. Exit 2: bad usage or a tool
# failed.

set -eu

[ $# -eq 2 ] || { echo "usage: scripts/acceptance.sh <orgspec-dir> <agenticspec-dir>" >&2; exit 2; }
org=$(CDPATH= cd -- "$1" && pwd)
agentic=$(CDPATH= cd -- "$2" && pwd)

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$root/target/acceptance
rm -rf "$work"
mkdir -p "$work"

status=0
"$root/scripts/check-pack-dirs.sh" || status=1
"$root/scripts/check-pack-dirs.sh" "$org" "$agentic" || status=1

(cd "$root" && cargo build --quiet --locked -p specapp)
specapp=$root/target/debug/specapp

rows=$work/rows
: > "$rows"
# row <pack> <count> <specapp> <own> <source>
row() {
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" "$5" >> "$rows"
}

# check <name> <dir> <name|manifest-file>: specapp's report of a pack, in $work/<name>.json
check() {
  case "$3" in
    */*) "$specapp" pack check --manifest "$3" --pack "$2" --json > "$work/$1.json" || true ;;
    *) "$specapp" pack check "$3" --pack "$2" --json > "$work/$1.json" || true ;;
  esac
  if [ "$(jq -r .valid "$work/$1.json")" != true ]; then
    echo "acceptance: specapp refused the pack $1:" >&2
    jq -r '.errors[] | "  \(.code) \(.subject): \(.message)"' "$work/$1.json" >&2
    exit 1
  fi
}

# of <name> <jq path>: one value of specapp's report
of() {
  jq -r "$2" "$work/$1.json"
}

# documents <name> <class>: specapp's count of the documents of one class
documents() {
  jq -r --arg c "$2" '.documents[] | select(.class == $c) | .documents' "$work/$1.json"
}

# graph_layers <dir>: the distinct layer numbers of check-layers.sh --graph
graph_layers() {
  (cd "$1" && scripts/check-layers.sh --graph) \
    | awk 'NF == 6 && $4 == "->" { print $2; print $5 }' | sort -u | wc -l | tr -d ' '
}

# --- fixture packs ---
for manifest in "$root"/fixtures/manifests/*.yaml; do
  name=$(basename "$manifest" .yaml)
  dir=$root/fixtures/packs/$name
  check "$name" "$dir" "$manifest"
  own=$(ess specify compile --path "$dir/blocks" --format json | jq '.domains | length')
  row "$name" blocks "$(of "$name" .counts.blocks)" "$own" "ess specify compile"
  own=$(find "$dir/blocks" -mindepth 1 -maxdepth 1 -type d -name '[0-9]*-*' | wc -l | tr -d ' ')
  row "$name" layers "$(of "$name" .counts.layers)" "$own" "numbered directories"
  own=$(grep -c '^  - id: ' "$dir/kpis/catalogue.yaml")
  row "$name" kpis "$(of "$name" .counts.kpis)" "$own" "catalogue rows"
  for class in $(jq -r '.documents[].class' "$work/$name.json"); do
    pattern=$(yq -r ".documents[] | select(.class == \"$class\") | .paths[0]" "$manifest")
    own=$(cd "$dir" && ls -1 $pattern | wc -l | tr -d ' ')
    row "$name" "documents:$class" "$(documents "$name" "$class")" "$own" "ls $pattern"
  done
done

# --- orgspec ---
# specapp builds nothing of a pack: the tool its manifest runs is built first
(cd "$org" && cargo build --quiet --locked -p orgspec)
check orgspec "$org" orgspec
own=$(cd "$org" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row orgspec blocks "$(of orgspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row orgspec layers "$(of orgspec .counts.layers)" "$(graph_layers "$org")" "scripts/check-layers.sh --graph"
own=$(cd "$org" && scripts/check-kpis.sh | sed -n 's/^check-kpis: \([0-9]*\) KPIs ok$/\1/p')
row orgspec kpis "$(of orgspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
own=$(cd "$org" && { target/debug/orgspec check --json organizations/*.yaml || true; } | jq length)
row orgspec documents:organization "$(documents orgspec organization)" "$own" "orgspec check --json"
own=$(cd "$org" && { target/debug/orgspec type check --json types/*.yaml || true; } | jq length)
row orgspec documents:type "$(documents orgspec type)" "$own" "orgspec type check --json"

# --- agenticspec ---
check agenticspec "$agentic" agenticspec
own=$(cd "$agentic" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row agenticspec blocks "$(of agenticspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(graph_layers "$agentic")" "scripts/check-layers.sh --graph"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(jq '.rows | length' "$agentic/website/src/generated/blocks.json")" "generated blocks.json rows"
own=$(cd "$agentic" && scripts/check-kpis.sh | sed -n 's/^check-kpis: \([0-9]*\) KPIs ok$/\1/p')
row agenticspec kpis "$(of agenticspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
own=$(find "$agentic/docs/architectures" -maxdepth 1 -name '*.md' ! -name index.md | wc -l | tr -d ' ')
row agenticspec documents:architecture "$(documents agenticspec architecture)" "$own" "generated docs/architectures pages"

# --- wave 2: pages ---
# Every drawn document of every pack is a page; the tools run under a trust file of this run
# only ($work/state), never the user's.
XDG_STATE_HOME=$work/state
export XDG_STATE_HOME

# page_data <html>: the JSON of a page's data element
page_data() {
  sed -n 's|.*<script type="application/json" id="specapp-data">\(.*\)</script>.*|\1|p' "$1"
}

# pages <name> <dir> <out> <name|manifest-file>: trust the pack's tool when it names one,
# render it, print the number of pages (index.html not counted)
pages() {
  case "$4" in
    */*) set -- "$1" "$2" "$3" --manifest "$4" ;;
    *) set -- "$1" "$2" "$3" "$4" ;;
  esac
  name=$1 dir=$2 out=$3
  shift 3
  "$specapp" trust "$@" --pack "$dir" > /dev/null 2>&1 || true
  "$specapp" render "$@" --pack "$dir" --out "$out" > "$out.log" || { echo "acceptance: render $name failed" >&2; exit 1; }
  find "$out" -name '*.html' ! -name index.html | wc -l | tr -d ' '
}

for manifest in "$root"/fixtures/manifests/*.yaml; do
  name=$(basename "$manifest" .yaml)
  n=$(pages "$name" "$root/fixtures/packs/$name" "$work/pages-$name" "$manifest")
  own=$(jq '[.documents[] | select(.shape == "parts-links/1") | .documents] | add' "$work/$name.json")
  row "$name" pages "$n" "$own" "drawn documents of specapp pack check"
done

n=$(pages orgspec "$org" "$work/pages-orgspec" orgspec)
own=$(cd "$org" && { target/debug/orgspec check --json organizations/*.yaml || true; } | jq length)
row orgspec pages:organization "$n" "$own" "orgspec check --json organizations/*.yaml"
n=$(pages agenticspec "$agentic" "$work/pages-agenticspec" agenticspec)
own=$(find "$agentic/architectures" -maxdepth 1 -name '*.yaml' | wc -l | tr -d ' ')
row agenticspec pages:architecture "$n" "$own" "architectures/*.yaml"

# rules <pages> <file...>: per design, the rule ids of specapp's page against orgspec's own
# check, and every finding about a part or link drawn in a scene
rules() {
  out=$1
  shift
  for f in "$@"; do
    stem=$(basename "$f" .yaml)
    page=$out/organization/$stem.html
    [ -f "$page" ] || { echo "acceptance: no page for $f" >&2; echo "$f - - 0 1 0 0"; continue; }
    page_data "$page" > "$work/page-data.tmp"
    mine=$(jq -r '[.findings[].rule] | unique | join(" ")' "$work/page-data.tmp")
    theirs=$(cd "$org" && { target/debug/orgspec check --json "$f" || true; } | jq -r '[.[0].violations[].rule, (.[0].refusals[]?.refusal)] | unique | join(" ")')
    subjects=$(jq '[.findings[] | select(.subject != "document")] | length' "$work/page-data.tmp")
    drawn=$(jq '[.findings[] | select(.subject != "document") | select(.drawn_in | length > 0)] | length' "$work/page-data.tmp")
    if [ "$mine" = "$theirs" ]; then same=1; else same=0; echo "acceptance: $f: specapp [$mine], orgspec [$theirs]" >&2; fi
    echo "$f $same 1 $drawn $subjects"
  done
}

# broken designs: a manifest of this run, made from packs/orgspec.yaml, whose organizations are
# organizations/broken/*.yaml
sed 's#organizations/\*\.yaml#organizations/broken/*.yaml#' "$root/packs/orgspec.yaml" > "$work/orgspec-broken.yaml"
grep -q 'organizations/broken/\*\.yaml' "$work/orgspec-broken.yaml" || { echo "acceptance: cannot point a manifest at organizations/broken" >&2; exit 2; }
n=$(pages orgspec-broken "$org" "$work/pages-orgspec-broken" "$work/orgspec-broken.yaml")
own=$(find "$org/organizations/broken" -maxdepth 1 -name '*.yaml' | wc -l | tr -d ' ')
row orgspec pages:broken "$n" "$own" "organizations/broken/*.yaml"
(cd "$org" && rules "$work/pages-orgspec-broken" organizations/broken/*.yaml) > "$work/broken.rows"
row orgspec broken:rules-matched "$(awk '{ s += $2 } END { print s + 0 }' "$work/broken.rows")" "$(awk '{ s += $3 } END { print s + 0 }' "$work/broken.rows")" "rule ids of each page = orgspec check --json"
row orgspec broken:findings-drawn "$(awk '{ s += $4 } END { print s + 0 }' "$work/broken.rows")" "$(awk '{ s += $5 } END { print s + 0 }' "$work/broken.rows")" "findings on a part or link marked in a scene"
(cd "$org" && rules "$work/pages-orgspec" organizations/*.yaml) > "$work/examples.rows"
row orgspec examples:rules-matched "$(awk '{ s += $2 } END { print s + 0 }' "$work/examples.rows")" "$(awk '{ s += $3 } END { print s + 0 }' "$work/examples.rows")" "rule ids of each page = orgspec check --json"
row orgspec examples:findings-drawn "$(awk '{ s += $4 } END { print s + 0 }' "$work/examples.rows")" "$(awk '{ s += $5 } END { print s + 0 }' "$work/examples.rows")" "findings on a part or link marked in a scene"

# --- the table ---
printf '%-12s %-24s %8s %8s  %-5s %s\n' pack count specapp own equal source
while IFS="$(printf '\t')" read -r pack count mine own source; do
  if [ -n "$own" ] && [ "$mine" = "$own" ]; then equal=yes; else equal=NO; status=1; fi
  printf '%-12s %-24s %8s %8s  %-5s %s\n' "$pack" "$count" "$mine" "$own" "$equal" "$source"
done < "$rows"
for name in $(cd "$work" && ls -1 *.json | sed 's/\.json$//'); do
  printf 'warnings of %s: %s\n' "$name" "$(jq '.warnings | length' "$work/$name.json")"
done

echo
set --
for manifest in "$root"/fixtures/manifests/*.yaml; do
  set -- "$@" --pack "$root/fixtures/packs/$(basename "$manifest" .yaml)" --manifest "$manifest"
done
"$root/scripts/check-vocabulary.sh" "$@" \
  --pack "$org" --name orgspec --pack "$agentic" --name agenticspec || status=1

if [ "$status" = 0 ]; then
  echo "acceptance: every count equal, no word of a pack in the code, no pack holds a manifest of specapp"
else
  echo "acceptance: FAILED"
fi
exit "$status"
