#!/usr/bin/env sh
set -eu
BIN="${BIN:-cargo run -q -p logline-lab-cli --}"
HOME_DIR="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR"; }
trap cleanup EXIT
{
  echo "# Command Matrix"
  echo
  for cmd in "--version" "--help" "init --help" "act validate --help" "candidate add --help" "init --home $HOME_DIR --pack santo-andre --profile local-offline" "doctor --home $HOME_DIR" "status --home $HOME_DIR" "act validate" "act validate --file examples/acts/minimal.act.json" "act emit --file examples/acts/minimal.act.json" "candidate add --home $HOME_DIR --file examples/acts/minimal.act.json" "candidate list --home $HOME_DIR" "ghost list --home $HOME_DIR" "report generate daily-state --home $HOME_DIR" "status --home $HOME_DIR"; do
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
} | tee reports/COMMAND_MATRIX.md
