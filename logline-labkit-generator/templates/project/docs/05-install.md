# Install

Run `./install.sh` or `cargo install --path crates/logline-lab-cli` from the generated project root.

After building, initialize a local operational workspace with:

```sh
logline-lab init --home .
logline-lab doctor --home .
logline-lab status --home .
```

The initialized home is local workspace state only. It is not canon, not an official spine, and not a receipt store.


## Local Candidate smoke loop

After initialization, the generated CLI can run the local capture loop:

```sh
logline-lab act validate --file examples/acts/minimal.act.json
logline-lab candidate add --home . --file examples/acts/minimal.act.json
logline-lab candidate list --home .
logline-lab ghost list --home .
logline-lab report generate daily-state --home .
logline-lab status --home .
```

Candidate capture is local operational capture. It validates canonical Act shape, writes a local candidate queue record, and keeps authority limited to the local workspace. It does not admit to a remote spine, close receipts, or create official truth.

Ghosts preserve unresolved state. Daily State is a local report/projection over the workspace. Reports do not close receipts, do not prove evidence, and do not create official truth.
