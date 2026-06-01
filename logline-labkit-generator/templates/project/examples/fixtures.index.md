# Fixture Index

The current CLI uses the Rust Act validator for Act shape. JSON Schemas document file shapes and interoperability contracts for generated projects; they are not runtime authority in this release.

| Path | Kind | Expected result | Expected error kind | Notes |
|---|---|---:|---|---|
| `examples/acts/minimal.act.json` | valid_act | valid | n/a | Canonical nine-slot Act with flexible internal values. |
| `examples/candidates/ugly-candidate.json` | valid_candidate | valid | n/a | Ugly but shape-valid Candidate Act; capture first, improve legibility later. |
| `examples/invalid/missing-confirmed-by.json` | invalid_act | invalid | missing_slot | Missing one canonical Act slot. |
| `examples/invalid/extra-selected-branch.json` | invalid_act | invalid | reserved_metadata_slot | `selected_branch` is metadata/projection/practice outside the Act. |
| `examples/invalid/extra-runtime-envelope.json` | invalid_act | invalid | reserved_metadata_slot | `runtime_envelope` is provenance metadata or pack/profile practice outside the Act. |
| `examples/invalid/extra-top-level-field.json` | invalid_act | invalid | unknown_slot | Arbitrary extra top-level fields are rejected by the canonical Act validator. |
| `examples/invalid/malformed.json` | invalid_json | invalid | parse_error | Malformed JSON is rejected before Act shape checks. |
| `examples/candidates/candidate-metadata.json` | valid_candidate_metadata | valid | n/a | Example local operational Candidate metadata, not official admission. |
| `examples/candidates/candidate-index.json` | valid_candidate_index | valid | n/a | Example local operational Candidate queue index with relative paths. |
| `examples/invalid/candidate-index-inconsistent.json` | invalid_candidate_index | invalid | malformed_index | Example index shape with an unsafe relative candidate path. |
