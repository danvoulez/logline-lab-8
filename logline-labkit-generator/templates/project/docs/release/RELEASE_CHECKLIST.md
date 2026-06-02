# Release Checklist

This checklist is for local release/readiness review of the generated LogLine Lab Kit. It does not claim release readiness beyond commands actually run and outputs reviewed.

## Command sequence

Run from the repository root unless noted:

1. Generate dist:

   ```bash
   python3 logline-labkit-generator/generator/generate.py
   ```

2. Validate generated output paths:

   ```bash
   python3 logline-labkit-generator/generator/validate.py
   ```

3. Scan forbidden markers:

   ```bash
   cd dist/logline-lab-kit && scripts/scan-forbidden-markers.sh
   ```

4. Refresh command matrix:

   ```bash
   cd dist/logline-lab-kit && scripts/command-matrix.sh
   ```

5. Run Rust tests:

   ```bash
   cd dist/logline-lab-kit && cargo test
   ```

6. Run the local smoke journey:

   ```bash
   cd dist/logline-lab-kit && scripts/smoke-local.sh
   ```

7. Check install script syntax:

   ```bash
   cd dist/logline-lab-kit && bash -n install.sh
   ```

8. Check vendor diff is empty:

   ```bash
   git diff --name-only -- vendor
   ```

9. Check whitespace and patch cleanliness:

   ```bash
   git diff --check
   ```

## No-drift checks

Confirm the generated project does not introduce:

- Local workspace as official state.
- Candidate queue as a ledger.
- Report as receipt.
- Ghost list as evidence.
- LLM as authority.
- Supabase as universal canon red flag.
- A pack described as official.
- Non-canon Act slots.

## Generated-output note

`dist/logline-lab-kit` is regenerated from `logline-labkit-generator/templates/project`. Follow repository policy on whether generated `dist/` output is committed. If `dist/` is ignored, commit template, generator, acceptance, and docs changes only.

## Remaining Ghosts to confirm

Keep remaining Ghosts listed unless they are actually implemented:

- remote-spine-unconfigured
- supabase-ingest-unconfigured
- supabase-env-unconfigured
- evidence-registry-unimplemented
- receipt-closure-unimplemented
- interactive-lab-surface-unimplemented
- llm-translator-unimplemented
- yaml-act-parser-unimplemented
- passkey-checkpointing-unimplemented
- batch-signing-unimplemented
- personal-adapters-unimplemented
- selective-disclosure-unimplemented
