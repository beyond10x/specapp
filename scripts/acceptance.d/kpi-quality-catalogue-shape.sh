# kpi-quality-catalogue-shape: the rows of `metaspec quality kpis`, sourced by scripts/acceptance.sh
# after its wave rows (it uses row, of, $metaspec, $work, $root, $org and $agentic).
#
# For harbor, orchard, orgspec and agenticspec:
#
#   quality:kpis-report     `metaspec quality kpis --json` exits 0 and prints a report of format
#                           metaspec-quality/1; the command validates the report against
#                           schema/metaspec-quality-1.schema.json before it prints it, and exits 1
#                           when it does not follow it.
#   quality:<measure>       coverage.blocks, completeness.fields and planned are values.
#   quality:planned-kpis    the KPIs behind planned are exactly the rows of the pack's catalogue
#                           (the manifest's kpis.path, read with yq) whose block is no block that
#                           `metaspec pack words` prints.
#   quality:planned-blocks  the distinct blocks of those rows are as many as the planned blocks
#                           `metaspec pack check` counts.
#
# The reports are $work/quality/<pack>/kpis.json.

# quality_kpis <name> <dir> <name|manifest-file>
quality_kpis() {
  qk_name=$1 qk_dir=$2
  case "$3" in
    */*) qk_manifest=$3; set -- --manifest "$3" ;;
    *) qk_manifest=$root/packs/$3.yaml; set -- "$3" ;;
  esac
  qk_out=$work/quality/$qk_name
  mkdir -p "$qk_out"
  if "$metaspec" quality kpis "$@" --pack "$qk_dir" --json > "$qk_out/kpis.json" 2> "$qk_out/kpis.log" \
    && [ "$(jq -r .format "$qk_out/kpis.json")" = metaspec-quality/1 ]; then
    qk_ok=1
  else
    qk_ok=0
    echo "acceptance: metaspec quality kpis $qk_name failed:" >&2
    cat "$qk_out/kpis.log" >&2
  fi
  row "$qk_name" quality:kpis-report "$qk_ok" 1 "metaspec quality kpis --json: exit 0, schema/metaspec-quality-1.schema.json"
  [ "$qk_ok" = 1 ] || return 0
  for qk_measure in coverage.blocks completeness.fields planned; do
    qk_kind=$(jq -r --arg m "$qk_measure" '[.measurements[] | select(.measure == $m) | .reading.kind] | join(" ")' "$qk_out/kpis.json")
    row "$qk_name" "quality:$qk_measure" "$qk_kind" value "the reading of $qk_measure"
  done
  "$metaspec" pack words "$@" --pack "$qk_dir" | awk '$1 == "block" { print $2 }' > "$qk_out/blocks"
  qk_catalogue=$qk_dir/$(yq -r .kpis.path "$qk_manifest")
  yq -r '.kpis[] | select(.block != null) | [.id, .block] | @tsv' "$qk_catalogue" \
    | awk -F '\t' 'NR == FNR { b[$1] = 1; next } !($2 in b)' "$qk_out/blocks" - > "$qk_out/planned.own"
  qk_mine=$(jq -r '.measurements[] | select(.measure == "planned") | .behind[].name' "$qk_out/kpis.json" | sort | tr '\n' ' ')
  qk_own=$(cut -f1 "$qk_out/planned.own" | sort | tr '\n' ' ')
  if [ "$qk_mine" = "$qk_own" ]; then
    qk_same=1
  else
    qk_same=0
    echo "acceptance: planned KPIs of $qk_name: metaspec [$qk_mine], catalogue [$qk_own]" >&2
  fi
  row "$qk_name" quality:planned-kpis "$qk_same" 1 "planned = catalogue rows whose block is no block of metaspec pack words"
  qk_blocks=$(cut -f2 "$qk_out/planned.own" | sort -u | grep -c . || true)
  row "$qk_name" quality:planned-blocks "$qk_blocks" "$(of "$qk_name" .counts.planned_blocks)" "planned blocks of metaspec pack check"
}

for qk_file in "$root"/fixtures/manifests/*.yaml; do
  quality_kpis "$(basename "$qk_file" .yaml)" "$root/fixtures/packs/$(basename "$qk_file" .yaml)" "$qk_file"
done
quality_kpis orgspec "$org" orgspec
quality_kpis agenticspec "$agentic" agenticspec
