# Install

The generated LogLine Lab Kit is local-first. The `local-offline` profile does not require Supabase, network services, external databases, or LLM credentials.

## Install methods

### Default local user install

```bash
./install.sh
export PATH="$HOME/.local/bin:$PATH"
logline-lab --version
logline-lab serve
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

Use the browser product:

```bash
logline-lab serve
```

Then open `http://127.0.0.1:8787`.

For terminal setup:

```bash
logline-lab setup
```

For automation or support scripts:

```bash
logline-lab setup --yes --home ./demo-lab --pack santo-andre --profile local-offline
```

Setup creates the Lab home, captures a starter Candidate, writes the Daily State report, generates the local-summary projection, and prints next commands. It remains local workspace state only: not an official spine, not a receipt store, not evidence proof, and not remote sync.

The same flow can be run manually:

```bash
logline-lab init --home ./demo-lab --pack santo-andre --profile local-offline
logline-lab doctor --home ./demo-lab
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab candidate add --home ./demo-lab --file examples/acts/minimal.act.json
logline-lab candidate list --home ./demo-lab
logline-lab candidate index rebuild --home ./demo-lab
logline-lab ghost list --home ./demo-lab
logline-lab report generate daily-state --home ./demo-lab
logline-lab projection list --home ./demo-lab
logline-lab projection generate local-summary --home ./demo-lab
logline-lab projection list --home ./demo-lab
logline-lab status --home ./demo-lab
```

The initialized home includes `.logline-lab/candidates/index.json` as a local Candidate index for listing/status/reporting and `.logline-lab/projections/` for local read-model outputs. The Candidate index is local operational metadata, not a ledger, official spine, receipt, evidence, or source of truth. The initialized home is local workspace state only. It is not an official spine, not a receipt store, not evidence proof, and not remote sync.

## What install does and does not do

`install.sh` does:

- Check that `cargo` is available.
- Build `logline-lab-cli`.
- Copy the `logline-lab` binary into `<prefix>/bin`.
- Print next commands and authority boundaries.

`install.sh` does not:

- Require sudo.
- Install or contact Supabase.
- Apply remote spine migrations.
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

Some Ghosts are expected. The local profile works while Supabase profile configuration, receipt closure, evidence registry, interactive Lab surface, LLM translator, and YAML Act parsing remain unresolved.

### Supabase profile

The Supabase profile uses versioned SQL under `supabase/migrations/`. Apply those migrations with the Supabase CLI or reviewed project migration flow before using remote emit.

Required server-side environment:

```bash
export SUPABASE_URL=...
export SUPABASE_SERVICE_ROLE_KEY=...
```

Then check and emit:

```bash
logline-lab supabase check
logline-lab act emit --file examples/acts/minimal.act.json --remote
```

Remote emit targets `ops.logline_acts` through `ops.ingest_logline_act`. It does not close receipts and does not write projection tables directly.

### Candidate index warnings

If `doctor` reports `candidate index: missing`, rebuild local queue metadata from candidate directories:

```bash
logline-lab candidate index rebuild --home ./demo-lab
```

If `doctor` reports `candidate index: malformed`, inspect the local file and rebuild only when you intend to replace local queue metadata.

### `candidate add` fails before init

Initialize a Lab home first:

```bash
logline-lab init --home ./demo-lab --pack santo-andre --profile local-offline
```

### Projection generation

Use `logline-lab projection generate local-summary --home ./demo-lab` to write a regenerated local read model under `.logline-lab/projections/local-summary.md`. The projection is an operator-facing summary only; it does not close receipts, prove evidence, amend canon, or sync remote state.
