# V1.1 Changelog

## Purpose

Surgical semantic patch over v1.

## Changes

### 1. selected_branch language correction in 01_LOG_LINE_ACT.md

**Before:**
- Section "Ramo Semântico" stated: "O Act deve materializar o ramo selecionado: selected_branch: ok | doubt | not"

**After:**
- Section now clarifies: "O Act declara rotas possíveis. A prática/projection/runtime pode registrar qual rota foi selecionada. Mas selected_branch não é slot do Act canon."
- Added explicit statement: "The Act canon does not contain a selected_branch slot."
- Added: "A practice, runtime, validator, admission process, receipt process, or projection MAY materialize a selected branch outside the Act canon as metadata or projection output."
- Added: "selected_branch may be useful, but it is not part of the nine-slot Act."

**Rationale:**
- selected_branch is metadata/projection/practice output, not an Act canon slot
- The Act contains the three semantic routes (if_ok, if_doubt, if_not), not the selected branch

### 2. status vocabulary softening in 01_LOG_LINE_ACT.md

**Before:**
- "Estados do Act" section listed statuses as if they were a closed canonical list

**After:**
- Added: "status é slot do Act. O vocabulário de status pode ser disciplinado por canon/conformance/pack/prática. A lista recomendada por um pack não deve ser confundida com ontologia universal."
- Changed "Estados do Act" to "Estados do Act" with explanatory note that these are "Exemplos comuns"

**Rationale:**
- status is one of the nine Act slots, but the allowed status vocabulary may be defined by canon, conformance profile, pack, or local practice depending on the context
- A pack may recommend statuses, but this vocabulary must not be mistaken for an extra semantic ontology

### 3. ledger correction in 02_LOGLINE_LAB_KIT_PRODUCT.md

**Before:**
- "SQLite não pode ser chamado de: ... ledger"
- "Arquivos não podem ser: ... official ledger"

**After:**
- "SQLite não pode ser chamado de: ... spine"
- "Arquivos não podem ser: ... official spine"

**Rationale:**
- "ledger" is ambiguous - use "spine" for official storage
- SQLite is not ledger/spine official

## Targeted Scan Results

Scanned for: selected_branch, primitives, artifact, ledger, Supabase, official pack, runtime envelope

**Findings:**
1. **selected_branch** - Never appears as slot/canon in final documents. Always appears as metadata/projection/practice output. ✓
2. **primitive/primitives** - Only appears as old/incorrect framing or in corrected context. Never appears as valid concept. ✓
3. **artifact** - Only appears as rejected/removed category in V1_CHANGELOG.md and 11_IMPLEMENTATION_REPAIR_NOTES.md. Never appears as valid category in final documents. ✓
4. **ledger** - Corrected in 02_LOGLINE_LAB_KIT_PRODUCT.md. Now appears only in historical context or as what should NOT be used. ✓
5. **Supabase** - Always appears as v0/Santo André profile specific, never as universal rule. ✓
6. **official pack** - Does not appear in final documents. ✓
7. **runtime envelope** - Always appears as provenance metadata/pack practice, never as canon. ✓

## Remaining Ghosts

- Human review still required before calling the corpus authoritative.
- No implementation/build/test receipt is implied by this corpus.
- Corpus remains subject to further semantic refinement based on Dan approval.

## Files Modified

- 01_LOG_LINE_ACT.md (selected_branch correction, status vocabulary softening)
- 02_LOGLINE_LAB_KIT_PRODUCT.md (ledger → spine correction)
