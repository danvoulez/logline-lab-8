# 03_LABKIT_EXPERIENCE_LOOP.md

## LabKit Experience Loop

Este documento descreve a experiência viva do LogLine Lab como jornada operacional: intenção → Candidate → Act graph → projection → ghost/evidence/receipt/report.

## Lab como Act Graph Vivo

LogLine Lab não é uma coleção de módulos. É um act graph vivo com superfícies, envelopes, projectors, packages e rotinas ao redor.

A unidade semântica do produto é o LogLine Act. Tudo que importa semanticamente no Lab deve poder aparecer como LogLine Act: declaração de prática, intenção congelada, experimento, tick de clock, ghost aberto, evidência registrada, receipt preparado, probe executado, dispatch preparado, report gerado, aprendizado observado.

## Loop de Experiência Essencial

O loop essencial do produto é:

```txt
intenção humana
→ Translator
→ experimento candidato
→ LogLine Acts candidatos
→ prática declarada
→ primeiro ato emitido
→ ghosts nomeados
→ probes sugeridos/rodados
→ evidência registrada
→ receipt preparado ou bloqueado
→ clock força revisão
→ report mostra estado/aprendizado
```

Este loop é o produto em movimento.

## Experiment UX

Experiment é a unidade acima do act graph.

```txt
Experiment = canon + hooks + loop + clock
```

Slots obrigatórios:

```txt
question/hypothesis
canon
hooks
loop dynamics
clock
machine binding / substrate / scale
```

O experimento organiza a prática, mas os movimentos continuam sendo registrados como atos.

## Translator

Translator transforma intenção humana em estrutura LogLine Lab.

Translator não é autoridade. Translator não fecha verdade. Translator não substitui evidence, receipt ou runtime observation.

Outputs esperados:

```txt
Lab intent brief
Experiment candidate
LogLine act candidates
Ghost candidates
Probe suggestions
Manifest patch suggestion
Hook suggestions
```

Bloqueios obrigatórios:

```txt
hypothesis without metric
success without scope
canon confused with hook
loop without if_doubt
clock without stop condition
infra without scale fitness
receipt without evidence
```

## Clock

O Lab Clock é pulso institucional.

Regra:

```txt
The Lab Clock does not primarily schedule jobs.
The Lab Clock emits timed LogLine Acts that may lead to jobs.
```

Fluxo:

```txt
clock tick act
→ due check projection
→ probe candidate
→ dispatch candidate
→ admitted workorder, if allowed
→ evidence
→ report/ghost/receipt
```

O clock dá vida ao Lab porque faz o tempo aparecer como ato.

## Ghosts

Ghost é ausência estruturada.

Ghost não é erro genérico. Ghost é objeto de primeira classe.

Ghost shape:

```txt
ghost_id
opened_at
opened_by
organ
process
claim_blocked
missing
why_it_blocks
owner
next_act
closure_condition
status
source_refs
evidence_refs
```

Ghosts impedem mentira porque impedem fechamento sobre ausência.

Ghosts v0 conhecidos incluem:

```txt
lab-manifest-schema-unformalized
runtime-envelope-practice-unformalized
runtime-registry-projector-unimplemented
ghost-registry-template-unimplemented
receipt-template-unimplemented
interval-semantics-unformalized
standards-adapter-map-unimplemented
santo-andre-first-lab-manifest-missing
lab-clock-implementation-missing
outbox-supabase-bridge-missing
labd-serve-missing
experiment-ux-unimplemented
translator-unimplemented
```

## Evidence

Evidence é observação referenciável.

Evidence não é receipt. Model output não é evidence por si só. Runtime success não é receipt.

Evidence record:

```txt
evidence_id
kind
produced_by
observed_at
runtime
claim_ref
act_ref
payload_ref
hash
redaction_status
secret_redacted
scope
limits
```

## Receipts

Receipt é fechamento escopado projetado de acts admitidos e evidence.

Receipt não é sucesso genérico. Receipt não esconde ramo.

Receipt shape:

```txt
receipt_id
scope
closed_acts
evidence_refs
ghosts_remaining
limits
produced_by
produced_at
status
```

## Projections

Projections transformam acts em leituras úteis.

Projections podem produzir:

```txt
registry
runtime registry
ghost registry
evidence views
receipt views
experiment views
audit views
reports
standards adapters
```

Regra:

```txt
Projection gives domain meaning.
Projection is not semantic source.
```

## Daily Lab State / Expedition

O Lab opera em ritmo diário através de Daily Lab State / Expedition.

Fluxo:

```txt
clock tick
→ due check
→ probe execution
→ evidence collection
→ ghost review
→ receipt preparation
→ report generation
→ learning capture
```

## First Session

A primeira experiência do Lab não é infraestrutura.

A experiência desejada:

```txt
pessoa lê poucos docs/guide
fala com Translator sobre algo da área dela
Translator congela intenção em forma profissional LogLine
define hypothesis / canon / hooks / loop / clock / machine binding
Lab clock roda loops 24/7
Operator opera máquina real
terminal mostra trabalho real
evidence / ghost / receipt nascem
```

A pessoa não está "usando uma CLI". Ela está praticando LogLine num laboratório vivo.

## Jornada Operacional

### 1. Intenção

Operador tem intenção: estudar algo, construir algo, provar algo, duvidar de algo.

### 2. Translator

Translator ajuda a congelar intenção em forma LogLine profissional.

### 3. Experiment Candidate

Experiment candidate emerge com hypothesis, canon, hooks, loop, clock, machine binding.

### 4. LogLine Acts Candidates

Primeiros acts candidatos são gerados.

### 5. Practice Declaration

Lab manifest é declarado com practice profile, runtime procedures, projectors, policies.

### 6. First Act Emitted

Primeiro ato é emitido para o spine oficial.

### 7. Ghosts Named

Ghosts são abertos para o que não pode ser fechado ainda.

### 8. Probes Suggested/Executed

Probes são sugeridas e executadas para testar claims.

### 9. Evidence Registered

Evidence é registrada com origem, escopo, limites.

### 10. Receipt Prepared or Blocked

Receipt é preparado se houver evidence suficiente, ou bloqueado se não.

### 11. Clock Forces Review

Clock força revisão periódica de ghosts, probes, evidence.

### 12. Report Shows State/Learning

Report mostra estado atual, aprendizado, próximos passos.

## Experiência Não É

A experiência do Lab não é:

```txt
apenas CLI
apenas Supabase
apenas schema
apenas infraestrutura
apenas dashboard
apenas framework de agente
```

A experiência é:

```txt
prática LogLine viva
act graph em movimento
ghosts nomeados
evidence registrada
receipts escopados
reports gerados
aprendizado capturado
```

## Translator Não É Autoridade

Translator não é autoridade. Translator não fecha verdade. Translator não substitui evidence, receipt ou runtime observation.

Translator é ferramenta para:

```txt
congelar intenção
estruturar experimento
gerar candidates
sugerir probes
```

Translator não é:

```txt
fonte de canon
autoridade de produto
substituto de evidence
substituto de receipt
substituto de runtime observation
```

## Clock Não É Cron

Clock não é cron como deus invisível.

Clock emite timed LogLine Acts que podem levar a jobs.

Clock dá ritmo institucional ao Lab.

## Ghosts Não São Erros

Ghosts não são erros genéricos. Ghosts são ausência estruturada.

Ghosts impedem mentira porque impedem fechamento sobre ausência.

Ghosts são objetos de primeira classe no Lab.

## Evidence Não É Receipt

Evidence é observação referenciável.

Receipt é fechamento escopado.

Evidence não é automaticamente receipt.

Model output não é evidence por si só.

Runtime success não é receipt.

## Reports Não São Enfeite

Report não é enfeite. Report é a forma do Lab mostrar o que estudou, o que construiu, o que provou, o que não provou e o que precisa de próxima probe.

## Loop Contínuo

O loop do Lab é contínuo:

```txt
intenção → Translator → experiment → acts → ghosts → probes → evidence → receipts → clock → reports → learning → nova intenção
```

O Lab está sempre em movimento, sempre aprendendo, sempre duvidando, sempre preservando o que não pode ser fechado.