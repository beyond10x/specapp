#!/bin/sh
# check-public.sh: refuse content that does not belong in a public repository.
#
#   scripts/check-public.sh [--path DIR] [--show]
#
# Checks every file git tracks or would track (in a directory that is no git checkout: every
# file under it):
#
#   home      no path under a home directory (/home/<name>, /Users/<name>)
#   mail      no e-mail address outside example.org, example.com and users.noreply.github.com
#   address   no IPv4 address outside 127.0.0.0/8, 0.0.0.0 and the documentation ranges
#   deny      no term of the deny list, in a file or in a file name
#
# The deny list is the file SPECAPP_DENYLIST names. It lives outside this repository. One term
# per line, matched as fixed text without regard to case. A line starting with `=` matches the
# term only as a whole word. Empty lines and lines starting with `#` are skipped. With
# SPECAPP_DENYLIST unset (CI, a build node) the deny check is skipped and the last line says so.
#
# A finding prints `<check> <file>:<line>`. The matched text is printed only with --show, so a
# log of this script names no term of the deny list.
#
# Exit 0: nothing found. Exit 1: at least one finding. Exit 2: bad usage or unreadable input.

set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
path=$root
show=0

while [ $# -gt 0 ]; do
  case "$1" in
    --path) [ $# -ge 2 ] || { echo "check-public: --path needs a directory" >&2; exit 2; }
            path=$2; shift 2 ;;
    --show) show=1; shift ;;
    *) echo "check-public: unknown argument: $1" >&2; exit 2 ;;
  esac
done

[ -d "$path" ] || { echo "check-public: no such directory: $path" >&2; exit 2; }
cd "$path"

tmp=$(mktemp -d "${TMPDIR:-/tmp}/check-public.XXXXXX")
trap 'rm -rf "$tmp"' EXIT INT TERM

if git rev-parse --show-toplevel >/dev/null 2>&1 && [ "$(git rev-parse --show-toplevel)" = "$(pwd -P)" ]; then
  git ls-files -co --exclude-standard
else
  find . -type f ! -path './.git/*' ! -path './target/*' ! -path '*/node_modules/*' | sed 's|^\./||'
fi | grep -v -e '^LICENSE$' -e '^Cargo\.lock$' | sort > "$tmp/files"

: > "$tmp/found"

# report <check> : reads `file:line:text` on stdin
report() {
  if [ "$show" -eq 1 ]; then
    sed "s|^|$1 |" >> "$tmp/found"
  else
    awk -F: -v c="$1" '{ print c " " $1 ":" $2 }' >> "$tmp/found"
  fi
}

# scan <check> <grep options...> : run grep over every file of the list
scan() {
  check=$1; shift
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    grep -I -n -H "$@" -- "$f" 2>/dev/null || true
  done < "$tmp/files" | report "$check"
}

# home
scan home -E '(/home|/Users)/[A-Za-z0-9._-]+'

# mail
while IFS= read -r f; do
  [ -f "$f" ] || continue
  grep -I -n -H -o -E '[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}' -- "$f" 2>/dev/null || true
done < "$tmp/files" \
  | grep -v -i -E '@(example\.org|example\.com|users\.noreply\.github\.com)$' \
  | report mail || true

# address
while IFS= read -r f; do
  [ -f "$f" ] || continue
  grep -I -n -H -o -E '(^|[^0-9.])[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}($|[^0-9])' -- "$f" 2>/dev/null || true
done < "$tmp/files" \
  | grep -v -E ':[^0-9]?(127\.[0-9]+\.[0-9]+\.[0-9]+|0\.0\.0\.0|192\.0\.2\.[0-9]+|198\.51\.100\.[0-9]+|203\.0\.113\.[0-9]+)[^0-9]?$' \
  | report address || true

# deny
deny_note="no deny list (SPECAPP_DENYLIST is not set)"
if [ -n "${SPECAPP_DENYLIST:-}" ]; then
  [ -r "$SPECAPP_DENYLIST" ] || { echo "check-public: cannot read the deny list" >&2; exit 2; }
  grep -v -e '^[[:space:]]*$' -e '^#' "$SPECAPP_DENYLIST" | grep -v '^=' > "$tmp/sub" || true
  grep '^=' "$SPECAPP_DENYLIST" | sed 's/^=//' | grep -v '^[[:space:]]*$' > "$tmp/word" || true
  terms=$(( $(wc -l < "$tmp/sub") + $(wc -l < "$tmp/word") ))
  [ "$terms" -gt 0 ] || { echo "check-public: the deny list has no term" >&2; exit 2; }
  if [ -s "$tmp/sub" ]; then
    scan deny -i -F -f "$tmp/sub"
    grep -n -i -F -f "$tmp/sub" "$tmp/files" | sed 's|^\([0-9]*\):\(.*\)$|\2:0:\2|' | report deny-name || true
  fi
  if [ -s "$tmp/word" ]; then
    scan deny -i -w -F -f "$tmp/word"
    grep -n -i -w -F -f "$tmp/word" "$tmp/files" | sed 's|^\([0-9]*\):\(.*\)$|\2:0:\2|' | report deny-name || true
  fi
  deny_note="deny list with $terms term(s)"
fi

files=$(wc -l < "$tmp/files" | tr -d ' ')
found=$(sort -u "$tmp/found" | wc -l | tr -d ' ')
sort -u "$tmp/found"

if [ "$found" -gt 0 ]; then
  echo "check-public: $found finding(s) in $files file(s), $deny_note"
  exit 1
fi
echo "check-public: $files file(s), 0 findings, $deny_note"
