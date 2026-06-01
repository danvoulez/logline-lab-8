#!/usr/bin/env python3
from pathlib import Path
import sys

MARKERS = [
    ("selected_branch as Act slot", ["selected_branch:"], ["metadata", "projection"]),
    ("runtime envelope as Act slot", ["runtime envelope"], ["provenance", "not canon", "does not", "treated as", "red flag"]),
    ("primitive system framing", ["primitive system", "primitives"], ["forbidden", "rejected", "historical", "does not", "red flag", "framing returns"]),
    ("artifact as positive category", ["artifact"], ["forbidden", "rejected", "removed", "not use", "does not", "red flag", "used as positive category"]),
    ("Supabase as universal canon", ["Supabase as universal canon", "universal: true"], ["does not", "described as", "red flag"]),
    ("Santo André as official pack", ["official: true", "official pack"], ["not official", "official: false", "does not", "described as", "red flag"]),
    ("local SQLite truth", ["local SQLite truth", "SQLite ledger", "SQLite spine"], ["forbidden"]),
    ("file/JSON/JSONL ledger", ["file ledger", "JSON ledger", "JSONL ledger"], ["forbidden"]),
    ("LLM decided", ["LLM decided"], []),
    ("Accepted/DECIDED without Dan approval", ["DECIDED", "Accepted"], ["without Dan approval", "Dan Amarilho"]),
    ("receipt without evidence", ["receipt without evidence"], ["forbidden", "no_receipt_without_evidence", "red flag", "block", "without evidence"]),
    ("tests passed without output", ["tests passed", "build success"], ["without command output", "without output", "do not claim", "cannot claim"]),
    ("vendor edits", ["vendor edits"], ["forbidden", "no_vendor_edits"]),
]
SKIP_DIRS = {"target", ".git"}

def files(root):
    for p in root.rglob("*"):
        if any(part in SKIP_DIRS for part in p.parts):
            continue
        if p.is_file():
            yield p

def allowed(line, allow):
    low = line.lower()
    return any(a.lower() in low for a in allow)

def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".")
    findings=[]
    for p in files(root):
        try:
            text=p.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for lineno,line in enumerate(text.splitlines(),1):
            for name, terms, allow in MARKERS:
                if any(t.lower() in line.lower() for t in terms) and not allowed(line, allow):
                    findings.append((name,p,lineno,line.strip()))
    report = root / "reports" / "FORBIDDEN_MARKER_SCAN.md"
    report.parent.mkdir(parents=True, exist_ok=True)
    if findings:
        body = ["# Forbidden Marker Scan", "", "Status: ghost", ""]
        for name,p,lineno,line in findings:
            body.append(f"- {name}: {p}:{lineno}: {line}")
        report.write_text("\n".join(body) + "\n", encoding="utf-8")
        print(f"forbidden marker scan ghost: {len(findings)} findings")
        return 1
    report.write_text("# Forbidden Marker Scan\n\nStatus: implemented\n\nNo forbidden generated-project markers found by the local scanner.\n", encoding="utf-8")
    print("forbidden marker scan implemented: no findings")
    return 0

if __name__ == "__main__":
    sys.exit(main())
