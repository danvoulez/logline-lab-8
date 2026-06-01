from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[2]
GEN = ROOT / "logline-labkit-generator"
BLUEPRINTS = GEN / "blueprints"

PACK_BLUEPRINTS = {
    "packages/santo-andre/package.yaml": BLUEPRINTS / "package.santo-andre.yaml",
    "packages/personal-offline/package.yaml": BLUEPRINTS / "package.personal-offline.yaml",
}
PROFILE_BLUEPRINTS = {
    "profiles/local-offline.profile.yaml": BLUEPRINTS / "profile.local-offline.yaml",
    "profiles/supabase.profile.yaml": BLUEPRINTS / "profile.supabase.yaml",
}
REQUIRED_GHOSTS = [
    "remote-spine-unconfigured",
    "supabase-ingest-unimplemented",
    "supabase-env-unverified",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
    "interactive-lab-surface-unimplemented",
    "llm-translator-unimplemented",
    "yaml-act-parser-unimplemented",
    "passkey-checkpointing-unimplemented",
    "batch-signing-unimplemented",
    "personal-adapters-unimplemented",
    "selective-disclosure-unimplemented",
]


def write(path, content):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def load_blueprint(path):
    path = Path(path)
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def generated_marker(path, style):
    rel = Path(path).relative_to(ROOT)
    if style == "md":
        return f"<!-- generated from {rel.as_posix()} -->\n\n"
    return f"# generated from {rel.as_posix()}\n"


def yaml_scalar(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if value is None:
        return "null"
    text = str(value)
    simple = text and all(ch.isalnum() or ch in "-_./ " for ch in text)
    if simple and not text.startswith((" ", "-", "?", ":", "@", "`")) and ": " not in text:
        return text
    return json.dumps(text, ensure_ascii=False)


def to_yaml(value, indent=0):
    lines = []
    prefix = " " * indent
    if isinstance(value, dict):
        for key, item in value.items():
            if isinstance(item, (dict, list)):
                lines.append(f"{prefix}{key}:")
                lines.extend(to_yaml(item, indent + 2))
            else:
                lines.append(f"{prefix}{key}: {yaml_scalar(item)}")
    elif isinstance(value, list):
        for item in value:
            if isinstance(item, (dict, list)):
                lines.append(f"{prefix}-")
                lines.extend(to_yaml(item, indent + 2))
            else:
                lines.append(f"{prefix}- {yaml_scalar(item)}")
    else:
        lines.append(f"{prefix}{yaml_scalar(value)}")
    return lines


def render_yaml_manifest(blueprint_path):
    data = load_blueprint(blueprint_path)
    data.pop("source_kind", None)
    return generated_marker(blueprint_path, "yaml") + "\n".join(to_yaml(data)) + "\n"


def markdown_cell(value):
    return str(value).replace("|", "\\|").replace("\n", " ")


def render_command_matrix():
    source = BLUEPRINTS / "cli.commands.yaml"
    data = load_blueprint(source)
    lines = [
        generated_marker(source, "md").rstrip(),
        "# Command Matrix",
        "",
        "| Command | Status | Scope | Authority limit | Ghosts or notes |",
        "| --- | --- | --- | --- | --- |",
    ]
    for item in data["commands"]:
        lines.append(
            "| `{}` | {} | {} | {} | {} |".format(
                markdown_cell(item["command"]),
                markdown_cell(item["status"]),
                markdown_cell(item["scope"]),
                markdown_cell(item["authority_limit"]),
                markdown_cell(item["ghosts_or_notes"]),
            )
        )
    return "\n".join(lines) + "\n"


def collect_ghosts():
    ghosts = []
    cli = load_blueprint(BLUEPRINTS / "cli.commands.yaml")
    ghosts.extend(cli.get("required_ghosts", []))
    for command in cli.get("commands", []):
        notes = command.get("ghosts_or_notes", "")
        for part in str(notes).replace(",", ";").split(";"):
            key = part.strip().strip("`.")
            if key.endswith(("-unimplemented", "-unconfigured", "-unverified")):
                ghosts.append(key)
    for path in list(PACK_BLUEPRINTS.values()) + list(PROFILE_BLUEPRINTS.values()):
        ghosts.extend(load_blueprint(path).get("default_ghosts", []))
    for key in REQUIRED_GHOSTS:
        ghosts.append(key)
    return list(dict.fromkeys(ghosts))


def render_ghost_report():
    sources = [
        BLUEPRINTS / "cli.commands.yaml",
        *PACK_BLUEPRINTS.values(),
        *PROFILE_BLUEPRINTS.values(),
    ]
    marker = "<!-- generated from {} -->\n\n".format(
        ", ".join(path.relative_to(ROOT).as_posix() for path in sources)
    )
    lines = [
        marker.rstrip(),
        "# Ghosts",
        "",
        "These Ghost keys are compiled from the command, pack, and profile blueprints. They remain Ghosts unless runtime capability is actually implemented.",
        "",
    ]
    lines.extend(f"- {key}" for key in collect_ghosts())
    lines.extend(
        [
            "",
            "## Closed in this project state",
            "",
            "- Implemented: local Lab home init creates `.logline-lab/` workspace files and directories.",
            "- Implemented: local Lab home doctor checks required local and generated project paths.",
            "- Implemented: local Lab home status reports workspace state, candidate_count, and remaining Ghosts.",
            "- Implemented: local Candidate add/list/get captures validated JSON Acts into the local candidate queue only.",
            "- Implemented: local Ghost listing reads `.logline-lab/GHOSTS.md` and `.logline-lab/ghosts/`.",
            "- Implemented: Daily State report generation writes a local workspace projection under `.logline-lab/reports/`.",
            "",
            "## Pack/profile catalog state",
            "",
            "- Implemented: pack/profile manifests exist for `santo-andre`, `personal-offline`, `local-offline`, and `supabase`.",
            "- Implemented: init validates selected pack/profile ids and materializes them in `.logline-lab/lab.manifest.yaml`.",
            "- Ghost: Supabase profile capabilities are declarations only; no Supabase ingest or environment verification runs in this project state.",
            "- Ghost: Personal Offline passkey checkpointing, batch signing, adapters, and selective disclosure are declared only.",
        ]
    )
    return "\n".join(lines) + "\n"


def render_blueprint_outputs(dist):
    dist = Path(dist)
    write(dist / "reports" / "COMMAND_MATRIX.md", render_command_matrix())
    write(dist / "reports" / "GHOSTS.md", render_ghost_report())
    for rel, source in PACK_BLUEPRINTS.items():
        write(dist / rel, render_yaml_manifest(source))
    for rel, source in PROFILE_BLUEPRINTS.items():
        write(dist / rel, render_yaml_manifest(source))
