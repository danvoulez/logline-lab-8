# CLI

The binary is `logline-lab`. Commands return implemented, partial, ghost, or unverified status text.

## Local Lab home commands

A local Lab home is an operational workspace. It is not canon, not an official spine, and not a receipt store.

Default behavior uses the current directory as the Lab home. You can pass an explicit path:

```sh
logline-lab init --home .
logline-lab doctor --home .
logline-lab status --home .
```

`logline-lab init` creates `.logline-lab/` with an editable `lab.manifest.yaml`, `STATUS.md`, `GHOSTS.md`, and local operational directories for candidates, reports, ghosts, profiles, and packs. Init is idempotent and does not overwrite existing manifest/status/ghost files.

`logline-lab doctor` checks the local home structure and required generated project docs, examples, and schemas. It returns non-zero when required local structure is missing.

`logline-lab status` reads the local workspace state, lists Ghost records, and reports remote spine, receipt closure, interactive UX, and LLM translator surfaces as ghosted or unimplemented.

## Act commands

`logline-lab act validate --file <path>` validates a JSON LogLine Act against the nine-slot shape.

`logline-lab act emit --file <path>` validates the Act and returns a preview-only message. It does not write remote state and does not close a receipt.

## Ghost commands

`logline-lab lab` and `logline-lab chat` remain Ghosts in this generated kit.
