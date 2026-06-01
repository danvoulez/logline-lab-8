# Acceptance Gates

## Required Scanners

Create and run scanners where feasible.

At minimum, produce reports for:

FORBIDDEN_MARKER_SCAN.md
COMMAND_MATRIX.md
GHOSTS.md
SALVAGE_REPORT.md
GENERATION_PLAN.md

## Forbidden Marker Scan

The forbidden marker scan must check for:

* selected_branch as Act slot
* runtime envelope as Act slot
* primitive system framing
* artifact as positive category
* Supabase as universal canon
* Santo André as official pack
* local SQLite truth
* file/JSON/JSONL ledger
* LLM decided
* Accepted/DECIDED without Dan approval
* receipt without evidence
* tests passed without output
* vendor edits

If anything fails and cannot be fixed safely, create a Ghost. Do not hide it.

## Command Matrix

Verify CLI commands are generated correctly:

logline-lab --version
logline-lab init
logline-lab doctor
logline-lab status
logline-lab act validate
logline-lab act emit --file <path>
logline-lab lab
logline-lab chat

Initial behavior may be partial but must be honest.

Allowed initial results:

* implemented
* partial
* ghost
* unverified

Not allowed:

* fake success
* fake receipt
* fake test pass
* fake Supabase connection
* broad claim from narrow evidence

## Build/Test Rule

Do not claim build success unless you actually run build commands and include output.

Do not claim tests passed unless tests actually run and output is captured.

If toolchain is unavailable, write:

Ghost: toolchain-unavailable

If commands are generated but not run, write:

unverified
