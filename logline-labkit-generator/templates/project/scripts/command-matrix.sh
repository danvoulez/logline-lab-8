#!/usr/bin/env sh
set -eu
BIN="${BIN:-cargo run -q -p logline-lab-cli --}"
HOME_DIR="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR"; }
trap cleanup EXIT
{
  echo "# Command Matrix"
  echo
  for cmd in "--version" "init --home $HOME_DIR" "doctor --home $HOME_DIR" "status --home $HOME_DIR" "act validate" "act validate --file examples/acts/minimal.act.json" "act emit --file examples/acts/minimal.act.json"; do
    echo "## logline-lab $cmd"
    $BIN $cmd
    echo
  done
  echo "## logline-lab lab"
  if $BIN lab; then echo "unexpected implemented"; else echo "ghost expected"; fi
  echo
  echo "## logline-lab chat"
  if $BIN chat; then echo "unexpected implemented"; else echo "ghost expected"; fi
} | tee reports/COMMAND_MATRIX.md
