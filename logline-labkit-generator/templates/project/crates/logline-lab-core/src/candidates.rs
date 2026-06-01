use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use logline_act::parse_act_json;

use crate::lab_home::{LabHome, MANIFEST_FILE};

pub const CANDIDATES_DIR: &str = "candidates";
pub const CANDIDATE_FILE: &str = "candidate.json";
pub const METADATA_FILE: &str = "metadata.json";
pub const INDEX_FILE: &str = "index.json";
pub const CANDIDATE_AUTHORITY: &str = "local_capture_only_not_official_spine";
pub const CANDIDATE_INDEX_AUTHORITY: &str = "local_candidate_queue_index_not_official_spine";

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
    pub index_status: CandidateIndexState,
}

#[derive(Debug, Clone)]
pub struct CandidateList {
    pub home: PathBuf,
    pub records: Vec<CandidateMetadata>,
    pub index_status: CandidateIndexState,
}

#[derive(Debug, Clone)]
pub struct CandidateIndex {
    pub index_version: u64,
    pub authority: String,
    pub updated_at: String,
    pub candidates: Vec<CandidateIndexEntry>,
}

#[derive(Debug, Clone)]
pub struct CandidateIndexEntry {
    pub candidate_id: String,
    pub path: String,
    pub metadata_path: String,
    pub captured_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateIndexState {
    Available,
    Missing,
    Malformed,
    Rebuilt,
}

#[derive(Debug, Clone)]
pub struct CandidateIndexInspection {
    pub state: CandidateIndexState,
    pub entries: usize,
    pub directories: usize,
    pub failures: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub enum CandidateError {
    LabHomeNotInitialized(PathBuf),
    Io(String),
    InvalidAct(String),
    CandidateNotFound(String),
    InvalidCandidateId(String),
    InvalidMetadata(String),
    MalformedIndex(String),
    InconsistentIndex(String),
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
            CandidateError::InvalidAct(message) => {
                write!(f, "validation failed\n{message}\nindex: unchanged")
            }
            CandidateError::CandidateNotFound(id) => write!(f, "candidate not found: {id}"),
            CandidateError::InvalidCandidateId(id) => write!(f, "invalid candidate id: {id}"),
            CandidateError::InvalidMetadata(message) => {
                write!(f, "invalid candidate metadata: {message}")
            }
            CandidateError::MalformedIndex(message) => {
                write!(f, "index: malformed\ncandidate index malformed: {message}")
            }
            CandidateError::InconsistentIndex(message) => {
                write!(
                    f,
                    "index: malformed\ncandidate index inconsistent: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CandidateError {}

impl LabHome {
    pub fn candidates_dir(&self) -> PathBuf {
        self.local_dir().join(CANDIDATES_DIR)
    }

    pub fn candidate_index_path(&self) -> PathBuf {
        self.candidates_dir().join(INDEX_FILE)
    }

    pub fn initialize_candidate_index_if_missing(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.candidates_dir())?;
        if !self.candidate_index_path().exists() {
            write_candidate_index_atomic(&self.candidates_dir(), &CandidateIndex::empty())
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
        }
        Ok(())
    }

    pub fn candidate_count(&self) -> usize {
        self.candidate_listing_for_projection()
            .map(|list| list.records.len())
            .unwrap_or(0)
    }

    pub fn candidate_index_status(&self) -> CandidateIndexState {
        self.candidate_listing_for_projection()
            .map(|list| list.index_status)
            .unwrap_or(CandidateIndexState::Missing)
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

        let index_status = self.update_candidate_index(&metadata).map_err(|err| {
            CandidateError::Io(format!(
                "candidate captured, but local Candidate index update failed: {err}"
            ))
        })?;

        Ok(CaptureReport {
            home: self.home().to_path_buf(),
            candidate_id,
            candidate_dir,
            index_status,
        })
    }

    pub fn list_candidates(&self) -> Result<CandidateList, CandidateError> {
        self.require_initialized_for_candidates()?;
        match self.load_candidate_index() {
            Ok(index) => self.list_candidates_from_index(index, CandidateIndexState::Available),
            Err(CandidateError::MalformedIndex(message)) => {
                Err(CandidateError::MalformedIndex(message))
            }
            Err(CandidateError::Io(message)) if message == "candidate index missing" => {
                self.rebuild_candidate_index()
            }
            Err(CandidateError::Io(message)) => Err(CandidateError::Io(message)),
            Err(_) => self.rebuild_candidate_index(),
        }
    }

    pub fn rebuild_candidate_index(&self) -> Result<CandidateList, CandidateError> {
        self.require_initialized_for_candidates()?;
        let records = sorted_candidate_metadata(&self.candidates_dir())?;
        let index = CandidateIndex::from_metadata(&records);
        write_candidate_index_atomic(&self.candidates_dir(), &index)?;
        Ok(CandidateList {
            home: self.home().to_path_buf(),
            records,
            index_status: CandidateIndexState::Rebuilt,
        })
    }

    pub fn candidate_index_inspection(&self) -> CandidateIndexInspection {
        let candidates_dir = self.candidates_dir();
        let directories = candidate_directory_ids(&candidates_dir).unwrap_or_default();
        if !self.candidate_index_path().is_file() {
            return CandidateIndexInspection {
                state: CandidateIndexState::Missing,
                entries: 0,
                directories: directories.len(),
                failures: Vec::new(),
                warnings: vec![
                    "candidate index missing; run logline-lab candidate index rebuild --home <path>"
                        .to_string(),
                ],
            };
        }
        let index = match self.load_candidate_index() {
            Ok(index) => index,
            Err(err) => {
                return CandidateIndexInspection {
                    state: CandidateIndexState::Malformed,
                    entries: 0,
                    directories: directories.len(),
                    failures: vec![format!("candidate index: malformed ({err})")],
                    warnings: Vec::new(),
                };
            }
        };
        let mut failures = Vec::new();
        let mut warnings = Vec::new();
        let mut seen = BTreeSet::new();
        for entry in &index.candidates {
            if !seen.insert(entry.candidate_id.clone()) {
                failures.push(format!(
                    "candidate index duplicate entry: {}",
                    entry.candidate_id
                ));
            }
            let candidate_path = candidates_dir.join(&entry.path);
            if !candidate_path.is_file() {
                failures.push(format!(
                    "candidate index entry missing candidate file: {} -> {}",
                    entry.candidate_id, entry.path
                ));
            }
            let metadata_path = candidates_dir.join(&entry.metadata_path);
            if !metadata_path.is_file() {
                failures.push(format!(
                    "candidate index entry missing metadata file: {} -> {}",
                    entry.candidate_id, entry.metadata_path
                ));
            }
        }
        let indexed_ids = index
            .candidates
            .iter()
            .map(|entry| entry.candidate_id.clone())
            .collect::<BTreeSet<_>>();
        for dir_id in directories
            .iter()
            .filter(|dir_id| !indexed_ids.contains(*dir_id))
        {
            warnings.push(format!(
                "candidate directory not represented in index: {dir_id}"
            ));
        }
        CandidateIndexInspection {
            state: CandidateIndexState::Available,
            entries: index.candidates.len(),
            directories: directories.len(),
            failures,
            warnings,
        }
    }

    pub fn get_candidate(&self, candidate_id: &str) -> Result<CandidateRecord, CandidateError> {
        self.require_initialized_for_candidates()?;
        if !is_safe_candidate_id(candidate_id) {
            return Err(CandidateError::InvalidCandidateId(candidate_id.to_string()));
        }
        let candidate_dir = self.candidates_dir().join(candidate_id);
        if let Ok(index) = self.load_candidate_index() {
            if let Some(entry) = index
                .candidates
                .iter()
                .find(|entry| entry.candidate_id == candidate_id)
            {
                let candidate_file = self.candidates_dir().join(&entry.path);
                let metadata_file = self.candidates_dir().join(&entry.metadata_path);
                if !candidate_file.is_file() || !metadata_file.is_file() {
                    return Err(CandidateError::CandidateNotFound(candidate_id.to_string()));
                }
                let metadata = read_metadata(&metadata_file)?;
                let content = fs::read_to_string(candidate_file).map_err(|err| {
                    CandidateError::Io(format!("unable to read candidate content: {err}"))
                })?;
                return Ok(CandidateRecord { metadata, content });
            }
        }
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

    fn load_candidate_index(&self) -> Result<CandidateIndex, CandidateError> {
        let path = self.candidate_index_path();
        if !path.is_file() {
            return Err(CandidateError::Io("candidate index missing".to_string()));
        }
        let text = fs::read_to_string(&path).map_err(|err| {
            CandidateError::Io(format!(
                "unable to read candidate index {}: {err}",
                path.display()
            ))
        })?;
        CandidateIndex::from_json(&text)
    }

    fn list_candidates_from_index(
        &self,
        index: CandidateIndex,
        index_status: CandidateIndexState,
    ) -> Result<CandidateList, CandidateError> {
        let mut records = Vec::new();
        for entry in &index.candidates {
            let candidate_file = self.candidates_dir().join(&entry.path);
            let metadata_file = self.candidates_dir().join(&entry.metadata_path);
            if !candidate_file.is_file() {
                return Err(CandidateError::InconsistentIndex(format!(
                    "entry {} points to missing candidate file {}",
                    entry.candidate_id, entry.path
                )));
            }
            if !metadata_file.is_file() {
                return Err(CandidateError::InconsistentIndex(format!(
                    "entry {} points to missing metadata file {}",
                    entry.candidate_id, entry.metadata_path
                )));
            }
            records.push(read_metadata(&metadata_file)?);
        }
        records.sort_by(|a, b| {
            a.captured_at
                .cmp(&b.captured_at)
                .then(a.candidate_id.cmp(&b.candidate_id))
        });
        Ok(CandidateList {
            home: self.home().to_path_buf(),
            records,
            index_status,
        })
    }

    fn update_candidate_index(
        &self,
        metadata: &CandidateMetadata,
    ) -> Result<CandidateIndexState, CandidateError> {
        let mut records = sorted_candidate_metadata(&self.candidates_dir())?;
        if !records
            .iter()
            .any(|record| record.candidate_id == metadata.candidate_id)
        {
            records.push(metadata.clone());
        }
        records.sort_by(|a, b| {
            a.captured_at
                .cmp(&b.captured_at)
                .then(a.candidate_id.cmp(&b.candidate_id))
        });
        let index = CandidateIndex::from_metadata(&records);
        write_candidate_index_atomic(&self.candidates_dir(), &index)?;
        Ok(CandidateIndexState::Available)
    }

    fn candidate_listing_for_projection(&self) -> Result<CandidateList, CandidateError> {
        if !self.candidates_dir().is_dir() {
            return Ok(CandidateList {
                home: self.home().to_path_buf(),
                records: Vec::new(),
                index_status: CandidateIndexState::Missing,
            });
        }
        match self.list_candidates() {
            Ok(list) => Ok(list),
            Err(CandidateError::MalformedIndex(_)) => Ok(CandidateList {
                home: self.home().to_path_buf(),
                records: Vec::new(),
                index_status: CandidateIndexState::Malformed,
            }),
            Err(err) => Err(err),
        }
    }
}

impl CaptureReport {
    pub fn to_text(&self) -> String {
        [
            "candidate captured".to_string(),
            format!("id: {}", self.candidate_id),
            format!("home: {}", self.home.display()),
            format!("candidate_dir: {}", self.candidate_dir.display()),
            format!("index: {}", self.index_status.as_cli_status()),
            "authority: local capture only; not official spine; not receipt; not remote synced"
                .to_string(),
        ]
        .join("\n")
    }
}

impl CandidateList {
    pub fn to_text(&self) -> String {
        let mut lines = vec![
            format!("candidates: {}", self.records.len()),
            format!("index: {}", self.index_status.as_cli_status()),
        ];
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

impl CandidateIndex {
    pub fn empty() -> Self {
        Self {
            index_version: 1,
            authority: CANDIDATE_INDEX_AUTHORITY.to_string(),
            updated_at: now_unix_millis().to_string(),
            candidates: Vec::new(),
        }
    }

    pub fn from_metadata(records: &[CandidateMetadata]) -> Self {
        let mut deduped = BTreeMap::new();
        for record in records {
            deduped.insert(
                record.candidate_id.clone(),
                CandidateIndexEntry::from(record),
            );
        }
        let mut candidates = deduped.into_values().collect::<Vec<_>>();
        candidates.sort_by(|a, b| {
            a.captured_at
                .cmp(&b.captured_at)
                .then(a.candidate_id.cmp(&b.candidate_id))
        });
        Self {
            index_version: 1,
            authority: CANDIDATE_INDEX_AUTHORITY.to_string(),
            updated_at: now_unix_millis().to_string(),
            candidates,
        }
    }

    pub fn to_json(&self) -> String {
        let mut lines = vec![
            "{".to_string(),
            format!("  \"index_version\": {},", self.index_version),
            format!(
                "  \"authority\": \"{}\",",
                escape_json_string(&self.authority)
            ),
            format!(
                "  \"updated_at\": \"{}\",",
                escape_json_string(&self.updated_at)
            ),
            "  \"candidates\": [".to_string(),
        ];
        for (index, candidate) in self.candidates.iter().enumerate() {
            let suffix = if index + 1 == self.candidates.len() {
                ""
            } else {
                ","
            };
            lines.push(format!(
                "    {{\"candidate_id\": \"{}\", \"path\": \"{}\", \"metadata_path\": \"{}\", \"captured_at\": \"{}\", \"status\": \"{}\"}}{}",
                escape_json_string(&candidate.candidate_id),
                escape_json_string(&candidate.path),
                escape_json_string(&candidate.metadata_path),
                escape_json_string(&candidate.captured_at),
                escape_json_string(&candidate.status),
                suffix,
            ));
        }
        lines.extend(["  ]".to_string(), "}".to_string()]);
        lines.join("\n") + "\n"
    }

    pub fn from_json(text: &str) -> Result<Self, CandidateError> {
        let value: serde_json::Value = serde_json::from_str(text)
            .map_err(|err| CandidateError::MalformedIndex(err.to_string()))?;
        let index_version = value
            .get("index_version")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| CandidateError::MalformedIndex("missing index_version".to_string()))?;
        if index_version != 1 {
            return Err(CandidateError::MalformedIndex(format!(
                "unsupported index_version {index_version}"
            )));
        }
        let authority = required_index_string(&value, "authority")?;
        if authority != CANDIDATE_INDEX_AUTHORITY {
            return Err(CandidateError::MalformedIndex(
                "unexpected candidate index authority".to_string(),
            ));
        }
        let updated_at = required_index_string(&value, "updated_at")?;
        let candidates_value = value
            .get("candidates")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| {
                CandidateError::MalformedIndex("missing candidates array".to_string())
            })?;
        let mut candidates = Vec::new();
        let mut seen = BTreeSet::new();
        for candidate in candidates_value {
            let entry = CandidateIndexEntry {
                candidate_id: required_index_string(candidate, "candidate_id")?,
                path: required_index_string(candidate, "path")?,
                metadata_path: required_index_string(candidate, "metadata_path")?,
                captured_at: required_index_string(candidate, "captured_at")?,
                status: required_index_string(candidate, "status")?,
            };
            if !is_safe_candidate_id(&entry.candidate_id) {
                return Err(CandidateError::MalformedIndex(format!(
                    "invalid candidate id {}",
                    entry.candidate_id
                )));
            }
            if !is_safe_relative_candidate_path(&entry.path)
                || !entry.path.ends_with(&format!("/{CANDIDATE_FILE}"))
            {
                return Err(CandidateError::MalformedIndex(format!(
                    "invalid candidate path {}",
                    entry.path
                )));
            }
            if !is_safe_relative_candidate_path(&entry.metadata_path)
                || !entry.metadata_path.ends_with(&format!("/{METADATA_FILE}"))
            {
                return Err(CandidateError::MalformedIndex(format!(
                    "invalid metadata path {}",
                    entry.metadata_path
                )));
            }
            if !seen.insert(entry.candidate_id.clone()) {
                return Err(CandidateError::MalformedIndex(format!(
                    "duplicate candidate id {}",
                    entry.candidate_id
                )));
            }
            candidates.push(entry);
        }
        Ok(Self {
            index_version,
            authority,
            updated_at,
            candidates,
        })
    }
}

impl From<&CandidateMetadata> for CandidateIndexEntry {
    fn from(value: &CandidateMetadata) -> Self {
        Self {
            candidate_id: value.candidate_id.clone(),
            path: format!("{}/{}", value.candidate_id, CANDIDATE_FILE),
            metadata_path: format!("{}/{}", value.candidate_id, METADATA_FILE),
            captured_at: value.captured_at.clone(),
            status: value.status.clone(),
        }
    }
}

impl CandidateIndexState {
    pub fn as_cli_status(self) -> &'static str {
        match self {
            CandidateIndexState::Available => "available",
            CandidateIndexState::Missing => "missing",
            CandidateIndexState::Malformed => "malformed",
            CandidateIndexState::Rebuilt => "rebuilt",
        }
    }
}

fn sorted_candidate_metadata(
    candidates_dir: &Path,
) -> Result<Vec<CandidateMetadata>, CandidateError> {
    let mut records = list_candidate_metadata(candidates_dir)?;
    records.sort_by(|a, b| {
        a.captured_at
            .cmp(&b.captured_at)
            .then(a.candidate_id.cmp(&b.candidate_id))
    });
    Ok(records)
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

fn candidate_directory_ids(candidates_dir: &Path) -> Result<BTreeSet<String>, CandidateError> {
    if !candidates_dir.is_dir() {
        return Ok(BTreeSet::new());
    }
    let mut ids = BTreeSet::new();
    for entry in fs::read_dir(candidates_dir).map_err(|err| CandidateError::Io(err.to_string()))? {
        let entry = entry.map_err(|err| CandidateError::Io(err.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|value| value.to_str()) {
                ids.insert(name.to_string());
            }
        }
    }
    Ok(ids)
}

fn write_candidate_index_atomic(
    candidates_dir: &Path,
    index: &CandidateIndex,
) -> Result<(), CandidateError> {
    fs::create_dir_all(candidates_dir).map_err(|err| CandidateError::Io(err.to_string()))?;
    let index_path = candidates_dir.join(INDEX_FILE);
    let tmp_path = candidates_dir.join("index.json.tmp");
    fs::write(&tmp_path, index.to_json()).map_err(|err| {
        CandidateError::Io(format!(
            "unable to write candidate index temp file {}: {err}",
            tmp_path.display()
        ))
    })?;
    fs::rename(&tmp_path, &index_path).map_err(|err| {
        CandidateError::Io(format!(
            "unable to replace candidate index {}: {err}",
            index_path.display()
        ))
    })?;
    Ok(())
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

fn required_index_string(value: &serde_json::Value, key: &str) -> Result<String, CandidateError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| CandidateError::MalformedIndex(format!("missing string field {key}")))
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

fn is_safe_relative_candidate_path(path: &str) -> bool {
    let parts = path.split('/').collect::<Vec<_>>();
    parts.len() == 2
        && is_safe_candidate_id(parts[0])
        && matches!(parts[1], CANDIDATE_FILE | METADATA_FILE)
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
