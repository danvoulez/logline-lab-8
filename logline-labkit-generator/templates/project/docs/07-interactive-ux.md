# Interactive UX

`logline-lab serve` starts the local browser product for a first human operator.

The first screen is not a marketing page. It is the working Lab setup surface:

- choose Lab home, pack, and profile
- create the local Lab
- show the six setup motions: Load, Declare, Observe, Emit, Project, Learn
- display Candidate, Ghost, and projection counts
- keep raw CLI output visible
- print the next commands after setup

This surface is intentionally local. It does not write official spine state, close receipts, prove evidence, run remote sync, or grant LLM authority.

The reserved `logline-lab lab` command still returns `Ghost: interactive-lab-surface-unimplemented` until the richer interactive runtime is implemented.
