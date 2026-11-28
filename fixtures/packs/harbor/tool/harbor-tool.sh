#!/bin/sh
# The inspector of the invented harbor pack. It implements no rule: tool/answers/<name>.json is
# its answer for harbors/<name>.yaml, printed as it is. A plan without such a file passes.
#
#   harbor-tool.sh inspect --json <plan>
#
# Prints a JSON list with one entry per plan. Exit 0 when the plan passes, 1 when it does not,
# 2 when the plan cannot be read (the entry then holds "error").

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

fail() {
  printf '[{"error": "%s"}]\n' "$1"
  exit 2
}

[ "${1:-}" = inspect ] || fail "usage: harbor-tool.sh inspect --json <plan>"
[ "${2:-}" = --json ] || fail "inspect prints JSON only: --json"
[ $# -eq 3 ] || fail "inspect takes one plan"
[ -f "$3" ] || fail "no such plan"
name=$(basename -- "$3" .yaml)
if [ -f "$here/answers/$name.json" ]; then
  cat "$here/answers/$name.json"
  if grep -q '"passes": false' "$here/answers/$name.json"; then
    exit 1
  fi
else
  printf '[{"plan": "%s", "passes": true, "breaches": [], "notices": []}]\n' "$name"
fi
