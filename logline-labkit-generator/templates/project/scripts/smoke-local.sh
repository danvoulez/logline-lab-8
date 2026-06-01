#!/usr/bin/env sh
set -eu
HOME_DIR="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR"; }
trap cleanup EXIT
cargo test
cargo run -p logline-lab-cli -- --version
cargo run -p logline-lab-cli -- init --home "$HOME_DIR" --pack santo-andre --profile local-offline
cargo run -p logline-lab-cli -- doctor --home "$HOME_DIR"
cargo run -p logline-lab-cli -- status --home "$HOME_DIR"
cargo run -p logline-lab-cli -- act validate --file examples/acts/minimal.act.json
cargo run -p logline-lab-cli -- candidate add --home "$HOME_DIR" --file examples/acts/minimal.act.json
cargo run -p logline-lab-cli -- candidate list --home "$HOME_DIR"
cargo run -p logline-lab-cli -- ghost list --home "$HOME_DIR"
cargo run -p logline-lab-cli -- report generate daily-state --home "$HOME_DIR"
test -f "$HOME_DIR/.logline-lab/reports/daily-state.md"
cargo run -p logline-lab-cli -- status --home "$HOME_DIR"
