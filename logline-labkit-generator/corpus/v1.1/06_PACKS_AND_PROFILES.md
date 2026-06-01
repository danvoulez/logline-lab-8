# 06_PACKS_AND_PROFILES.md

## Packs and Profiles

Este documento define a separação entre canon, pack, profile, lab e overlay no ecossistema LogLine.

## Canon

Canon é gramática mínima, Foundation, conformance.

Características:

- Pequeno
- Estável
- Não alterado por Lab local
- Fonte de verdade para forma LogLine

Exemplos:

```txt
canon/foundation.refs.yaml
canon/conformance.refs.yaml
```

## Pack

Pack é bundle opinativo com doutrina, dados, opiniões e golden configs.

Características:

- Rico em conteúdo específico
- Opinativo sobre prática
- Pode incluir doutrina, data, configs
- Não é oficial Foundation canon

Exemplos:

```txt
santo-andre
classroom-lab
contract-bench
runtime-observatory
simulation-bench
```

### Santo André

Santo André é pack recomendado pelo Dan, não pack oficial.

Santo André pode ser:

- Primeira instância/reference lab
- Exemplo de prática LogLine
- Package com doutrina específica

Santo André não é:

- Definição universal de LogLine Lab
- Canon Foundation
- Produto raiz

## Profile

Profile é backend/capability, configuração de infraestrutura e runtime.

Características:

- Define capacidades técnicas
- Configura storage, runtime, adapters
- Não contém doutrina de domínio
- É placa de infraestrutura

Exemplos:

```txt
logline-lab.practice.v0.yaml
santo-andre.practice.v0.yaml
personal-offline.practice.v0.yaml
```

### Profiles de Storage/Runtime

```txt
local-only
supabase
filesystem-manual only as export/debug/manual mode
postgres
future adapters
```

### Supabase Profile

Supabase spine é prática/profile do Santo André ou do LabKit v0, não regra universal de todos os Labs.

Regra:

```txt
Supabase é v0/Santo André profile spine, não Foundation canon ou regra universal
```

## Lab

Lab é instância declarada por Act.

Características:

- Declarado por LogLine Act
- Instância específica do kit
- Tem prática local
- Pode ter pack próprio

Exemplos:

```txt
Santo André Laboratory
Personal Offline Lab
Community Lab
Company Lab
```

### Personal Offline Lab

Personal Offline Lab também é um LogLine Lab completo, com pack próprio.

Não é "Lab de segunda categoria".

É Lab com pack/profile diferente.

## Overlay

Overlay é deployment privado com infra, identidade, política e gosto específicos.

Características:

- Infraestrutura específica
- Identidade e credenciais
- Política local
- Gosto/opinião de deployment

Exemplos:

```txt
dan.minilab.work
institution-specific policy
machines
identities
credentials
UX
```

## Frontend

Frontend é superfície opcional de leitura/controle.

Características:

- Opcional
- Substituível
- Consumidor de projeções
- Não governa semanticamente

Exemplos:

```txt
minilab.work
web UI
CLI
dashboard
```

## Fluxo de Carga

```txt
Canon loads
  → Pack interprets
  → Profile provides capability
  → Lab runs
  → Overlay adapta
  → Frontend expõe
```

## Separação Clara

```txt
Canon loads
  gramática, canon, conformance, receipt discipline

Pack interprets
  doutrina, dados, opiniões, golden configs

Profile provides capability
  backend, storage, runtime, adapters

Lab runs
  instância declarada por ato
  prática local
  act graph vivo

Overlay adapta
  infra, identidade, política, gosto

Frontend expõe
  superfície opcional de leitura/controle
```

## Regras de Separação

```txt
profile = backend/capability
package = doctrine, data, opinions, golden configs
lab = instância declarada
overlay = deployment privado
frontend = superfície opcional
```

## Exemplo Completo

```txt
Canon: LogLine Foundation
Pack: Santo André (recomendado, não oficial)
Profile: Supabase spine (v0, não universal)
Lab: Santo André Laboratory (primeira instância)
Overlay: dan.minilab.work (deployment privado)
Frontend: minilab.work (opcional)
```

## Personal Offline Lab

```txt
Canon: LogLine Foundation
Pack: Personal Offline (pack próprio)
Profile: local-only (profile específico)
Lab: Personal Offline Lab (instância completa)
Overlay: infra local
Frontend: CLI ou nenhum
```

## Regra de Ouro

```txt
Canon pequeno.
Packs ricos.
Profiles técnicos.
Labs declarados.
Overlays privados.
Frontends opcionais.
```

## Não Confundir

```txt
Santo André ≠ LogLine Lab Kit
Santo André ≠ Foundation canon
Santo André ≠ definição universal
Supabase ≠ regra universal de todos os Labs
Personal Offline Lab ≠ Lab de segunda categoria
minilab.work ≠ produto raiz
```

## Storage Profile-Specific

Storage é profile-specific, não canon universal.

```txt
Supabase profile = ops.logline_acts como spine oficial
Local-only profile = SQLite como buffer/outbox
Filesystem profile = export/debug/manual mode apenas
```

Nenhum profile é "a forma correta universal".

Cada profile serve a um caso de uso específico.

## Separação Clara Entre Packs e Profiles

Packs e profiles têm papéis distintos e não devem ser confundidos:

```txt
Pack = doctrine, data, opinions, golden configs (domínio)
Profile = backend, storage, runtime, adapters (infraestrutura)
```

Packs contêm opiniões sobre prática e domínio. Profiles contêm configurações técnicas de infraestrutura.

Um pack pode funcionar com múltiplos profiles. Um profile pode servir múltiplos packs.

Santo André é pack (doutrina/opinião). Supabase é profile (infraestrutura). Eles são camadas separadas.