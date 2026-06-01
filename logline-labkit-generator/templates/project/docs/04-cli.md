# CLI

The binary is `logline-lab`. Commands return implemented, partial, ghost, or unverified status text.

## Local Lab home commands

A local Lab home is an operational workspace. It is not canon, not an official spine, and not a receipt store.

Default behavior uses the current directory as the Lab home. You can pass an explicit path:

```sh
logline-lab init --home .
logline-lab doctor --home .
logline-lab status --home .
```

`logline-lab init` creates `.logline-lab/` with an editable `lab.manifest.yaml`, `STATUS.md`, `GHOSTS.md`, and local operational directories for candidates, reports, ghosts, profiles, and packs. Init is idempotent and does not overwrite existing manifest/status/ghost files.

`logline-lab doctor` checks the local home structure, verifies `.logline-lab/candidates/` exists, and checks required generated project docs, examples, and schemas. An empty local candidate queue is healthy. Doctor returns non-zero when required local structure is missing.

`logline-lab status` reads the local workspace state, includes `candidate_count`, reports `local_candidate_queue: available` when initialized, lists Ghost records, includes report count/latest report when present, and reports remote spine, evidence registry, receipt closure, interactive UX, YAML parsing, and LLM translator surfaces as ghosted or unimplemented.

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

The local candidate queue is a local capture queue and local workspace record only. It does not admit an Act to any remote spine, does not close receipts, does not prove truth, and is not remote synced.

## Ghost list and report commands

Ghosts preserve unresolved local state without making unresolved work fatal by default:

```sh
logline-lab ghost list --home .
```

`logline-lab ghost list [--home <path>]` requires an initialized Lab home, reads `.logline-lab/GHOSTS.md` and local `.logline-lab/ghosts/` entries, and prints the current local Ghost keys. Its authority is a local workspace Ghost list only.

Daily State is a local report/projection over the workspace:

```sh
logline-lab report generate daily-state --home .
```

`logline-lab report generate daily-state [--home <path>]` requires an initialized Lab home, counts local Candidates, reads local Ghosts, and writes `.logline-lab/reports/daily-state.md`. Reports are local read models and operator reports. Reports do not close receipts, do not prove evidence, do not write remote state, and do not create official truth.

## Act commands

`logline-lab act validate --file <path>` validates a JSON LogLine Act against the nine-slot shape.

`logline-lab act emit --file <path>` validates the Act and returns a preview-only message. It does not write remote state and does not close a receipt.

## Ghost commands

`logline-lab ghost list` is implemented for local Ghost visibility. `logline-lab lab` and `logline-lab chat` remain Ghosts in this generated kit.
