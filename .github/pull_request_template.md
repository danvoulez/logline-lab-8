# Motivation

Explain why this PR is needed. A PR is not allowed to improve the architecture by surprise.

# Scope

State the exact scope and the affected area: product docs, generator source/templates, generated output, packs, profiles, CLI, schemas, tests, or documentation only.

## Scope checklist
- [ ] This PR does not amend Foundation canon.
- [ ] This PR does not edit `vendor/`.
- [ ] This PR does not promote pack/profile practice to canon.
- [ ] This PR does not introduce `artifact` as a semantic category.
- [ ] This PR does not use primitive-system framing.
- [ ] This PR does not treat projections as source of truth.
- [ ] This PR does not treat LLM output as authority, evidence, receipt, or canon.
- [ ] This PR does not treat Supabase as universal canon.
- [ ] This PR does not call Santo André an official pack.
- [ ] This PR does not make `selected_branch` an Act slot.
- [ ] This PR does not make runtime envelope an Act slot.
- [ ] This PR marks missing/partial/unverified behavior as Ghost.

# What changed

List concrete changed files and behavior.

# What did not change

List boundaries that remain unchanged, especially canon, authority, generated behavior, pack/profile semantics, and vendor material.

# Authority / canon impact

State whether the PR changes authority or canon. If yes, identify the approved governance process. If no, say that no authority upgrade or canon amendment is intended.

# Pack/profile impact

State whether the PR affects Santo André, Personal Offline, or generic LogLine Lab behavior. Do not promote any pack/profile practice to canon.

# Generator impact

State whether generator blueprints, templates, scripts, validation, or reports changed.

## Generator checklist
- [ ] If generator templates changed, the generated output was regenerated.
- [ ] If generated output changed, the source template/blueprint change explains why.
- [ ] Generated output is not treated as authority.
- [ ] Generated snapshot changes are reviewable and expected.

# Generated output impact

State whether `dist/logline-lab-kit/` changed. Generated output is reviewable output, not source authority.

# Validation

Paste command output or link to logs. Do not claim build/test success without actual command output.

## Validation checklist
- [ ] `python3 logline-labkit-generator/generator/generate.py`
- [ ] `python3 logline-labkit-generator/generator/validate.py`
- [ ] `cd dist/logline-lab-kit && scripts/scan-forbidden-markers.sh`
- [ ] `cd dist/logline-lab-kit && scripts/command-matrix.sh`
- [ ] `cd dist/logline-lab-kit && cargo test`
- [ ] `cd dist/logline-lab-kit && scripts/smoke-local.sh`
- [ ] `git diff --name-only -- vendor` produced no output.

If any command was not run, explain why and open/list a Ghost.

# Ghosts

Use this rule:

- If missing: create Ghost.
- If partial: mark partial.
- If unverified: mark unverified.
- If unsafe: reject or ghost.

List all missing, partial, unsafe, or unverified behavior. If none remain, say why the PR has no remaining Ghosts.

# Reviewer notes

Call out risky areas, source-template relationships, expected generated diffs, and any evidence reviewers should inspect.
