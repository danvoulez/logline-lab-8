# Install

The generated LogLine Lab Kit is local-first. The `local-offline` profile does not require Supabase, network services, external databases, or LLM credentials.

## Install methods

### Default local user install

```bash
./install.sh
export PATH="$HOME/.local/bin:$PATH"
logline-lab --version
```

By default, `install.sh` builds the release CLI and copies `logline-lab` to `$HOME/.local/bin/logline-lab`.

### Chosen prefix

```bash
./install.sh --prefix "$PWD/.local"
export PATH="$PWD/.local/bin:$PATH"
logline-lab --help
```

The script writes only to the requested prefix and prints the installed binary path.

### Local development install

```bash
./install.sh --dev --prefix "$PWD/.local"
export PATH="$PWD/.local/bin:$PATH"
logline-lab --version
```

`--dev` builds a debug binary and installs that copy. For direct development without installing, use:

```bash
cargo run -p logline-lab-cli -- --help
```

## Expected first local flow

```bash
logline-lab init --home ./demo-lab --pack santo-andre --profile local-offline
logline-lab doctor --home ./demo-lab
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab candidate add --home ./demo-lab --file examples/acts/minimal.act.json
logline-lab candidate list --home ./demo-lab
logline-lab ghost list --home ./demo-lab
logline-lab report generate daily-state --home ./demo-lab
logline-lab status --home ./demo-lab
```

The initialized home is local workspace state only. It is not an official spine, not a receipt store, not evidence proof, and not remote sync.

## What install does and does not do

`install.sh` does:

- Check that `cargo` is available.
- Build `logline-lab-cli`.
- Copy the `logline-lab` binary into `<prefix>/bin`.
- Print next commands and authority boundaries.

`install.sh` does not:

- Require sudo.
- Install or contact Supabase.
- Configure a remote spine.
- Close receipts.
- Install an LLM provider or TUI.
- Claim production readiness.

## Troubleshooting

### `cargo` not found

Install Rust/Cargo, then rerun:

```bash
./install.sh --prefix "$HOME/.local"
```

### `logline-lab` not found after install

Add the install bin directory to `PATH` for the current shell:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

If you used a custom prefix, replace `$HOME/.local` with that prefix.

### `doctor` reports Ghosts

Some Ghosts are expected. The local profile works while remote spine writes, receipt closure, evidence registry, interactive Lab surface, LLM translator, and YAML Act parsing remain unimplemented.

### `candidate add` fails before init

Initialize a Lab home first:

```bash
logline-lab init --home ./demo-lab --pack santo-andre --profile local-offline
```
