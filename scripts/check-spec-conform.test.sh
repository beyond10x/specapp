#!/bin/sh
# Tests of scripts/check-spec-conform.sh, its validation step, with a stub `ess` on PATH: the
# script is copied into a tree whose path holds the word "warnings", and the dispatcher's note,
# which names that path, is not a warning; a warning ess prints is one, also one that says "is
# the dispatcher". The stub refuses to synthesize, so a run that passes validation ends at
# synthesis.

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/check-spec-conform-test.XXXXXX")
trap 'rm -rf "$work"' EXIT INT TERM

fail() { echo "check-spec-conform.test: FAIL: $1" >&2; exit 1; }

mkdir -p "$work/bin"
cat > "$work/bin/ess" <<'EOF'
#!/bin/sh
case "$1 $2" in
  "--version "*) echo "ess 0.0.0" ;;
  "specify validate")
    echo "note: ess 0.0.0 is the dispatcher; delegating to ess 0.0.1 (pin: \`requires: ess 0.0.1\` in $PWD/ess-inputs.yaml)"
    [ -n "${STUB_WARN:-}" ] && echo "warning: $STUB_WARN"
    echo "sample v1 — 1 file(s), valid" ;;
  *) echo "stub: no synthesis" >&2; exit 3 ;;
esac
EOF
chmod +x "$work/bin/ess"

# tree <name> : a copy of the script in <name>/scripts with a minimal spec/
tree() {
  d=$work/$1
  mkdir -p "$d/scripts" "$d/spec"
  cp "$here/check-spec-conform.sh" "$d/scripts/"
  printf 'requires: ess 0.0.1\nspecification:\n  - a.yaml\nscenarios: []\n' > "$d/spec/ess-inputs.yaml"
  printf 'x: 1\n' > "$d/spec/a.yaml"
  echo "$d"
}

# run <tree> : exit code in $got, output in $work/out
run() {
  got=0
  PATH=$work/bin:$PATH sh "$1/scripts/check-spec-conform.sh" > "$work/out" 2>&1 || got=$?
}

d=$(tree cloudspec-warnings)
run "$d"
if grep -q 'a warning is not clean' "$work/out"; then
  cat "$work/out" >&2
  fail "a path that holds \"warning\": the dispatcher's note was counted as a warning"
fi
grep -q 'synthesis failed' "$work/out" || { cat "$work/out" >&2; fail "clean validation: did not reach synthesis"; }
[ "$got" -eq 2 ] || fail "clean validation: exit $got, wanted 2 from the stub's synthesis"

d=$(tree plain)
STUB_WARN="sample: a field is never read"
export STUB_WARN
run "$d"
unset STUB_WARN
[ "$got" -eq 2 ] || { cat "$work/out" >&2; fail "a warning: exit $got, wanted 2"; }
grep -q 'a warning is not clean' "$work/out" || { cat "$work/out" >&2; fail "a warning: not reported"; }
grep -q 'warning: sample: a field is never read' "$work/out" || fail "a warning: not printed"

d=$(tree words)
STUB_WARN="sample: the part that is the dispatcher has no link"
export STUB_WARN
run "$d"
unset STUB_WARN
[ "$got" -eq 2 ] || { cat "$work/out" >&2; fail "a warning that says \"is the dispatcher\": exit $got, wanted 2"; }
grep -q 'a warning is not clean' "$work/out" || { cat "$work/out" >&2; fail "a warning that says \"is the dispatcher\": not reported"; }
grep -q 'warning: sample: the part that is the dispatcher has no link' "$work/out" \
  || fail "a warning that says \"is the dispatcher\": not printed"

echo "check-spec-conform.test: 3 cases passed"
