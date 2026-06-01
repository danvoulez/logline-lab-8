# 01_LOG_LINE_ACT.md

## LogLine Act

LogLine Act é a unidade semântica central do modelo LogLine. Todo movimento semântico relevante deve poder aparecer como LogLine Act.

## Shape Canônico

Um LogLine Act tem nove slots:

```txt
who
did
this
when
confirmed_by
if_ok
if_doubt
if_not
status
```

### Descrição dos Slots

**who**: Quem originou o ato. Pode ser pessoa, sistema, runtime, procedimento ou identidade declarada.

**did**: O que foi feito. Verbo ou ação que qualifica o movimento semântico.

**this**: O conteúdo do ato. Payload, claim, pergunta, decisão, binding, contrato ou estado.

**when**: Quando o ato ocorreu. Timestamp ou referência temporal.

**confirmed_by**: Caminho de evidência ou testemunha. Preserva origem, método, runtime ou autoridade que confirma o ato.

**if_ok**: Caminho quando há suporte suficiente. Sucesso, validação, confirmação ou execução bem-sucedida.

**if_doubt**: Caminho de simulação, dúvida, falta de prova, planejamento ou proposta.

**if_not**: Caminho de negação, falha, rejeição ou contradição.

**status**: Estado do ato. Não esconde o ramo percorrido.

## Estados do Act

status é slot do Act.
O vocabulário de status pode ser disciplinado por canon/conformance/pack/prática.
A lista recomendada por um pack não deve ser confundida com ontologia universal.

Exemplos comuns:

```txt
candidate
admitted
ghosted
rejected
simulated
closed
```

**candidate**: Material que ainda não cruzou a fronteira de admissão do Lab.

**admitted**: Ato que foi admitido na prática declarada do Lab.

**ghosted**: Ato que preserva ausência ou bloqueio.

**rejected**: Ato que foi rejeitado por falha de validação, admissão ou política.

**simulated**: Ato que representa simulação, planejamento ou hipótese.

**closed**: Ato que atingiu fechamento escopado. Uma closure/receipt practice pode preservar selected_branch como metadata/projection, mas o Act canon permanece como shape de nove slots.

## Ramo Semântico

O Act declara rotas possíveis.
A prática/projection/runtime pode registrar qual rota foi selecionada.
Mas selected_branch não é slot do Act canon.

The LogLine Act contains the three semantic routes:

```txt
if_ok
if_doubt
if_not
```

The Act canon does not contain a selected_branch slot.

A practice, runtime, validator, admission process, receipt process, or projection MAY materialize a selected branch outside the Act canon as metadata or projection output.

selected_branch may be useful, but it is not part of the nine-slot Act.

Interpretação:

```txt
if_ok     = caminho com suporte suficiente
if_doubt  = caminho de simulação, dúvida, falta de prova, plano ou proposta
if_not    = caminho de negação, falha, rejeição ou contradição
```

`confirmed_by` é o pivô que preserva caminho de evidência/testemunha e escolhe ramo semântico.

## Act vs Provenance Metadata

O Act é conteúdo semântico. O envelope é provenance metadata ou pack/profile practice.

Envelope conceitual:

```yaml
runtime_id:
engine_id:
run_id:
admission_status:
emitted_at:
observed_at:
tuple_hash:
content_hash:
previous_act_refs: []
```

Regra:

```txt
The act is content.
The envelope is provenance metadata or pack/profile practice.
```

Runtime envelope é provenance metadata ou pack/profile practice. Não é um décimo slot semântico e não é parte do LogLine Act canon.

## Act como Unidade de Primeira Classe

O LogLine Act não é:

- Uma tabela auxiliar
- Um schema JSON qualquer
- Um arquivo local
- Uma row em projection
- Mais um contrato no meio de outros contratos

O LogLine Act é:

- A unidade semântica que todos os módulos tratam como primeira classe
- O contrato mínimo para movimento semântico preservável, projetável, questionável e fechável
- A fonte de verdade para projections, evidence, ghosts e receipts

## O que Pode Ser Representado como Act

```txt
claims
questions
probes
runtime results
decisions
bindings
permissions
contracts
experiments
receipts
ghosts
state changes
reports
lab instantiation
practice declaration
clock ticks
dispatch preparation
workorder admission
evidence attachment
ghost opening
receipt preparation
learning observations
```

## Candidate Feio

Candidate material pode ser feio. Isso é informação.

O Act original não precisa ser "melhorado". Projections qualificam leituras posteriores.

Se o Act é feio, talvez o evento fosse feio. Preservar a feiura pode ser informação.

Capture first. Improve legibility later. Ugly Candidates may be valid records of ugly events. Projections qualify readings; they do not rewrite the original Act.

The right to ugly registration is part of honest LogLine practice. Strictness belongs at promotion, execution, receipt, and closure — not at the capture door.

## O que Não É Slot do Act Canon

Estes elementos são prática/profile/metadata/projection, não slots do Act canon:

- Hashes
- runtime_id
- type hints
- selected_branch
- runtime envelope
- signing/checkpoints

Eles podem existir como envelope, metadata ou projection, mas não expandem os nove slots canônicos.

## Projections

Projection transforma Acts em leituras úteis:

```txt
registry
runtime view
evidence view
receipt view
ghost view
experiment view
audit view
report
standards adapter
```

Regra:

```txt
Projection gives domain meaning.
Projection is not semantic source.
```

## Act-Centered Operational Semantics

O modelo atual é tudo LogLine Act. "Primitive system" é framing antigo/errado.

Substituir "primitives" por "Act-centered operational semantics" em todos os contextos.