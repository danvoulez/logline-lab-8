# Repository Hygiene

This repository keeps durable source, generator, documentation, and policy files in git. It does not keep transient Codex dropzone files, binary corpus archives, or generated build output as source authority.

## Corpus storage

- Corpus `.zip` archives are not committed.
- The approved corpus is stored as text under `logline-labkit-generator/corpus/v1.1/`.
- The corpus SHA is recorded as text in `logline-labkit-generator/corpus/SHA256.txt`.

## Root files

Root-level Codex input files should not be kept as product authority, canon, implementation receipts, or generated output. Historical bootstrap materials, when preserved, belong under `logline-labkit-generator/corpus/bootstrap/` with their non-authoritative status stated explicitly.

## Immutable and generated areas

- `vendor/` is immutable. Treat vendored canon/reference snapshots as read-only.
- `dist/` is generated output and is ignored by repository policy.
