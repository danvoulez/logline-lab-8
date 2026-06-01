# Generator Workflow

Use this workflow for any PR that changes generator-driven files.

1. Change blueprints/templates/source files first.
2. Run generator.
3. Validate generated output.
4. Run forbidden-marker scan.
5. Run command matrix.
6. Run tests/smoke when available.
7. Commit source + generated output only if repository policy currently tracks generated snapshot.

## Source before output

Do not hand-edit generated output as source of authority. Generated output is reviewable output, not the source that creates project authority.

If `dist` is edited manually, the PR must either move the change back into templates/blueprints or explain why this is a one-off debugging change and mark it unverified/ghosted.

## Blueprint-rendered files

The generator copies the base project template first, then renders selected files from blueprints afterward. For these files, the blueprint/catalog is source and the `dist` file is output:

- command matrix: `logline-labkit-generator/blueprints/cli.commands.yaml`
- Ghost report entries: command, pack, and profile blueprints
- pack manifests: `package.santo-andre.yaml` and `package.personal-offline.yaml`
- profile manifests: `profile.local-offline.yaml` and `profile.supabase.yaml`

Update blueprints first for command matrix, Ghosts, packs, and profiles. Then run generator, validator, forbidden-marker scan, command matrix, Rust tests, smoke checks, and shell syntax checks.

## Validation evidence

Paste command output or link to logs for generator, validation, scan, matrix, tests, and smoke checks. Do not claim success without output.
