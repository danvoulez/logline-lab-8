# Ghosts

- remote-spine-unconfigured
- evidence-registry-unimplemented
- receipt-closure-unimplemented
- interactive-lab-surface-unimplemented
- llm-translator-unimplemented
- yaml-act-parser-unimplemented
- supabase-ingest-unimplemented
- supabase-env-unverified
- passkey-checkpointing-unimplemented
- batch-signing-unimplemented
- personal-adapters-unimplemented
- selective-disclosure-unimplemented

## Closed in this project state

- Implemented: local Lab home init creates `.logline-lab/` workspace files and directories.
- Implemented: local Lab home doctor checks required local and generated project paths.
- Implemented: local Lab home status reports workspace state, candidate_count, and remaining Ghosts.
- Implemented: local Candidate add/list/get captures validated JSON Acts into the local candidate queue only.

- Implemented: local Ghost listing reads `.logline-lab/GHOSTS.md` and `.logline-lab/ghosts/`.
- Implemented: Daily State report generation writes a local workspace projection under `.logline-lab/reports/`.

## Pack/profile catalog state

- Implemented: pack/profile manifests exist for `santo-andre`, `personal-offline`, `local-offline`, and `supabase`.
- Implemented: init validates selected pack/profile ids and materializes them in `.logline-lab/lab.manifest.yaml`.
- Ghost: Supabase profile capabilities are declarations only; no Supabase ingest or environment verification runs in this project state.
- Ghost: Personal Offline passkey checkpointing, batch signing, adapters, and selective disclosure are declared only.
