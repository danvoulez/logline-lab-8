# Packs and Profiles

Canon loads. Pack interprets. Profile provides capabilities. Lab runs. Projection reads. Runtime observes or executes. LLMs suggest Candidates only.

## Definitions

- **Canon** is the small stable LogLine foundation and Act shape.
- **Pack** is local/domain practice and interpretation. A pack does not amend canon.
- **Profile** is a capability/runtime/storage/environment declaration. A profile does not prove that external capabilities are configured or implemented.
- **Lab** is a concrete initialized workspace using canon plus a selected pack and selected profile.

## Initial local catalog

This generated kit has a small built-in catalog that mirrors the YAML manifests in `packages/` and `profiles/`.

### Packs

- `santo-andre`: Dan's recommended/reference practice pack for experiments, product/runtime practice, and local control-plane oriented work. Santo André is **not official** and does not amend canon.
- `personal-offline`: a private/offline full LogLine Lab pack for longitudinal personal event capture, local Candidate queues, future batch checkpointing, and future selective disclosure. Passkeys, batch signing, personal adapters, and selective disclosure remain Ghosts.

### Profiles

- `local-offline`: safe local workspace profile. It declares local home, Act validation, Candidate capture, Ghost listing, and Daily State report as available. Remote spine, evidence registry, receipt closure, LLM translator, and interactive lab surface remain unavailable/Ghost.
- `supabase`: online spine profile. It provides the generic Lab Kit base Supabase spine for `ops.logline_acts`, `ops.ingest_logline_act(payload jsonb)`, and PGMQ queues. This baseline follows the Santo Andre reference spine shape, but it is not the `santo-andre` practice pack. Missing Supabase environment or unapplied migrations are reported as configuration Ghosts, not as universal canon failure.

## Init selection

Initialize with explicit pack/profile selection:

```sh
logline-lab init --home . --pack santo-andre --profile local-offline
logline-lab init --home . --pack personal-offline --profile local-offline
logline-lab init --home . --pack santo-andre --profile supabase
```

If omitted, init defaults to `--pack santo-andre --profile local-offline` because that is the safe local reference practice for this generated kit.

The selection is materialized in `.logline-lab/lab.manifest.yaml` under `selected:`. Selection means local Lab practice only. It does not make a pack canon, does not make a profile universal, and does not prove external capability.
