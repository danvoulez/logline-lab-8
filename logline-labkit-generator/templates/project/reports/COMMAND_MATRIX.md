# Command Matrix

## logline-lab --version
logline-lab 0.1.0

## logline-lab init
partial: lab manifest template is available in manifests/lab.manifest.example.yaml

## logline-lab doctor
implemented: core loaded; profile checks are partial; no external provider required

## logline-lab status
partial: generated lab kit is present; runtime execution surfaces are ghosts

## logline-lab act validate
partial: provide --file <path> to validate a JSON LogLine Act

## logline-lab act validate --file examples/acts/minimal.act.json
valid LogLine Act
slots: 9/9
status: candidate

## logline-lab act emit --file examples/acts/minimal.act.json
valid LogLine Act
slots: 9/9
status: candidate
partial: act validated; emit preview only; no storage, no receipt, no remote spine write

## logline-lab lab
ghost expected

## logline-lab chat
ghost expected
