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
logline-lab init --home . --pack santo-andre --profile local-offline
logline-lab doctor --home .
logline-lab status --home .
```

`logline-lab init [--home <path>] [--pack <id>] [--profile <id>]` validates the selected pack/profile against the initial local catalog, creates `.logline-lab/` with an editable `lab.manifest.yaml`, `STATUS.md`, `GHOSTS.md`, and local operational directories for candidates, reports, ghosts, profiles, and packs. Defaults are `--pack santo-andre --profile local-offline`. Init is idempotent and does not overwrite existing manifest/status/ghost files.

`logline-lab doctor [--home <path>]` checks the local home structure, validates selected pack/profile ids from the manifest when present, verifies `.logline-lab/candidates/` exists, and checks required generated project docs, examples, and schemas. An empty local candidate queue is healthy. Supabase profile declarations are reported as Ghost/unconfigured rather than requiring env vars in this PR state. Doctor returns non-zero when required local structure is missing or a selected pack/profile id is unknown.

`logline-lab status [--home <path>]` reads the local workspace state, shows selected pack/profile, includes `candidate_count`, reports `local_candidate_queue: available` when initialized, lists Ghost records, includes report count/latest report when present, and reports profile capability state plus remote spine, evidence registry, receipt closure, interactive UX, YAML parsing, and LLM translator surfaces as ghosted or unimplemented.

## Candidate commands

Candidate capture is local operational capture for the first capture loop:

```sh
logline-lab candidate add --home . --file examples/acts/minimal.act.json
logline-lab candidate list --home .
logline-lab candidate get <candidate_id> --home .
```

`logline-lab candidate add --file <path> [--home <path>]` requires an initialized Lab home and validates the input file with the canonical nine-slot JSON Act validator. Invalid Acts are rejected before any Candidate record is created. Valid input is copied unchanged into `.logline-lab/candidates/<candidate_id>/candidate.json`, with lightweight metadata in `metadata.json`.

`logline-lab candidate list [--home <path>]` reads only `.logline-lab/candidates/` and reports count, ids, timestamps, and Candidate status where metadata is available.

`logline-lab candidate get <candidate_id> [--home <path>]` prints metadata and the captured Candidate content. Missing Candidates return non-zero with `candidate not found: <candidate_id>`.

The local candidate queue is a local capture queue and local workspace record only. It does not admit an Act to any remote spine, does not close receipts, does not prove evidence, and is not remote synced.

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

`logline-lab report generate daily-state [--home <path>]` requires an initialized Lab home, includes selected pack/profile and profile capability state, counts local Candidates, reads local Ghosts, and writes `.logline-lab/reports/daily-state.md`. Reports are local read models and operator reports. Reports do not close receipts, do not prove evidence, do not write remote state, and do not create official state.

## Act commands

`logline-lab act validate --file <path>` validates a JSON LogLine Act against the nine-slot shape. Validation is local shape validation only.

`logline-lab act emit --file <path>` validates the Act and returns a preview-only message. It does not write remote state and does not close a receipt.

## Ghost commands

`logline-lab ghost list` is implemented for local Ghost visibility.

`logline-lab lab` returns `interactive-lab-surface-unimplemented`.

`logline-lab chat` returns `llm-translator-unimplemented`; no LLM provider is configured or authoritative.
