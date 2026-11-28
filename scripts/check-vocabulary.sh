#!/bin/sh
# check-vocabulary.sh: fail when a word of a pack appears in metaspec's code.
#
#   scripts/check-vocabulary.sh [--pack DIR (--manifest FILE | --name NAME)]...
#
# The words come from each pack's own files at check time, through `metaspec pack words`: its
# block names, layer names, link names, KPI ids and rule ids. A pack is its directory and its
# manifest: a file (--manifest) or the name of a pack metaspec knows (--name, its manifest is
# packs/<NAME>.yaml). Without --pack the packs are the fixture packs: fixtures/packs/<name>
# with fixtures/manifests/<name>.yaml. The acceptance script adds the real packs.
#
# Searched: every file under crates/ and, when they exist, web/src/ and web/*/src/ (the source
# of each web package; their tests, node_modules and dist/ are not searched). Not searched: the
# manifests under packs/ and fixtures/manifests/, which are where a pack's words belong.
#
#   - a word with an underscore, a hyphen or a digit (reports_to, a KPI id, a rule id) is found
#     as a whole word anywhere: in code, in a string, in a comment;
#   - a word of letters only (a block name such as a common noun) is found as a whole string
#     literal, "word", which is how a table of block names in the code is written.
#
# A pack word that is also a word of one of metaspec's own formats is metaspec's word, not a
# leak, and is not searched: every property name and every string enum or const value of the
# schemas schema/metaspec-*.schema.json (read with jq). So is a word of the format metaspec reads
# every pack's blocks in: a field name of a declaration of ESS's compiled model (`ess specify
# compile --format json`) of the fixture packs fixtures/packs/*/blocks, a key without a dot
# below a qualified name (a dotted key, such as a domain or an entity). The model's top-level
# keys, the qualified names and the values are not left out. The last line says how many pack
# words were left out that way.
#
# Exit 0: no word found. Exit 1: at least one, listed as `<file>:<line>: <kind> <word>`.
# Exit 2: bad usage, or a pack that metaspec refuses.
#
# Needs jq, ess (for the fixture packs' blocks) and the metaspec binary: $METASPEC, or
# target/debug/metaspec.

set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
metaspec=${METASPEC:-$root/target/debug/metaspec}
[ -x "$metaspec" ] || { echo "check-vocabulary: no metaspec binary at $metaspec (cargo build -p metaspec)" >&2; exit 2; }

work=$root/target/check-vocabulary
rm -rf "$work"
mkdir -p "$work"
# one line per pack: <dir> TAB <manifest|name> TAB <value>
: > "$work/packs"

dir=
while [ $# -gt 0 ]; do
  case "$1" in
    --pack)
      [ $# -ge 2 ] || { echo "check-vocabulary: --pack needs a directory" >&2; exit 2; }
      [ -z "$dir" ] || { echo "check-vocabulary: --pack $dir has no --manifest or --name" >&2; exit 2; }
      dir=$2
      shift 2 ;;
    --manifest|--name)
      [ $# -ge 2 ] || { echo "check-vocabulary: $1 needs a value" >&2; exit 2; }
      [ -n "$dir" ] || { echo "check-vocabulary: $1 comes after --pack" >&2; exit 2; }
      printf '%s\t%s\t%s\n' "$dir" "${1#--}" "$2" >> "$work/packs"
      dir=
      shift 2 ;;
    *) echo "check-vocabulary: unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -z "$dir" ] || { echo "check-vocabulary: --pack $dir has no --manifest or --name" >&2; exit 2; }

if [ ! -s "$work/packs" ]; then
  for manifest in "$root"/fixtures/manifests/*.yaml; do
    name=$(basename "$manifest" .yaml)
    printf '%s\tmanifest\t%s\n' "$root/fixtures/packs/$name" "$manifest" >> "$work/packs"
  done
fi

: > "$work/words"
npacks=0
while IFS="$(printf '\t')" read -r dir how value; do
  if [ "$how" = manifest ]; then
    "$metaspec" pack words --manifest "$value" --pack "$dir" > "$work/one" || { echo "check-vocabulary: metaspec refused the pack $dir" >&2; exit 2; }
  else
    "$metaspec" pack words "$value" --pack "$dir" > "$work/one" || { echo "check-vocabulary: metaspec refused the pack $dir" >&2; exit 2; }
  fi
  cat "$work/one" >> "$work/words"
  npacks=$((npacks + 1))
done < "$work/packs"

sort -u "$work/words" -o "$work/words"
nwords=$(wc -l < "$work/words" | tr -d ' ')

# metaspec's own words: the property names and the string enum and const values of its schemas
: > "$work/own"
for schema in "$root"/schema/metaspec-*.schema.json; do
  [ -f "$schema" ] || continue
  jq -r '[.. | objects
    | ((.properties? // {}) | if type == "object" then keys[] else empty end),
      ((.enum? // []) | if type == "array" then .[] | strings else empty end),
      (.const? | strings)] | .[]' "$schema" >> "$work/own" \
    || { echo "check-vocabulary: $schema cannot be read" >&2; exit 2; }
done
# and the words of the format metaspec reads every pack's blocks in: the field names of the
# declarations of ESS's compiled model (`ess specify compile --format json`) of the fixture
# packs, the keys without a dot below a qualified name (a dotted key, such as a domain or an
# entity). A dotted key is a name of a pack, and a key of the model's top level is the name of a
# collection, so neither is left out; no value is.
for blocks in "$root"/fixtures/packs/*/blocks; do
  [ -d "$blocks" ] || continue
  (cd "$blocks" && ess specify compile --path . --format json) > "$work/model.json" \
    || { echo "check-vocabulary: ess specify compile refused $blocks" >&2; exit 2; }
  jq -r '[paths | . as $p | range(1; length) as $i
      | select(any($p[:$i][]; type == "string" and contains(".")))
      | $p[$i] | strings | select(contains(".") | not)] | unique | .[]' "$work/model.json" >> "$work/own" \
    || { echo "check-vocabulary: the compiled model of $blocks is no JSON" >&2; exit 2; }
done
sort -u "$work/own" -o "$work/own"
awk 'NR == FNR { own[$0] = 1; next } !($2 in own)' "$work/own" "$work/words" > "$work/searched"
nown=$((nwords - $(wc -l < "$work/searched" | tr -d ' ')))
mv "$work/searched" "$work/words"
# kind word -> the patterns of each form
awk '$2 ~ /^[A-Za-z]+$/ { print "\"" $2 "\"" }' "$work/words" | sort -u > "$work/literal"
awk '$2 !~ /^[A-Za-z]+$/ { print $2 }' "$work/words" | sort -u > "$work/whole"
# word -> kinds, for the report
awk '{ k[$2] = ($2 in k) ? k[$2] "," $1 : $1 } END { for (w in k) print w, k[w] }' "$work/words" > "$work/kinds"

set -- "$root/crates"
for src in "$root"/web/src "$root"/web/*/src; do
  [ -d "$src" ] && set -- "$@" "$src"
done

: > "$work/found"
if [ -s "$work/whole" ]; then
  grep -r -n -o -w -F -f "$work/whole" "$@" >> "$work/found" || true
fi
if [ -s "$work/literal" ]; then
  grep -r -n -o -F -f "$work/literal" "$@" | sed 's/:"\([^"]*\)"$/:\1/' >> "$work/found" || true
fi

if [ -s "$work/found" ]; then
  sed "s|^$root/||" "$work/found" | sort -u | awk -F: '
    NR == FNR { split($0, p, " "); kinds[p[1]] = p[2]; next }
    { word = $NF; printf "%s:%s: %s %s\n", $1, $2, kinds[word], word; n++ }
    END { printf "check-vocabulary: %d word(s) of a pack in the code\n", n }
  ' "$work/kinds" -
  exit 1
fi
echo "check-vocabulary: $nwords word(s) of $npacks pack(s), $nown of them metaspec's own, none in the code"
