You are Codex Online operating on the LogLine Lab Kit project.

You are not the architect.
You are not changing canon.
You are not improving the system conceptually.
You are implementing a generation pack from an approved planning corpus.

## Inputs

Required input:

* codex-input/logline-lab-kit-corpus-v1.1.zip

Optional salvage input:

* codex-input/raw/Archive(1).zip

The approved corpus is the source pack for this task.

Archive(1).zip, if present, is salvage input only. It is not the base repository.

## Human Approval

Dan Amarilho has approved logline-lab-kit-corpus-v1.1.zip as the current planning corpus and source pack.

Approval scope:

* product direction
* Act-centered model
* authority boundaries
* packs/profiles semantics
* Santo André / Personal Offline framing
* recovery protocol
* implementation blueprint direction

Not approved as:

* implementation receipt
* build/test proof
* production readiness
* Foundation canon amendment
* upstream vendor canon edit

## Primary Task

Create:

logline-labkit-generator/

This generator must produce:

dist/logline-lab-kit/

The generated project must be an installable, CLI-first LogLine Lab Kit.

Do not directly hand-build dist/logline-lab-kit/ as the primary artifact. The generation pack is the main product of this task.

## Conceptual Rules

The current model is:

LogLine Act is the semantic unit.
Canon stays small.
Packs and profiles carry local practice.
Labs run canon + pack/profile.
Projections qualify readings.
Runtimes observe/execute.
LLMs are scribes/translators, not authority.

Do not promote any of the following to canon:

* Supabase
* Santo André
* Personal Offline Lab
* runtime envelope
* selected_branch
* hashes
* runtime_id
* type hints
* passkey checkpoints
* artifacts
* primitive system framing

## Act Rule

The LogLine Act canon has nine slots:

who
did
this
when
confirmed_by
if_ok
if_doubt
if_not
status

selected_branch is not an Act slot.

It may exist only as:

* metadata
* projection output
* admission/runtime metadata
* receipt/practice materialization
* pack/profile-specific output

Runtime envelope is not a tenth semantic slot.
It is provenance metadata or pack/profile practice.

## Candidate Rule

Capture first. Improve legibility later.

Ugly Candidates may be valid records of ugly events.

Projections qualify readings; they do not rewrite the original Act.

Strictness belongs at promotion, execution, receipt, protected action, and closure boundaries — not at the capture door.

## Pack/Profile Rule

Canon loads.
Pack interprets.
Profile provides capabilities.
Lab runs.
Projection reads.
Runtime observes/executes.

Santo André is Dan's recommended/reference practice pack.
It is not an official pack.

Personal Offline Lab is also a full LogLine Lab with its own pack/profile.

Supabase is the v0/Santo André/Supabase-profile online spine.
It is not a universal rule for every LogLine Lab.

A Personal Offline Lab may use a local/offline spine with queued batches and passkey-signed checkpoints.

## UX Rule

The generated project must be CLI-installable and scriptable first.

It should be future interactive/LLM-UX-ready, but it does not need to implement the full TUI/LLM experience in the first generation.

Reserve future commands:

logline-lab lab
logline-lab chat

These commands may initially return explicit Ghosts:

interactive-lab-surface-unimplemented
llm-translator-unimplemented

Do not build a web dashboard.
Do not require an LLM provider for basic operation.
Do not let the LLM admit Acts, close receipts, edit canon, or execute protected actions.

## No Drift Rules

No architecture invention.
No canon amendment.
No vendor edits.
No authority upgrade.
No generated "DECIDED" or "Accepted" architecture decisions.
No Supabase as universal canon.
No Santo André as official pack.
No selected_branch as Act slot.
No runtime envelope as Act slot.
No artifact as positive semantic category.
No primitive system framing.
No local SQLite truth.
No file/JSON/JSONL ledger.
No receipt without evidence.
No tests-passed claim without actual output.

If missing: create Ghost.
If partial: mark partial.
If unverified: mark unverified.
If unsafe: reject or ghost.

## Vendor Rule

vendor/ is immutable.

Upstream canon/reference material must be treated as read-only snapshots.

Any desired canon change must be emitted as a proposal, not as an edit.

Allowed destinations for local interpretation:

packs/
profiles/
proposals/
docs/questions/
docs/notes/

Any generated diff under vendor/ is a failed run.

## Artifact Rule

Do not use artifact as a positive semantic category.

If you need to describe files, outputs, blobs, hashes, or exports, use:

* file
* blob
* output
* raw file
* content hash
* file hash
* debug output
* export
* stdout preview
* raw path without semantic validity
* file referenced by an Act/evidence path

The term artifact may appear only when describing rejected historical material, such as:

* removed artifact spool
* rejected artifact cleanup queue
* old artifact crate removed

## Expected Generator Structure

Create:

logline-labkit-generator/
  README.md
  generator.manifest.yaml
  corpus/
    logline-lab-kit-corpus-v1.1.zip
    APPROVAL_NOTE.md
  blueprints/
    product.blueprint.yaml
    act-model.blueprint.yaml
    authority.blueprint.yaml
    packs-and-profiles.blueprint.yaml
    cli.commands.yaml
    docs.index.yaml
    schemas.index.yaml
    package.santo-andre.yaml
    package.personal-offline.yaml
    recovery.rules.yaml
  templates/
    README.md.tmpl
    Cargo.toml.tmpl
    install.sh.tmpl
    rust-toolchain.toml.tmpl
    crates/
    docs/
    schemas/
    manifests/
    profiles/
    packages/
    examples/
    scripts/
  generator/
    generate.py
    validate.py
    render.py
    scan.py
  acceptance/
    expected-tree.yaml
    command-matrix.expected.yaml
    forbidden-markers.yaml
    smoke-local.expected.yaml
    no-vendor-edits.yaml
  salvage/
    archive-map.yaml
    reusable-material.yaml
    rejected-material.yaml
  reports/
    GENERATION_PLAN.md
    SALVAGE_REPORT.md
    FORBIDDEN_MARKER_SCAN.md
    COMMAND_MATRIX.md
    GHOSTS.md

## Expected Generated Project Structure

The generator must produce:

dist/logline-lab-kit/
  README.md
  LICENSE
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  install.sh
  .env.example
  crates/
    logline-act/
    logline-lab-core/
    logline-lab-cli/
  docs/
    00-overview.md
    01-logline-act.md
    02-product.md
    03-packs-and-profiles.md
    04-cli.md
    05-install.md
    06-recovery.md
    07-interactive-ux.md
    08-llm-boundary.md
  schemas/
    logline-act.schema.json
    lab-manifest.schema.json
    pack-manifest.schema.json
    profile.schema.json
  manifests/
    lab.manifest.example.yaml
    santo-andre.manifest.example.yaml
    personal-offline.manifest.example.yaml
  profiles/
    supabase.profile.yaml
    local-offline.profile.yaml
  packages/
    santo-andre/
      package.yaml
      docs/
      benches/
      reports/
      examples/
    personal-offline/
      package.yaml
      docs/
      adapters/
      batch-signing/
      reports/
      examples/
  examples/
    acts/
    candidates/
    projections/
  scripts/
    smoke-local.sh
    scan-forbidden-markers.sh
    command-matrix.sh
  reports/
    GHOSTS.md
    COMMAND_MATRIX.md
    FORBIDDEN_MARKER_SCAN.md

## CLI Requirements

The generated project must expose an installable CLI binary named:

logline-lab

Minimum commands:

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

## Salvage Rules

If Archive(1).zip is present, inspect it only for reusable material.

Do not use it as the base repository.

Any reused file or content must be listed in:

logline-labkit-generator/reports/SALVAGE_REPORT.md

With:

* original path
* new path
* reason reused
* modifications applied
* authority status
* whether human review is required

Do not reuse:

* edited vendor canon
* docs/decisions as authority
* docs/inventory as authority
* target/
* .claude/
* __MACOSX/
* .DS_Store
* old README claims as authority
* FileLabStore official path
* artifact spool
* local SQLite truth/ledger language

## Acceptance Gates

Create and run scanners where feasible.

At minimum, produce reports for:

FORBIDDEN_MARKER_SCAN.md
COMMAND_MATRIX.md
GHOSTS.md
SALVAGE_REPORT.md
GENERATION_PLAN.md

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

## Build/Test Rule

Do not claim build success unless you actually run build commands and include output.

Do not claim tests passed unless tests actually run and output is captured.

If toolchain is unavailable, write:

Ghost: toolchain-unavailable

If commands are generated but not run, write:

unverified

## Final Output

At the end, provide:

logline-labkit-generator/
dist/logline-lab-kit/
reports/ summary

And a short final note listing:

* files created
* commands run
* commands not run
* ghosts remaining
* whether any salvage material was used
* whether any vendor diff occurred

Remember:

No drift.
No architecture invention.
No authority upgrade.
Compile the approved corpus into a generator.
