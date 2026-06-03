use std::{env, fmt};

use logline_act::{act_sha256, parse_act_json};
use postgres::{Client as PgClient, NoTls};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use tokio_postgres_rustls::MakeRustlsConnect;

const DATABASE_URL: &str = "DATABASE_URL";
const SUPABASE_URL: &str = "SUPABASE_URL";
const SUPABASE_SERVICE_ROLE_KEY: &str = "SUPABASE_SERVICE_ROLE_KEY";
const SUPABASE_SECRET_KEY: &str = "SUPABASE_SECRET_KEY";

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub database_url: Option<String>,
    pub url: Option<String>,
    pub service_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SupabaseCheckReport {
    pub target: String,
    pub ops_logline_acts_available: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct SupabaseEmitReport {
    pub act_id: String,
    pub tuple_hash: String,
    pub content_hash: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug)]
pub enum SupabaseError {
    MissingEnv(Vec<&'static str>),
    InvalidEnv(String),
    InvalidAct(String),
    Database(String),
    Http(String),
    Api { status: u16, body: String },
}

impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupabaseError::MissingEnv(vars) => write!(
                f,
                "supabase: missing required env: {}\nremote spine: ghost remote-spine-unconfigured",
                vars.join(", ")
            ),
            SupabaseError::InvalidEnv(message) => write!(f, "supabase: invalid env: {message}"),
            SupabaseError::InvalidAct(message) => {
                write!(f, "supabase: invalid LogLine Act\n{message}")
            }
            SupabaseError::Database(message) => write!(f, "supabase: database error: {message}"),
            SupabaseError::Http(message) => write!(f, "supabase: http error: {message}"),
            SupabaseError::Api { status, body } => write!(
                f,
                "supabase: api error status={status}\n{body}\nremote spine: not written"
            ),
        }
    }
}

impl std::error::Error for SupabaseError {}

impl SupabaseCheckReport {
    pub fn to_text(&self) -> String {
        [
            "supabase check".to_string(),
            format!("target: {}", self.target),
            format!(
                "ops.logline_acts: {}",
                if self.ops_logline_acts_available {
                    "available"
                } else {
                    "missing-or-unreachable"
                }
            ),
            format!("detail: {}", self.detail),
            "authority: Supabase profile check only; Supabase is v0 spine, not Foundation canon"
                .to_string(),
        ]
        .join("\n")
    }
}

impl SupabaseEmitReport {
    pub fn to_text(&self) -> String {
        [
            "remote LogLine Act emitted".to_string(),
            format!("act_id: {}", self.act_id),
            format!("tuple_hash: {}", self.tuple_hash),
            format!("content_hash: {}", self.content_hash),
            format!("status: {}", self.status),
            format!("detail: {}", self.detail),
            "authority: write target is ops.logline_acts only; not receipt; not evidence"
                .to_string(),
        ]
        .join("\n")
    }
}

pub fn config_from_env() -> Result<SupabaseConfig, SupabaseError> {
    let database_url = env_nonempty(DATABASE_URL);
    let url = env_nonempty(SUPABASE_URL).map(|value| value.trim_end_matches('/').to_string());
    let service_key = env_nonempty(SUPABASE_SERVICE_ROLE_KEY).or_else(|| env_nonempty(SUPABASE_SECRET_KEY));

    if database_url.is_none() && (url.is_none() || service_key.is_none()) {
        return Err(SupabaseError::MissingEnv(vec![
            "DATABASE_URL or SUPABASE_URL",
            "SUPABASE_SERVICE_ROLE_KEY or SUPABASE_SECRET_KEY",
        ]));
    }

    if let Some(url) = &url {
        if !url.starts_with("https://")
            && !url.starts_with("http://127.0.0.1")
            && !url.starts_with("http://localhost")
        {
            return Err(SupabaseError::InvalidEnv(
                "SUPABASE_URL must be https://... or local Supabase http://127.0.0.1/localhost"
                    .to_string(),
            ));
        }
    }

    Ok(SupabaseConfig {
        database_url,
        url,
        service_key,
    })
}

pub fn check_from_env() -> Result<String, String> {
    let config = config_from_env().map_err(|err| err.to_string())?;
    check(&config)
        .map(|report| report.to_text())
        .map_err(|err| err.to_string())
}

pub fn emit_act_from_env(input: &str) -> Result<String, String> {
    let config = config_from_env().map_err(|err| err.to_string())?;
    emit_act(&config, input)
        .map(|report| report.to_text())
        .map_err(|err| err.to_string())
}

pub fn check(config: &SupabaseConfig) -> Result<SupabaseCheckReport, SupabaseError> {
    if let Some(database_url) = &config.database_url {
        return check_database(database_url);
    }
    check_rest(config)
}

pub fn emit_act(config: &SupabaseConfig, input: &str) -> Result<SupabaseEmitReport, SupabaseError> {
    parse_act_json(input).map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let tuple_hash = act_sha256(input).map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let mut act =
        serde_json::from_str::<Value>(input).map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let object = act
        .as_object_mut()
        .ok_or_else(|| SupabaseError::InvalidAct("Act must be a JSON object".to_string()))?;
    object.insert("tuple_hash".to_string(), Value::String(tuple_hash.clone()));
    object.insert("content_hash".to_string(), Value::String(tuple_hash.clone()));
    object.insert(
        "runtime_envelope".to_string(),
        json!({
            "client": "logline-lab-cli",
            "storage_profile": "supabase"
        }),
    );

    if let Some(database_url) = &config.database_url {
        return emit_database(database_url, &act, &tuple_hash);
    }
    emit_rest(config, &act, &tuple_hash)
}

fn check_database(database_url: &str) -> Result<SupabaseCheckReport, SupabaseError> {
    let mut client = postgres_client(database_url)?;
    let row = client
        .query_one(
            "select
               to_regclass('ops.logline_acts') is not null as has_acts,
               to_regprocedure('ops.ingest_logline_act(jsonb)') is not null as has_ingest,
               to_regclass('pgmq.q_q_lab_outbox') is not null as has_outbox,
               (select count(*)::bigint from ops.logline_acts) as act_count",
            &[],
        )
        .map_err(|err| SupabaseError::Database(err.to_string()))?;
    let has_acts: bool = row.get("has_acts");
    let has_ingest: bool = row.get("has_ingest");
    let has_outbox: bool = row.get("has_outbox");
    let act_count: i64 = row.get("act_count");
    Ok(SupabaseCheckReport {
        target: "DATABASE_URL".to_string(),
        ops_logline_acts_available: has_acts && has_ingest,
        detail: format!(
            "ingest_function={} queue_lab_outbox={} act_count={}",
            has_ingest, has_outbox, act_count
        ),
    })
}

fn emit_database(
    database_url: &str,
    payload: &Value,
    tuple_hash: &str,
) -> Result<SupabaseEmitReport, SupabaseError> {
    let mut client = postgres_client(database_url)?;
    let row = client
        .query_one(
            "select
               id::text as act_id,
               coalesce(tuple_hash, '') as tuple_hash,
               coalesce(content_hash, '') as content_hash,
               status
             from ops.ingest_logline_act($1::jsonb)",
            &[payload],
        )
        .map_err(|err| SupabaseError::Database(err.to_string()))?;
    let act_id: String = row.get("act_id");
    let content_hash: String = row.get("content_hash");
    let status: String = row.get("status");
    Ok(SupabaseEmitReport {
        act_id,
        tuple_hash: tuple_hash.to_string(),
        content_hash,
        status,
        detail: "called ops.ingest_logline_act(payload) through DATABASE_URL".to_string(),
    })
}

fn check_rest(config: &SupabaseConfig) -> Result<SupabaseCheckReport, SupabaseError> {
    let client = HttpClient::new();
    let response = client
        .post(format!("{}/rest/v1/rpc/logline_spine_health", rest_url(config)?))
        .headers(rest_headers(config)?)
        .json(&json!({}))
        .send()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    let status = response.status();
    let body = response
        .text()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    if status.is_success() {
        let available = serde_json::from_str::<Value>(&body)
            .ok()
            .and_then(|value| value.get("ops_logline_acts").and_then(Value::as_bool))
            .unwrap_or(false);
        Ok(SupabaseCheckReport {
            target: rest_url(config)?.to_string(),
            ops_logline_acts_available: available,
            detail: compact(&body),
        })
    } else {
        Ok(SupabaseCheckReport {
            target: rest_url(config)?.to_string(),
            ops_logline_acts_available: false,
            detail: format!("status={} body={}", status.as_u16(), compact(&body)),
        })
    }
}

fn emit_rest(
    config: &SupabaseConfig,
    payload: &Value,
    tuple_hash: &str,
) -> Result<SupabaseEmitReport, SupabaseError> {
    let client = HttpClient::new();
    let response = client
        .post(format!("{}/rest/v1/rpc/ingest_logline_act", rest_url(config)?))
        .headers(rest_headers(config)?)
        .json(&json!({ "payload": payload }))
        .send()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    let status = response.status();
    let body = response
        .text()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    if status.is_success() {
        let value = serde_json::from_str::<Value>(&body).unwrap_or(Value::Null);
        Ok(SupabaseEmitReport {
            act_id: value
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            tuple_hash: tuple_hash.to_string(),
            content_hash: value
                .get("content_hash")
                .and_then(Value::as_str)
                .unwrap_or(tuple_hash)
                .to_string(),
            status: value
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            detail: "called public RPC ingest_logline_act(payload) through Supabase REST".to_string(),
        })
    } else {
        Err(SupabaseError::Api {
            status: status.as_u16(),
            body: compact(&body),
        })
    }
}

fn postgres_client(database_url: &str) -> Result<PgClient, SupabaseError> {
    if database_url.contains("sslmode=disable") {
        return PgClient::connect(database_url, NoTls)
            .map_err(|err| SupabaseError::Database(err.to_string()));
    }

    let _ = rustls::crypto::ring::default_provider().install_default();
    let tls = MakeRustlsConnect::with_webpki_roots();
    let connection = database_url_with_required_ssl(database_url);
    match PgClient::connect(&connection, tls) {
        Ok(client) => Ok(client),
        Err(err) if !database_url.contains("sslmode=") => {
            let plaintext_connection = database_url_with_disabled_ssl(database_url);
            PgClient::connect(&plaintext_connection, NoTls).map_err(|fallback_err| {
                SupabaseError::Database(format!(
                    "TLS failed: {}; non-TLS fallback failed: {}",
                    err, fallback_err
                ))
            })
        }
        Err(err) => Err(SupabaseError::Database(err.to_string())),
    }
}

fn database_url_with_required_ssl(database_url: &str) -> String {
    if database_url.contains("sslmode=") {
        database_url.to_string()
    } else if database_url.contains('?') {
        format!("{database_url}&sslmode=require")
    } else {
        format!("{database_url}?sslmode=require")
    }
}

fn database_url_with_disabled_ssl(database_url: &str) -> String {
    if database_url.contains("sslmode=") {
        database_url.to_string()
    } else if database_url.contains('?') {
        format!("{database_url}&sslmode=disable")
    } else {
        format!("{database_url}?sslmode=disable")
    }
}

fn rest_url(config: &SupabaseConfig) -> Result<&str, SupabaseError> {
    config
        .url
        .as_deref()
        .ok_or_else(|| SupabaseError::MissingEnv(vec![SUPABASE_URL]))
}

fn rest_headers(config: &SupabaseConfig) -> Result<HeaderMap, SupabaseError> {
    let service_key = config
        .service_key
        .as_deref()
        .ok_or_else(|| SupabaseError::MissingEnv(vec![
            "SUPABASE_SERVICE_ROLE_KEY or SUPABASE_SECRET_KEY",
        ]))?;
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {service_key}");
    headers.insert(
        "apikey",
        HeaderValue::from_str(service_key)
            .map_err(|err| SupabaseError::InvalidEnv(err.to_string()))?,
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&bearer).map_err(|err| SupabaseError::InvalidEnv(err.to_string()))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

fn env_nonempty(key: &'static str) -> Option<String> {
    env::var(key).ok().filter(|value| !value.trim().is_empty())
}

fn compact(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.len() > 500 {
        format!("{}...", &trimmed[..500])
    } else if trimmed.is_empty() {
        "empty response".to_string()
    } else {
        trimmed.to_string()
    }
}
