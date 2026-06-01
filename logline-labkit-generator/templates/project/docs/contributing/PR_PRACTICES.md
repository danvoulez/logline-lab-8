# PR Practices

A PR is not allowed to improve the architecture by surprise.

## Declare scope explicitly

Every PR must state its scope before review. Name the affected layer: product docs, generator templates, generator scripts, generated output, packs, profiles, schemas, CLI behavior, reports, or tests.

## Do not change authority silently

A PR must not silently change authority, canon, pack/profile boundaries, generated output meaning, or the semantic model. If authority might be affected, the PR must include an authority note and identify the approving process.

## Keep authority status separate

Product docs, generator templates, and generated output have different authority status:

- Product docs describe intended behavior and review expectations.
- Generator blueprints/templates are source inputs for generated project files.
- Generated output is reviewable output, not source authority.

Do not treat generated output as the place where authority is created.

## Ghost missing or unsafe behavior

Missing behavior must be ghosted. Use this rule in every PR:

- If missing: create Ghost.
- If partial: mark partial.
- If unverified: mark unverified.
- If unsafe: reject or ghost.

## Validate honestly

A PR cannot claim build/test success without command output. If a validation command was not run or could not complete, list it and create or reference a Ghost.

## State pack/profile impact

Any change to pack/profile semantics must say whether it affects Santo André, Personal Offline, or generic LogLine Lab behavior. Pack/profile practice must not be promoted to canon by implication.
