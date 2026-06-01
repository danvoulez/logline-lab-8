#!/usr/bin/env python3
from pathlib import Path
import json
import subprocess
import sys

from render import (
    BLUEPRINTS,
    PACK_BLUEPRINTS,
    PROFILE_BLUEPRINTS,
    REQUIRED_GHOSTS,
    load_blueprint,
)


ROOT = Path(__file__).resolve().parents[2]
DIST = ROOT / "dist" / "logline-lab-kit"

IGNORED_HYGIENE_DIRS = {".git", "codex-input", "dist", "target"}


def repository_zip_files():
    zip_files = []
    for path in ROOT.rglob("*.zip"):
        relative = path.relative_to(ROOT)
        if any(part in IGNORED_HYGIENE_DIRS for part in relative.parts):
            continue
        zip_files.append(relative)
    return sorted(zip_files)


def tracked_zip_files():
    result = subprocess.run(
        ["git", "ls-files", "*.zip"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if result.returncode != 0:
        return [f"git ls-files failed: {result.stderr.strip()}"]
    return [line for line in result.stdout.splitlines() if line.strip()]


REQUIRED = [
    "README.md", "VERSION", "LICENSE", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "install.sh", ".env.example",
    "crates/logline-act/Cargo.toml", "crates/logline-lab-core/Cargo.toml", "crates/logline-lab-core/src/lab_home.rs", "crates/logline-lab-core/src/candidates.rs", "crates/logline-lab-core/src/catalog.rs", "crates/logline-lab-core/src/ghosts.rs", "crates/logline-lab-core/src/reports.rs", "crates/logline-lab-cli/Cargo.toml",
    "docs/00-overview.md", "docs/01-logline-act.md", "docs/02-product.md", "docs/03-packs-and-profiles.md",
    "docs/04-cli.md", "docs/05-install.md", "docs/06-recovery.md", "docs/07-interactive-ux.md", "docs/08-llm-boundary.md", "docs/09-schemas-and-fixtures.md", "docs/release/RELEASE_CHECKLIST.md",
    ".github/pull_request_template.md", "docs/contributing/PR_PRACTICES.md", "docs/contributing/NO_DRIFT_REVIEW.md",
    "docs/contributing/GENERATOR_WORKFLOW.md", "docs/contributing/AUTHORITY_BOUNDARIES.md",
    "schemas/logline-act.schema.json", "schemas/lab-manifest.schema.json", "schemas/candidate-metadata.schema.json", "schemas/candidate-index.schema.json", "schemas/pack-manifest.schema.json", "schemas/profile.schema.json",
    "manifests/lab.manifest.example.yaml", "manifests/santo-andre.manifest.example.yaml", "manifests/personal-offline.manifest.example.yaml",
    "profiles/supabase.profile.yaml", "profiles/local-offline.profile.yaml",
    "packages/santo-andre/package.yaml", "packages/personal-offline/package.yaml",
    "examples/acts/minimal.act.json", "examples/candidates/ugly-candidate.json", "examples/candidates/candidate-metadata.json", "examples/candidates/candidate-index.json", "examples/fixtures.index.md",
    "examples/invalid/missing-confirmed-by.json", "examples/invalid/extra-selected-branch.json", "examples/invalid/extra-runtime-envelope.json",
    "examples/invalid/malformed.json", "examples/invalid/extra-top-level-field.json", "examples/invalid/candidate-index-inconsistent.json",
    "crates/logline-act/tests/act_validation.rs", "crates/logline-lab-cli/tests/cli_act.rs", "crates/logline-lab-cli/tests/lab_home.rs", "crates/logline-lab-cli/tests/candidates.rs", "crates/logline-lab-cli/tests/candidate_index.rs", "crates/logline-lab-cli/tests/reports.rs", "crates/logline-lab-cli/tests/packs_profiles.rs", "crates/logline-lab-cli/tests/help_version.rs", "scripts/smoke-local.sh", "scripts/scan-forbidden-markers.sh", "scripts/command-matrix.sh",
    "reports/GHOSTS.md", "reports/COMMAND_MATRIX.md", "reports/FORBIDDEN_MARKER_SCAN.md",
]


def contains_line(text, needle):
    return any(line.strip() == needle for line in text.splitlines())


def validate_schema_files():
    errors = []
    required_schema_paths = [
        "schemas/logline-act.schema.json",
        "schemas/lab-manifest.schema.json",
        "schemas/candidate-metadata.schema.json",
        "schemas/candidate-index.schema.json",
        "schemas/pack-manifest.schema.json",
        "schemas/profile.schema.json",
    ]
    for rel in required_schema_paths:
        if not (DIST / rel).is_file():
            errors.append(f"missing schema file: {rel}")

    act_schema_path = DIST / "schemas/logline-act.schema.json"
    if act_schema_path.is_file():
        act_schema = json.loads(act_schema_path.read_text(encoding="utf-8"))
        canonical_slots = [
            "who",
            "did",
            "this",
            "when",
            "confirmed_by",
            "if_ok",
            "if_doubt",
            "if_not",
            "status",
        ]
        if act_schema.get("required") != canonical_slots:
            errors.append("logline-act schema required slots do not match canonical nine slots")
        if act_schema.get("additionalProperties") is not False:
            errors.append("logline-act schema must reject additional top-level properties")
        text = act_schema_path.read_text(encoding="utf-8")
        for forbidden in [
            "selected_branch",
            "runtime_envelope",
            "content_hash",
            "runtime_id",
            "type_hint",
        ]:
            if forbidden in text:
                errors.append(
                    f"logline-act schema must not include reserved metadata slot: {forbidden}"
                )

    fixture_index = DIST / "examples/fixtures.index.md"
    if fixture_index.is_file():
        index_text = fixture_index.read_text(encoding="utf-8")
        for fixture in [
            "examples/acts/minimal.act.json",
            "examples/candidates/ugly-candidate.json",
            "examples/invalid/missing-confirmed-by.json",
            "examples/invalid/extra-selected-branch.json",
            "examples/invalid/extra-runtime-envelope.json",
            "examples/invalid/extra-top-level-field.json",
            "examples/invalid/malformed.json",
            "examples/candidates/candidate-metadata.json",
            "examples/candidates/candidate-index.json",
        ]:
            if fixture not in index_text:
                errors.append(f"fixture index missing fixture: {fixture}")
    return errors

def validate_blueprint_outputs():
    errors = []
    command_matrix = DIST / "reports" / "COMMAND_MATRIX.md"
    ghost_report = DIST / "reports" / "GHOSTS.md"
    command_text = command_matrix.read_text(encoding="utf-8") if command_matrix.exists() else ""
    ghost_text = ghost_report.read_text(encoding="utf-8") if ghost_report.exists() else ""

    cli = load_blueprint(BLUEPRINTS / "cli.commands.yaml")
    for command in cli.get("commands", []):
        if command["command"] not in command_text:
            errors.append(f"command matrix missing command: {command['command']}")
        if command["status"] not in command_text:
            errors.append(f"command matrix missing status for {command['command']}: {command['status']}")

    for rel in PACK_BLUEPRINTS:
        if not (DIST / rel).exists():
            errors.append(f"missing generated pack manifest: {rel}")
    for rel in PROFILE_BLUEPRINTS:
        if not (DIST / rel).exists():
            errors.append(f"missing generated profile manifest: {rel}")

    santo = (DIST / "packages/santo-andre/package.yaml").read_text(encoding="utf-8")
    personal = (DIST / "packages/personal-offline/package.yaml").read_text(encoding="utf-8")
    supabase = (DIST / "profiles/supabase.profile.yaml").read_text(encoding="utf-8")
    if contains_line(santo, "official: true") or contains_line(santo, "official_pack: true"):
        errors.append("Santo André manifest must not declare official: true")
    if contains_line(personal, "official: true") or contains_line(personal, "official_pack: true"):
        errors.append("Personal Offline manifest must not declare official: true")
    if contains_line(supabase, "universal_canon: true"):
        errors.append("Supabase profile must not declare universal_canon: true")

    for key in REQUIRED_GHOSTS:
        if key not in ghost_text:
            errors.append(f"ghost report missing required Ghost key: {key}")
    return errors


def main():
    zip_files = repository_zip_files()
    if zip_files:
        print("repository hygiene failed: zip files are not allowed in source-controlled areas:")
        for path in zip_files:
            print(f"- {path}")
        return 1

    tracked_zips = tracked_zip_files()
    if tracked_zips:
        print("repository hygiene failed: tracked zip files are not allowed:")
        for path in tracked_zips:
            print(f"- {path}")
        return 1

    missing = [p for p in REQUIRED if not (DIST / p).exists()]
    if missing:
        print("missing required paths:")
        for p in missing:
            print(f"- {p}")
        return 1

    schema_errors = validate_schema_files()
    if schema_errors:
        print("schema/fixture validation failed:")
        for error in schema_errors:
            print(f"- {error}")
        return 1

    blueprint_errors = validate_blueprint_outputs()
    if blueprint_errors:
        print("blueprint-derived validation failed:")
        for error in blueprint_errors:
            print(f"- {error}")
        return 1

    print("validation implemented: expected generated project paths, blueprint-derived outputs, and zip hygiene passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
