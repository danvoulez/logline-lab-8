# 05_AUTHORITY_AND_SCOPE.md

## Authority and Scope

Este documento estabelece a constituição operacional para este corpus, protegendo contra falsa autoridade e definindo limites claros entre canon, practice, implementation e recovery.

## Autoridade do Projeto

**Dan Amarilho** é o autor do projeto LogLine Lab.

Esta é a única autoridade de produto para o LogLine Lab Kit.

## LLMs Não São Autoridade

LLMs são ferramentas/escribas, não autoridade.

LLM-generated material não é:

- Canon
- Architecture decision
- Proof
- Receipt
- Validation
- Project ownership
- Source of truth

Transcripts são material histórico/contextual, não autoridade de produto.

LLM-generated documents (including this corpus) are not authority without explicit Dan approval.

## Docs e ADRs Gerados

Documentos e ADRs gerados por assistentes não decidem.

Eles podem ser:

- Material exploratório
- Probes de arquitetura
- Candidatos a implementação
- Registro de conversação

Eles não são:

- Decisão final
- Canon Foundation
- Authority de produto
- Receipt de implementação

## Lab Definition Não É Canon

Lab definition é prática local declarada.

Uma Lab instance pode declarar:

```txt
runtime envelopes required
content addressing required
Supabase profile enabled
clock discipline active
receipt policy strict
```

Mas não pode dizer que sua prática local virou LogLine Foundation.

## Pack Não É Oficial

Pack é bundle opinativo, não canon oficial.

Exemplos:

```txt
santo-andre
classroom-lab
contract-bench
runtime-observatory
simulation-bench
```

Santo André é pack recomendado pelo Dan, não pack oficial.

Santo André pode ser primeira instância/reference lab, não definição universal.

## Separação de Camadas

```txt
LogLine Foundation
  gramática, canon, conformance, receipt discipline

LogLine Lab Kit
  produto público / kit operacional / Act-centered operational semantics

Lab Instance
  qualquer instância declarada do kit
  exemplo: Santo André Laboratory

Local Overlay
  infra, identidade, política e gosto de uma implantação
  exemplo: dan.minilab.work

Frontend
  superfície opcional de leitura/controle
  exemplo: minilab.work
```

## Hard Boundary

```txt
Santo André Laboratory é primeira instância/reference lab.
minilab.work é cockpit/superfície de uma instância.
Nenhum dos dois é o produto raiz.
```

## Recovery Não É Release

Recovery protocol descreve como recuperar corpus contaminado sem falsa autoridade.

Recovery não é release.

Recovery não é produto novo.

Recovery é protocolo de restauração de produto existente.

## Canon vs Practice vs Recovery

### Canon

Canon é gramática mínima, Foundation, conformance.

Canon não é alterado por Lab local.

### Practice

Practice é disciplina operacional declarada por Lab instance.

Practice pode adicionar:

- Práticas específicas
- Projections
- Policies
- Validators
- Adapters
- Reports
- Vocabulários
- Entendimentos específicos

Practice não altera Foundation canon.

### Recovery

Recovery é protocolo para restaurar corpus contaminado.

Recovery não cria novo produto.

Recovery não altera Foundation canon.

## Regras de Autoridade

```txt
Dan Amarilho is the author of the LogLine Lab project.
LLMs are tools only.
LLM-generated material is not canon, architecture decision, proof, receipt, validation, or project ownership.
Transcripts are historical/contextual material, not product authority.
Lab definition is not canon.
Pack is not official.
Recovery is not release.
```

## Regras de Escopo

```txt
LogLine Foundation = gramática/canon
LogLine Lab Kit = kit operacional instalável
Lab Instance = instância declarada por ato
Local Overlay = deployment privado
Frontend = superfície opcional
```

## Regras de Storage

```txt
Official LogLine Acts live in the online database.
SQLite is provisional only.
File storage is not official storage.
JSON is not official storage.
JSONL is not official storage.
There is no official acts/*.json spine.
There is no official receipts/*.json spine.
There is no official ghosts/*.json spine.
```

## Regras de Produto

```txt
frontend is optional
minilab.work can consume the kit but is not the kit
Santo André can be first instance but is not the universal definition
Hermes is execution boundary, not product identity
Supabase is v0/Santo André profile spine, not Foundation canon or universal rule
projections are derived read models, never semantic source
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

## Regra de Ouro

```txt
Se o texto tenta definir a realidade universal, desconfiar.
Se ele descreve uma prática, pack, profile, recovery ou projection, salvar.
```