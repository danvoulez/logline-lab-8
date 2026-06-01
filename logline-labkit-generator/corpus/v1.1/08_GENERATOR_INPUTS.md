# 08_GENERATOR_INPUTS.md

## Generator Inputs

Este documento prepara as entradas para um gerador futuro, servindo como contrato de entrada para gerador, não como gerador em si.

## Propósito

Preparar catálogs e especificações que um gerador pode usar para:

- Gerar código de crate
- Gerar schemas
- Gerar migrations
- Gerar projectors
- Gerar benches
- Gerar templates
- Gerar testes
- Gerar documentação

## Restrições do Gerador

O gerador não deve:

- Inventar produto
- Inventar canon
- Gerar fake receipt
- Decidir arquitetura
- Substituir autoridade do Dan

O gerador deve:

- Seguir catálogos fornecidos
- Respeitar constraints especificados
- Gerar código alinhado com blueprint
- Gerar material que passa acceptance criteria

## Catálogos de Entrada

### 1. Product Catalog

**Arquivo:** `product.yaml`

**Conteúdo:**

```yaml
name: LogLine Lab Kit
version: v0
author: Dan Amarilho
position: "LogLine Foundation is the grammar. LogLine Lab is the practice kit. A local lab is a declared instantiation."
thesis: "Study LogLine itself. Run it. Project it. Doubt it. Prove what can be proved. Preserve what cannot."
public_promise:
  - load LogLine references
  - load or declare practice
  - name runtimes or human procedures
  - emit or preserve LogLine Acts
  - keep acts separate from projections
  - run probes
  - record evidence
  - preserve ghosts
  - prepare scoped receipts
  - produce at least one Lab report
experience:
  first_session:
    - open the kit
    - read the product sentence
    - create or inspect a Lab manifest
    - load references and practice material
    - emit or preserve an act
    - project the act into a readable view
    - open a ghost for unresolved state
    - attach or reference evidence if available
    - prepare a receipt candidate only within scope
    - write the first Lab report
  operating_model:
    - Load
    - Declare
    - Observe
    - Emit
    - Project
    - Learn
surfaces:
  - Start
  - Practice
  - Study
  - Build
  Reference
```

### 2. Command Catalog

**Arquivo:** `commands.yaml`

**Conteúdo:**

```yaml
cli:
  name: logline-lab
  commands:
    init:
      description: Initialize a new Lab
      args:
        - name: manifest
          type: file
          required: false
    doctor:
      description: Check Lab health
      subcommands:
        - canon
        - storage
        - projections
    status:
      description: Show Lab status
    act:
      subcommands:
        emit:
          description: Emit a LogLine Act
          args:
            - name: file
              type: file
              required: true
        get:
          description: Get a specific Act
          args:
            - name: id
              type: string
              required: true
        list:
          description: List Acts
    ghost:
      subcommands:
        list:
          description: List ghosts
        open:
          description: Open a ghost
          args:
            - name: claim
              type: string
              required: true
        close:
          description: Close a ghost
          args:
            - name: id
              type: string
              required: true
    evidence:
      subcommands:
        add:
          description: Add evidence
          args:
            - name: file
              type: file
              required: true
        list:
          description: List evidence
    receipt:
      subcommands:
        prepare:
          description: Prepare receipt
          args:
            - name: scope
              type: string
              required: true
        list:
          description: List receipts
    report:
      subcommands:
        generate:
          description: Generate report
          args:
            - name: type
              type: string
              required: true
              choices: [daily-lab-state, expedition, conformance, audit, learning]
        list:
          description: List reports
    canon:
      subcommands:
        status:
          description: Show canon status
        check:
          description: Check canon conformance
    projector:
      subcommands:
        run:
          description: Run a projector
          args:
            - name: name
              type: string
              required: true
        list:
          description: List projectors
    hook:
      subcommands:
        run:
          description: Run a hook
          args:
            - name: name
              type: string
              required: true
        list:
          description: List hooks
    clock:
      subcommands:
        tick:
          description: Emit clock tick act
        status:
          description: Show clock status
    dispatch:
      subcommands:
        prepare:
          description: Prepare dispatch
          args:
            - name: workorder
              type: string
              required: true
    workorder:
      subcommands:
        prepare:
          description: Prepare workorder
          args:
            - name: dispatch
              type: string
              required: true
        list:
          description: List workorders
```

### 3. Schema Catalog

**Arquivo:** `schemas.yaml`

**Conteúdo:**

```yaml
schemas:
  logline-act:
    file: schemas/logline-act.schema.json
    description: LogLine Act canonical shape
    slots:
      - who
      - did
      - this
      - when
      - confirmed_by
      - if_ok
      - if_doubt
      - if_not
      - status
  runtime-envelope:
    file: schemas/runtime-envelope.schema.json
    description: Runtime envelope for provenance
    fields:
      - runtime_id
      - engine_id
      - run_id
      - admission_status
      - emitted_at
      - observed_at
      - tuple_hash
      - content_hash
      - previous_act_refs
  lab-manifest:
    file: schemas/lab-manifest.schema.json
    description: Lab manifest structure
    fields:
      - lab_name
      - purpose
      - steward
      - practice_profile
      - runtime_procedures
      - projectors
      - ghost_policy
      - receipt_policy
      - evidence_policy
      - proof_policy
      - clock_discipline
      - constraints
  ghost:
    file: schemas/ghost.schema.json
    description: Ghost structure
    fields:
      - ghost_id
      - opened_at
      - opened_by
      - organ
      - process
      - claim_blocked
      - missing
      - why_it_blocks
      - owner
      - next_act
      - closure_condition
      - status
      - source_refs
      - evidence_refs
  evidence:
    file: schemas/evidence.schema.json
    description: Evidence structure
    fields:
      - evidence_id
      - kind
      - produced_by
      - observed_at
      - runtime
      - claim_ref
      - act_ref
      - payload_ref
      - hash
      - redaction_status
      - secret_redacted
      - scope
      - limits
  receipt-candidate:
    file: schemas/receipt-candidate.schema.json
    description: Receipt candidate structure
    fields:
      - receipt_id
      - scope
      - closed_acts
      - evidence_refs
      - ghosts_remaining
      - limits
      - produced_by
      - produced_at
      - status
  dispatch-packet:
    file: schemas/dispatch-packet.schema.json
    description: Dispatch packet structure
    fields:
      - packet_id
      - dispatch_type
      - target
      - payload
      - created_at
      - status
  hermes-workorder:
    file: schemas/hermes-workorder.schema.json
    description: Hermes workorder structure
    fields:
      - workorder_id
      - dispatch_ref
      - execution_boundary
      - steps
      - created_at
      - started_at
      - completed_at
      - status
  execution-report:
    file: schemas/execution-report.schema.json
    description: Execution report structure
    fields:
      - report_id
      - workorder_ref
      - steps_executed
      - results
      - errors
      - evidence_refs
      - ghost_refs
      - completed_at
  promotion-control:
    file: schemas/promotion-control.schema.json
    description: Promotion control structure
    fields:
      - control_id
      - candidate_ref
      - promotion_criteria
      - evidence_required
      - approval_required
      - status
```

### 4. Migration Catalog

**Arquivo:** `migrations.yaml`

**Conteúdo:**

```yaml
migrations:
  supabase:
    profile: supabase
    order:
      - 0001_ops_logline_acts.sql
      - 0002_registry.sql
      - 0003_audit_views.sql
      - 0004_lab_observability.sql
      - 0005_evidence.sql
      - 0006_receipts.sql
      - 0007_workorders.sql
      - 0008_authz.sql
      - 0009_functions_projectors.sql
      - 0010_rls_safe_reads.sql
    0001_ops_logline_acts:
      description: Create ops.logline_acts table
      table: ops.logline_acts
      columns:
        - id
        - who
        - did
        - this
        - when
        - confirmed_by
        - if_ok
        - if_doubt
        - if_not
        - status
        - tuple_hash
        - content_hash
        - created_at
    0002_registry:
      description: Create registry tables
      tables:
        - registry.entities
        - registry.runtimes
        - registry.projectors
    0003_audit_views:
      description: Create audit views
      views:
        - audit.v_mobile_today
        - audit.acts_by_runtime
    0004_lab_observability:
      description: Create lab observability views
      views:
        - lab_observability.current_state
        - lab_observability.ghosts_open
    0005_evidence:
      description: Create evidence table
      table: evidence.evidence
    0006_receipts:
      description: Create receipts table
      table: receipts.receipts
    0007_workorders:
      description: Create workorders table
      table: dispatch.workorders
    0008_authz:
      description: Create authz tables
      tables:
        - authz.roles
        - authz.permissions
    0009_functions_projectors:
      description: Create projector functions
      functions:
        - project_registry
        - project_audit
        - project_ghosts
    0010_rls_safe_reads:
      description: Create RLS policies for safe reads
      policies:
        - acts_read
        - ghosts_read
        - evidence_read
```

### 5. Projector Catalog

**Arquivo:** `projectors.yaml`

**Conteúdo:**

```yaml
projectors:
  registry:
    - name: project_registry
      description: Project registry from acts
      input: ops.logline_acts
      output: registry.*
      frequency: on_act
  audit:
    - name: project_audit
      description: Project audit views
      input: ops.logline_acts
      output: audit.*
      frequency: on_act
    - name: v_mobile_today
      description: Project mobile acts today
      input: ops.logline_acts
      output: audit.v_mobile_today
      frequency: on_act
  ghosts:
    - name: project_ghosts
      description: Project ghost registry
      input: ops.logline_acts
      output: lab_observability.ghosts_open
      frequency: on_act
  evidence:
    - name: project_evidence
      description: Project evidence views
      input: evidence.evidence
      output: evidence.*
      frequency: on_evidence
  receipts:
    - name: project_receipts
      description: Project receipt views
      input: receipts.receipts
      output: receipts.*
      frequency: on_receipt
```

### 6. Bench Catalog

**Arquivo:** `benches.yaml`

**Conteúdo:**

```yaml
benches:
  acts:
    description: Study LogLine Acts
    examples:
      - valid/basic
      - valid/complex
      - invalid/missing_slots
      - invalid/invalid_branch
  runtimes:
    description: Study runtimes
    examples:
      - cli_runtime
      - labd_runtime
      - hermes_runtime
  engines:
    description: Study engines
    examples:
      - logline_engine
      - constitutional_runtime
  projections:
    description: Study projections
    examples:
      - registry_projection
      - audit_projection
      - ghost_projection
  ghosts:
    description: Study ghosts
    examples:
      - basic_ghost
      - complex_ghost
      - ghost_closure
  receipts:
    description: Study receipts
    examples:
      - basic_receipt
      - scoped_receipt
      - receipt_with_evidence
  intervals:
    description: Study intervals
    examples:
      - basic_interval
      - clock_interval
  experiments:
    description: Study experiments
    examples:
      - basic_experiment
      - clock_experiment
      - ai_to_act_experiment
  ai-to-act-translation:
    description: Study AI to Act translation
    examples:
      - basic_translation
      - complex_translation
      - translation_with_ghosts
  promotion-control:
    description: Study promotion control
    examples:
      - basic_promotion
      - promotion_with_evidence
      - promotion_rejection
  what-runs-natively:
    description: Study what runs natively above software
    examples:
      - human_act
      - institutional_act
      - legal_act
```

### 7. Acceptance Criteria

**Arquivo:** `acceptance.yaml`

**Conteúdo:**

```yaml
acceptance:
  product:
    - name: installable
      description: Kit must be installable
      test: ./test_install.sh
    - name: first_session
      description: First session must complete
      test: ./test_first_session.sh
    - name: emit_act
      description: Must emit act to online spine
      test: ./test_emit_act.sh
    - name: projections
      description: Projections must work
      test: ./test_projections.sh
    - name: ghosts
      description: Ghosts must work
      test: ./test_ghosts.sh
    - name: evidence
      description: Evidence must work
      test: ./test_evidence.sh
    - name: receipts
      description: Receipts must work
      test: ./test_receipts.sh
    - name: reports
      description: Reports must work
      test: ./test_reports.sh
    - name: benches
      description: Benches must be available
      test: ./test_benches.sh
  conformance:
    - name: canon_conformance
      description: Must pass canon conformance
      test: ./test_canon_conformance.sh
    - name: schema_conformance
      description: Must pass schema conformance
      test: ./test_schema_conformance.sh
  storage:
    - name: supabase_spine
      description: ops.logline_acts must be spine
      test: ./test_supabase_spine.sh
    - name: local_buffer
      description: SQLite must be buffer/outbox only
      test: ./test_local_buffer.sh
    - name: no_file_spine
      description: No file spine must exist
      test: ./test_no_file_spine.sh
```

## Constraints do Gerador

### Não Inventar Produto

Gerador não deve:

- Criar novos comandos não especificados
- Criar novos schemas não especificados
- Criar novas migrations não especificadas
- Criar novos projectors não especificados
- Criar novos benches não especificados

### Não Inventar Canon

Gerador não deve:

- Alterar LogLine Act shape
- Alterar Foundation canon
- Criar novo canon
- Interpretar canon de forma diferente

### Não Gerar Fake Receipt

Gerador não deve:

- Gerar receipt sem evidence
- Gerar receipt sem scope
- Gerar receipt sem ghosts_remaining
- Fingir closure onde não há evidence

### Seguir Blueprint

Gerador deve:

- Seguir estrutura de crates especificada
- Seguir ordem de migrations especificada
- Seguir command catalog especificado
- Seguir schema catalog especificado
- Respeitar acceptance criteria especificados

## Uso do Gerador

Gerador deve ler estes catálogos e gerar:

- Código de crate Rust
- Schemas JSON
- Migrations SQL
- Projector functions
- Bench examples
- Test templates
- Doc templates

Gerador não deve tomar decisões de arquitetura não especificadas nos catálogos.