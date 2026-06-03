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
logline-lab serve
logline-lab status --home ./demo-lab
```

`serve` starts the local browser product at `http://127.0.0.1:8787`. The browser wizard asks for a Lab home, pack, and profile, then creates the local Lab, validates the starter Act, captures one local Candidate, writes the Daily State report, generates the local-summary projection, lists Ghosts, and prints the next commands. For terminals and automation, `logline-lab setup` runs the same setup flow.

The initialized home is local workspace state only. It is not an official spine, not a receipt store, not evidence proof, not remote sync, and not an LLM authority layer.

## Current status

Implemented:

- Act JSON validation.
- Local Lab home init, doctor, and status.
- First-run setup wizard.
- Local browser product with setup API.
- Supabase baseline for `ops.logline_acts`, `ops.ingest_logline_act(payload jsonb)`, and PGMQ queues.
- Supabase configuration check.
- Remote Act emit through the Supabase spine when configured.
- Local Candidate capture, list, and get.
- Ghost list.
- Daily State report generation.
- Local projection list and local-summary generation.
- Pack/profile selection.
- CLI help and version output.

Ghosted / future:

- Supabase project configuration when env vars or migrations are absent.
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

## Supabase spine profile

The Supabase profile ships the generic Lab Kit base spine at `supabase/migrations/0001_ops_logline_acts.sql`. This baseline follows the Santo Andre reference spine shape, but it is not the `santo-andre` practice pack. Apply it with the Supabase CLI or your reviewed project migration flow, then provide server-side env:

```bash
export DATABASE_URL=...
```

`DATABASE_URL` is preferred because it lets the CLI call private `ops` functions and queue-backed database features directly. REST env is supported only for projects that expose compatible RPC wrappers:

```bash
export SUPABASE_URL=...
export SUPABASE_SERVICE_ROLE_KEY=...
# or Doppler Santo Andre:
export SUPABASE_SECRET_KEY=...
```

Then:

```bash
logline-lab supabase check
logline-lab act emit --file examples/acts/minimal.act.json --remote
```

Remote emit writes only through `ops.ingest_logline_act(payload jsonb)` into `ops.logline_acts`. It does not write projection tables directly, close receipts, or prove evidence.

## First Daily State report and local summary projection

```bash
logline-lab report generate daily-state --home ./demo-lab
logline-lab projection generate local-summary --home ./demo-lab
logline-lab status --home ./demo-lab
```

Daily State is a local report/projection over the workspace. The local summary projection is a regenerated local read model under `.logline-lab/projections/`. These read-side views do not close receipts, do not prove evidence, and do not create official state.

## More docs

- `docs/04-cli.md` — command matrix and authority boundaries.
- `docs/05-install.md` — install methods and troubleshooting.
- `docs/10-projections.md` — local projection contract and local-summary behavior.
- `reports/GHOSTS.md` — current Ghost inventory.
