# LogLine Lab Kit Generator

This directory is the generation pack for a CLI-first LogLine Lab Kit. It compiles the approved planning corpus into `dist/logline-lab-kit/` without changing canon or vendor snapshots.

## Source pack

- `corpus/v1.1/` extracted text files
- Original zip SHA256: `155413651ff1614152970de6c806adf8b4679c0d002a3ed41f4e427bd208a9bb`
- Approval note: `corpus/APPROVAL_NOTE.md`

## Blueprint-rendered outputs

The base project template is copied first. After that copy step, selected generated project files are rendered from blueprints and overwrite their template placeholders:

- `reports/COMMAND_MATRIX.md` from `blueprints/cli.commands.yaml`
- `reports/GHOSTS.md` from command, pack, and profile blueprints
- `packages/santo-andre/package.yaml` from `blueprints/package.santo-andre.yaml`
- `packages/personal-offline/package.yaml` from `blueprints/package.personal-offline.yaml`
- `profiles/local-offline.profile.yaml` from `blueprints/profile.local-offline.yaml`
- `profiles/supabase.profile.yaml` from `blueprints/profile.supabase.yaml`

For those files, the blueprint is source; the generated file in `dist` is output. Do not hand-edit the generated `dist` copies as source authority. Update the blueprint/catalog first, then regenerate and validate.

## Use

```sh
python3 generator/generate.py
python3 generator/validate.py
python3 generator/scan.py dist/logline-lab-kit
```

The generated project is intentionally CLI-first. Interactive and LLM-facing surfaces are reserved as explicit Ghosts until implemented by approved work.
