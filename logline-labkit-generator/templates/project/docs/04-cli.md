# CLI

The binary is `logline-lab`. The generated kit is CLI-first. Commands return implemented, partial, Ghost, or unverified status text and preserve local authority boundaries.

```sh
logline-lab --version
logline-lab --help
```

`--version` prints the generated kit version from `VERSION`. `--help` lists the main command groups and states that `local-offline` works without Supabase or external services.

## Local Lab home commands

A local Lab home is an operational workspace. It is not canon, not an official spine, and not a receipt store.

```sh
logline-lab serve
logline-lab setup
logline-lab init --home . --pack santo-andre --profile local-offline
logline-lab doctor --home .
logline-lab status --home .
```

`logline-lab serve [--host <ip>] [--port <port>]` starts the local browser product. Default address is `http://127.0.0.1:8787`. The browser UI provides a first-run Lab wizard and calls local APIs served by the installed binary. It does not require a cloud service and does not create official spine, receipt, evidence proof, remote sync, or LLM authority.

`logline-lab setup [--home <path>] [--pack <id>] [--profile <id>] [--yes]` is the first-run wizard for a human operator. Without `--yes`, it prompts for Lab home, pack, and profile. It creates the local Lab, validates the starter Act, captures one local Candidate, writes the Daily State report, generates the local-summary projection, lists Ghosts, and prints next commands. `--yes` accepts defaults/flags for automated installs and tests.

Setup creates local workspace state only. It is not an official spine, not a receipt store, not evidence proof, not remote sync, and not an LLM authority layer.

`logline-lab init [--home <path>] [--pack <id>] [--profile <id>]` validates the selected pack/profile against the initial local catalog, creates `.logline-lab/` with an editable `lab.manifest.yaml`, `STATUS.md`, `GHOSTS.md`, and local operational directories for candidates, reports, ghosts, profiles, packs, and projections. It also creates an empty `.logline-lab/candidates/index.json` local Candidate index and `.logline-lab/projections/projection-index.json` local projection metadata. Defaults are `--pack santo-andre --profile local-offline`. Init is idempotent and does not overwrite existing manifest/status/ghost files.

`logline-lab doctor [--home <path>]` checks the local home structure, validates selected pack/profile ids from the manifest when present, verifies `.logline-lab/candidates/` exists, checks the local Candidate index is parseable and consistent, and checks required generated project docs, examples, and schemas. An empty local candidate queue is healthy. Supabase profile declarations are reported as Ghost/unconfigured rather than requiring env vars in this PR state. Doctor returns non-zero when required local structure is missing, a selected pack/profile id is unknown, or the candidate index is malformed/inconsistent. Missing candidate index files are reported as warnings with the rebuild command.

`logline-lab status [--home <path>]` reads the local workspace state, shows selected pack/profile, includes `candidate_count` and `candidate_index`, reports `local_candidate_queue: available` when initialized, lists Ghost records, includes report count/latest report and projection count/latest projection when present, and reports profile capability state plus remote spine, evidence registry, receipt closure, interactive UX, YAML parsing, and LLM translator surfaces as ghosted or unimplemented.

## Candidate commands

Candidate capture is local operational capture for the first capture loop:

```sh
logline-lab candidate add --home . --file examples/acts/minimal.act.json
logline-lab candidate list --home .
logline-lab candidate get <candidate_id> --home .
logline-lab candidate index rebuild --home .
```

`logline-lab candidate add --file <path> [--home <path>]` requires an initialized Lab home and validates the input file with the canonical nine-slot JSON Act validator. Invalid Acts are rejected before any Candidate record is created. Valid input is copied unchanged into `.logline-lab/candidates/<candidate_id>/candidate.json`, with lightweight metadata in `metadata.json`, then `.logline-lab/candidates/index.json` is updated atomically as a local operational index.

`logline-lab candidate list [--home <path>]` reads `.logline-lab/candidates/index.json` when available and valid, reports `index: available`, and still verifies indexed candidate files exist. If the index is missing, list rebuilds the local Candidate index from candidate directories and reports `index: rebuilt`. If the index is malformed, list returns non-zero and reports the malformed index instead of pretending the queue is healthy.

`logline-lab candidate get <candidate_id> [--home <path>]` prints metadata and the captured Candidate content. It may use the local Candidate index to locate files, but still verifies the candidate and metadata files exist. Missing Candidates return non-zero with `candidate not found: <candidate_id>`.

`logline-lab candidate index rebuild [--home <path>]` scans local candidate directories and rewrites `.logline-lab/candidates/index.json` as queue metadata only.

The Candidate index is local operational metadata used for listing/status/reporting. It is not a ledger, official spine, receipt, evidence, or source of truth. The local candidate queue is a local capture queue and local workspace record only. It does not admit an Act to any remote spine, does not close receipts, does not prove evidence, and is not remote synced.

## Ghost list and report commands

Ghosts preserve unresolved local state without making unresolved work fatal by default:

```sh
logline-lab ghost list --home .
```

`logline-lab ghost list [--home <path>]` requires an initialized Lab home, reads `.logline-lab/GHOSTS.md` and local `.logline-lab/ghosts/` entries, and prints the current local Ghost keys. Its authority is a local workspace Ghost list only; it is not evidence proof.

Daily State is a local report/projection over the workspace:

```sh
logline-lab report generate daily-state --home .
```

`logline-lab report generate daily-state [--home <path>]` requires an initialized Lab home, includes selected pack/profile and profile capability state, counts local Candidates through the local Candidate index when available, includes Candidate Index state/list details, reads local Ghosts, and writes `.logline-lab/reports/daily-state.md`. Reports are local read models and operator reports. Reports do not close receipts, do not prove evidence, do not write remote state, and do not create official state.


## Projection commands

Projections are local read models over workspace state. They are derived views and regenerated summaries for operators. They are not canon, not receipts, not evidence, and not remote sync.

```sh
logline-lab projection list --home .
logline-lab projection generate local-summary --home .
```

`logline-lab projection list [--home <path>]` requires an initialized Lab home. It lists generated local projections, initially recognizes the `local-summary` projection kind, and shows the generated path/state when `.logline-lab/projections/local-summary.md` exists.

`logline-lab projection generate local-summary [--home <path>]` requires an initialized Lab home, reads selected pack/profile, Candidate count and index state, Ghosts, reports, projection metadata, and profile capabilities, then writes `.logline-lab/projections/local-summary.md`. It also updates `.logline-lab/projections/projection-index.json` as local read-model metadata only.

## Act commands

`logline-lab act validate --file <path>` validates a JSON LogLine Act against the nine-slot shape. Validation is local shape validation only.

The Rust validator is authoritative for valid/invalid Act shape in the CLI. The JSON Schemas in `schemas/` document generated file shapes and interoperability contracts; the CLI does not replace the Rust validator with runtime JSON Schema validation in this release. `examples/fixtures.index.md` lists valid and invalid fixtures with expected results.

`logline-lab act emit --file <path>` validates the Act and returns a preview-only message. It does not write remote state and does not close a receipt.

`logline-lab act emit --file <path> --remote` validates the Act, computes the tuple hash, and calls `ops.ingest_logline_act` through the configured Supabase REST/RPC boundary. The only semantic write target is `ops.logline_acts`. It is not a receipt and not evidence.

`logline-lab supabase check` verifies that `SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY` are configured and that `ops.logline_acts` is reachable through the Supabase API. It does not write.

## Ghost commands

`logline-lab ghost list` is implemented for local Ghost visibility.

`logline-lab lab` returns `interactive-lab-surface-unimplemented`.

`logline-lab chat` returns `llm-translator-unimplemented`; no LLM provider is configured or authoritative.
