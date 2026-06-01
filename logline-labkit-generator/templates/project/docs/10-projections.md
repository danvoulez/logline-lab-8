# Projections

A projection is a local projection: a local read model over current workspace state. It is a derived view, a regenerated summary, and an operator-facing summary for a Lab home.

A projection is not canon. It does not replace Candidate records, does not close receipts, does not prove evidence, does not amend canon, and does not write or sync remote state. Projections can be deleted and regenerated from the local workspace state they summarize.

## Local projection directory

`logline-lab init --home <path>` creates:

```text
<lab-home>/.logline-lab/projections/
```

The generated kit also maintains `.logline-lab/projections/projection-index.json` as local read-model metadata. The index is authority-limited metadata for generated local projections. It is not canon, not a receipt, not evidence, and not remote sync.

## Available projections

The first generated projection is `local-summary`.

```sh
logline-lab projection list --home ./demo-lab
```

Before any projection is generated, the list command reports zero generated projections and shows `local-summary` as an available projection kind. After generation, it shows the local path and available state.

`projection list` requires an initialized Lab home and returns non-zero for an uninitialized path.

## Local summary projection

Generate the first workspace projection with:

```sh
logline-lab projection generate local-summary --home ./demo-lab
```

The command writes:

```text
<lab-home>/.logline-lab/projections/local-summary.md
```

The local summary projection reads existing local workspace state:

- selected pack/profile;
- Candidate count;
- Candidate index state;
- Ghost count and Ghost list;
- available local reports and latest local report;
- profile capability state;
- available local projections.

The generated Markdown states its authority boundary explicitly: it is a local read model, a derived view, a regenerated summary, and an operator-facing summary only.

## Relationship to Candidates and reports

Candidate records remain local capture records. The local summary projection reads them; it does not replace them.

Daily State remains a local report/projection under `.logline-lab/reports/`. The local summary projection is the named Projection-layer read model under `.logline-lab/projections/`.

Both are local workspace read-side views. They do not close receipts, do not prove evidence, do not amend canon, and do not write remote state.
