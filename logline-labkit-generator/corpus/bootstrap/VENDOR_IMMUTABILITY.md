# Vendor Immutability

`vendor/` is immutable.

Upstream canon/reference material must be treated as read-only snapshots.

Any desired canon change must be emitted as a proposal, not as an edit:

- `proposals/`
- `packs/<pack>/practice/`
- `docs/questions/`

Any generated diff under `vendor/` is a failed run.
