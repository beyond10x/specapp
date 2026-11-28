# spec-quality-static: the rows of `metaspec quality spec`, sourced by scripts/acceptance.sh
# after its wave rows (it uses row, of, $metaspec, $work, $root, $org and $agentic).
#
# For harbor, orchard, orgspec and agenticspec, with the ess on PATH (no METASPEC_ESS):
#
#   quality:spec-report        `metaspec quality spec --json` exits 0 with question
#                              specification.
#   quality:spec-schema        the report is valid against schema/metaspec-quality-1.schema.json,
#                              checked apart from metaspec with ajv (draft 2020-12) of web/
#                              (needs `task web:install`).
#   quality:validity.refusals  equal to the refusals of `ess specify validate --format json` on
#                              the same blocks, run here in the blocks directory: its `problems`
#                              (its `diagnostics` when a refusal lists no problem) and its
#                              `scenario_refusals`.
#   quality:validity.warnings  equal to its `warnings`.
#   quality:ess_completeness.* each of unscenarioed, unanswered and outside equal to its
#                              `completeness.counts` (0 when it has no `completeness`; "gap" on
#                              both sides when ess refuses the blocks).
#   quality:honesty            equal to the number of lines `grep -ra 'UNMAPPED:'` finds under
#                              the manifest's blocks.path.
#   quality:adoption_level     equal to the `level` of `metaspec pack check`.
#   quality:verb-gaps          the verbs of the gaps of kind verb equal the verbs metaspec offers
#                              (check, L3) that the manifest's tool.verbs does not declare.
#
# And for a manifest of this run made from packs/agenticspec.yaml without its `tool`
# (agenticspec-no-tool): the verb gap check that needs L3, and the adoption level equal to the
# level `metaspec pack check` prints for that manifest.
#
# The reports are $work/quality/<pack>/spec.json, ess's own answers
# $work/quality/<pack>/validate.json. ess runs once here and once (or from the user's cache)
# inside metaspec, side by side.

# The verbs metaspec offers, with their levels: OFFERED_VERBS of
# crates/metaspec-core/src/quality/spec/static_measures.rs.
sq_offered="check:L3"

# sq_measure <report> <measure>: the value of a measure, or "gap"
sq_measure() {
  jq -r --arg m "$2" '.measurements[] | select(.measure == $m) | if .reading.kind == "value" then .reading.value else "gap" end' "$1"
}

# sq_schema <report>: 1 when ajv of web/ finds the report valid against the schema, 0 when not,
# - when ajv is not installed
sq_schema() {
  if [ ! -f "$root/web/node_modules/ajv/dist/2020.js" ]; then
    echo -
    return 0
  fi
  if NODE_PATH=$root/web/node_modules node -e '
const Ajv = require("ajv/dist/2020").default;
const fs = require("fs");
const validate = new Ajv({ strict: false, allErrors: true })
  .compile(JSON.parse(fs.readFileSync(process.argv[1], "utf8")));
if (!validate(JSON.parse(fs.readFileSync(process.argv[2], "utf8")))) {
  console.error(JSON.stringify(validate.errors));
  process.exit(1);
}' "$root/schema/metaspec-quality-1.schema.json" "$1" 2> "$1.schema.log"; then
    echo 1
  else
    echo 0
  fi
}

# quality_spec <name> <dir> <name|manifest-file>
quality_spec() {
  sq_name=$1 sq_dir=$2
  case "$3" in
    */*) sq_manifest=$3; set -- --manifest "$3" ;;
    *) sq_manifest=$root/packs/$3.yaml; set -- "$3" ;;
  esac
  sq_out=$work/quality/$sq_name
  mkdir -p "$sq_out"
  sq_blocks=$(yq -r .blocks.path "$sq_manifest")
  # in the blocks directory with --path ., as metaspec runs it: there ess follows the release
  # the blocks pin
  (cd "$sq_dir/$sq_blocks" && ess specify validate --path . --format json > "$sq_out/validate.json" 2> "$sq_out/validate.log") &
  sq_ess=$!
  if "$metaspec" quality spec "$@" --pack "$sq_dir" --json > "$sq_out/spec.json" 2> "$sq_out/spec.log" \
    && [ "$(jq -r .question "$sq_out/spec.json")" = specification ]; then
    sq_ok=1
  else
    sq_ok=0
    echo "acceptance: metaspec quality spec $sq_name failed:" >&2
    cat "$sq_out/spec.log" >&2
  fi
  wait "$sq_ess" || true
  row "$sq_name" quality:spec-report "$sq_ok" 1 "metaspec quality spec --json: exit 0, question specification"
  [ "$sq_ok" = 1 ] || return 0
  row "$sq_name" quality:spec-schema "$(sq_schema "$sq_out/spec.json")" 1 "ajv: schema/metaspec-quality-1.schema.json"

  sq_own=$sq_out/validate.json
  if ! jq -e 'type == "object"' "$sq_own" > /dev/null 2>&1; then
    echo "acceptance: ess specify validate $sq_name printed no JSON:" >&2
    cat "$sq_out/validate.log" >&2
  fi
  row "$sq_name" quality:validity.refusals "$(sq_measure "$sq_out/spec.json" validity.refusals)" \
    "$(jq '(if .compiled == false and ((.problems // []) | length) == 0 then (.diagnostics // []) else (.problems // []) end) + (.scenario_refusals // []) | length' "$sq_own")" \
    "ess specify validate: problems (or diagnostics) + scenario_refusals"
  row "$sq_name" quality:validity.warnings "$(sq_measure "$sq_out/spec.json" validity.warnings)" \
    "$(jq '(.warnings // []) | length' "$sq_own")" "ess specify validate: warnings"
  for sq_list in unscenarioed unanswered outside; do
    row "$sq_name" "quality:ess_completeness.$sq_list" "$(sq_measure "$sq_out/spec.json" "ess_completeness.$sq_list")" \
      "$(jq -r --arg l "$sq_list" 'if .compiled == false then "gap" else (.completeness.counts[$l] // 0) end' "$sq_own")" \
      "ess specify validate: completeness.counts.$sq_list"
  done
  sq_markers=$(grep -ra 'UNMAPPED:' "$sq_dir/$sq_blocks" | wc -l | tr -d ' ')
  row "$sq_name" quality:honesty "$(sq_measure "$sq_out/spec.json" honesty)" "$sq_markers" "grep -ra UNMAPPED: $sq_blocks"
  sq_level=$(of "$sq_name" .level | sed 's/^L//')
  row "$sq_name" quality:adoption_level "$(sq_measure "$sq_out/spec.json" adoption_level)" "$sq_level" "metaspec pack check level"
  sq_verb_gaps "$sq_name" "$sq_manifest" "$sq_out/spec.json"
}

# sq_verb_gaps <name> <manifest> <report>: the row of the verb gaps
sq_verb_gaps() {
  sq_declared=$(yq -r '(.tool.verbs // {}) | keys | .[]' "$2" | sort | tr '\n' ' ')
  sq_want=
  for sq_pair in $sq_offered; do
    case " $sq_declared " in
      *" ${sq_pair%%:*} "*) ;;
      *) sq_want="$sq_want$sq_pair " ;;
    esac
  done
  sq_want=$(printf '%s' "$sq_want" | tr ' ' '\n' | grep . | sort | tr '\n' ' ')
  sq_mine=$(jq -r '.gaps[] | select(.kind == "verb") | "\(.feature):\(.needs)"' "$3" | sort | tr '\n' ' ')
  [ -n "$sq_want" ] || sq_want=none
  [ -n "$sq_mine" ] || sq_mine=none
  row "$1" quality:verb-gaps "$(printf '%s' "$sq_mine" | sed 's/ $//; s/ /,/g')" "$(printf '%s' "$sq_want" | sed 's/ $//; s/ /,/g')" \
    "verbs offered ($sq_offered) not in tool.verbs"
}

for sq_file in "$root"/fixtures/manifests/*.yaml; do
  quality_spec "$(basename "$sq_file" .yaml)" "$root/fixtures/packs/$(basename "$sq_file" .yaml)" "$sq_file"
done
quality_spec orgspec "$org" orgspec
quality_spec agenticspec "$agentic" agenticspec

# agenticspec without its tool: a manifest of this run
yq 'del(.tool)' "$root/packs/agenticspec.yaml" > "$work/agenticspec-no-tool.yaml"
"$metaspec" pack check --manifest "$work/agenticspec-no-tool.yaml" --pack "$agentic" --json > "$work/quality/agenticspec-no-tool.check.json" || true
mkdir -p "$work/quality/agenticspec-no-tool"
if "$metaspec" quality spec --manifest "$work/agenticspec-no-tool.yaml" --pack "$agentic" --json \
  > "$work/quality/agenticspec-no-tool/spec.json" 2> "$work/quality/agenticspec-no-tool/spec.log"; then
  sq_verb_gaps agenticspec-no-tool "$work/agenticspec-no-tool.yaml" "$work/quality/agenticspec-no-tool/spec.json"
  row agenticspec-no-tool quality:adoption_level "$(sq_measure "$work/quality/agenticspec-no-tool/spec.json" adoption_level)" \
    "$(jq -r .level "$work/quality/agenticspec-no-tool.check.json" | sed 's/^L//')" "metaspec pack check level"
else
  echo "acceptance: metaspec quality spec agenticspec-no-tool failed:" >&2
  cat "$work/quality/agenticspec-no-tool/spec.log" >&2
  row agenticspec-no-tool quality:spec-report 0 1 "metaspec quality spec --json: exit 0"
fi
