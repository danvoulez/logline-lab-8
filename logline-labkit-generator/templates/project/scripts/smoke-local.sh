#!/usr/bin/env sh
set -eu
HOME_DIR="$(mktemp -d)"
cleanup() { rm -rf "$HOME_DIR"; }
trap cleanup EXIT
cargo test
cargo run -p logline-lab-cli -- --version
cargo run -p logline-lab-cli -- init --home "$HOME_DIR"
cargo run -p logline-lab-cli -- doctor --home "$HOME_DIR"
cargo run -p logline-lab-cli -- status --home "$HOME_DIR"
cargo run -p logline-lab-cli -- act validate --file examples/acts/minimal.act.json
