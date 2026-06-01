# Generation Plan

1. Preserve the approved corpus in `corpus/`.
2. Compile the corpus rules into blueprints and templates.
3. Generate `dist/logline-lab-kit/` as a CLI-first Rust workspace.
4. Copy the base project template first.
5. Render selected generated-project outputs from blueprints after the copy step:
   - command matrix;
   - Ghost report;
   - Santo André and Personal Offline pack manifests;
   - Local Offline and Supabase profile manifests.
6. Produce reports for ghosts, command behavior, forbidden marker scan, and salvage.
7. Run validation, command matrix, scanner, and Rust checks before final delivery.

Some generated project files are rendered from blueprints after the base project template is copied. For those files, the blueprint is source; the generated file in `dist` is output.
