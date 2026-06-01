# LogLine Lab Kit Generator

This directory is the generation pack for a CLI-first LogLine Lab Kit. It compiles the approved planning corpus into `dist/logline-lab-kit/` without changing canon or vendor snapshots.

## Source pack

- `corpus/v1.1/` extracted text files
- Original zip SHA256: `155413651ff1614152970de6c806adf8b4679c0d002a3ed41f4e427bd208a9bb`
- Approval note: `corpus/APPROVAL_NOTE.md`

## Use

```sh
python3 generator/generate.py
python3 generator/validate.py
python3 generator/scan.py dist/logline-lab-kit
```

The generated project is intentionally CLI-first. Interactive and LLM-facing surfaces are reserved as explicit Ghosts until implemented by approved work.
