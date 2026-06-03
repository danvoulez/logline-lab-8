#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
HOME_DIR="$(mktemp -d)"
SETUP_HOME="$(mktemp -d)"
OUT_DIR="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR" "$SETUP_HOME" "$OUT_DIR"; }
trap cleanup EXIT

cd "$ROOT"

cargo build -q -p logline-lab-cli
BIN="$ROOT/target/debug/logline-lab"
ACT="examples/acts/minimal.act.json"

run_capture() {
  name="$1"
  shift
  "$BIN" "$@" >"$OUT_DIR/$name.out" 2>"$OUT_DIR/$name.err"
}

assert_contains() {
  file="$1"
  needle="$2"
  if ! grep -F "$needle" "$file" >/dev/null 2>&1; then
    echo "smoke-local: expected '$needle' in $file" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

run_capture version --version
assert_contains "$OUT_DIR/version.out" "logline-lab"
assert_contains "$OUT_DIR/version.out" "0.1.0-alpha.0"

run_capture help --help
assert_contains "$OUT_DIR/help.out" "CLI-first local LogLine Lab Kit"
assert_contains "$OUT_DIR/help.out" "setup"
assert_contains "$OUT_DIR/help.out" "serve"
assert_contains "$OUT_DIR/help.out" "candidate add"
assert_contains "$OUT_DIR/help.out" "report generate daily-state"
assert_contains "$OUT_DIR/help.out" "projection generate local-summary"

run_capture setup setup --yes --home "$SETUP_HOME" --pack santo-andre --profile local-offline
assert_contains "$OUT_DIR/setup.out" "LogLine Lab is ready."
assert_contains "$OUT_DIR/setup.out" "candidate captured"
assert_contains "$OUT_DIR/setup.out" "daily-state report generated"
assert_contains "$OUT_DIR/setup.out" "local-summary projection generated"
assert_contains "$OUT_DIR/setup.out" "Open your Lab:"
assert_contains "$OUT_DIR/setup.out" "not official spine"
assert_contains "$OUT_DIR/setup.out" "not receipt"
assert_contains "$OUT_DIR/setup.out" "not evidence"
test -f "$SETUP_HOME/.logline-lab/lab.manifest.yaml"
test -f "$SETUP_HOME/.logline-lab/reports/daily-state.md"
test -f "$SETUP_HOME/.logline-lab/projections/local-summary.md"

run_capture serve_help serve --help
assert_contains "$OUT_DIR/serve_help.out" "Usage: logline-lab serve"
assert_contains "$OUT_DIR/serve_help.out" "local browser product"

run_capture init init --home "$HOME_DIR" --pack santo-andre --profile local-offline
assert_contains "$OUT_DIR/init.out" "initialized local LogLine Lab home"
assert_contains "$OUT_DIR/init.out" "profile: local-offline"

run_capture doctor doctor --home "$HOME_DIR"
assert_contains "$OUT_DIR/doctor.out" "doctor: ok"
assert_contains "$OUT_DIR/doctor.out" "remote spine: ghost remote-spine-unconfigured"

run_capture validate act validate --file "$ACT"
assert_contains "$OUT_DIR/validate.out" "valid LogLine Act"

if env -u DATABASE_URL -u SUPABASE_URL -u SUPABASE_SERVICE_ROLE_KEY -u SUPABASE_SECRET_KEY "$BIN" supabase check >"$OUT_DIR/supabase_check.out" 2>"$OUT_DIR/supabase_check.err"; then
  echo "smoke-local: supabase check unexpectedly succeeded without env" >&2
  exit 1
fi
assert_contains "$OUT_DIR/supabase_check.err" "missing required env"
assert_contains "$OUT_DIR/supabase_check.err" "remote-spine-unconfigured"

run_capture add candidate add --home "$HOME_DIR" --file "$ACT"
assert_contains "$OUT_DIR/add.out" "candidate captured"
assert_contains "$OUT_DIR/add.out" "not official spine"
assert_contains "$OUT_DIR/add.out" "index: available"
test -f "$HOME_DIR/.logline-lab/candidates/index.json"
CANDIDATE_ID="$(awk '/^id: / { print $2; exit }' "$OUT_DIR/add.out")"
if [ -z "$CANDIDATE_ID" ]; then
  echo "smoke-local: missing candidate id" >&2
  cat "$OUT_DIR/add.out" >&2
  exit 1
fi

run_capture list candidate list --home "$HOME_DIR"
assert_contains "$OUT_DIR/list.out" "candidates: 1"
assert_contains "$OUT_DIR/list.out" "index: available"
assert_contains "$OUT_DIR/list.out" "$CANDIDATE_ID"

run_capture get candidate get "$CANDIDATE_ID" --home "$HOME_DIR"
assert_contains "$OUT_DIR/get.out" "$CANDIDATE_ID"
assert_contains "$OUT_DIR/get.out" "candidate:"

run_capture ghosts ghost list --home "$HOME_DIR"
assert_contains "$OUT_DIR/ghosts.out" "remote-spine-unconfigured"
assert_contains "$OUT_DIR/ghosts.out" "authority: local workspace Ghost list only"

run_capture projection_list_empty projection list --home "$HOME_DIR"
assert_contains "$OUT_DIR/projection_list_empty.out" "projections: 0"
assert_contains "$OUT_DIR/projection_list_empty.out" "local-summary"

run_capture report report generate daily-state --home "$HOME_DIR"
assert_contains "$OUT_DIR/report.out" "daily-state report generated"
test -f "$HOME_DIR/.logline-lab/reports/daily-state.md"
assert_contains "$HOME_DIR/.logline-lab/reports/daily-state.md" "# Daily Lab State"
assert_contains "$HOME_DIR/.logline-lab/reports/daily-state.md" "Candidates: 1"
assert_contains "$HOME_DIR/.logline-lab/reports/daily-state.md" "## Candidate Index"
assert_contains "$HOME_DIR/.logline-lab/reports/daily-state.md" "State: available"

run_capture projection_generate projection generate local-summary --home "$HOME_DIR"
assert_contains "$OUT_DIR/projection_generate.out" "local-summary projection generated"
test -f "$HOME_DIR/.logline-lab/projections/local-summary.md"
assert_contains "$HOME_DIR/.logline-lab/projections/local-summary.md" "# Local Summary Projection"
assert_contains "$HOME_DIR/.logline-lab/projections/local-summary.md" "Candidates: 1"

run_capture projection_list projection list --home "$HOME_DIR"
assert_contains "$OUT_DIR/projection_list.out" "projections: 1"
assert_contains "$OUT_DIR/projection_list.out" "state=available"

run_capture status status --home "$HOME_DIR"
assert_contains "$OUT_DIR/status.out" "status: local LogLine Lab workspace"
assert_contains "$OUT_DIR/status.out" "candidate_count: 1"
assert_contains "$OUT_DIR/status.out" "candidate_index: available"
assert_contains "$OUT_DIR/status.out" "reports_available: 1"
assert_contains "$OUT_DIR/status.out" "projections_available: 1"
assert_contains "$OUT_DIR/status.out" "local-summary.md"

echo "smoke-local: ok"
