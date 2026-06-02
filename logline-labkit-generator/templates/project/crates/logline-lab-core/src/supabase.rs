use std::{env, fmt};

use logline_act::{act_sha256, parse_act_json};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};

const SUPABASE_URL: &str = "SUPABASE_URL";
const SUPABASE_SERVICE_ROLE_KEY: &str = "SUPABASE_SERVICE_ROLE_KEY";

#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub service_role_key: String,
}

#[derive(Debug, Clone)]
pub struct SupabaseCheckReport {
    pub url: String,
    pub ops_logline_acts_available: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct SupabaseEmitReport {
    pub tuple_hash: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug)]
pub enum SupabaseError {
    MissingEnv(Vec<&'static str>),
    InvalidEnv(String),
    InvalidAct(String),
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
            SupabaseError::InvalidAct(message) => write!(f, "supabase: invalid LogLine Act\n{message}"),
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
            format!("url: {}", redact_url(&self.url)),
            format!(
                "ops.logline_acts: {}",
                if self.ops_logline_acts_available {
                    "available"
                } else {
                    "missing-or-unreachable"
                }
            ),
            format!("detail: {}", self.detail),
            "authority: Supabase profile check only; Supabase is v0 spine, not Foundation canon".to_string(),
        ]
        .join("\n")
    }
}

impl SupabaseEmitReport {
    pub fn to_text(&self) -> String {
        [
            "remote LogLine Act emit attempted".to_string(),
            format!("tuple_hash: {}", self.tuple_hash),
            format!("status: {}", self.status),
            format!("detail: {}", self.detail),
            "authority: write target is ops.logline_acts only; not receipt; not evidence".to_string(),
        ]
        .join("\n")
    }
}

pub fn config_from_env() -> Result<SupabaseConfig, SupabaseError> {
    let mut missing = Vec::new();
    let url = match env::var(SUPABASE_URL) {
        Ok(value) if !value.trim().is_empty() => value.trim().trim_end_matches('/').to_string(),
        _ => {
            missing.push(SUPABASE_URL);
            String::new()
        }
    };
    let service_role_key = match env::var(SUPABASE_SERVICE_ROLE_KEY) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            missing.push(SUPABASE_SERVICE_ROLE_KEY);
            String::new()
        }
    };
    if !missing.is_empty() {
        return Err(SupabaseError::MissingEnv(missing));
    }
    if !url.starts_with("https://") && !url.starts_with("http://127.0.0.1") && !url.starts_with("http://localhost") {
        return Err(SupabaseError::InvalidEnv(
            "SUPABASE_URL must be https://... or local Supabase http://127.0.0.1/localhost".to_string(),
        ));
    }
    Ok(SupabaseConfig {
        url,
        service_role_key,
    })
}

pub fn check_from_env() -> Result<SupabaseCheckReport, SupabaseError> {
    let config = config_from_env()?;
    check(&config)
}

pub fn check(config: &SupabaseConfig) -> Result<SupabaseCheckReport, SupabaseError> {
    let client = Client::new();
    let response = client
        .get(format!(
            "{}/rest/v1/logline_acts?select=id&limit=1",
            config.url
        ))
        .headers(headers(config, true)?)
        .send()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    let status = response.status();
    let body = response
        .text()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    if status.is_success() {
        Ok(SupabaseCheckReport {
            url: config.url.clone(),
            ops_logline_acts_available: true,
            detail: "PostgREST can read ops.logline_acts through the configured Supabase project".to_string(),
        })
    } else {
        Ok(SupabaseCheckReport {
            url: config.url.clone(),
            ops_logline_acts_available: false,
            detail: format!("status={} body={}", status.as_u16(), compact(&body)),
        })
    }
}

pub fn emit_act_from_env(input: &str) -> Result<SupabaseEmitReport, SupabaseError> {
    let config = config_from_env()?;
    emit_act(&config, input)
}

pub fn emit_act(config: &SupabaseConfig, input: &str) -> Result<SupabaseEmitReport, SupabaseError> {
    parse_act_json(input).map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let tuple_hash = act_sha256(input).map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let act = serde_json::from_str::<Value>(input)
        .map_err(|err| SupabaseError::InvalidAct(err.to_string()))?;
    let body = json!({
        "p_act": act,
        "p_tuple_hash": tuple_hash,
        "p_source": "logline-lab-cli",
        "p_correlation_id": null,
        "p_metadata_json": {
            "client": "logline-lab-cli",
            "storage_profile": "supabase"
        }
    });
    let client = Client::new();
    let response = client
        .post(format!("{}/rest/v1/rpc/ingest_logline_act", config.url))
        .headers(headers(config, true)?)
        .json(&body)
        .send()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    let status = response.status();
    let response_body = response
        .text()
        .map_err(|err| SupabaseError::Http(err.to_string()))?;
    if status.is_success() {
        Ok(SupabaseEmitReport {
            tuple_hash,
            status: "written-or-idempotent".to_string(),
            detail: compact(&response_body),
        })
    } else {
        Err(SupabaseError::Api {
            status: status.as_u16(),
            body: compact(&response_body),
        })
    }
}

fn headers(config: &SupabaseConfig, ops_schema: bool) -> Result<HeaderMap, SupabaseError> {
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", config.service_role_key);
    headers.insert(
        "apikey",
        HeaderValue::from_str(&config.service_role_key)
            .map_err(|err| SupabaseError::InvalidEnv(err.to_string()))?,
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&bearer).map_err(|err| SupabaseError::InvalidEnv(err.to_string()))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if ops_schema {
        headers.insert("Accept-Profile", HeaderValue::from_static("ops"));
        headers.insert("Content-Profile", HeaderValue::from_static("ops"));
    }
    Ok(headers)
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

fn redact_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}
