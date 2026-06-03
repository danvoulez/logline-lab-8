create schema if not exists ops;
create schema if not exists audit;
create schema if not exists registry;
create schema if not exists authz;
create schema if not exists receipts;
create schema if not exists evidence;
create schema if not exists lab_observability;
create schema if not exists workorders;
create schema if not exists extensions;
create schema if not exists pgmq;

create extension if not exists pgcrypto with schema extensions;
create extension if not exists pgmq with schema pgmq;

create table if not exists ops.logline_acts (
  id uuid primary key default extensions.gen_random_uuid(),
  who text not null,
  did text not null,
  "this" jsonb not null,
  "when" timestamptz not null default now(),
  confirmed_by jsonb not null default '{}'::jsonb,
  if_ok jsonb not null default '{}'::jsonb,
  if_doubt jsonb not null default '{}'::jsonb,
  if_not jsonb not null default '{}'::jsonb,
  status text not null,
  runtime_envelope jsonb not null default '{}'::jsonb,
  tuple_hash text,
  content_hash text unique,
  previous_act_refs text[] not null default '{}'::text[],
  evidence_state text not null default 'declared',
  promotion_state text not null default 'candidate',
  created_at timestamptz not null default now(),
  constraint logline_act_who_nonempty check (length(trim(who)) > 0),
  constraint logline_act_did_nonempty check (length(trim(did)) > 0),
  constraint logline_act_status_nonempty check (length(trim(status)) > 0),
  constraint logline_evidence_redaction_required
    check (
      did <> 'report_execution_result'
      or coalesce(("this" ->> 'secret_redacted')::boolean, false) = true
    ),
  constraint logline_receipt_candidate_requires_evidence
    check (
      did <> 'prepare_receipt_candidate'
      or (
        jsonb_typeof("this" -> 'evidence_refs') = 'array'
        and jsonb_array_length("this" -> 'evidence_refs') > 0
      )
    )
);

create unique index if not exists logline_acts_content_hash_uk
  on ops.logline_acts (content_hash);
create index if not exists logline_acts_did_idx
  on ops.logline_acts (did);
create index if not exists logline_acts_status_idx
  on ops.logline_acts (status);
create index if not exists logline_acts_this_gin_idx
  on ops.logline_acts using gin ("this");
create index if not exists logline_acts_when_idx
  on ops.logline_acts ("when" desc);

create or replace function ops.forbid_act_mutation()
returns trigger
language plpgsql
as $$
begin
  raise exception 'ops.logline_acts is append-only; emit a new LogLine Act instead';
end;
$$;

drop trigger if exists logline_acts_forbid_update_delete on ops.logline_acts;
create trigger logline_acts_forbid_update_delete
before update or delete on ops.logline_acts
for each row execute function ops.forbid_act_mutation();

create table if not exists registry.entities (
  entity_id text primary key,
  entity_type text not null default 'unknown',
  source_act_id uuid references ops.logline_acts(id),
  profile jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);

create table if not exists registry.passports (
  passport_id text primary key,
  entity_id text,
  source_act_id uuid references ops.logline_acts(id),
  payload jsonb not null default '{}'::jsonb
);

create table if not exists registry.visas (
  visa_id text primary key,
  entity_id text,
  source_act_id uuid references ops.logline_acts(id),
  payload jsonb not null default '{}'::jsonb
);

create table if not exists registry.auth_bindings (
  binding_id text primary key,
  entity_id text,
  source_act_id uuid references ops.logline_acts(id),
  payload jsonb not null default '{}'::jsonb
);

create table if not exists registry.links (
  link_id text primary key,
  source_act_id uuid references ops.logline_acts(id),
  payload jsonb not null default '{}'::jsonb
);

create table if not exists registry.runtimes (
  runtime_id text primary key,
  runtime_type text not null default 'unknown',
  source_act_id uuid references ops.logline_acts(id),
  profile jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);

create or replace function ops.ingest_logline_act(payload jsonb)
returns ops.logline_acts
language plpgsql
security definer
set search_path to 'ops', 'public'
as $$
declare
  result ops.logline_acts;
  v_content_hash text := payload ->> 'content_hash';
begin
  if coalesce(trim(payload ->> 'who'), '') = '' then
    raise exception 'ingest_logline_act: who is required';
  end if;
  if coalesce(trim(payload ->> 'did'), '') = '' then
    raise exception 'ingest_logline_act: did is required';
  end if;
  if coalesce(trim(payload ->> 'status'), '') = '' then
    raise exception 'ingest_logline_act: status is required';
  end if;

  if v_content_hash is not null then
    select * into result from ops.logline_acts where content_hash = v_content_hash;
    if found then
      return result;
    end if;
  end if;

  insert into ops.logline_acts
    (who, did, "this", "when", confirmed_by, if_ok, if_doubt, if_not, status,
     runtime_envelope, tuple_hash, content_hash)
  values (
    payload ->> 'who',
    payload ->> 'did',
    coalesce(payload -> 'this', '""'::jsonb),
    coalesce((payload ->> 'when')::timestamptz, now()),
    coalesce(payload -> 'confirmed_by', '{}'::jsonb),
    coalesce(payload -> 'if_ok', '{}'::jsonb),
    coalesce(payload -> 'if_doubt', '{}'::jsonb),
    coalesce(payload -> 'if_not', '{}'::jsonb),
    payload ->> 'status',
    coalesce(payload -> 'runtime_envelope', '{}'::jsonb),
    payload ->> 'tuple_hash',
    v_content_hash
  )
  on conflict (content_hash) do nothing
  returning * into result;

  if result.id is null and v_content_hash is not null then
    select * into result from ops.logline_acts where content_hash = v_content_hash;
  end if;

  return result;
end;
$$;

do $$
begin
  perform pgmq.create('q_lab_outbox');
exception when duplicate_table then
  null;
end;
$$;

do $$
begin
  perform pgmq.create('q_projection_rebuild');
exception when duplicate_table then
  null;
end;
$$;

do $$
begin
  perform pgmq.create('q_receipts');
exception when duplicate_table then
  null;
end;
$$;

do $$
begin
  perform pgmq.create('q_workorders');
exception when duplicate_table then
  null;
end;
$$;

do $$
begin
  perform pgmq.create('q_artifact_cleanup');
exception when duplicate_table then
  null;
end;
$$;

create or replace view audit.v_recent_acts
with (security_invoker = true)
as
select
  id,
  who,
  did,
  "this",
  "when",
  confirmed_by,
  if_ok,
  if_doubt,
  if_not,
  status,
  runtime_envelope,
  tuple_hash,
  content_hash,
  previous_act_refs,
  evidence_state,
  promotion_state,
  created_at
from ops.logline_acts
order by "when" desc
limit 100;

create or replace view audit.v_open_ghosts
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'open_ghost' and status = any (array['open', 'declared'])
order by "when" desc;

create or replace view audit.v_receipt_candidates
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'prepare_receipt_candidate'
order by "when" desc;

create or replace view audit.v_daily_lab_state
with (security_invoker = true)
as
select
  now() as observed_at,
  (select count(*) from ops.logline_acts)::bigint as acts,
  (select count(*) from audit.v_open_ghosts)::bigint as open_ghosts,
  (select count(*) from audit.v_receipt_candidates)::bigint as receipt_candidates;

create or replace view audit.v_mobile_today
with (security_invoker = true)
as
select
  observed_at,
  acts,
  open_ghosts,
  receipt_candidates
from audit.v_daily_lab_state;

create or replace view authz.visas
with (security_invoker = true)
as
select visa_id, entity_id, source_act_id, payload
from registry.visas;

create or replace view evidence.artifacts
with (security_invoker = true)
as
select id, "this" -> 'artifact_refs' as artifact_refs, "when"
from ops.logline_acts
where "this" ? 'artifact_refs';

create or replace view evidence.command_outputs
with (security_invoker = true)
as
select id, "this" ->> 'stdout_ref' as stdout_ref, "this" ->> 'stderr_ref' as stderr_ref, "when"
from ops.logline_acts
where "this" ? 'stdout_ref' or "this" ? 'stderr_ref';

create or replace view evidence.records
with (security_invoker = true)
as
select id, who, did, "this", "when", confirmed_by, status
from ops.logline_acts
where did = any (array['report_execution_result', 'add_evidence', 'observe', 'record_evidence'])
  or evidence_state = 'observed'
order by "when" desc;

create or replace view evidence.runtime_observations
with (security_invoker = true)
as
select id, who, did, "this", "when", confirmed_by, status
from evidence.records
where did = any (array['report_execution_result', 'report_runtime_status']);

create or replace view lab_observability.current_state
with (security_invoker = true)
as
select observed_at, acts, open_ghosts, receipt_candidates
from audit.v_daily_lab_state;

create or replace view lab_observability.ghosts
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from audit.v_open_ghosts;

create or replace view lab_observability.heartbeats
with (security_invoker = true)
as
select id, who, "this", "when"
from ops.logline_acts
where did = any (array['heartbeat', 'machine_heartbeat'])
order by "when" desc;

create or replace view lab_observability.runtime_status
with (security_invoker = true)
as
select id, who, "this", "when", status
from ops.logline_acts
where did = any (array['register_runtime', 'report_runtime_status'])
order by "when" desc;

create or replace view lab_observability.machine_state
with (security_invoker = true)
as
select id, who, "this", "when", status
from lab_observability.runtime_status;

create or replace view receipts.blocked
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'prepare_receipt_candidate'
  and not ("this" ? 'evidence_refs');

create or replace view receipts.candidates
with (security_invoker = true)
as
select id, who, did, "this", "when", confirmed_by, status
from ops.logline_acts
where did = 'prepare_receipt_candidate'
  and status = 'candidate'
order by "when" desc;

create or replace view receipts.closed
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = any (array['close_receipt', 'emit_receipt'])
  and status = any (array['closed', 'emitted']);

create or replace view receipts.index
with (security_invoker = true)
as
select id, who, did, "this", "when", confirmed_by, status
from receipts.candidates;

create or replace view receipts.rejected
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'reject_receipt_candidate';

create or replace view workorders.dispatch_packets
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'prepare_dispatch_packet';

create or replace view workorders.execution_reports
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'report_execution_result';

create or replace view workorders.hermes_workorders
with (security_invoker = true)
as
select id, who, did, "this", "when", status
from ops.logline_acts
where did = 'prepare_hermes_workorder';

grant usage on schema ops to service_role;
grant usage on schema audit to service_role;
grant usage on schema registry to service_role;
grant usage on schema authz to service_role;
grant usage on schema receipts to service_role;
grant usage on schema evidence to service_role;
grant usage on schema lab_observability to service_role;
grant usage on schema workorders to service_role;
grant select, insert on ops.logline_acts to service_role;
grant select, insert on all tables in schema registry to service_role;
grant select on all tables in schema audit to service_role;
grant select on all tables in schema authz to service_role;
grant select on all tables in schema receipts to service_role;
grant select on all tables in schema evidence to service_role;
grant select on all tables in schema lab_observability to service_role;
grant select on all tables in schema workorders to service_role;
grant execute on function ops.ingest_logline_act(jsonb) to service_role;
