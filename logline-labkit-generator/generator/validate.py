#!/usr/bin/env python3
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
DIST = ROOT / "dist" / "logline-lab-kit"
REQUIRED = [
    "README.md", "LICENSE", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "install.sh", ".env.example",
    "crates/logline-act/Cargo.toml", "crates/logline-lab-core/Cargo.toml", "crates/logline-lab-cli/Cargo.toml",
    "docs/00-overview.md", "docs/01-logline-act.md", "docs/02-product.md", "docs/03-packs-and-profiles.md",
    "docs/04-cli.md", "docs/05-install.md", "docs/06-recovery.md", "docs/07-interactive-ux.md", "docs/08-llm-boundary.md",
    "schemas/logline-act.schema.json", "schemas/lab-manifest.schema.json", "schemas/pack-manifest.schema.json", "schemas/profile.schema.json",
    "manifests/lab.manifest.example.yaml", "manifests/santo-andre.manifest.example.yaml", "manifests/personal-offline.manifest.example.yaml",
    "profiles/supabase.profile.yaml", "profiles/local-offline.profile.yaml",
    "packages/santo-andre/package.yaml", "packages/personal-offline/package.yaml",
    "examples/acts/minimal.act.json", "scripts/smoke-local.sh", "scripts/scan-forbidden-markers.sh", "scripts/command-matrix.sh",
    "reports/GHOSTS.md", "reports/COMMAND_MATRIX.md", "reports/FORBIDDEN_MARKER_SCAN.md",
]

def main():
    missing = [p for p in REQUIRED if not (DIST / p).exists()]
    if missing:
        print("missing required paths:")
        for p in missing: print(f"- {p}")
        return 1
    print("validation implemented: expected generated project paths exist")
    return 0

if __name__ == "__main__":
    sys.exit(main())
