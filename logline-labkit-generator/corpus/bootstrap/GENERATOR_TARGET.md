# Generator Target

Primary task:
Generate `logline-labkit-generator/`.

The generator must produce a clean, installable, CLI-first LogLine Lab Kit project:
`dist/logline-lab-kit/`

The generated project must be:
- Act-centered
- pack/profile aware
- CLI-installable
- scriptable
- future interactive/LLM-UX-ready
- honest about ghosts/partials/unverified components
- free of artifact/primitives/false-authority contamination

The generated project must not be a web dashboard.

Interactive/TUI/LLM UX may be reserved as future surfaces:
- `logline-lab lab`
- `logline-lab chat`

These may initially return explicit Ghosts:
- `interactive-lab-surface-unimplemented`
- `llm-translator-unimplemented`

The core must not depend on TUI or LLM providers.
