# No-Drift Review Guide

Use this guide to stop PRs from silently changing authority, canon, pack/profile boundaries, generated output meaning, or the semantic model.

## Reviewer stance

Block surprise architecture changes. Require explicit scope, source authority, generated-output explanation, and command output for validation claims.

## Red flags

Block or request clarification when any of these appears without an approved source and explicit authority note:

- `vendor/` changed.
- Canon amended.
- `DECIDED` or `Accepted` introduced without Dan approval.
- Supabase described as universal canon.
- Santo André described as official pack.
- `selected_branch` treated as Act slot.
- Runtime envelope treated as Act slot.
- `artifact` used as positive category.
- Primitive system framing returns.
- SQLite described as truth/ledger.
- File/JSON/JSONL described as official storage.
- Projection described as source of truth.
- LLM output described as evidence/receipt/authority.
- Receipt without evidence.
- Tests passed without command output.

## Review actions

Choose the smallest action that prevents drift:

- **Block** when the PR amends canon, edits vendor material, upgrades authority, or introduces unsafe semantics without approval.
- **Request Ghost** when behavior is missing, partial, unsafe, or unverified.
- **Request source template change** when generated output was hand-edited but the repository generator owns the file.
- **Request generated output regeneration** when templates or blueprints changed and the generated snapshot is expected to track them.
- **Request authority note** when a change could affect canon, packs, profiles, generated output meaning, or semantic categories.
- **Request corpus update** only through the approved corpus/governance path; do not let implementation PRs amend canon by implication.
