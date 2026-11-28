#!/bin/sh
# Tests of scripts/check-vocabulary.sh on a tree of its own: a pack word that is also a word of
# one of metaspec's own formats (a property name, an enum or const value of
# schema/metaspec-*.schema.json) is not reported; a pack word that is not stays reported, in
# both forms (a string literal of letters, a whole word with a hyphen). The pack's words come
# from a stand-in for `metaspec pack words` ($METASPEC). Work files are under target/.

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$here/../target/check-vocabulary-test
rm -rf "$work"
mkdir -p "$work"
trap 'rm -rf "$work"' EXIT INT TERM

fail() { echo "check-vocabulary.test: FAIL: $1" >&2; exit 1; }

# tree <name>: a repository with the script under test, one schema of metaspec, a pack and a
# stand-in for metaspec that prints four words of that pack: two of them metaspec's own
tree() {
  d=$work/$1
  mkdir -p "$d/scripts" "$d/schema" "$d/crates/a/src" "$d/pack"
  cp "$here/check-vocabulary.sh" "$d/scripts/"
  printf '%s\n' '{"$schema": "https://json-schema.org/draft/2020-12/schema",' \
    ' "properties": {"format": {"const": "metaspec-a/1"}, "unit": {"type": "string"},' \
    '   "rows": {"type": "array", "items": {"properties": {"kind": {"enum": ["kpi", 3]}}}}}}' \
    > "$d/schema/metaspec-a-1.schema.json"
  printf '%s\n' '#!/bin/sh' 'printf "block unit\nblock kpi\nblock seawall\nrule wall-01\n"' > "$d/metaspec"
  chmod +x "$d/metaspec"
  echo "$d"
}

# expect <exit> <label> <dir>: run the script of the tree; output in $work/out
expect() {
  want=$1; label=$2; dir=$3
  got=0
  METASPEC=$dir/metaspec sh "$dir/scripts/check-vocabulary.sh" --pack "$dir/pack" --name a > "$work/out" 2>&1 || got=$?
  [ "$got" -eq "$want" ] || { cat "$work/out" >&2; fail "$label: exit $got, wanted $want"; }
}

d=$(tree own)
printf '%s\n' 'const UNIT: &str = "unit";' 'const KIND: &str = "kpi";' > "$d/crates/a/src/lib.rs"
expect 0 "metaspec's own words are not reported" "$d"
grep -qx "check-vocabulary: 4 word(s) of 1 pack(s), 2 of them metaspec's own, none in the code" "$work/out" \
  || { cat "$work/out" >&2; fail "metaspec's own words: the last line does not count them"; }

d=$(tree leak)
printf '%s\n' 'const UNIT: &str = "unit";' 'const WALL: &str = "seawall";' '// the rule wall-01' > "$d/crates/a/src/lib.rs"
expect 1 "a pack word that is no word of metaspec stays reported" "$d"
grep -qx 'crates/a/src/lib.rs:2: block seawall' "$work/out" || { cat "$work/out" >&2; fail "leak: the literal is not reported"; }
grep -qx 'crates/a/src/lib.rs:3: rule wall-01' "$work/out" || { cat "$work/out" >&2; fail "leak: the whole word is not reported"; }
if grep -q ' unit$' "$work/out"; then cat "$work/out" >&2; fail "leak: a word of metaspec is reported"; fi
grep -qx 'check-vocabulary: 2 word(s) of a pack in the code' "$work/out" || { cat "$work/out" >&2; fail "leak: wrong count"; }

d=$(tree broken)
printf '{ not json\n' > "$d/schema/metaspec-b-1.schema.json"
expect 2 "a schema that is no JSON is refused" "$d"

echo "check-vocabulary.test: ok"
