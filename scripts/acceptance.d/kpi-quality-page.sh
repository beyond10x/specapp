# kpi-quality-page: the rows of the quality page, sourced by scripts/acceptance.sh after
# kpi-quality-catalogue-shape.sh, whose reports $work/quality/<pack>/kpis.json it reads (it uses
# row, $metaspec, $work, $root, $org and $agentic, and the trust file of the run that
# acceptance.sh exports as XDG_STATE_HOME).
#
# For harbor, orchard, orgspec and agenticspec:
#
#   quality:page            `metaspec render --quality $work/quality/<pack>/kpis.json` exits 0
#                           and writes quality.html beside index.html.
#   quality:page-measures   the measures of the page (one row `id="measure-<id>"` each) are the
#                           measures of the report, every one of them and no other.
#   quality:page-link       index.html links quality.html.
#
# The pages are $work/quality/<pack>/pages/. The golden pages of the fixture packs are rendered
# from the recorded reports fixtures/quality-reports/<pack>/kpis.json by the tests
# (crates/metaspec/tests/main.rs, which names the command that writes a recorded report again).

# quality_page <name> <dir> <name|manifest-file>
quality_page() {
  qp_name=$1 qp_dir=$2
  case "$3" in
    */*) set -- --manifest "$3" ;;
    *) set -- "$3" ;;
  esac
  qp_report=$work/quality/$qp_name/kpis.json
  qp_out=$work/quality/$qp_name/pages
  if [ -s "$qp_report" ] \
    && "$metaspec" render "$@" --pack "$qp_dir" --out "$qp_out" --quality "$qp_report" > "$qp_out.log" 2>&1 \
    && [ -f "$qp_out/quality.html" ]; then
    qp_ok=1
  else
    qp_ok=0
    echo "acceptance: metaspec render --quality $qp_name failed:" >&2
    [ -f "$qp_out.log" ] && cat "$qp_out.log" >&2
  fi
  row "$qp_name" quality:page "$qp_ok" 1 "metaspec render --quality: exit 0, quality.html"
  [ "$qp_ok" = 1 ] || return 0
  qp_mine=$(sed -n 's/.*<tr id="measure-\([^"]*\)">.*/\1/p' "$qp_out/quality.html" | sort | tr '\n' ' ')
  qp_own=$(jq -r '.measurements[].measure' "$qp_report" | sort | tr '\n' ' ')
  if [ -n "$qp_own" ] && [ "$qp_mine" = "$qp_own" ]; then
    qp_same=1
  else
    qp_same=0
    echo "acceptance: measures of the quality page of $qp_name: page [$qp_mine], report [$qp_own]" >&2
  fi
  row "$qp_name" quality:page-measures "$qp_same" 1 "rows of quality.html = measurements of quality kpis --json"
  if grep -q '<a href="quality.html">' "$qp_out/index.html"; then qp_link=1; else qp_link=0; fi
  row "$qp_name" quality:page-link "$qp_link" 1 "index.html links quality.html"
}

for qp_file in "$root"/fixtures/manifests/*.yaml; do
  quality_page "$(basename "$qp_file" .yaml)" "$root/fixtures/packs/$(basename "$qp_file" .yaml)" "$qp_file"
done
quality_page orgspec "$org" orgspec
quality_page agenticspec "$agentic" agenticspec
