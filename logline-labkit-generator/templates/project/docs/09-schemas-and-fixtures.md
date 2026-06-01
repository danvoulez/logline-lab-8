# Schemas and Fixtures

This generated kit includes JSON Schemas and fixtures to reduce drift between the local implementation, examples, and documentation.

## Authority boundary

The current CLI uses the Rust validator for Act shape. JSON Schemas are documentation and interoperability contracts for generated project file shapes; they are not runtime authority in this release.

Schemas must not amend canon. They describe files the generated project already reads or writes locally:

- `schemas/logline-act.schema.json` documents the canonical nine-slot JSON Act shape validated by the Rust validator.
- `schemas/lab-manifest.schema.json` documents `.logline-lab/lab.manifest.yaml` as local Lab home metadata, including selected pack/profile.
- `schemas/candidate-metadata.schema.json` documents `.logline-lab/candidates/<candidate_id>/metadata.json` as local operational metadata only.
- `schemas/candidate-index.schema.json` documents `.logline-lab/candidates/index.json` as a local operational queue index only.
- `schemas/pack-manifest.schema.json` documents blueprint-rendered local practice pack manifests.
- `schemas/profile.schema.json` documents blueprint-rendered profile manifests.

There is no runtime JSON Schema validation dependency in this release. Doctor and generator validation may check that schema files exist, while Act acceptance/rejection remains the Rust validator behavior.

## LogLine Act schema

A canonical Act has exactly these nine top-level slots:

1. `who`
2. `did`
3. `this`
4. `when`
5. `confirmed_by`
6. `if_ok`
7. `if_doubt`
8. `if_not`
9. `status`

`schemas/logline-act.schema.json` requires all nine slots and rejects unknown top-level slots with `additionalProperties: false`. Internal values remain flexible so ugly Candidates can still be shape-valid.

The schema intentionally does not include `selected_branch`, `runtime_envelope`, `content_hash`, `runtime_id`, or `type_hint` as Act slots. When those concepts are useful, they belong outside the Act as metadata, projection output, provenance, or pack/profile practice.

## Candidate metadata and index schemas

Candidate metadata and the Candidate index are local operational files under `.logline-lab/candidates/`.

They are not official spine entries, receipts, evidence records, or proof of truth. Candidate capture means the CLI validated the local JSON Act shape and copied it into the local Candidate queue.

The Candidate index uses relative paths such as `cand_.../candidate.json` and `cand_.../metadata.json`. It exists to make local listing/status/reporting fast and inspectable; if missing, the CLI can rebuild it from local Candidate directories.

## Pack/profile schemas

Pack manifests describe local practice packs such as Santo André and Personal Offline. The rendered pack manifests set `pack.official: false`, `authority.canon_amendment: false`, and `authority.official_pack: false`.

Profile manifests describe capability declarations such as Local Offline and Supabase. Supabase is profile-specific and may declare unimplemented/unconfigured remote-spine capability, but it is not universal canon and does not implement ingest in this release.

## Fixture index

`examples/fixtures.index.md` lists valid and invalid fixtures, the expected validator result, expected error kind where applicable, and notes about the authority boundary.

Use these fixtures when checking drift:

```sh
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab act validate --file examples/candidates/ugly-candidate.json
logline-lab act validate --file examples/invalid/missing-confirmed-by.json
logline-lab act validate --file examples/invalid/extra-selected-branch.json
logline-lab act validate --file examples/invalid/extra-runtime-envelope.json
logline-lab act validate --file examples/invalid/extra-top-level-field.json
logline-lab act validate --file examples/invalid/malformed.json
```
