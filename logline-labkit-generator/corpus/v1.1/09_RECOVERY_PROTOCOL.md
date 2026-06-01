# 09_RECOVERY_PROTOCOL.md

## Recovery Protocol

Status: recovery scope, not a release.
Input base: Archive(1).zip.
Do not use prior assistant-generated zips as base.

## Purpose

Recover the LogLine Lab Kit that is already present inside the archive.

The task is not to invent a new product, not to write a new canon, not to make a minimal demo, and not to clean files by taste.

The task is to extract the real Lab product from a contaminated corpus, remove false authority, restore the intended product shape, and produce an installable kit whose claims match its actual implementation.

## Hard Constraints from Dan

These constraints override generated documents, README claims, ADRs, inventory files, and any assistant plan.

```txt
Official LogLine Act storage = online database.
Official semantic spine = online SQL / Supabase / Postgres, specifically ops.logline_acts where implemented.
SQLite = provisional / high-frequency / cache / queue / outbox only.
SQLite is not the official spine.
File storage is not official storage.
JSON is not official storage.
JSONL is not official storage.
There is no official acts/*.json spine.
There is no official receipts/*.json spine.
There is no official ghosts/*.json spine.
Lab definition is not canon.
LLM-generated material is not author, architect, canon, decision, validator, receipt, or source of truth.
Transcripts are not product.
Prior assistant-generated zips are not base.
```

## Storage Profile Correction

Storage é profile-specific, não canon universal.

### Supabase Profile (v0)

Para o profile Supabase:

```txt
Official LogLine Acts live in the online database.
ops.logline_acts é o spine oficial.
```

Esta regra é específica do profile Supabase do LabKit v0.

Não é regra universal de todos os Labs.

### Outros Profiles

Outros profiles podem ter storage diferente:

```txt
local-only: SQLite como buffer/outbox
postgres: Postgres como spine
filesystem-manual: export/debug/manual mode apenas
```

Cada profile define sua própria disciplina de storage.

## Actual Product Definition

The product is:

```txt
LogLine Lab Kit:
  an installable operational Lab kit for instantiating and studying a real LogLine Lab.
```

It is not:

```txt
OpenAI Lab
ChatGPT Lab
Claude Lab
Minilab frontend
Santo André branding package
Dashboard
Transcript package
Canon rewrite
Minimal scaffold
Rust-only ideology
SQLite-first local product
File/JSON spine product
```

The final product must support, or honestly mark as Ghost, the product surface defined in the Lab Kit spec:

```txt
installation
Supabase act spine (v0 profile)
CLI
labd
manifest
runtime registry
projectors
ghosts
evidence
receipts
clock
hooks
dispatch/Hermes boundary
reports
conformance hooks
study benches
first-run Lab experience
```

## Product Experience Requirement

Because this is a Lab, not just infrastructure, the final kit must include an operator experience.

The product must let a user understand and attempt the first Lab flow:

```txt
unzip
install/configure env
run doctor
initialize Lab
emit first official act to online spine
read status from projections
open first ghost
add evidence
prepare receipt candidate
run/report Daily Lab Expedition
explore study benches
```

If a step is not implemented, it must be marked as Ghost. It must not be hidden, renamed, or described as working.

Required experience materials from the spec and archive:

```txt
docs/00-overview.md
docs/01-install.md
docs/02-supabase-spine.md
docs/03-cli.md
docs/04-labd.md
docs/05-manifest.md
docs/06-projectors.md
docs/07-ghosts.md
docs/08-evidence.md
docs/09-receipts.md
docs/10-clock.md
docs/11-hooks.md
docs/12-dispatch-and-hermes.md
docs/13-study-benches.md
docs/14-frontends.md
benches/*
reports/templates/*
examples/*
hooks/default/*
```

## Product Tree Target

The target tree should follow the spec that already exists in the context, not an invented tree.

Target root:

```txt
logline-lab-kit/
```

Target product areas:

```txt
README.md
LICENSE
.env.example
install.sh or equivalent install instructions
Cargo.toml if Rust components are present
package.json only if Node/TS components are actually present

docs/
canon/
profiles/
manifests/
schemas/
supabase/migrations/
cli/ or crates/logline-lab-cli/
labd/ or crates/logline-lab-labd/
projectors/
clock/
hooks/
benches/
reports/templates/
examples/
tests/
vendor/ only where technically required
```

The tree may keep Rust crates if the archive implementation is Rust, but the product documentation must still reflect the Lab Kit spec: CLI/labd/runtime/spine/experience/benches.

## Corpus Classification

### Keep as Product Candidates

```txt
lab-kit/crates/logline-lab-cli/
lab-kit/crates/logline-lab-core/
lab-kit/crates/logline-lab-labd/
lab-kit/crates/logline-lab-hermes/
lab-kit/crates/logline-lab-outbox/
lab-kit/crates/logline-lab-supabase/
lab-kit/crates/logline-lab-local-sql/  # only as provisional cache/outbox, not spine
lab-kit/ops/supabase/migrations/
lab-kit/templates/
lab-kit/examples/
lab-kit/docs/contracts/
lab-kit/docs/experiments/
lab-kit/docs/guides/ after correction
lab-kit/vendor/logline-foundation/ only if used by Cargo/build/conformance
_inbox/LAB-KIT-main/docs/
_inbox/LAB-KIT-main/canon/*.refs.yaml
_inbox/LAB-KIT-main/profiles/
_inbox/LAB-KIT-main/manifests/
_inbox/LAB-KIT-main/schemas/
_inbox/LAB-KIT-main/supabase/migrations/
_inbox/LAB-KIT-main/benches/
_inbox/LAB-KIT-main/reports/templates/
_inbox/LAB-KIT-main/examples/
_inbox/LAB-KIT-main/hooks/
```

### Remove from Final Product

```txt
.claude/
target/
__MACOSX/
.DS_Store
sources/
KIT_MANIFEST.json
docs/inventory/
docs/decisions/
assistant-generated repair files
prior generated manifests/checksums that are not product files
```

### Do Not Delete Blindly

```txt
_inbox/LAB-KIT-main/
```

Reason: it contains product-shaped material that must not be discarded blindly. The Lab product is present but split between the main Rust kit and `_inbox/LAB-KIT-main/`, with contaminating authority layers around it.

## Recovery Steps

### Step 1: Extract Archive

Extract Archive(1).zip to working directory.

Verify contents match expected structure:

```txt
lab-kit/
logline_lab_kit_rust/
_inbox/LAB-KIT-main/
```

### Step 2: Classify Corpus

Separate files into:

- Product candidates (keep)
- Remove from final product (delete)
- Manual review needed (inspect)

### Step 3: Remove Debris

Delete files marked as "remove from final product":

```txt
.claude/
target/
__MACOSX/
.DS_Store
sources/
KIT_MANIFEST.json
docs/inventory/
docs/decisions/
assistant-generated repair files
prior generated manifests/checksums
```

### Step 4: Merge Product Material

Merge product-shaped material from:

```txt
lab-kit/
_inbox/LAB-KIT-main/
```

Into unified tree structure.

### Step 5: Correct Authority

Remove false authority claims:

- LLM-generated material marked as canon
- Assistant-generated ADRs marked as decisions
- Transcripts marked as product
- Local practice marked as Foundation canon

### Step 6: Correct Storage Language

Replace incorrect storage language:

- "SQLite ledger" → "SQLite provisional cache/outbox"
- "local JSON spine" → "local JSON examples/fixtures only"
- "file spine" → "file examples/fixtures only"
- "official acts/*.json" → remove or mark as non-official

### Step 7: Mark Ghosts

Mark unimplemented features as Ghosts:

```txt
lab-manifest-schema-unformalized
runtime-envelope-practice-unformalized
runtime-registry-projector-unimplemented
ghost-registry-template-unimplemented
receipt-template-unimplemented
interval-semantics-unformalized
standards-adapter-map-unimplemented
santo-andre-first-lab-manifest-missing
lab-clock-implementation-missing
outbox-supabase-bridge-missing
labd-serve-missing
experiment-ux-unimplemented
translator-unimplemented
```

### Step 8: Verify Product Shape

Verify final tree matches target structure:

```txt
logline-lab-kit/
  README.md
  Cargo.toml
  docs/
  canon/
  profiles/
  manifests/
  schemas/
  supabase/migrations/
  crates/
  templates/
  examples/
  benches/
  reports/templates/
  hooks/
```

### Step 9: Verify Experience

Verify first Lab flow is possible:

```txt
unzip
install/configure env
run doctor
initialize Lab
emit first official act to online spine
read status from projections
open first ghost
add evidence
prepare receipt candidate
run/report Daily Lab Expedition
explore study benches
```

### Step 10: Generate Recovery Receipt

Generate recovery receipt documenting:

- What was recovered
- What was removed
- What was marked as Ghost
- What was corrected
- What remains unimplemented

## Recovery Receipt

Recovery receipt must include:

```txt
archive_source: Archive(1).zip
recovery_date: <timestamp>
product_version: v0
storage_profile: Supabase (v0)
recovered_components:
  - CLI
  - labd
  - Supabase spine
  - Projections
  - Ghosts
  - Evidence
  - Receipts
  - Clock
  - Hooks
  - Reports
  - Benches
removed_components:
  - .claude/
  - target/
  - __MACOSX/
  - .DS_Store
  - sources/
  - KIT_MANIFEST.json
  - docs/inventory/
  - docs/decisions/
  - assistant-generated repair files
  - prior generated manifests/checksums
ghosts_marked:
  - lab-manifest-schema-unformalized
  - runtime-envelope-practice-unformalized
  - runtime-registry-projector-unimplemented
  - ghost-registry-template-unimplemented
  - receipt-template-unimplemented
  - interval-semantics-unformalized
  - standards-adapter-map-unimplemented
  - santo-andre-first-lab-manifest-missing
  - lab-clock-implementation-missing
  - outbox-supabase-bridge-missing
  - labd-serve-missing
  - experiment-ux-unimplemented
  - translator-unimplemented
corrections_applied:
  - storage language corrected
  - false authority removed
  - LLM-generated material marked as non-canon
  - local practice marked as non-Foundation
storage_note: Supabase spine is v0 profile, not universal rule
```

## Recovery vs Release

Recovery is not release.

Recovery restores existing product from contaminated corpus.

Release is public distribution of verified product.

This protocol is recovery only.

## LLM Non-Authority

LLM-generated material in this recovery protocol is not:

- Canon
- Architecture decision
- Proof
- Receipt
- Validation
- Project ownership

This recovery protocol is itself LLM-generated material and must be treated as historical/contextual, not product authority.