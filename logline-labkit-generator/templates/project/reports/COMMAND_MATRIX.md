# Command Matrix

| Command | Status | Notes |
| --- | --- | --- |
| `logline-lab init --home <path>` | implemented | Creates an idempotent local workspace under `.logline-lab/`; not official spine and not receipt. |
| `logline-lab doctor --home <path>` | implemented | Checks required local workspace paths, including `.logline-lab/candidates/`, and generated docs/examples/schemas. Empty candidate queue is allowed. |
| `logline-lab status --home <path>` | implemented | Reports local workspace status, `candidate_count`, report count/latest report, local candidate queue availability, and remaining Ghost records. |
| `logline-lab candidate add --file <path> --home <path>` | implemented | Validates a canonical nine-slot JSON Act and captures the original input as a local Candidate only. |
| `logline-lab candidate list --home <path>` | implemented | Lists local Candidate records from `.logline-lab/candidates/` only. |
| `logline-lab candidate get <candidate_id> --home <path>` | implemented | Prints local Candidate metadata and captured content; missing ids return non-zero. |
| `logline-lab ghost list --home <path>` | implemented | Lists local Ghost keys from the initialized Lab home; Ghosts are not fatal by default. |
| `logline-lab report generate daily-state --home <path>` | implemented | Writes `.logline-lab/reports/daily-state.md` as a local workspace projection only; no receipt, no evidence proof, and no remote sync. |
| `logline-lab act validate --file <path>` | implemented | Validates JSON LogLine Acts against the nine canonical slots. |
| `logline-lab act emit --file <path>` | partial / preview-only | Validates and previews only; no remote write and no receipt closure. |
| `logline-lab lab` | ghost | `interactive-lab-surface-unimplemented`. |
| `logline-lab chat` | ghost | `llm-translator-unimplemented`. |
