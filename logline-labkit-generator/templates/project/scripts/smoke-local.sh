#!/usr/bin/env sh
set -eu
cargo test
cargo run -p logline-lab-cli -- --version
cargo run -p logline-lab-cli -- act validate --file examples/acts/minimal.act.json
