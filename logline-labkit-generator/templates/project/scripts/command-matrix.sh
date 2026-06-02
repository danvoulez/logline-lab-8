#!/usr/bin/env sh
set -eu
BIN="${BIN:-cargo run -q -p logline-lab-cli --}"
HOME_DIR="$(mktemp -d)"
SETUP_HOME="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR" "$SETUP_HOME"; }
trap cleanup EXIT
{
  echo "# Command Matrix"
  echo
  for cmd in "--version" "--help" "setup --help" "setup --yes --home $SETUP_HOME --pack santo-andre --profile local-offline" "serve --help" "supabase check --help" "init --help" "act validate --help" "candidate add --help" "init --home $HOME_DIR --pack santo-andre --profile local-offline" "doctor --home $HOME_DIR" "status --home $HOME_DIR" "act validate" "act validate --file examples/acts/minimal.act.json" "act emit --file examples/acts/minimal.act.json" "candidate add --home $HOME_DIR --file examples/acts/minimal.act.json" "candidate list --home $HOME_DIR" "ghost list --home $HOME_DIR" "projection list --home $HOME_DIR" "report generate daily-state --home $HOME_DIR" "projection generate local-summary --home $HOME_DIR" "projection list --home $HOME_DIR" "status --home $HOME_DIR"; do
    echo "## logline-lab $cmd"
    $BIN $cmd
    echo
  done
  CANDIDATE_ID="$($BIN candidate list --home "$HOME_DIR" | awk '/^- cand_/ { print $2; exit }')"
  echo "## logline-lab candidate get $CANDIDATE_ID --home $HOME_DIR"
  $BIN candidate get "$CANDIDATE_ID" --home "$HOME_DIR"
  echo
  echo "## logline-lab lab"
  if $BIN lab; then echo "unexpected implemented"; else echo "ghost expected"; fi
  echo
  echo "## logline-lab chat"
  if $BIN chat; then echo "unexpected implemented"; else echo "ghost expected"; fi
  echo
  echo "## logline-lab supabase check"
  if env -u SUPABASE_URL -u SUPABASE_SERVICE_ROLE_KEY $BIN supabase check; then echo "unexpected configured"; else echo "configuration ghost expected"; fi
  echo
  echo "## logline-lab act emit --file examples/acts/minimal.act.json --remote"
  if env -u SUPABASE_URL -u SUPABASE_SERVICE_ROLE_KEY $BIN act emit --file examples/acts/minimal.act.json --remote; then echo "unexpected remote write"; else echo "configuration ghost expected"; fi
} | tee reports/COMMAND_MATRIX_RUN.md
