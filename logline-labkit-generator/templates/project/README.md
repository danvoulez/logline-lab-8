# LogLine Lab Kit

## What this is

An installable, CLI-first LogLine Lab Kit for local validation, local Lab home initialization, Candidate capture, Ghost visibility, and Daily State reporting.

The kit treats the LogLine Act as the semantic unit. Canon stays small; packs and profiles carry local practice; labs run canon plus a selected pack/profile; projections read; runtimes observe or execute.

## What this is not

- Not a dashboard.
- Not a canon amendment.
- Not an official spine by itself.
- Not a receipt or evidence system yet.
- Not an LLM authority layer.
- Not a Supabase requirement for the `local-offline` profile.

## Quickstart

```bash
./install.sh --prefix "$HOME/.local"
export PATH="$HOME/.local/bin:$PATH"
logline-lab --version
logline-lab --help
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

## Current status

Implemented:

- Act JSON validation.
- Local Lab home init, doctor, and status.
- Local Candidate capture, list, and get.
- Ghost list.
- Daily State report generation.
- Pack/profile selection.
- CLI help and version output.

Ghosted / future:

- Remote spine configuration and writes.
- Supabase ingest and environment verification.
- Evidence registry.
- Receipt closure.
- Interactive Lab surface.
- LLM translator.
- YAML Act parser.
- Passkey checkpointing and batch signing.
- Personal Offline adapters.
- Selective disclosure.

## First local Candidate capture

```bash
logline-lab init --home ./demo-lab --pack santo-andre --profile local-offline
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab candidate add --home ./demo-lab --file examples/acts/minimal.act.json
logline-lab candidate list --home ./demo-lab
```

Candidate capture is local operational capture. It validates canonical Act shape and writes a local Candidate record; it does not admit to a remote spine or close a receipt.

## First Daily State report

```bash
logline-lab report generate daily-state --home ./demo-lab
logline-lab status --home ./demo-lab
```

Daily State is a local report/projection over the workspace. Reports do not close receipts, do not prove evidence, and do not create official state.

## More docs

- `docs/04-cli.md` — command matrix and authority boundaries.
- `docs/05-install.md` — install methods and troubleshooting.
- `reports/GHOSTS.md` — current Ghost inventory.
