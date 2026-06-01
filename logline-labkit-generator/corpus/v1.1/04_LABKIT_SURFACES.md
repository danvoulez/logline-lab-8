# 04_LABKIT_SURFACES.md

## LabKit Surfaces

Este documento organiza as superfícies do produto LogLine Lab Kit: Start, Practice, Study, Build, Reference.

## Superfícies do Produto

O produto tem cinco superfícies principais:

```txt
Start
Practice
Study
Build
Reference
```

## Start

### Função

Primeira experiência, instalação, initial session, getting started.

### Conteúdo

```txt
docs/00-overview.md
docs/01-install.md
docs/02-spine-profiles.md
docs/03-cli.md
docs/04-labd.md
docs/05-manifest.md
```

### O que cada superfície precisa conter

**00-overview.md**
- Product sentence
- Position (Foundation → Lab Kit → Lab Instance)
- Public promise
- First session preview
- Links para docs principais

**01-install.md**
- Prerequisites
- Installation steps
- Environment setup
- First run verification
- Troubleshooting básico

**02-spine-profiles.md**
- Spine profile concept (profile-specific storage)
- Available profiles (e.g., Supabase Santo André profile)
- Profile setup instructions
- Storage discipline (spine oficial vs local buffer)
- Profile vs canon distinction

**03-cli.md**
- CLI commands principais
- `logline-lab init`
- `logline-lab doctor`
- `logline-lab status`
- `logline-lab act emit`
- `logline-lab ghost list`
- Common workflows

**04-labd.md**
- labd daemon/API
- Routes principais
- Status endpoints
- Act endpoints
- Ghost endpoints
- Evidence endpoints

**05-manifest.md**
- Lab manifest structure
- Manifest fields
- Manifest examples
- Manifest validation
- Manifest evolution

## Practice

### Função

Experiência viva do Lab: Load → Declare → Observe → Emit → Project → Learn.

### Conteúdo

```txt
docs/06-projectors.md
docs/07-ghosts.md
docs/08-evidence.md
docs/09-receipts.md
docs/10-clock.md
docs/11-hooks.md
docs/12-dispatch-and-hermes.md
```

### O que cada superfície precisa conter

**06-projectors.md**
- Projection concept
- Projection types
- Projection examples
- Projection implementation
- Projection vs Act distinction

**07-ghosts.md**
- Ghost concept
- Ghost shape
- Ghost lifecycle
- Ghost examples
- Ghost discipline

**08-evidence.md**
- Evidence concept
- Evidence types
- Evidence recording
- Evidence vs receipt distinction
- Evidence examples

**09-receipts.md**
- Receipt concept
- Receipt shape
- Receipt preparation
- Receipt scope
- Receipt examples

**10-clock.md**
- Clock concept
- Clock discipline
- Clock tick acts
- Clock scheduling
- Clock examples

**11-hooks.md**
- Hook concept
- Hook types
- Hook implementation
- Hook examples
- Hook discipline

**12-dispatch-and-hermes.md**
- Dispatch concept
- Hermes boundary
- Workorder flow
- Dispatch examples
- Hermes contract

## Study

### Função

Benches para estudar acts, runtimes, engines, projections, ghosts, receipts, intervals, experiments.

### Conteúdo

```txt
docs/13-study-benches.md
benches/*/README.md
benches/*/examples/
benches/*/fixtures/
```

### O que cada superfície precisa conter

**13-study-benches.md**
- Bench concept
- Available benches
- How to use benches
- Bench discipline
- Bench contribution guide

**Benches individuais**
- acts/ - estudo de LogLine Acts
- runtimes/ - estudo de runtimes
- engines/ - estudo de engines
- projections/ - estudo de projections
- ghosts/ - estudo de ghosts
- receipts/ - estudo de receipts
- intervals/ - estudo de intervals
- experiments/ - estudo de experiments
- ai-to-act-translation/ - estudo de tradução AI→Act
- promotion-control/ - estudo de controle de promoção
- what-runs-natively/ - estudo do que roda nativamente acima de software

Cada bench deve ter:
- README.md com propósito
- examples/ com exemplos válidos
- fixtures/ com fixtures de teste
- invalid/ com exemplos inválidos
- exercises/ com exercícios práticos

## Build

### Função

Ferramentas para construir com LogLine: projectors, adapters, reports, hooks.

### Conteúdo

```txt
docs/14-frontends.md
projectors/
reports/templates/
hooks/default/
examples/
```

### O que cada superfície precisa conter

**14-frontends.md**
- Frontend concept
- Frontend as optional surface
- Frontend types (CLI, web UI, minilab.work)
- Frontend integration
- Frontend vs Lab distinction

**projectors/**
- Projector examples
- Projector templates
- Projector implementation guide
- Standards adapters

**reports/templates/**
- Report templates
- Report examples
- Report generation guide
- Report types (Daily Lab State, Expedition, etc.)

**hooks/default/**
- Default hooks
- Hook examples
- Hook implementation guide
- Hook discipline

**examples/**
- Complete examples
- Integration examples
- End-to-end examples
- Best practices

## Reference

### Função

Documentação de referência, canon, schemas, exemplos, conformance.

### Conteúdo

```txt
canon/
schemas/
manifests/
profiles/
examples/
conformance/
```

### O que cada superfície precisa conter

**canon/**
- foundation.refs.yaml
- conformance.refs.yaml
- Canon loading guide
- Canon vs practice distinction

**schemas/**
- logline-act.schema.json
- runtime-envelope.schema.json
- lab-manifest.schema.json
- ghost.schema.json
- evidence.schema.json
- receipt-candidate.schema.json
- dispatch-packet.schema.json
- hermes-workorder.schema.json
- execution-report.schema.json
- promotion-control.schema.json

**manifests/**
- lab.manifest.example.yaml
- santo-andre.manifest.example.yaml
- Manifest validation rules
- Manifest evolution guide

**profiles/**
- logline-lab.practice.v0.yaml
- santo-andre.practice.v0.yaml
- Profile concept
- Profile vs pack distinction
- Profile implementation guide

**examples/**
- acts/valid/
- acts/invalid/
- runtime-envelopes/
- ghosts/
- evidence/
- receipts/
- experiments/

**conformance/**
- Conformance test vectors
- Conformance fixtures
- Conformance running guide
- Conformance contribution guide

## Inventário de Produto

Este documento é inventário de produto: o que cada superfície precisa conter e por quê.

Não transforma superfície em fonte semântica. Cada superfície é organização de material, não fonte de verdade.

A fonte semântica continua sendo LogLine Acts no spine oficial.

## Superfície Não É Fonte

Superfície não é fonte semântica.

Superfície é:

- Organização de material
- Guia de experiência
- Inventário de recursos
- Documentação de referência

Superfície não é:

- Fonte de verdade
- Canon
- Authority semântica
- Substituto de Acts

## Docs 00..14

Os docs numerados 00..14 são a espinha dorsal da documentação do produto:

```txt
00-overview.md
01-install.md
02-spine-profiles.md
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
```

Cada doc tem função clara na experiência do produto.

## Benches

Benches são laboratórios de estudo:

```txt
acts/
runtimes/
engines/
projections/
ghosts/
receipts/
intervals/
experiments/
ai-to-act-translation/
promotion-control/
what-runs-natively/
```

Cada bench é foco de estudo específico.

## Reports

Reports são templates de saída:

```txt
Daily Lab State
Expedition
Conformance
Audit
Learning
```

Cada report template tem propósito específico.

## Hooks

Hooks são pontos de extensão:

```txt
default/
custom/
examples/
```

Hooks permitem extensão sem alterar core.

## Exemplos

Exemplos são material de aprendizado:

```txt
acts/valid/
acts/invalid/
runtime-envelopes/
ghosts/
evidence/
receipts/
experiments/
```

Exemplos mostram como usar o produto na prática.