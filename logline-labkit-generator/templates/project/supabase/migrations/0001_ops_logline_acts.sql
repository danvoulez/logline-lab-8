create schema if not exists ops;

create table if not exists ops.logline_acts (
  id bigint generated always as identity primary key,

  tuple_hash text null unique,
  receipt_hash text null,
  correlation_id text null,

  who_entity_id bigint null,
  did text not null,
  this_ref jsonb not null default '{}',
  when_ref timestamptz null,

  confirmed_by_ref jsonb not null default '{}',
  if_ok_ref jsonb not null default '{}',
  if_doubt_ref jsonb not null default '{}',
  if_not_ref jsonb not null default '{}',

  status text not null,
  act_state text not null default 'candidate',
  evidence_mode text not null default 'unverified',

  source text null,
  storage_uri text null,
  metadata_json jsonb not null default '{}',

  created_at timestamptz not null default now(),

  constraint logline_acts_act_state_check
    check (act_state in ('candidate', 'ghosted', 'rejected', 'closed', 'draft', 'walked', 'admitted', 'committed', 'superseded'))
);

create index if not exists logline_acts_did_idx on ops.logline_acts (did);
create index if not exists logline_acts_status_idx on ops.logline_acts (status);
create index if not exists logline_acts_created_at_idx on ops.logline_acts (created_at desc);
create index if not exists logline_acts_metadata_gin_idx on ops.logline_acts using gin (metadata_json);

create or replace function ops.ingest_logline_act(
  p_act jsonb,
  p_tuple_hash text,
  p_source text default 'logline-lab-cli',
  p_correlation_id text default null,
  p_metadata_json jsonb default '{}'::jsonb
)
returns ops.logline_acts
language plpgsql
security definer
set search_path = ops, public
as $$
declare
  required_slots text[] := array[
    'who',
    'did',
    'this',
    'when',
    'confirmed_by',
    'if_ok',
    'if_doubt',
    'if_not',
    'status'
  ];
  key text;
  inserted ops.logline_acts;
begin
  if jsonb_typeof(p_act) is distinct from 'object' then
    raise exception 'LogLine Act must be a JSON object';
  end if;

  foreach key in array required_slots loop
    if not p_act ? key then
      raise exception 'missing LogLine Act slot: %', key;
    end if;
  end loop;

  for key in select jsonb_object_keys(p_act) loop
    if not key = any(required_slots) then
      raise exception 'unknown LogLine Act slot: %', key;
    end if;
  end loop;

  insert into ops.logline_acts (
    tuple_hash,
    correlation_id,
    did,
    this_ref,
    confirmed_by_ref,
    if_ok_ref,
    if_doubt_ref,
    if_not_ref,
    status,
    act_state,
    evidence_mode,
    source,
    metadata_json
  )
  values (
    p_tuple_hash,
    p_correlation_id,
    p_act ->> 'did',
    p_act -> 'this',
    p_act -> 'confirmed_by',
    p_act -> 'if_ok',
    p_act -> 'if_doubt',
    p_act -> 'if_not',
    p_act ->> 'status',
    'candidate',
    'unverified',
    p_source,
    coalesce(p_metadata_json, '{}'::jsonb) || jsonb_build_object(
      'act', p_act,
      'who', p_act -> 'who',
      'when', p_act -> 'when'
    )
  )
  on conflict (tuple_hash) do update
    set tuple_hash = excluded.tuple_hash
  returning * into inserted;

  return inserted;
end;
$$;

grant usage on schema ops to service_role;
grant select, insert, update on ops.logline_acts to service_role;
grant execute on function ops.ingest_logline_act(jsonb, text, text, text, jsonb) to service_role;
