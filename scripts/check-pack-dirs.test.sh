#!/bin/sh
# Tests of scripts/check-pack-dirs.sh: a pack without anything of metaspec passes; a manifest of
# metaspec in a pack fails; a mention of metaspec fails only with --strict; the fixture packs pass.

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
sut=$here/check-pack-dirs.sh
work=$(mktemp -d "${TMPDIR:-/tmp}/check-pack-dirs-test.XXXXXX")
trap 'rm -rf "$work"' EXIT INT TERM

fail() { echo "check-pack-dirs.test: FAIL: $1" >&2; exit 1; }

# expect <exit> <label> [args...] : run and compare the exit code; output in $work/out
expect() {
  want=$1; label=$2; shift 2
  got=0
  "$sut" "$@" > "$work/out" 2>&1 || got=$?
  [ "$got" -eq "$want" ] || { cat "$work/out" >&2; fail "$label: exit $got, wanted $want"; }
}

# pack <name> : a pack with blocks and a document, nothing of metaspec
pack() {
  d=$work/$1
  mkdir -p "$d/blocks" "$d/docs"
  printf 'domain: sample.block\n' > "$d/blocks/block.yaml"
  printf 'name: one\n' > "$d/docs/one.yaml"
  echo "$d"
}

d=$(pack clean)
expect 0 "clean pack" "$d"
expect 0 "clean pack, strict" --strict "$d"

d=$(pack manifest)
printf 'format: x\n' > "$d/metaspec.yaml"
expect 1 "a manifest at the root" "$d"
grep -q 'a manifest of metaspec in a pack: .*/metaspec.yaml$' "$work/out" || fail "manifest: not named"

d=$(pack nested)
printf 'format: x\n' > "$d/docs/MetaSpec.yml"
expect 1 "a manifest deeper, any case" "$d"

d=$(pack ignored)
mkdir -p "$d/target/x" "$d/.git"
printf 'format: x\n' > "$d/target/x/metaspec.yaml"
printf 'format: x\n' > "$d/.git/metaspec.json"
expect 0 "target and .git are skipped" "$d"

d=$(pack mention)
printf '# drawn by metaspec\n' > "$d/docs/notes.md"
expect 0 "a mention without --strict" "$d"
expect 1 "a mention with --strict" --strict "$d"
grep -q 'names metaspec: .*/docs/notes.md$' "$work/out" || fail "mention: not named"

expect 2 "no such directory" "$work/none"

expect 0 "the fixture packs, strictly"
grep -q 'no file of metaspec' "$work/out" || fail "fixture packs: no summary line"

echo "check-pack-dirs.test: 9 cases passed"
