#!/usr/bin/env python3
from pathlib import Path
import shutil

from render import render_blueprint_outputs

ROOT = Path(__file__).resolve().parents[2]
GEN = ROOT / "logline-labkit-generator"
DIST = ROOT / "dist" / "logline-lab-kit"
TEMPLATE_PROJECT = GEN / "templates" / "project"

def main():
    if not TEMPLATE_PROJECT.exists():
        raise SystemExit("missing template project")
    if DIST.exists():
        shutil.rmtree(DIST)
    shutil.copytree(TEMPLATE_PROJECT, DIST)
    render_blueprint_outputs(DIST)
    for script in (DIST / "scripts").glob("*.sh"):
        script.chmod(0o755)
    (DIST / "install.sh").chmod(0o755)
    print(f"generated: {DIST}")

if __name__ == "__main__":
    main()
