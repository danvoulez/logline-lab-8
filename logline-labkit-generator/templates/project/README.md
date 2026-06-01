# LogLine Lab Kit

A generated, installable, CLI-first LogLine Lab Kit.

The Lab Kit treats the LogLine Act as the semantic unit. Canon stays small; packs and profiles carry local practice; labs run canon plus a selected pack/profile; projections read; runtimes observe or execute.

## Install

```sh
./install.sh
```

## CLI

```sh
logline-lab --version
logline-lab init
logline-lab doctor
logline-lab status
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab act emit --file examples/acts/minimal.act.json
logline-lab lab
logline-lab chat
```

`lab` and `chat` currently return explicit Ghosts. No LLM provider is required for basic operation.
