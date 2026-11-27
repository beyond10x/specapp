#!/bin/sh
# Tests of scripts/check-public.sh: a clean tree passes, each check refuses its case, and the
# output names no term of the deny list unless --show is given.

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
sut=$here/check-public.sh
work=$(mktemp -d "${TMPDIR:-/tmp}/check-public-test.XXXXXX")
trap 'rm -rf "$work"' EXIT INT TERM

fail() { echo "check-public.test: FAIL: $1" >&2; exit 1; }

# tree <name> : a clean tree
tree() {
  d=$work/$1
  mkdir -p "$d/design"
  printf 'Write to someone@example.org. The loopback is 127.0.0.1.\n' > "$d/design/notes.md"
  echo "$d"
}

# expect <exit> <label> <dir> [args...] : run and compare the exit code; output in $work/out
expect() {
  want=$1; label=$2; dir=$3; shift 3
  got=0
  "$sut" --path "$dir" "$@" > "$work/out" 2>&1 || got=$?
  [ "$got" -eq "$want" ] || { cat "$work/out" >&2; fail "$label: exit $got, wanted $want"; }
}

unset SPECAPP_DENYLIST

d=$(tree clean)
expect 0 "clean tree" "$d"
grep -q 'no deny list' "$work/out" || fail "clean tree: last line does not say that no deny list is set"

d=$(tree home)
printf 'see %s/someone/notes\n' /home > "$d/design/path.md"
expect 1 "home path" "$d"
grep -q '^home design/path.md:1$' "$work/out" || fail "home path: finding not printed"

d=$(tree mail)
printf 'ask someone@%s\n' bakery.test > "$d/design/mail.md"
expect 1 "mail address" "$d"
grep -q '^mail design/mail.md:1$' "$work/out" || fail "mail address: finding not printed"

d=$(tree address)
printf 'host 10.%s.3.4\n' 2 > "$d/design/host.md"
expect 1 "address" "$d"
grep -q '^address design/host.md:1$' "$work/out" || fail "address: finding not printed"

# deny list: a substring term, an exact-word term
list=$work/deny.txt
printf '# comment\n\nMillstone\n=mark\n' > "$list"

d=$(tree deny-unset)
printf 'our supplier is the millstone company\n' > "$d/design/a.md"
expect 0 "deny list unset: the deny check is skipped" "$d"
grep -q 'no deny list' "$work/out" || fail "deny list unset: last line does not say so"

d=$(tree deny-sub)
printf 'our supplier is the millstone company\n' > "$d/design/a.md"
SPECAPP_DENYLIST=$list expect 1 "deny substring" "$d"
grep -q '^deny design/a.md:1$' "$work/out" || fail "deny substring: finding not printed"
if grep -q -i 'millstone' "$work/out"; then fail "deny substring: the output names the term"; fi
SPECAPP_DENYLIST=$list expect 1 "deny substring --show" "$d" --show
grep -q -i 'millstone' "$work/out" || fail "deny substring: --show does not print the line"

d=$(tree deny-word)
printf 'a marker and a remark\n' > "$d/design/b.md"
SPECAPP_DENYLIST=$list expect 0 "deny exact word, only inside other words" "$d"
printf 'ask Mark\n' > "$d/design/c.md"
SPECAPP_DENYLIST=$list expect 1 "deny exact word" "$d"
grep -q '^deny design/c.md:1$' "$work/out" || fail "deny exact word: finding not printed"

d=$(tree deny-name)
printf 'nothing\n' > "$d/design/millstone-plan.md"
SPECAPP_DENYLIST=$list expect 1 "deny file name" "$d"
grep -q '^deny-name design/millstone-plan.md:0$' "$work/out" || fail "deny file name: finding not printed"

d=$(tree deny-missing)
SPECAPP_DENYLIST=$work/none.txt expect 2 "deny list missing" "$d"

echo "check-public.test: 11 cases passed"
