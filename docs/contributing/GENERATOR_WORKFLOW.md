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

If `dist` is edited manually, the PR must either move the change back into templates or explain why this is a one-off debugging change and mark it unverified/ghosted.

## Validation evidence

Paste command output or link to logs for generator, validation, scan, matrix, tests, and smoke checks. Do not claim success without output.
