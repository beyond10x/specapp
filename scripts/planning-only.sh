#!/bin/sh
# planning-only.sh: say whether a change touches planning and prose files only.
#
#   scripts/planning-only.sh < paths
#
# Reads changed paths, one per line, on stdin. A planning or prose file is a file under
# .engineering/ or a *.md file. Such a change needs no build and no test: the CI gate
# (.github/workflows/gate.yml) passes it without running `task check`.
#
# Exit 0: every path is a planning or prose file. Exit 1: at least one path is not, or the list
# is empty. Exit 2: bad usage.

[ $# -eq 0 ] || { echo "usage: scripts/planning-only.sh < paths" >&2; exit 2; }

seen=0
while IFS= read -r path; do
  [ -n "$path" ] || continue
  seen=1
  case "$path" in
    .engineering/*) ;;
    *.md) ;;
    *) echo "not planning-only: $path"; exit 1 ;;
  esac
done
[ "$seen" = 1 ] || { echo "not planning-only: no changed paths"; exit 1; }
echo "planning-only"
exit 0
