#!/bin/sh
# check-pack-dirs.sh: fail when a pack directory holds a manifest of specapp.
#
#   scripts/check-pack-dirs.sh [--strict] [DIR...]
#
# specapp knows the packs; a pack knows nothing of specapp. A pack's manifest is specapp's
# (packs/<name>.yaml, fixtures/manifests/<name>.yaml, or a file given at run time), never a
# file in the pack. This check fails when a file under DIR is named like a manifest of specapp
# (specapp*.yaml, specapp*.yml, specapp*.json, any case). With --strict it also fails when a
# file under DIR names specapp at all, in its name or its text: the fixture packs are held to
# that. Without DIR the directories are the fixture packs under fixtures/packs/, strictly.
# .git, target and node_modules are skipped.
#
# Exit 0: nothing found. Exit 1: at least one file, listed. Exit 2: bad usage.

set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
strict=no
if [ "${1:-}" = --strict ]; then
  strict=yes
  shift
fi
if [ $# -eq 0 ]; then
  strict=yes
  for dir in "$root"/fixtures/packs/*/; do
    set -- "$@" "${dir%/}"
  done
fi

found=0
for dir in "$@"; do
  [ -d "$dir" ] || { echo "check-pack-dirs: no directory $dir" >&2; exit 2; }
  named=$(find "$dir" \( -name .git -o -name target -o -name node_modules \) -prune -o \
    -type f \( -iname 'specapp*.yaml' -o -iname 'specapp*.yml' -o -iname 'specapp*.json' \) -print)
  if [ -n "$named" ]; then
    printf '%s\n' "$named" | sed 's/^/check-pack-dirs: a manifest of specapp in a pack: /'
    found=1
  fi
  if [ "$strict" = yes ]; then
    mentions=$(find "$dir" \( -name .git -o -name target -o -name node_modules \) -prune -o \
      -type f \( -iname '*specapp*' -o -exec grep -qi specapp {} \; \) -print)
    if [ -n "$mentions" ]; then
      printf '%s\n' "$mentions" | sed 's/^/check-pack-dirs: names specapp: /'
      found=1
    fi
  fi
done

if [ "$found" = 1 ]; then
  exit 1
fi
echo "check-pack-dirs: $# pack director(y/ies), no file of specapp"
