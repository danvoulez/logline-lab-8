# 07_LABKIT_IMPLEMENTATION_BLUEPRINT.md

## LabKit Implementation Blueprint

Este documento é o blueprint técnico de execução do LogLine Lab Kit, focado em Act-centered architecture.

## Definição Operacional

```txt
LogLine LabKit é o pacote instalável que cria um Lab operacional onde todo estado semântico nasce como LogLine Act, entra em cache/outbox local, sincroniza para o spine oficial online, gera projeções, roda processos/probes/workorders, captura evidência, abre ghosts e prepara receipts escopados.
```

Forma curta:

```txt
LabKit = installable Act machine.
```

Forma de engenharia:

```txt
LabKit = CLI + core Act engine + local cache/outbox + Supabase spine profile + projectors + ghost/evidence/receipt discipline + Clock + hooks + packages + reports + recovery gates.
```

## Invariantes Não Negociáveis

```txt
no file as truth
no semantic write outside LogLine Act
no async consequence outside queue/outbox
no silent drop
no fake success
no infinite retry
no receipt without evidence
no package embedding secrets
no runtime claim without runtime evidence
no AI-authored authority
```

## Modelo de Armazenamento

### Online Spine Oficial (Supabase Profile v0)

O spine oficial é:

```txt
ops.logline_acts
```

Toda escrita semântica oficial vai para lá.

```txt
semantic_writes_go_to: ops.logline_acts
```

Nenhuma tabela de projection é fonte semântica.
Nenhum JSON local é fonte semântica.
Nenhum arquivo de report é fonte semântica.

### SQLite Local (Todos os Profiles)

SQLite existe, mas com linguagem corrigida:

```txt
provisional cache
high-frequency local buffer
queue
retry state
outbox
```

Não pode ser chamado de:

```txt
truth
spine
official
semantic store
source of truth
```

Se o código tiver nomes antigos como `LocalLedger` ou `LocalSpine`, o plano correto é:

```txt
preferir rename seguro para LocalOutboxCache / ProvisionalCache
ou deixar Ghost se o rename exigir build não executado
```

### Files / JSON / JSONL

Arquivos podem ser:

```txt
examples
fixtures
stdout previews
exports
debug outputs
report output
templates
```

Arquivos não podem ser:

```txt
official storage
official spine
official LogLine Act spine
official receipt storage
official ghost storage
```

## Caminho Operacional Principal

O fluxo central do LabKit é este:

```txt
logline-lab act emit
  -> validate LogLine Act shape
  -> compute act identity / hashes
  -> write local provisional cache
  -> enqueue outbox row
  -> outbox sync
  -> call remote ingest function
  -> insert/upsert into ops.logline_acts
  -> update projections
  -> read status/report from projections
  -> produce evidence/ghost/receipt candidate as appropriate
```

O primeiro circuito real de entrega é:

```txt
local emit
-> local outbox
-> supabase check
-> outbox sync
-> ops.ingest_logline_act
-> remote ops.logline_acts count 0->1
-> repeat same act
-> count remains 1
```

Isto é o primeiro receipt técnico que importa para o LabKit.

## Camadas Oficiais do LabKit

```txt
engine:
  logline-lab binary, domain-agnostic

act-core:
  LogLine Act shape, branch, identity, validation, canonicalization, hashing

bootstrap:
  logline-lab init, lab home, local provisional cache/outbox

profiles:
  local-only
  supabase
  filesystem-manual only as export/debug/manual mode
  postgres
  future adapters

packages:
  santo-andre
  examples
  benches
  community labs

surfaces:
  CLI
  labd API
  optional web UI
  optional minilab.work consumer
```

A ordem é importante:

```txt
Act-core antes de CLI bonita.
Outbox antes de report bonito.
Remote spine antes de projections grandiosas.
Evidence antes de receipt.
Ghost antes de mentira.
```

## Repo Final do Pacote Instalável

Root obrigatório:

```txt
logline-lab-kit/
```

Estrutura planejada:

```txt
logline-lab-kit/
  Cargo.toml
  README.md
  AUTHORSHIP.md
  AI_NON_AUTHORITY.md
  GHOSTS.md
  RECOVERY_RECEIPT.md

  crates/
    logline-act/
      src/
        lib.rs
        model.rs
        branch.rs
        canonical.rs
        hash.rs
        validate.rs
    logline-lab-core/
      src/
        lib.rs
        manifest.rs
        admission.rs
        errors.rs
    logline-lab-local/
      src/
        lib.rs
        sqlite.rs
        outbox.rs
        retry.rs
    logline-lab-supabase/
      src/
        lib.rs
        client.rs
        ingest.rs
        idempotency.rs
    logline-lab-projectors/
      src/
        lib.rs
        registry.rs
        audit.rs
        ghosts.rs
        evidence.rs
        receipts.rs
    logline-lab-clock/
      src/
        lib.rs
        tick.rs
        scheduler.rs
    logline-lab-cli/
      src/
        main.rs
        commands/
    logline-lab-labd/
      src/
        main.rs
        routes/
    logline-lab-hermes/
      src/
        lib.rs
        boundary.rs
        workorder.rs

  supabase/
    migrations/
      0001_ops_logline_acts.sql
      0002_registry.sql
      0003_audit_views.sql
      0004_lab_observability.sql
      0005_evidence.sql
      0006_receipts.sql
      0007_workorders.sql
      0008_authz.sql
      0009_functions_projectors.sql
      0010_rls_safe_reads.sql

  templates/
    manifests/
    schemas/
    hooks/

  examples/
    acts/
    ghosts/
    evidence/
    receipts/

  docs/
    00-overview.md
    01-install.md
    02-supabase-spine.md
    03-cli.md
    04-labd.md
    05-manifest.md
    06-projectors.md
    07-ghosts.md
    08-evidence.md
    09-receipts.md
    10-clock.md
    11-hooks.md
    12-dispatch-and-hermes.md
    13-study-benches.md
    14-frontends.md

  benches/
    acts/
    runtimes/
    engines/
    projections/
    ghosts/
    receipts/
    intervals/
    experiments/

  reports/
    templates/
      daily-lab-state.md
      expedition.md
      conformance.md
      audit.md
      learning.md
```

## Rust Crates

### logline-act

Responsável por:

- LogLine Act shape
- Branch selection (ok/doubt/not)
- Identity computation
- Canonicalization
- Hashing
- Validation

### logline-lab-core

Responsável por:

- Lab manifest parsing/validation
- Admission logic
- Core errors
- Domain models

### logline-lab-local

Responsável por:

- SQLite como provisional cache/outbox
- Outbox queue
- Retry state
- Local buffer operations

### logline-lab-supabase

Responsável por:

- Supabase client
- Remote ingest function
- Idempotency handling
- Outbox sync

### logline-lab-projectors

Responsável por:

- Registry projections
- Audit views
- Ghost registry
- Evidence views
- Receipt views

### logline-lab-clock

Responsável por:

- Clock tick acts
- Scheduler
- Due checks
- Time-based triggers

### logline-lab-cli

Responsável por:

- CLI commands
- User interaction
- Command parsing
- Output formatting

### logline-lab-labd

Responsável por:

- Daemon/API server
- HTTP routes
- Status endpoints
- Act endpoints
- Ghost endpoints
- Evidence endpoints

### logline-lab-hermes

Responsável por:

- Execution boundary
- Workorder flow
- Dispatch logic
- Hermes contract

## CLI Commands

Command matrix:

```txt
logline-lab init
logline-lab doctor
logline-lab status
logline-lab act emit
logline-lab act get
logline-lab act list
logline-lab ghost list
logline-lab ghost open
logline-lab ghost close
logline-lab evidence add
logline-lab evidence list
logline-lab receipt prepare
logline-lab receipt list
logline-lab report generate
logline-lab report list
logline-lab canon status
logline-lab canon check
logline-lab projector run
logline-lab projector list
logline-lab hook run
logline-lab hook list
logline-lab clock tick
logline-lab clock status
logline-lab dispatch prepare
logline-lab workorder prepare
logline-lab workorder list
```

## Supabase Migrations

Migrations em ordem:

```txt
0001_ops_logline_acts.sql
0002_registry.sql
0003_audit_views.sql
0004_lab_observability.sql
0005_evidence.sql
0006_receipts.sql
0007_workorders.sql
0008_authz.sql
0009_functions_projectors.sql
0010_rls_safe_reads.sql
```

## Milestones

### Milestone 1: Core Act Engine

- logline-act crate
- Act shape, branch, identity, validation
- Canonicalization, hashing
- Basic tests

### Milestone 2: Local Outbox

- logline-lab-local crate
- SQLite provisional cache
- Outbox queue
- Retry state
- Basic CLI emit

### Milestone 3: Supabase Spine

- logline-lab-supabase crate
- Supabase client
- Remote ingest
- Idempotency
- Outbox sync
- First end-to-end emit

### Milestone 4: Projections

- logline-lab-projectors crate
- Registry projections
- Audit views
- Ghost registry
- Evidence views
- Receipt views

### Milestone 5: Clock

- logline-lab-clock crate
- Clock tick acts
- Scheduler
- Due checks

### Milestone 6: CLI Completa

- logline-lab-cli crate
- Full command matrix
- Error handling
- Output formatting

### Milestone 7: labd API

- logline-lab-labd crate
- Daemon server
- HTTP routes
- Status endpoints
- Act endpoints
- Ghost endpoints
- Evidence endpoints

### Milestone 8: Hermes Boundary

- logline-lab-hermes crate
- Execution boundary
- Workorder flow
- Dispatch logic

### Milestone 9: Experience

- Templates
- Examples
- Benches
- Reports
- Docs completas

## PR Plan

PRs devem seguir ordem de dependência:

1. Core Act Engine
2. Local Outbox
3. Supabase Spine
4. Projections
5. Clock
6. CLI Completa
7. labd API
8. Hermes Boundary
9. Experience

Cada PR deve ter:

- Clear scope
- Tests
- Docs updates
- Migration scripts se necessário

## Forbidden Marker Scan

Proibir:

```txt
"primitive system" -> usar "Act-centered operational semantics"
"artifact" como categoria válida -> não usar
"Supabase = spine oficial" como regra universal -> é v0/Santo André profile
"local SQL universal" -> é profile-specific
"selected_branch/status" como canon -> é metadata/projection
"runtime envelope" como décimo slot -> é provenance metadata/pack practice
```

## v0 vs Genérico

Este blueprint é para v0 com Supabase profile.

Elementos v0/Santo André/Supabase profile:

- ops.logline_acts como spine oficial
- Supabase migrations
- Supabase client
- Outbox sync para Supabase

Elementos genéricos:

- Act shape (universal)
- Local outbox/cache (todos os profiles)
- Projection logic (universal)
- Clock discipline (universal)
- CLI interface (universal)
- labd API (universal)

Para outros profiles (local-only, postgres, filesystem), substituir:

- Supabase client por profile-specific client
- Supabase migrations por profile-specific migrations
- Outbox sync por profile-specific sync

## Recovery Gates

Recovery gates em pontos críticos:

- Após Milestone 3: primeiro end-to-end emit
- Após Milestone 4: projections funcionando
- Após Milestone 9: experiência completa

Cada gate deve ter:

- Verification steps
- Rollback plan
- Recovery protocol