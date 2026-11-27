#!/bin/sh
# The tool of the invented harbor pack. It answers the specapp tool protocol from files and
# implements no rule: tool/describe.json is its answer to `describe`, and
# tool/findings/<name>.json its answer to `check harbors/<name>.yaml`. A document without such
# a file is valid.
#
#   harbor-tool.sh describe
#   harbor-tool.sh check <document>
#
# Exit 0 with a JSON document on stdout. Exit 2 with {"error": ...} on stdout.

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

fail() {
  printf '{"error": "%s"}\n' "$1"
  exit 2
}

case "${1:-}" in
  describe)
    [ $# -eq 1 ] || fail "describe takes no argument"
    cat "$here/describe.json"
    ;;
  check)
    [ $# -eq 2 ] || fail "check takes one document"
    [ -f "$2" ] || fail "no such document"
    name=$(basename -- "$2" .yaml)
    if [ -f "$here/findings/$name.json" ]; then
      cat "$here/findings/$name.json"
    else
      printf '{"format": "specapp-findings/1", "valid": true, "findings": []}\n'
    fi
    ;;
  *)
    fail "unknown verb"
    ;;
esac
