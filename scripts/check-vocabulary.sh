#!/bin/sh
# check-vocabulary.sh: fail when a word of a pack appears in specapp's code.
#
#   scripts/check-vocabulary.sh [--pack DIR (--manifest FILE | --name NAME)]...
#
# The words come from each pack's own files at check time, through `specapp pack words`: its
# block names, layer names, link names, KPI ids and rule ids. A pack is its directory and its
# manifest: a file (--manifest) or the name of a pack specapp knows (--name, its manifest is
# packs/<NAME>.yaml). Without --pack the packs are the fixture packs: fixtures/packs/<name>
# with fixtures/manifests/<name>.yaml. The acceptance script adds the real packs.
#
# Searched: every file under crates/ and under web/*/src (when they exist). Not searched: the
# manifests under packs/ and fixtures/manifests/, which are where a pack's words belong.
#
#   - a word with an underscore, a hyphen or a digit (reports_to, a KPI id, a rule id) is found
#     as a whole word anywhere: in code, in a string, in a comment;
#   - a word of letters only (a block name such as a common noun) is found as a whole string
#     literal, "word", which is how a table of block names in the code is written.
#
# Exit 0: no word found. Exit 1: at least one, listed as `<file>:<line>: <kind> <word>`.
# Exit 2: bad usage, or a pack that specapp refuses.
#
# Needs the specapp binary: $SPECAPP, or target/debug/specapp.

set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
specapp=${SPECAPP:-$root/target/debug/specapp}
[ -x "$specapp" ] || { echo "check-vocabulary: no specapp binary at $specapp (cargo build -p specapp)" >&2; exit 2; }

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
    "$specapp" pack words --manifest "$value" --pack "$dir" > "$work/one" || { echo "check-vocabulary: specapp refused the pack $dir" >&2; exit 2; }
  else
    "$specapp" pack words "$value" --pack "$dir" > "$work/one" || { echo "check-vocabulary: specapp refused the pack $dir" >&2; exit 2; }
  fi
  cat "$work/one" >> "$work/words"
  npacks=$((npacks + 1))
done < "$work/packs"

sort -u "$work/words" -o "$work/words"
# kind word -> the patterns of each form
awk '$2 ~ /^[A-Za-z]+$/ { print "\"" $2 "\"" }' "$work/words" | sort -u > "$work/literal"
awk '$2 !~ /^[A-Za-z]+$/ { print $2 }' "$work/words" | sort -u > "$work/whole"
# word -> kinds, for the report
awk '{ k[$2] = ($2 in k) ? k[$2] "," $1 : $1 } END { for (w in k) print w, k[w] }' "$work/words" > "$work/kinds"

set -- "$root/crates"
for src in "$root"/web/*/src; do
  [ -d "$src" ] && set -- "$@" "$src"
done

: > "$work/found"
if [ -s "$work/whole" ]; then
  grep -r -n -o -w -F -f "$work/whole" "$@" >> "$work/found" || true
fi
if [ -s "$work/literal" ]; then
  grep -r -n -o -F -f "$work/literal" "$@" | sed 's/:"\([^"]*\)"$/:\1/' >> "$work/found" || true
fi

nwords=$(wc -l < "$work/words" | tr -d ' ')
if [ -s "$work/found" ]; then
  sed "s|^$root/||" "$work/found" | sort -u | awk -F: '
    NR == FNR { split($0, p, " "); kinds[p[1]] = p[2]; next }
    { word = $NF; printf "%s:%s: %s %s\n", $1, $2, kinds[word], word; n++ }
    END { printf "check-vocabulary: %d word(s) of a pack in the code\n", n }
  ' "$work/kinds" -
  exit 1
fi
echo "check-vocabulary: $nwords word(s) of $npacks pack(s), none in the code"
