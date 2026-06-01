# Install

Run `./install.sh` or `cargo install --path crates/logline-lab-cli` from the generated project root.

After building, initialize a local operational workspace with:

```sh
logline-lab init --home .
logline-lab doctor --home .
logline-lab status --home .
```

The initialized home is local workspace state only. It is not canon, not an official spine, and not a receipt store.
