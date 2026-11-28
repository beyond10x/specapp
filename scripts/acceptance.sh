#!/bin/sh
# acceptance.sh: the acceptance of waves 1 ("Contract"), 2 ("Graph") and 3 ("Docs") on both fixture
# packs and the three real packs.
#
#   scripts/acceptance.sh <orgspec-dir> <agenticspec-dir> <cloudspec-dir>
#
# metaspec knows the packs; a pack knows nothing of metaspec. The real packs are read with their
# manifests under packs/, by name (`metaspec pack check orgspec --pack <dir>`); the fixture packs
# with fixtures/manifests/<name>.yaml. First scripts/check-pack-dirs.sh: no pack directory given
# here holds a file named like a manifest of metaspec (the fixture packs: nothing of metaspec at
# all).
#
# Runs `metaspec pack check --json` on each pack and compares its counts with what the pack's
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
#                  website/src/generated/blocks.json; KPIs: scripts/check-kpis.sh (the count
#                  of the catalogue at the manifest's kpis.path);
#                  architectures: the pages its generator wrote, docs/architectures/*.md
#                  without index.md.
#   cloudspec      level: L3 (blocks, documents and the verb check); blocks: the domains of
#                  blocks/system.yaml and those `ess specify compile` prints; layers: the
#                  numbered directories under blocks/; architectures and broken architectures:
#                  the files `cloudspec check --json` reads as an architecture (an entry with
#                  `architecture`) among architectures/*.yaml and architectures/broken/*.yaml,
#                  which leaves out its bindings and contracts.
#
# Wave 2: `metaspec render` draws every parts-links/1 document of every pack, with the tools
# trusted in a state directory of this run (target/acceptance/state): one page per document
# (fixture packs: the drawn documents of pack check; orgspec: `orgspec check --json`;
# agenticspec: architectures/*.yaml). For every broken design of orgspec
# (organizations/broken/*.yaml, through a manifest of this run made from packs/orgspec.yaml)
# and every example, the rule ids in the page data equal those `orgspec check --json` reports,
# and every finding about a part or link is marked in a scene. The findings come from
# `orgspec check --json` itself, mapped by the adapter in packs/orgspec.yaml. The same for
# agenticspec: every architecture, and the invented cases of
# fixtures/answers/agenticspec/architecture (copied into the pack's target/acceptance-cases,
# through a manifest of this run made from packs/agenticspec.yaml), against
# `agenticspec sim check --json`, mapped by the adapter in packs/agenticspec.yaml; its tool is
# built first. The same for cloudspec: every architecture and every broken architecture (the
# classes architecture and broken-architecture of packs/cloudspec.yaml), against
# `cloudspec check --json` (its `violations` and `warnings`), and every finding whose subject
# names a part or a link of the document (a key of `parts`, or `<from>-><to>:<carries>` of
# `links`, read with yq) is drawn in a scene, and each graph has the parts and links cloudspec
# counts. The page of architectures/journeys.yaml shows the warnings of its check, none gated,
# and stays valid. Every part of architectures/groups.yaml, node-pools.yaml and l4-media.yaml
# whose block word is a key of `blocks.aliases` in packs/cloudspec.yaml has the aliased block in
# the graph of its page, and every part of an architecture whose word is neither an alias nor a
# block of `metaspec pack check --json` keeps its word. Pages are under
# target/acceptance/pages-<pack>/.
#
# Wave 3: `metaspec docs build` writes a site of each pack alone under target/acceptance/docs-<pack>;
# its block pages, KPI rows and document pages equal the counts of `metaspec pack check`, and the
# one site of both fixture packs equals the committed one (--check). The one site of the three real
# packs is written to target/acceptance/docs-site, and the site shell of web/site builds it
# (target/acceptance/site; needs `task web:install web:build`): per pack its HTML pages equal its
# pages, and the site has its page /. The parity of the generated agenticspec site with its own
# pages is printed as a summary (scripts/parity.sh).
#
# Then the rows of every story of the quality initiative: each scripts/acceptance.d/*.sh, in name
# order, sourced after the rows above; reports land under target/acceptance/quality/.
#
# Then runs scripts/check-vocabulary.sh with all five packs.
#
# Not part of `task check` and not run in CI: it needs local checkouts of the real packs.
# The pack scripts write into their pack's target/ directory. Needs ess, jq, yq (for the
# packs' KPI scripts) and cargo.
#
# Exit 0: every count is equal and no word of a pack is in the code. Exit 1: a count differs,
# a word was found or a pack directory holds a manifest of metaspec. Exit 2: bad usage or a tool
# failed.

set -eu

[ $# -eq 3 ] || { echo "usage: scripts/acceptance.sh <orgspec-dir> <agenticspec-dir> <cloudspec-dir>" >&2; exit 2; }
org=$(CDPATH= cd -- "$1" && pwd)
agentic=$(CDPATH= cd -- "$2" && pwd)
cloud=$(CDPATH= cd -- "$3" && pwd)

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$root/target/acceptance
rm -rf "$work"
mkdir -p "$work"

status=0
"$root/scripts/check-pack-dirs.sh" || status=1
"$root/scripts/check-pack-dirs.sh" "$org" "$agentic" "$cloud" || status=1

(cd "$root" && cargo build --quiet --locked -p metaspec)
metaspec=$root/target/debug/metaspec

rows=$work/rows
: > "$rows"
# row <pack> <count> <metaspec> <own> <source>
row() {
  printf '%s\t%s\t%s\t%s\t%s\n' "$1" "$2" "$3" "$4" "$5" >> "$rows"
}

# check <name> <dir> <name|manifest-file>: metaspec's report of a pack, in $work/<name>.json
check() {
  case "$3" in
    */*) "$metaspec" pack check --manifest "$3" --pack "$2" --json > "$work/$1.json" || true ;;
    *) "$metaspec" pack check "$3" --pack "$2" --json > "$work/$1.json" || true ;;
  esac
  if [ "$(jq -r .valid "$work/$1.json")" != true ]; then
    echo "acceptance: metaspec refused the pack $1:" >&2
    jq -r '.errors[] | "  \(.code) \(.subject): \(.message)"' "$work/$1.json" >&2
    exit 1
  fi
}

# of <name> <jq path>: one value of metaspec's report
of() {
  jq -r "$2" "$work/$1.json"
}

# documents <name> <class>: metaspec's count of the documents of one class
documents() {
  jq -r --arg c "$2" '.documents[] | select(.class == $c) | .documents' "$work/$1.json"
}

# graph_layers <dir>: the distinct layer numbers of check-layers.sh --graph
graph_layers() {
  (cd "$1" && scripts/check-layers.sh --graph) \
    | awk 'NF == 6 && $4 == "->" { print $2; print $5 }' | sort -u | wc -l | tr -d ' '
}

# kpis_ok <dir> <name>: the KPI count scripts/check-kpis.sh of a pack prints for the catalogue its
# manifest packs/<name>.yaml names at kpis.path. A pack that reads several catalogues prints
# "check-kpis: N KPIs ok in M catalogue(s): <path> <n>, ..."; one that reads one prints
# "check-kpis: N KPIs ok".
kpis_ok() {
  path=$(yq -r .kpis.path "$root/packs/$2.yaml")
  line=$(cd "$1" && scripts/check-kpis.sh | sed -n '/^check-kpis: /p')
  case "$line" in
    *' catalogue(s): '*)
      printf '%s\n' "${line#*catalogue(s): }" | tr ',' '\n' | awk -v p="$path" '$1 == p { print $2 }' ;;
    *) printf '%s\n' "$line" | sed -n 's/^check-kpis: \([0-9]*\) KPIs ok$/\1/p' ;;
  esac
}

# --- fixture packs ---
for manifest in "$root"/fixtures/manifests/*.yaml; do
  name=$(basename "$manifest" .yaml)
  dir=$root/fixtures/packs/$name
  check "$name" "$dir" "$manifest"
  own=$(cd "$dir/blocks" && ess specify compile --path . --format json | jq '.domains | length')
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
# metaspec builds nothing of a pack: the tool its manifest runs is built first
(cd "$org" && cargo build --quiet --locked -p orgspec)
check orgspec "$org" orgspec
own=$(cd "$org" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row orgspec blocks "$(of orgspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row orgspec layers "$(of orgspec .counts.layers)" "$(graph_layers "$org")" "scripts/check-layers.sh --graph"
own=$(kpis_ok "$org" orgspec)
row orgspec kpis "$(of orgspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
own=$(cd "$org" && { target/debug/orgspec check --json organizations/*.yaml || true; } | jq length)
row orgspec documents:organization "$(documents orgspec organization)" "$own" "orgspec check --json"
own=$(cd "$org" && { target/debug/orgspec type check --json types/*.yaml || true; } | jq length)
row orgspec documents:type "$(documents orgspec type)" "$own" "orgspec type check --json"

# --- agenticspec ---
(cd "$agentic" && cargo build --quiet --locked -p agenticspec)
check agenticspec "$agentic" agenticspec
own=$(cd "$agentic" && scripts/check-layers.sh | sed -n 's/^layers: \([0-9]*\) domains.*/\1/p')
row agenticspec blocks "$(of agenticspec .counts.blocks)" "$own" "scripts/check-layers.sh"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(graph_layers "$agentic")" "scripts/check-layers.sh --graph"
row agenticspec layers "$(of agenticspec .counts.layers)" "$(jq '.rows | length' "$agentic/website/src/generated/blocks.json")" "generated blocks.json rows"
own=$(kpis_ok "$agentic" agenticspec)
row agenticspec kpis "$(of agenticspec .counts.kpis)" "$own" "scripts/check-kpis.sh"
own=$(find "$agentic/docs/architectures" -maxdepth 1 -name '*.md' ! -name index.md | wc -l | tr -d ' ')
row agenticspec documents:architecture "$(documents agenticspec architecture)" "$own" "generated docs/architectures pages"

# --- cloudspec ---
(cd "$cloud" && cargo build --quiet --locked --bin cloudspec)
check cloudspec "$cloud" cloudspec
row cloudspec level "$(of cloudspec .level)" L3 "blocks, documents and the verb check"
own=$(yq '.domains | length' "$cloud/blocks/system.yaml")
row cloudspec blocks "$(of cloudspec .counts.blocks)" "$own" "domains of blocks/system.yaml"
own=$(cd "$cloud/blocks" && ess specify compile --path . --format json | jq '.domains | length')
row cloudspec blocks "$(of cloudspec .counts.blocks)" "$own" "ess specify compile"
own=$(find "$cloud/blocks" -mindepth 1 -maxdepth 1 -type d -name '[0-9]*-*' | wc -l | tr -d ' ')
row cloudspec layers "$(of cloudspec .counts.layers)" "$own" "numbered directories"
# cloud_architectures <glob>: the files among <glob> that `cloudspec check --json` reads as an
# architecture, one per line, run in the pack's directory
cloud_architectures() {
  (cd "$cloud" && { target/debug/cloudspec check --json $1 || true; } | jq -r '.[] | select(has("architecture")) | .file')
}
cloud_architectures 'architectures/*.yaml' > "$work/cloud-architectures"
cloud_architectures 'architectures/broken/*.yaml' > "$work/cloud-broken"
row cloudspec documents:architecture "$(documents cloudspec architecture)" "$(wc -l < "$work/cloud-architectures" | tr -d ' ')" "cloudspec check --json architectures/*.yaml"
row cloudspec documents:broken-architecture "$(documents cloudspec broken-architecture)" "$(wc -l < "$work/cloud-broken" | tr -d ' ')" "cloudspec check --json architectures/broken/*.yaml"

# --- wave 2: pages ---
# Every drawn document of every pack is a page; the tools run under a trust file of this run
# only ($work/state), never the user's.
XDG_STATE_HOME=$work/state
export XDG_STATE_HOME

# page_data <html>: the JSON of a page's data element
page_data() {
  sed -n 's|.*<script type="application/json" id="metaspec-data">\(.*\)</script>.*|\1|p' "$1"
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
  "$metaspec" trust "$@" --pack "$dir" > /dev/null 2>&1 || true
  "$metaspec" render "$@" --pack "$dir" --out "$out" > "$out.log" || { echo "acceptance: render $name failed" >&2; exit 1; }
  find "$out" -name '*.html' ! -name index.html | wc -l | tr -d ' '
}

for manifest in "$root"/fixtures/manifests/*.yaml; do
  name=$(basename "$manifest" .yaml)
  n=$(pages "$name" "$root/fixtures/packs/$name" "$work/pages-$name" "$manifest")
  own=$(jq '[.documents[] | select(.shape == "parts-links/1") | .documents] | add' "$work/$name.json")
  row "$name" pages "$n" "$own" "drawn documents of metaspec pack check"
done

n=$(pages orgspec "$org" "$work/pages-orgspec" orgspec)
own=$(cd "$org" && { target/debug/orgspec check --json organizations/*.yaml || true; } | jq length)
row orgspec pages:organization "$n" "$own" "orgspec check --json organizations/*.yaml"
n=$(pages agenticspec "$agentic" "$work/pages-agenticspec" agenticspec)
own=$(find "$agentic/architectures" -maxdepth 1 -name '*.yaml' | wc -l | tr -d ' ')
row agenticspec pages:architecture "$n" "$own" "architectures/*.yaml"
pages cloudspec "$cloud" "$work/pages-cloudspec" cloudspec > /dev/null
n=$(find "$work/pages-cloudspec/architecture" -name '*.html' | wc -l | tr -d ' ')
row cloudspec pages:architecture "$n" "$(wc -l < "$work/cloud-architectures" | tr -d ' ')" "cloudspec check --json architectures/*.yaml"
n=$(find "$work/pages-cloudspec/broken-architecture" -name '*.html' | wc -l | tr -d ' ')
row cloudspec pages:broken "$n" "$(wc -l < "$work/cloud-broken" | tr -d ' ')" "cloudspec check --json architectures/broken/*.yaml"

# The rule ids a pack's own check reports for one document, run in the pack's directory.
theirs_orgspec() {
  { target/debug/orgspec check --json "$1" || true; } | jq -r '[.[0].violations[].rule, (.[0].refusals[]?.refusal)] | unique | join(" ")'
}
theirs_agenticspec() {
  { target/debug/agenticspec sim check --json "$1" || true; } | jq -r '[.violations[]?.rule] | unique | join(" ")'
}
theirs_cloudspec() {
  { target/debug/cloudspec check --json "$1" || true; } | jq -r '[(.[0].violations[]?, .[0].warnings[]?).rule] | unique | join(" ")'
}

# rules <pack> <pages> <class> <file...>: per document (run in the pack's directory), the rule
# ids of metaspec's page against the pack's own check (theirs_<pack>), and every finding about a
# part or link drawn in a scene
rules() {
  pack=$1 out=$2 class=$3
  shift 3
  for f in "$@"; do
    stem=$(basename "$f" .yaml)
    page=$out/$class/$stem.html
    [ -f "$page" ] || { echo "acceptance: no page for $f" >&2; echo "$f - - 0 1 0 0"; continue; }
    page_data "$page" > "$work/page-data.tmp"
    mine=$(jq -r '[.findings[].rule] | unique | join(" ")' "$work/page-data.tmp")
    theirs=$("theirs_$pack" "$f")
    subjects=$(jq '[.findings[] | select(.subject != "document")] | length' "$work/page-data.tmp")
    drawn=$(jq '[.findings[] | select(.subject != "document") | select(.drawn_in | length > 0)] | length' "$work/page-data.tmp")
    if [ "$mine" = "$theirs" ]; then same=1; else same=0; echo "acceptance: $f: metaspec [$mine], $pack [$theirs]" >&2; fi
    echo "$f $same 1 $drawn $subjects"
  done
}

# rule_rows <pack> <name> <rows> <source>: the two rows of a set of rules lines
rule_rows() {
  row "$1" "$2:rules-matched" "$(awk '{ s += $2 } END { print s + 0 }' "$3")" "$(awk '{ s += $3 } END { print s + 0 }' "$3")" "rule ids of each page = $4"
  row "$1" "$2:findings-drawn" "$(awk '{ s += $4 } END { print s + 0 }' "$3")" "$(awk '{ s += $5 } END { print s + 0 }' "$3")" "findings on a part or link marked in a scene"
}

# broken designs: a manifest of this run, made from packs/orgspec.yaml, whose organizations are
# organizations/broken/*.yaml
sed 's#organizations/\*\.yaml#organizations/broken/*.yaml#' "$root/packs/orgspec.yaml" > "$work/orgspec-broken.yaml"
grep -q 'organizations/broken/\*\.yaml' "$work/orgspec-broken.yaml" || { echo "acceptance: cannot point a manifest at organizations/broken" >&2; exit 2; }
n=$(pages orgspec-broken "$org" "$work/pages-orgspec-broken" "$work/orgspec-broken.yaml")
own=$(find "$org/organizations/broken" -maxdepth 1 -name '*.yaml' | wc -l | tr -d ' ')
row orgspec pages:broken "$n" "$own" "organizations/broken/*.yaml"
(cd "$org" && rules orgspec "$work/pages-orgspec-broken" organization organizations/broken/*.yaml) > "$work/broken.rows"
rule_rows orgspec broken "$work/broken.rows" "orgspec check --json"
(cd "$org" && rules orgspec "$work/pages-orgspec" organization organizations/*.yaml) > "$work/examples.rows"
rule_rows orgspec examples "$work/examples.rows" "orgspec check --json"

# agenticspec: every architecture, and the cases recorded under
# fixtures/answers/agenticspec/architecture (one it accepts, one it refuses), copied into the
# pack's target/ directory and drawn with a manifest of this run made from
# packs/agenticspec.yaml. The findings come from `agenticspec sim check --json` itself, mapped by
# the adapter in packs/agenticspec.yaml.
(cd "$agentic" && rules agenticspec "$work/pages-agenticspec" architecture architectures/*.yaml) > "$work/agentic-examples.rows"
rule_rows agenticspec examples "$work/agentic-examples.rows" "agenticspec sim check --json"
cases=$agentic/target/acceptance-cases
rm -rf "$cases"
mkdir -p "$cases"
cp "$root"/fixtures/answers/agenticspec/architecture/*.yaml "$cases/"
sed 's#architectures/\*\.yaml#target/acceptance-cases/*.yaml#' "$root/packs/agenticspec.yaml" > "$work/agenticspec-cases.yaml"
grep -q 'target/acceptance-cases/\*\.yaml' "$work/agenticspec-cases.yaml" || { echo "acceptance: cannot point a manifest at the recorded cases" >&2; exit 2; }
n=$(pages agenticspec-cases "$agentic" "$work/pages-agenticspec-cases" "$work/agenticspec-cases.yaml")
own=$(find "$cases" -maxdepth 1 -name '*.yaml' | wc -l | tr -d ' ')
row agenticspec pages:cases "$n" "$own" "fixtures/answers/agenticspec/architecture/*.yaml"
(cd "$agentic" && rules agenticspec "$work/pages-agenticspec-cases" architecture target/acceptance-cases/*.yaml) > "$work/agentic-cases.rows"
rule_rows agenticspec cases "$work/agentic-cases.rows" "agenticspec sim check --json"

# cloudspec: every architecture and every broken architecture, against `cloudspec check --json`,
# mapped by the adapter in packs/cloudspec.yaml.
# named <pages> <class> <file...>: per document (run in the pack's directory), whether the
# findings `cloudspec check --json` reports (`violations` and `warnings`) on a part or a link of
# the document itself (a key of `parts`, or `<from>-><to>:<carries>` of `links`) are, rule and
# subject, the findings the page draws in a scene; then whether the page's graph has as many parts and links as cloudspec counts
# (`parts`, `links` of its answer). Prints `<file> <named equal> 1 <counts equal> 1`.
named() {
  out=$1 class=$2
  shift 2
  for f in "$@"; do
    page=$out/$class/$(basename "$f" .yaml).html
    [ -f "$page" ] || { echo "$f 0 1 0 1"; continue; }
    yq -r '(.parts // {} | keys | .[]), (.links // [] | .[] | .from + "->" + .to + ":" + (.carries // ""))' "$f" > "$work/named.tmp"
    { target/debug/cloudspec check --json "$f" || true; } > "$work/answer.tmp"
    theirs=$(jq -r --rawfile named "$work/named.tmp" '($named | split("\n")) as $n
          | [(.[0].violations[]?, .[0].warnings[]?) | select(.subject as $s | any($n[]; . == $s)) | .rule + " " + .subject]
          | sort | join(",")' "$work/answer.tmp")
    page_data "$page" > "$work/page-data.tmp"
    mine=$(jq -r '[.findings[] | select(.subject != "document") | select(.drawn_in | length > 0)
          | .rule + " " + (.subject | if startswith("link:") then .[5:] | sub(":[?]$"; ":") else sub("^part:"; "") end)] | sort | join(",")' "$work/page-data.tmp")
    if [ "$mine" = "$theirs" ]; then same=1; else same=0; echo "acceptance: $f: drawn [$mine], cloudspec on its parts and links [$theirs]" >&2; fi
    drawn=$(jq -r '"\(.graph.parts | length) \(.graph.links | length)"' "$work/page-data.tmp")
    counted=$(jq -r '.[0] | "\(.parts) \(.links)"' "$work/answer.tmp")
    if [ "$drawn" = "$counted" ]; then whole=1; else whole=0; echo "acceptance: $f: parts and links drawn [$drawn], cloudspec [$counted]" >&2; fi
    echo "$f $same 1 $whole 1"
  done
}
for class in architecture broken-architecture; do
  case "$class" in architecture) list=$work/cloud-architectures name=examples ;; *) list=$work/cloud-broken name=broken ;; esac
  (cd "$cloud" && rules cloudspec "$work/pages-cloudspec" "$class" $(cat "$list")) > "$work/cloud-$name.rows"
  rule_rows cloudspec "$name" "$work/cloud-$name.rows" "cloudspec check --json"
  (cd "$cloud" && named "$work/pages-cloudspec" "$class" $(cat "$list")) > "$work/cloud-$name.named"
  row cloudspec "$name:named-drawn" "$(awk '{ s += $2 } END { print s + 0 }' "$work/cloud-$name.named")" \
    "$(awk '{ s += $3 } END { print s + 0 }' "$work/cloud-$name.named")" "findings on a part or link of the document drawn, rule and subject"
  row cloudspec "$name:parts-links" "$(awk '{ s += $4 } END { print s + 0 }' "$work/cloud-$name.named")" \
    "$(awk '{ s += $5 } END { print s + 0 }' "$work/cloud-$name.named")" "parts and links of each graph = cloudspec check --json"
done
# The warnings of `cloudspec check --json` (entries with `gate: false`) on the page of
# architectures/journeys.yaml: as many findings not gated as warnings, as many gated as gated
# entries of its answer, and the page's verdict is cloudspec's. Every number is read at run time.
f=architectures/journeys.yaml
(cd "$cloud" && { target/debug/cloudspec check --json "$f" || true; }) > "$work/journeys.answer"
page=$work/pages-cloudspec/architecture/$(basename "$f" .yaml).html
if [ -f "$page" ]; then
  page_data "$page" > "$work/journeys.page"
  set -- "$(jq '[.findings[] | select(.gate | not)] | length' "$work/journeys.page")" \
    "$(jq '[.findings[] | select(.gate)] | length' "$work/journeys.page")" \
    "$(jq -r '.check.result.valid' "$work/journeys.page")"
else
  echo "acceptance: no page for $f" >&2
  set -- - - -
fi
row cloudspec journeys:warnings "$1" "$(jq '.[0].warnings | length' "$work/journeys.answer")" "findings not gated on the page of $f = warnings of cloudspec check --json"
row cloudspec journeys:gated "$2" "$(jq '[(.[0].violations[]?, .[0].warnings[]?) | select(.gate)] | length' "$work/journeys.answer")" "gated findings on the page of $f = gated entries of cloudspec check --json"
row cloudspec journeys:valid "$3" "$(jq -r '.[0].valid' "$work/journeys.answer")" "valid on the page of $f = cloudspec check --json"

# The aliases of packs/cloudspec.yaml (`blocks.aliases`), read at run time with the pack's block
# list (`.layers[].blocks` of `metaspec pack check --json`). alias_counts <class> <file...>: per
# document (run in the pack's directory), the parts whose block word is an alias, those of them
# whose block in the graph of the page is the aliased block, the parts whose word is neither an
# alias nor a block, and those of them whose block on the page is still the word. Prints
# `<file> <aliased> <aliased on the page> <kept> <kept on the page>`.
yq -o=json '.blocks.aliases // {}' "$root/packs/cloudspec.yaml" > "$work/cloud-aliases.data"
jq '[.layers[].blocks[]]' "$work/cloudspec.json" > "$work/cloud-blocks.data"
alias_counts() {
  class=$1
  shift
  for f in "$@"; do
    page=$work/pages-cloudspec/$class/$(basename "$f" .yaml).html
    yq -o=json '.' "$f" > "$work/alias-doc.tmp"
    if [ -f "$page" ]; then page_data "$page" | jq '.graph.parts' > "$work/alias-page.tmp"; else echo '[]' > "$work/alias-page.tmp"; fi
    jq -r --arg f "$f" --slurpfile aliases "$work/cloud-aliases.data" --slurpfile blocks "$work/cloud-blocks.data" \
      --slurpfile page "$work/alias-page.tmp" '
      $aliases[0] as $a | $blocks[0] as $b
      | ([$page[0][] | {(.id): .block}] | add // {}) as $on
      | [(.parts? // {}) | objects | to_entries[] | select(.value | type == "object")
         | select(.value.block | type == "string") | {id: .key, word: .value.block}] as $parts
      | [$parts[] | select($a[.word] != null)] as $aliased
      | [$parts[] | select($a[.word] == null and (.word as $w | $b | index([$w]) | not))] as $kept
      | "\($f) \($aliased | length) \([$aliased[] | select($on[.id] == $a[.word])] | length) \($kept | length) \([$kept[] | select($on[.id] == .word)] | length)"' \
      "$work/alias-doc.tmp"
  done
}
# alias_row <count> <rows> <source>: the sums of a set of alias_counts lines; with no part to
# count the row is not equal, so a manifest without aliases or a pack without such parts fails it
alias_row() {
  case "$1" in aliases:aliased) a=2 b=3 ;; *) a=4 b=5 ;; esac
  own=$(awk -v c="$a" '{ s += $c } END { print s + 0 }' "$2")
  mine=$(awk -v c="$b" '{ s += $c } END { print s + 0 }' "$2")
  [ "$own" -gt 0 ] || mine=-
  row cloudspec "$1" "$mine" "$own" "$3"
}
(cd "$cloud" && alias_counts architecture architectures/groups.yaml architectures/node-pools.yaml architectures/l4-media.yaml) > "$work/cloud-aliased.rows"
alias_row aliases:aliased "$work/cloud-aliased.rows" "parts of groups, node-pools, l4-media whose word is an alias: the aliased block on the page"
(cd "$cloud" && alias_counts architecture $(cat "$work/cloud-architectures") && alias_counts broken-architecture $(cat "$work/cloud-broken")) > "$work/cloud-kept.rows"
alias_row aliases:kept "$work/cloud-kept.rows" "parts whose word is no alias and no block of pack check: the word on the page"

# --- wave 3: docs sites ---
# docs_rows <name> <dir> <name|manifest-file>: metaspec docs build of the site of one pack into
# $work/docs-<name> (its tool trusted above under $work/state); the counts of its pages against
# the counts of metaspec pack check
docs_rows() {
  name=$1 dir=$2 out=$work/docs-$1
  case "$3" in
    */*) set -- --manifest "$3" ;;
    *) set -- "$3" ;;
  esac
  "$metaspec" docs build "$@" --pack "$dir" --out "$out" > "$out.log" || { echo "acceptance: docs build $name failed" >&2; cat "$out.log" >&2; exit 1; }
  part=$out/$(jq -r '.packs[0].pack' "$out/index.json")
  kinds=$(jq -r '.pages[].kind' "$part/site.json")
  row "$name" docs:block-pages "$(echo "$kinds" | grep -cx block || true)" "$(of "$name" .counts.blocks)" "pack check blocks"
  # a pack without a KPI catalogue has no KPI page
  if [ -f "$part/pages/kpis.json" ]; then
    kpi_rows=$(jq '.data.catalogue.kpis | length' "$part/pages/kpis.json")
  else
    kpi_rows=0
  fi
  row "$name" docs:kpi-rows "$kpi_rows" "$(of "$name" .counts.kpis)" "pack check kpis"
  row "$name" docs:document-pages "$(echo "$kinds" | grep -cxE 'document|record' || true)" "$(of "$name" .counts.documents)" "pack check documents"
}

set --
for manifest in "$root"/fixtures/manifests/*.yaml; do
  name=$(basename "$manifest" .yaml)
  dir=$root/fixtures/packs/$name
  docs_rows "$name" "$dir" "$manifest"
  set -- "$@" --pack "$manifest=$dir"
done
if "$metaspec" docs build "$@" --out "$root/fixtures/expected/site" --check > "$work/docs-check.log"; then
  row fixtures docs:committed-site 1 1 "metaspec docs build --check of both fixture packs"
else
  row fixtures docs:committed-site 0 1 "metaspec docs build --check of both fixture packs"
fi
docs_rows orgspec "$org" orgspec
docs_rows agenticspec "$agentic" agenticspec
docs_rows cloudspec "$cloud" cloudspec

# The site of the three real packs: metaspec docs build into $work/docs-site and the site shell of
# web/site over it, built into $work/site. Needs `task web:install` and `task web:build`. The job
# stays on this host (slot --here): a build node does not have the pages under target/.
"$metaspec" docs build --pack "orgspec=$org" --pack "agenticspec=$agentic" --pack "cloudspec=$cloud" --out "$work/docs-site" > "$work/docs-site.log" \
  || { echo "acceptance: docs build of the site of the three real packs failed" >&2; cat "$work/docs-site.log" >&2; exit 1; }
if [ -d "$root/web/node_modules" ] && [ -f "$root/web/docusaurus/dist/index.js" ]; then
  if (cd "$root/web/site" && METASPEC_SITE_OUT="$work/docs-site" slot --here --timeout 15m npx docusaurus build --out-dir "$work/site") > "$work/site.log" 2>&1; then
    row site docs:site-builds 1 1 "docusaurus build of web/site over the three real packs"
    # A page is <route>.html; <route>/index.html is the redirect docs-system writes beside it.
    # A pack's own page is <pack>.html beside its directory.
    for name in orgspec agenticspec cloudspec; do
      n=$(find "$work/site/$name" -name '*.html' ! -name index.html | wc -l | tr -d ' ')
      [ -f "$work/site/$name.html" ] && n=$((n + 1))
      row "$name" docs:site-pages "$n" "$(jq '.pages | length' "$work/docs-site/$name/site.json")" "built HTML pages"
    done
    n=0
    [ -f "$work/site/index.html" ] && n=1
    row site docs:site-index "$n" 1 "the page / of the site"
  else
    row site docs:site-builds 0 1 "docusaurus build of web/site ($work/site.log)"
  fi
else
  row site docs:site-builds - 1 "web/ not installed and built (task web:install web:build)"
fi

# agenticspec parity: scripts/parity.sh, its summary line
"$root/scripts/parity.sh" "$agentic" > "$work/parity.log" 2>&1 || true
grep -E '^(green|missing|expected-missing) ' "$work/parity.log" > "$work/parity.items" || true

# --- row files ---
# Every scripts/acceptance.d/*.sh, in name order, sourced here: one file per story of the quality
# initiative, named after it. A row file may use row, of, check, documents, $metaspec, $work,
# $root, $org, $agentic and $cloud, and writes its reports under $work/quality/.
for rows_file in "$root"/scripts/acceptance.d/*.sh; do
  [ -f "$rows_file" ] || continue
  . "$rows_file"
done

# --- the table ---
printf '%-12s %-24s %8s %8s  %-5s %s\n' pack count metaspec own equal source
while IFS="$(printf '\t')" read -r pack count mine own source; do
  if [ -n "$own" ] && [ "$mine" = "$own" ]; then equal=yes; else equal=NO; status=1; fi
  printf '%-12s %-24s %8s %8s  %-5s %s\n' "$pack" "$count" "$mine" "$own" "$equal" "$source"
done < "$rows"
for name in $(cd "$work" && ls -1 *.json | sed 's/\.json$//'); do
  printf 'warnings of %s: %s\n' "$name" "$(jq '.warnings | length' "$work/$name.json")"
done

echo
echo "parity of the generated agenticspec site:"
sed 's/^/  /' "$work/parity.items"
tail -n 1 "$work/parity.log" | sed 's/^/  /'

echo
set --
for manifest in "$root"/fixtures/manifests/*.yaml; do
  set -- "$@" --pack "$root/fixtures/packs/$(basename "$manifest" .yaml)" --manifest "$manifest"
done
"$root/scripts/check-vocabulary.sh" "$@" \
  --pack "$org" --name orgspec --pack "$agentic" --name agenticspec --pack "$cloud" --name cloudspec || status=1

if [ "$status" = 0 ]; then
  echo "acceptance: every count equal, no word of a pack in the code, no pack holds a manifest of metaspec"
else
  echo "acceptance: FAILED"
fi
exit "$status"
