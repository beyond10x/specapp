#!/bin/sh
# acceptance.sh: the acceptance of wave 1 ("Contract") on both fixture packs and both real packs.
#
#   scripts/acceptance.sh <orgspec-dir> <agenticspec-dir> <agenticspec-manifest>
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
# Then runs scripts/check-vocabulary.sh with all four packs.
#
# Not part of `task check` and not run in CI: it needs local checkouts of the real packs.
# The pack scripts write into their pack's target/ directory. Needs ess, jq, yq (for the
# packs' KPI scripts) and cargo.
#
# Exit 0: every count is equal and no word of a pack is in the code. Exit 1: a count differs
# or a word was found. Exit 2: bad usage or a tool failed.

set -eu

[ $# -eq 3 ] || { echo "usage: scripts/acceptance.sh <orgspec-dir> <agenticspec-dir> <agenticspec-manifest>" >&2; exit 2; }
org=$(CDPATH= cd -- "$1" && pwd)
agentic=$(CDPATH= cd -- "$2" && pwd)
agentic_manifest=$(CDPATH= cd -- "$(dirname -- "$3")" && pwd)/$(basename -- "$3")

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$root/target/acceptance
rm -rf "$work"
mkdir -p "$work"

(cd "$root" && cargo build --quiet --locked -p specapp)
specapp=$root/target/debug/specapp

rows=$work/rows
: > "$rows"
# row <pack> <count> <specapp> <own> <source>
row() {
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" "$5" >> "$rows"
}

# check <name> <dir> [<manifest>]: specapp's report of a pack, in $work/<name>.json
check() {
  if [ $# -eq 3 ]; then
    "$specapp" pack check "$2" --manifest "$3" --json > "$work/$1.json" || true
  else
    "$specapp" pack check "$2" --json > "$work/$1.json" || true
  fi
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
for dir in "$root"/fixtures/packs/*/; do
  dir=${dir%/}
  name=$(basename "$dir")
  check "$name" "$dir"
  own=$(ess specify compile --path "$dir/blocks" --format json | jq '.domains | length')
  row "$name" blocks "$(of "$name" .counts.blocks)" "$own" "ess specify compile"
  own=$(find "$dir/blocks" -mindepth 1 -maxdepth 1 -type d -name '[0-9]*-*' | wc -l | tr -d ' ')
  row "$name" layers "$(of "$name" .counts.layers)" "$own" "numbered directories"
  own=$(grep -c '^  - id: ' "$dir/kpis/catalogue.yaml")
  row "$name" kpis "$(of "$name" .counts.kpis)" "$own" "catalogue rows"
  for class in $(jq -r '.documents[].class' "$work/$name.json"); do
    pattern=$(yq -r ".documents[] | select(.class == \"$class\") | .paths[0]" "$dir/specapp.yaml")
    own=$(cd "$dir" && ls -1 $pattern | wc -l | tr -d ' ')
    row "$name" "documents:$class" "$(documents "$name" "$class")" "$own" "ls $pattern"
  done
done

# --- orgspec ---
check orgspec "$org"
own=$(cd "$org" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row orgspec blocks "$(of orgspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row orgspec layers "$(of orgspec .counts.layers)" "$(graph_layers "$org")" "scripts/check-layers.sh --graph"
own=$(cd "$org" && scripts/check-kpis.sh | sed -n 's/^check-kpis: \([0-9]*\) KPIs ok$/\1/p')
row orgspec kpis "$(of orgspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
(cd "$org" && cargo build --quiet --locked -p orgspec)
own=$(cd "$org" && { target/debug/orgspec check --json organizations/*.yaml || true; } | jq length)
row orgspec documents:organization "$(documents orgspec organization)" "$own" "orgspec check --json"
own=$(cd "$org" && { target/debug/orgspec type check --json types/*.yaml || true; } | jq length)
row orgspec documents:type "$(documents orgspec type)" "$own" "orgspec type check --json"

# --- agenticspec ---
check agenticspec "$agentic" "$agentic_manifest"
own=$(cd "$agentic" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row agenticspec blocks "$(of agenticspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(graph_layers "$agentic")" "scripts/check-layers.sh --graph"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(jq '.rows | length' "$agentic/website/src/generated/blocks.json")" "generated blocks.json rows"
own=$(cd "$agentic" && scripts/check-kpis.sh | sed -n 's/^check-kpis: \([0-9]*\) KPIs ok$/\1/p')
row agenticspec kpis "$(of agenticspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
own=$(find "$agentic/docs/architectures" -maxdepth 1 -name '*.md' ! -name index.md | wc -l | tr -d ' ')
row agenticspec documents:architecture "$(documents agenticspec architecture)" "$own" "generated docs/architectures pages"

# --- the table ---
status=0
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
for dir in "$root"/fixtures/packs/*/; do
  set -- "$@" --pack "${dir%/}"
done
"$root/scripts/check-vocabulary.sh" "$@" \
  --pack "$org" --pack "$agentic" --manifest "$agentic_manifest" || status=1

if [ "$status" = 0 ]; then
  echo "acceptance: every count equal, no word of a pack in the code"
else
  echo "acceptance: FAILED"
fi
exit "$status"
