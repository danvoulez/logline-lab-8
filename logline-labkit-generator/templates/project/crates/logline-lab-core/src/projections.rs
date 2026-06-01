use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    ghosts::GhostError,
    lab_home::{LabHome, LOCAL_DIR},
};

pub const PROJECTIONS_DIR: &str = "projections";
pub const LOCAL_SUMMARY_FILE: &str = "local-summary.md";
pub const PROJECTION_INDEX_FILE: &str = "projection-index.json";
pub const LOCAL_SUMMARY_KIND: &str = "local-summary";
pub const PROJECTION_INDEX_AUTHORITY: &str = "local_projection_index_not_official_spine";
pub const LOCAL_READ_MODEL_AUTHORITY: &str = "local_read_model_not_truth";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionKind {
    LocalSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionIndexState {
    Available,
    Missing,
    Malformed,
}

#[derive(Debug, Clone)]
pub struct ProjectionState {
    pub projection_id: String,
    pub kind: String,
    pub path: PathBuf,
    pub generated_at: Option<String>,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct ProjectionList {
    pub home: PathBuf,
    pub projections: Vec<ProjectionState>,
    pub available_kinds: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct LocalSummaryProjection {
    pub home: PathBuf,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum ProjectionError {
    Ghost(GhostError),
    UnknownProjection(String),
    Io(String),
}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectionError::Ghost(err) => write!(f, "{err}"),
            ProjectionError::UnknownProjection(kind) => write!(f, "unknown projection: {kind}"),
            ProjectionError::Io(message) => write!(f, "projection I/O error: {message}"),
        }
    }
}

impl std::error::Error for ProjectionError {}

impl From<GhostError> for ProjectionError {
    fn from(value: GhostError) -> Self {
        Self::Ghost(value)
    }
}

impl ProjectionKind {
    pub fn parse(kind: &str) -> Result<Self, ProjectionError> {
        match kind {
            LOCAL_SUMMARY_KIND => Ok(Self::LocalSummary),
            other => Err(ProjectionError::UnknownProjection(other.to_string())),
        }
    }
}

impl ProjectionIndexState {
    pub fn as_cli_status(self) -> &'static str {
        match self {
            ProjectionIndexState::Available => "available",
            ProjectionIndexState::Missing => "missing",
            ProjectionIndexState::Malformed => "malformed",
        }
    }
}

impl LabHome {
    pub fn projections_dir(&self) -> PathBuf {
        self.local_dir().join(PROJECTIONS_DIR)
    }

    pub fn local_summary_projection_path(&self) -> PathBuf {
        self.projections_dir().join(LOCAL_SUMMARY_FILE)
    }

    pub fn projection_index_path(&self) -> PathBuf {
        self.projections_dir().join(PROJECTION_INDEX_FILE)
    }

    pub fn initialize_projection_index_if_missing(&self) -> std::io::Result<()> {
        fs::create_dir_all(self.projections_dir())?;
        if !self.projection_index_path().exists() {
            fs::write(self.projection_index_path(), empty_projection_index())?;
        }
        Ok(())
    }

    pub fn projection_count(&self) -> usize {
        self.list_projection_files().len()
    }

    pub fn latest_projection_path(&self) -> Option<PathBuf> {
        self.list_projection_files().into_iter().max()
    }

    pub fn projection_index_state(&self) -> ProjectionIndexState {
        let path = self.projection_index_path();
        if !path.is_file() {
            return ProjectionIndexState::Missing;
        }
        match fs::read_to_string(&path)
            .ok()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        {
            Some(_) => ProjectionIndexState::Available,
            None => ProjectionIndexState::Malformed,
        }
    }

    pub fn list_projections(&self) -> Result<ProjectionList, ProjectionError> {
        self.require_initialized_for_local_reads()?;
        let local_summary_path = self.local_summary_projection_path();
        let projections = if local_summary_path.is_file() {
            vec![ProjectionState {
                projection_id: LOCAL_SUMMARY_KIND.to_string(),
                kind: LOCAL_SUMMARY_KIND.to_string(),
                path: PathBuf::from(format!(
                    "{LOCAL_DIR}/{PROJECTIONS_DIR}/{LOCAL_SUMMARY_FILE}"
                )),
                generated_at: projection_generated_at(&local_summary_path),
                available: true,
            }]
        } else {
            Vec::new()
        };
        Ok(ProjectionList {
            home: self.home().to_path_buf(),
            projections,
            available_kinds: vec![LOCAL_SUMMARY_KIND],
        })
    }

    pub fn generate_projection(
        &self,
        kind: ProjectionKind,
    ) -> Result<LocalSummaryProjection, ProjectionError> {
        match kind {
            ProjectionKind::LocalSummary => self.generate_local_summary_projection(),
        }
    }

    pub fn generate_local_summary_projection(
        &self,
    ) -> Result<LocalSummaryProjection, ProjectionError> {
        self.require_initialized_for_local_reads()?;
        fs::create_dir_all(self.projections_dir())
            .map_err(|err| ProjectionError::Io(err.to_string()))?;
        let ghost_list = self.list_ghosts()?;
        let candidate_list = self
            .list_candidates()
            .map_err(|err| ProjectionError::Io(err.to_string()))?;
        let generated_at = format!("unix-ms-{}", now_unix_millis());
        let path = self.local_summary_projection_path();
        let body = render_local_summary_projection(
            self,
            &generated_at,
            &ghost_list.ghosts,
            &candidate_list.records,
            candidate_list.index_status,
        );
        fs::write(&path, body).map_err(|err| {
            ProjectionError::Io(format!(
                "unable to write local-summary projection {}: {err}",
                path.display()
            ))
        })?;
        write_projection_index_atomic(&self.projections_dir(), &generated_at)?;
        Ok(LocalSummaryProjection {
            home: self.home().to_path_buf(),
            path,
        })
    }

    fn list_projection_files(&self) -> Vec<PathBuf> {
        let Ok(entries) = fs::read_dir(self.projections_dir()) else {
            return Vec::new();
        };
        let mut paths = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| {
                path.file_name().and_then(|value| value.to_str()) != Some(PROJECTION_INDEX_FILE)
            })
            .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
            .collect::<Vec<_>>();
        paths.sort();
        paths
    }
}

impl ProjectionList {
    pub fn to_text(&self) -> String {
        let mut lines = vec![format!("projections: {}", self.projections.len())];
        if self.projections.is_empty() {
            lines.push("available projection kinds:".to_string());
            for kind in &self.available_kinds {
                lines.push(format!("- {kind}"));
            }
            lines.push("authority: local read models only".to_string());
        } else {
            for projection in &self.projections {
                let updated = projection
                    .generated_at
                    .as_ref()
                    .map(|value| format!(" updated={value}"))
                    .unwrap_or_default();
                lines.push(format!(
                    "- {} path={} state={}{}",
                    projection.projection_id,
                    projection.path.display(),
                    if projection.available {
                        "available"
                    } else {
                        "missing"
                    },
                    updated
                ));
            }
            lines.push(
                "authority: local read models only; not truth; not receipt; not evidence"
                    .to_string(),
            );
        }
        lines.join("\n")
    }
}

impl LocalSummaryProjection {
    pub fn to_text(&self) -> String {
        [
            "local-summary projection generated".to_string(),
            format!("path: {}", self.path.display()),
            "authority: local read model only; not truth; not receipt; not evidence; not remote sync".to_string(),
        ]
        .join("\n")
    }
}

fn render_local_summary_projection(
    lab_home: &LabHome,
    generated_at: &str,
    ghosts: &[String],
    candidates: &[crate::candidates::CandidateMetadata],
    candidate_index_state: crate::candidates::CandidateIndexState,
) -> String {
    let selected = lab_home.selected_pack_profile();
    let pack_id = selected
        .as_ref()
        .map(|selection| selection.0.as_str())
        .unwrap_or("unknown");
    let profile_id = selected
        .as_ref()
        .map(|selection| selection.1.as_str())
        .unwrap_or("unknown");
    let profile = crate::catalog::known_profile(profile_id);
    let reports_available = lab_home.report_count();
    let latest_report = lab_home
        .latest_report_path()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "none".to_string());
    let mut projection_count = lab_home.projection_count();
    if !lab_home.local_summary_projection_path().is_file() {
        projection_count += 1;
    }

    let mut lines = vec![
        "# Local Summary Projection".to_string(),
        String::new(),
        format!("Generated at: {generated_at}"),
        format!("Home: {}", lab_home.home().display()),
        String::new(),
        "## Authority".to_string(),
        String::new(),
        "This projection is a local read model over workspace state.".to_string(),
        "It is not official spine truth, not evidence, not a receipt, and not remote sync."
            .to_string(),
        "It can be regenerated.".to_string(),
        String::new(),
        "## Pack/Profile".to_string(),
        String::new(),
        format!("- Pack: {pack_id}"),
        format!("- Profile: {profile_id}"),
        String::new(),
        "## Counts".to_string(),
        String::new(),
        format!("- Candidates: {}", candidates.len()),
        format!("- Ghosts: {}", ghosts.len()),
        format!("- Reports available: {reports_available}"),
        format!("- Projections available: {projection_count}"),
        String::new(),
        "## Candidate Queue".to_string(),
        String::new(),
        format!("- Index state: {}", candidate_index_state.as_cli_status()),
        format!("- Candidates indexed: {}", candidates.len()),
        String::new(),
        "## Ghosts".to_string(),
        String::new(),
    ];
    if ghosts.is_empty() {
        lines.push("- none listed locally".to_string());
    } else {
        for ghost in ghosts {
            lines.push(format!("- {ghost}"));
        }
    }
    lines.extend([
        String::new(),
        "## Profile Capability State".to_string(),
        String::new(),
    ]);
    if let Some(profile) = profile {
        for capability in profile.capabilities {
            lines.push(format!("- {}: {}", capability.key, capability.state));
        }
    } else {
        lines.push("- unknown profile: unavailable/ghost".to_string());
    }
    lines.extend([
        String::new(),
        "## Latest Local Report".to_string(),
        String::new(),
        format!("- {latest_report}"),
        String::new(),
    ]);
    lines.join("\n")
}

fn empty_projection_index() -> String {
    format!(
        "{{\n  \"index_version\": 1,\n  \"authority\": \"{PROJECTION_INDEX_AUTHORITY}\",\n  \"projections\": []\n}}\n"
    )
}

fn write_projection_index_atomic(
    projections_dir: &Path,
    generated_at: &str,
) -> Result<(), ProjectionError> {
    let path = projections_dir.join(PROJECTION_INDEX_FILE);
    let tmp = projections_dir.join(format!("{PROJECTION_INDEX_FILE}.tmp"));
    let body = format!(
        "{{\n  \"index_version\": 1,\n  \"authority\": \"{PROJECTION_INDEX_AUTHORITY}\",\n  \"updated_at\": \"{generated_at}\",\n  \"projections\": [\n    {{\n      \"projection_id\": \"{LOCAL_SUMMARY_KIND}\",\n      \"kind\": \"{LOCAL_SUMMARY_KIND}\",\n      \"path\": \"{LOCAL_SUMMARY_FILE}\",\n      \"generated_at\": \"{generated_at}\",\n      \"authority\": \"{LOCAL_READ_MODEL_AUTHORITY}\"\n    }}\n  ]\n}}\n"
    );
    fs::write(&tmp, body).map_err(|err| ProjectionError::Io(err.to_string()))?;
    fs::rename(&tmp, &path).map_err(|err| ProjectionError::Io(err.to_string()))?;
    Ok(())
}

fn projection_generated_at(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("Generated at: "))
        .map(ToOwned::to_owned)
}

fn now_unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
