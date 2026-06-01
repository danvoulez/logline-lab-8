# Command Matrix

| Command | Status | Notes |
| --- | --- | --- |
| `logline-lab init --home <path>` | implemented | Creates an idempotent local workspace under `.logline-lab/`; not official spine and not receipt. |
| `logline-lab doctor --home <path>` | implemented | Checks required local workspace paths and generated docs/examples/schemas. |
| `logline-lab status --home <path>` | implemented | Reports local workspace status and remaining Ghost records. |
| `logline-lab act validate --file <path>` | implemented | Validates JSON LogLine Acts against the nine canonical slots. |
| `logline-lab act emit --file <path>` | partial / preview-only | Validates and previews only; no remote write and no receipt closure. |
| `logline-lab lab` | ghost | `interactive-lab-surface-unimplemented`. |
| `logline-lab chat` | ghost | `llm-translator-unimplemented`. |
