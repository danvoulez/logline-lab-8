use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use logline_act::parse_act_json;

use crate::lab_home::{LabHome, MANIFEST_FILE};

pub const CANDIDATES_DIR: &str = "candidates";
pub const CANDIDATE_FILE: &str = "candidate.json";
pub const METADATA_FILE: &str = "metadata.json";
pub const CANDIDATE_AUTHORITY: &str = "local_capture_only_not_official_spine";

#[derive(Debug, Clone)]
pub struct CandidateMetadata {
    pub candidate_id: String,
    pub captured_at: String,
    pub source_file: String,
    pub status: String,
    pub authority: String,
    pub validated_act_shape: bool,
    pub input_hash: String,
}

#[derive(Debug, Clone)]
pub struct CandidateRecord {
    pub metadata: CandidateMetadata,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct CaptureReport {
    pub home: PathBuf,
    pub candidate_id: String,
    pub candidate_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CandidateList {
    pub home: PathBuf,
    pub records: Vec<CandidateMetadata>,
}

#[derive(Debug)]
pub enum CandidateError {
    LabHomeNotInitialized(PathBuf),
    Io(String),
    InvalidAct(String),
    CandidateNotFound(String),
    InvalidCandidateId(String),
    InvalidMetadata(String),
}

impl std::fmt::Display for CandidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateError::LabHomeNotInitialized(home) => write!(
                f,
                "lab home is not initialized: {}; run logline-lab init --home <path>",
                home.display()
            ),
            CandidateError::Io(message) => write!(f, "candidate I/O error: {message}"),
            CandidateError::InvalidAct(message) => write!(f, "invalid LogLine Act\n{message}"),
            CandidateError::CandidateNotFound(id) => write!(f, "candidate not found: {id}"),
            CandidateError::InvalidCandidateId(id) => write!(f, "invalid candidate id: {id}"),
            CandidateError::InvalidMetadata(message) => {
                write!(f, "invalid candidate metadata: {message}")
            }
        }
    }
}

impl std::error::Error for CandidateError {}

impl LabHome {
    pub fn candidates_dir(&self) -> PathBuf {
        self.local_dir().join(CANDIDATES_DIR)
    }

    pub fn candidate_count(&self) -> usize {
        list_candidate_metadata(&self.candidates_dir())
            .map(|records| records.len())
            .unwrap_or(0)
    }

    pub fn capture_candidate(&self, source_file: &Path) -> Result<CaptureReport, CandidateError> {
        self.require_initialized_for_candidates()?;
        let content = fs::read_to_string(source_file).map_err(|err| {
            CandidateError::Io(format!(
                "unable to read candidate source {}: {err}",
                source_file.display()
            ))
        })?;
        parse_act_json(&content).map_err(|err| CandidateError::InvalidAct(err.to_string()))?;

        let captured_at = now_unix_millis();
        let input_hash = short_hash(&content);
        let candidate_id = format!("cand_{captured_at}_{input_hash}");
        let candidate_dir = self.candidates_dir().join(&candidate_id);
        fs::create_dir(&candidate_dir).map_err(|err| {
            CandidateError::Io(format!(
                "unable to create candidate directory {}: {err}",
                candidate_dir.display()
            ))
        })?;
        fs::write(candidate_dir.join(CANDIDATE_FILE), &content).map_err(|err| {
            CandidateError::Io(format!("unable to write candidate content: {err}"))
        })?;
        let metadata = CandidateMetadata {
            candidate_id: candidate_id.clone(),
            captured_at: captured_at.to_string(),
            source_file: source_file.display().to_string(),
            status: "candidate".to_string(),
            authority: CANDIDATE_AUTHORITY.to_string(),
            validated_act_shape: true,
            input_hash,
        };
        fs::write(candidate_dir.join(METADATA_FILE), metadata.to_json()).map_err(|err| {
            CandidateError::Io(format!("unable to write candidate metadata: {err}"))
        })?;

        Ok(CaptureReport {
            home: self.home().to_path_buf(),
            candidate_id,
            candidate_dir,
        })
    }

    pub fn list_candidates(&self) -> Result<CandidateList, CandidateError> {
        self.require_initialized_for_candidates()?;
        let mut records = list_candidate_metadata(&self.candidates_dir())?;
        records.sort_by(|a, b| {
            a.captured_at
                .cmp(&b.captured_at)
                .then(a.candidate_id.cmp(&b.candidate_id))
        });
        Ok(CandidateList {
            home: self.home().to_path_buf(),
            records,
        })
    }

    pub fn get_candidate(&self, candidate_id: &str) -> Result<CandidateRecord, CandidateError> {
        self.require_initialized_for_candidates()?;
        if !is_safe_candidate_id(candidate_id) {
            return Err(CandidateError::InvalidCandidateId(candidate_id.to_string()));
        }
        let candidate_dir = self.candidates_dir().join(candidate_id);
        if !candidate_dir.is_dir() {
            return Err(CandidateError::CandidateNotFound(candidate_id.to_string()));
        }
        let metadata = read_metadata(&candidate_dir.join(METADATA_FILE))?;
        let content = fs::read_to_string(candidate_dir.join(CANDIDATE_FILE)).map_err(|err| {
            CandidateError::Io(format!("unable to read candidate content: {err}"))
        })?;
        Ok(CandidateRecord { metadata, content })
    }

    fn require_initialized_for_candidates(&self) -> Result<(), CandidateError> {
        if !self.local_dir().is_dir()
            || !self.local_dir().join(MANIFEST_FILE).is_file()
            || !self.candidates_dir().is_dir()
        {
            return Err(CandidateError::LabHomeNotInitialized(
                self.home().to_path_buf(),
            ));
        }
        Ok(())
    }
}

impl CaptureReport {
    pub fn to_text(&self) -> String {
        [
            "candidate captured".to_string(),
            format!("id: {}", self.candidate_id),
            format!("home: {}", self.home.display()),
            format!("candidate_dir: {}", self.candidate_dir.display()),
            "authority: local capture only; not official spine; not receipt; not remote synced"
                .to_string(),
        ]
        .join("\n")
    }
}

impl CandidateList {
    pub fn to_text(&self) -> String {
        let mut lines = vec![format!("candidates: {}", self.records.len())];
        for record in &self.records {
            lines.push(format!(
                "- {} {} captured_at={}",
                record.candidate_id, record.status, record.captured_at
            ));
        }
        lines.push("authority: local candidate queue only; not official spine".to_string());
        lines.join("\n")
    }
}

impl CandidateRecord {
    pub fn to_text(&self) -> String {
        [
            "metadata:".to_string(),
            self.metadata.to_json(),
            "candidate:".to_string(),
            self.content.clone(),
            "authority: local workspace record only; not official spine; not receipt".to_string(),
        ]
        .join("\n")
    }
}

impl CandidateMetadata {
    pub fn to_json(&self) -> String {
        format!(
            "{{\n  \"candidate_id\": \"{}\",\n  \"captured_at\": \"{}\",\n  \"source_file\": \"{}\",\n  \"status\": \"{}\",\n  \"authority\": \"{}\",\n  \"validated_act_shape\": {},\n  \"input_hash\": \"{}\"\n}}\n",
            escape_json_string(&self.candidate_id),
            escape_json_string(&self.captured_at),
            escape_json_string(&self.source_file),
            escape_json_string(&self.status),
            escape_json_string(&self.authority),
            self.validated_act_shape,
            escape_json_string(&self.input_hash),
        )
    }
}

fn list_candidate_metadata(
    candidates_dir: &Path,
) -> Result<Vec<CandidateMetadata>, CandidateError> {
    if !candidates_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    for entry in fs::read_dir(candidates_dir).map_err(|err| CandidateError::Io(err.to_string()))? {
        let entry = entry.map_err(|err| CandidateError::Io(err.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            records.push(read_metadata(&path.join(METADATA_FILE))?);
        }
    }
    Ok(records)
}

fn read_metadata(path: &Path) -> Result<CandidateMetadata, CandidateError> {
    let text = fs::read_to_string(path).map_err(|err| {
        CandidateError::Io(format!(
            "unable to read candidate metadata {}: {err}",
            path.display()
        ))
    })?;
    metadata_from_json(&text)
}

fn metadata_from_json(text: &str) -> Result<CandidateMetadata, CandidateError> {
    let value: serde_json::Value = serde_json::from_str(text)
        .map_err(|err| CandidateError::InvalidMetadata(err.to_string()))?;
    Ok(CandidateMetadata {
        candidate_id: required_string(&value, "candidate_id")?,
        captured_at: required_string(&value, "captured_at")?,
        source_file: required_string(&value, "source_file")?,
        status: required_string(&value, "status")?,
        authority: required_string(&value, "authority")?,
        validated_act_shape: value
            .get("validated_act_shape")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        input_hash: required_string(&value, "input_hash")?,
    })
}

fn required_string(value: &serde_json::Value, key: &str) -> Result<String, CandidateError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| CandidateError::InvalidMetadata(format!("missing string field {key}")))
}

fn now_unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn short_hash(input: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:012x}").chars().take(12).collect()
}

fn is_safe_candidate_id(candidate_id: &str) -> bool {
    candidate_id.starts_with("cand_")
        && candidate_id
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn escape_json_string(input: &str) -> String {
    input
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}
