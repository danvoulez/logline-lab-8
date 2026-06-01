# 11_IMPLEMENTATION_REPAIR_NOTES.md

## Implementation Repair Notes

Este documento transforma o patch `logline_lab_repair.patch` em checklist técnico aplicável ao recovery do pacote existente.

## Contexto do Patch

O patch corrige problemas específicos no código Rust existente, alinhando a implementação com o modelo atual de Act-centered operational semantics e storage discipline correta.

## Repairs Aplicados

### 1. Remoção do Crate `logline-lab-artifacts`

**Problema:**
- Crate `logline-lab-artifacts` foi criado sob ADR-0002
- "Artifact" não é categoria semântica válida no modelo atual
- ADR-0002 está desatualizado em relação ao modelo atual

**Repair:**
- Remover `crates/logline-lab-artifacts/` completamente
- Remover referência em `Cargo.toml` workspace members
- Remover dependências de outros crates

**Checklist:**
- [ ] Remover diretório `crates/logline-lab-artifacts/`
- [ ] Remover `"crates/logline-lab-artifacts"` de `Cargo.toml`
- [ ] Remover imports de `logline_lab_artifacts` de outros crates
- [ ] Remover chamadas a `ArtifactSpool` do código
- [ ] Remover `q_artifact_cleanup` queue se existir
- [ ] Atualizar documentação que referencia artifacts

### 2. Correção de Storage: SQLite como Buffer/Outbox

**Problema:**
- Código tratava SQLite como "spine oficial" (ADR-0002)
- Modelo atual: SQLite é provisional cache/outbox apenas
- Spine oficial é online database (ops.logline_acts)

**Repair:**
- Corrigir comentários que chamam SQLite de "ledger"
- Corrigir comentários que chamam SQLite de "truth"
- Corrigir variáveis que chamam SQLite de "ledger"
- Substituir linguagem por "provisional cache", "buffer", "outbox"

**Checklist:**
- [ ] Buscar por "ledger" em código e comentários
- [ ] Substituir "local SQLite ledger" por "local SQLite buffer/outbox"
- [ ] Substituir "local SQLite spine" por "local SQLite buffer/outbox"
- [ ] Substituir "Truth lives in the local SQLite ledger" por "Truth requires the online spine"
- [ ] Substituir "act recorded in local ledger" por "act staged in transient local buffer/outbox"
- [ ] Renomear `LocalLedger` para `LocalOutboxCache` ou `ProvisionalCache` se possível
- [ ] Renomear `LocalSpine` para `LocalOutboxCache` ou `ProvisionalCache` se possível
- [ ] Se rename não for possível sem build, deixar Ghost com explicação
- [ ] Atualizar `Cargo.toml` comment: "storage model: online spine truth + transient local buffer"

### 3. Remoção de FileLabStore

**Problema:**
- `FileLabStore` tratava arquivos como storage de Acts
- Arquivos não são storage oficial no modelo atual
- Arquivos são examples/fixtures/exports/debug apenas

**Repair:**
- Remover `FileLabStore` do código
- Remover imports de `FileLabStore`
- Remover chamadas a `FileLabStore`
- Substituir por operações de examples/fixtures se necessário

**Checklist:**
- [ ] Buscar por `FileLabStore` em código
- [ ] Remover implementação de `FileLabStore`
- [ ] Remover imports de `FileLabStore`
- [ ] Remover uso de `FileLabStore` em CLI e outros crates
- [ ] Substituir por `examples/` ou `fixtures/` se apropriado
- [ ] Atualizar documentação que referencia FileLabStore

### 4. Correção de Artifact Cleanup

**Problema:**
- Código tinha cleanup de artifacts com TTL de 24h
- Como "artifact" não é categoria válida, cleanup não faz sentido
- ADR-0002 que definia artifacts está desatualizado

**Repair:**
- Remover lógica de cleanup de artifacts
- Remover queue `q_artifact_cleanup` se existir
- Remover funções de GC de artifacts
- Remover TTL constants para artifacts

**Checklist:**
- [ ] Remover `TTL_SECONDS` constant para artifacts
- [ ] Remover `ArtifactSpool::gc` function
- [ ] Remover queue `q_artifact_cleanup` de Supabase migrations
- [ ] Remover scheduled jobs para artifact cleanup
- [ ] Remover testes de artifact GC
- [ ] Atualizar documentação que referencia artifact cleanup

### 5. Correção de Receipt File Output

**Problema:**
- CLI permitia output de receipt para arquivo
- Arquivos não são storage oficial no modelo atual
- Receipt deve ser projection, não arquivo

**Repair:**
- Desabilitar output de receipt para arquivo
- Permitir apenas stdout
- Remover `--output` option para receipt commands
- Adicionar erro se usuário tentar output para arquivo

**Checklist:**
- [ ] Remover `--output` option de `receipt seal` command
- [ ] Adicionar erro: "receipt file output disabled: stdout only; no local receipt artifacts"
- [ ] Remover `fs::write` para receipt output
- [ ] Manter apenas `println!` para stdout
- [ ] Atualizar help text para commands de receipt
- [ ] Atualizar exemplos que usam receipt file output

### 6. Correção de Claim Limits

**Problema:**
- CLI output dizia "act recorded in local ledger and queued for outbox; not yet delivered to ops.logline_acts; not a remote receipt"
- Linguagem incorreta: "local ledger" ou "local spine" sugere truth local
- Deve enfatizar que local buffer não é truth

**Repair:**
- Corrigir mensagem de claim limits
- Enfatizar que local buffer não é truth
- Enfatizar que não é receipt até Supabase aceitar

**Checklist:**
- [ ] Substituir "act recorded in local ledger" por "act staged in transient local buffer/outbox"
- [ ] Substituir "act recorded in local spine" por "act staged in transient local buffer/outbox"
- [ ] Adicionar "not truth, not a receipt, not closed until Supabase accepts it"
- [ ] Manter "not yet delivered to ops.logline_acts"
- [ ] Manter "not a remote receipt"
- [ ] Atualizar outras mensagens similares no código

### 7. Rebaixamento de ADR Falso

**Problema:**
- Código referenciava ADR-0002 como autoridade
- ADR-0002 está desatualizado em relação ao modelo atual
- ADRs gerados por assistente não são autoridade

**Repair:**
- Remover referências a ADR-0002 como autoridade
- Marcar ADR-0002 como desatualizado se mantido
- Substituir por referência ao modelo atual

**Checklist:**
- [ ] Remover "# ADR-0002 storage model" de `Cargo.toml`
- [ ] Substituir por "# storage model: online spine truth + transient local buffer"
- [ ] Remover referências a ADR-0002 em comentários
- [ ] Remover referências a ADR-0002 em documentação
- [ ] Se ADR-0002 arquivo existir, marcar como deprecated
- [ ] Adicionar note explicando modelo atual

## Aplicação do Repair

### Ordem de Aplicação

1. Remover crate `logline-lab-artifacts`
2. Corrigir linguagem de storage (SQLite)
3. Remover `FileLabStore`
4. Remover artifact cleanup
5. Corrigir receipt file output
6. Corrigir claim limits
7. Rebaixar ADR falso

### Verificação Após Repair

Após aplicar repairs, verificar:

- [ ] Build compila sem erros
- [ ] Tests passam
- [ ] CLI commands funcionam
- [ ] Linguagem de storage está correta
- [ ] Não há referências a "artifact" como categoria válida
- [ ] Não há referências a ADR-0002 como autoridade
- [ ] SQLite é tratado como buffer/outbox, não ledger ou spine
- [ ] Arquivos não são tratados como storage oficial

## Notas de Implementação

### Rename Seguro

Se `LocalLedger` precisar ser renomeado para `LocalOutboxCache`:

- Fazer rename em um commit separado
- Verificar se há dependências externas
- Verificar se há scripts que usam o nome antigo
- Atualizar documentação
- Atualizar exemplos

Se rename não for possível sem build executado:

- Deixar Ghost com explicação
- Adicionar TODO comment explicando rename necessário
- Documentar em GHOSTS.md

### Profile-Specific

Estes repairs são específicos para o pacote existente com Supabase profile.

Para outros profiles (local-only, postgres, filesystem):

- Repairs de storage podem ser diferentes
- SQLite pode ter papel diferente em local-only profile
- FileLabStore pode fazer sentido em filesystem-manual profile

Aplicar repairs com contexto de profile em mente.

## Recovery Context

Este repair é aplicável ao recovery do pacote existente do archive.

Não é aplicável a nova implementação do zero.

Nova implementação deve seguir blueprint corrigido desde o início, sem precisar destes repairs.

## LLM Non-Authority

Este repair document é derivado de patch gerado por assistente.

Não é:

- Architecture decision
- Canon
- Authority de produto

É:

- Checklist técnico para alinhar implementação existente com modelo atual
- Material de recovery para pacote contaminado
- Guia prático para correções específicas