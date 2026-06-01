use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    candidates::CandidateError,
    ghosts::{GhostError, GhostList},
    lab_home::LabHome,
};

pub const REPORTS_DIR: &str = "reports";
pub const DAILY_STATE_FILE: &str = "daily-state.md";

#[derive(Debug, Clone)]
pub struct DailyStateReport {
    pub home: PathBuf,
    pub path: PathBuf,
    pub candidate_count: usize,
    pub candidate_index_state: crate::candidates::CandidateIndexState,
    pub ghost_count: usize,
}

#[derive(Debug)]
pub enum ReportError {
    Ghost(GhostError),
    Candidate(CandidateError),
    Io(String),
}

impl std::fmt::Display for ReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportError::Ghost(err) => write!(f, "{err}"),
            ReportError::Candidate(err) => write!(f, "{err}"),
            ReportError::Io(message) => write!(f, "report I/O error: {message}"),
        }
    }
}

impl std::error::Error for ReportError {}

impl From<GhostError> for ReportError {
    fn from(value: GhostError) -> Self {
        Self::Ghost(value)
    }
}

impl From<CandidateError> for ReportError {
    fn from(value: CandidateError) -> Self {
        Self::Candidate(value)
    }
}

impl LabHome {
    pub fn reports_dir(&self) -> PathBuf {
        self.local_dir().join(REPORTS_DIR)
    }

    pub fn daily_state_report_path(&self) -> PathBuf {
        self.reports_dir().join(DAILY_STATE_FILE)
    }

    pub fn report_count(&self) -> usize {
        list_report_paths(&self.reports_dir()).len()
    }

    pub fn latest_report_path(&self) -> Option<PathBuf> {
        list_report_paths(&self.reports_dir()).into_iter().max()
    }

    pub fn generate_daily_state_report(&self) -> Result<DailyStateReport, ReportError> {
        self.require_initialized_for_local_reads()?;
        let ghost_list = self.list_ghosts()?;
        let candidate_list = self.list_candidates()?;
        let path = self.daily_state_report_path();
        fs::create_dir_all(self.reports_dir()).map_err(|err| ReportError::Io(err.to_string()))?;
        let body = render_daily_state_report(
            self,
            &ghost_list,
            &candidate_list.records,
            candidate_list.index_status,
        );
        fs::write(&path, body).map_err(|err| {
            ReportError::Io(format!(
                "unable to write daily-state report {}: {err}",
                path.display()
            ))
        })?;
        Ok(DailyStateReport {
            home: self.home().to_path_buf(),
            path,
            candidate_count: candidate_list.records.len(),
            candidate_index_state: candidate_list.index_status,
            ghost_count: ghost_list.ghosts.len(),
        })
    }
}

impl DailyStateReport {
    pub fn to_text(&self) -> String {
        [
            "daily-state report generated".to_string(),
            format!("path: {}", self.path.display()),
            format!("candidate_count: {}", self.candidate_count),
            format!(
                "candidate_index: {}",
                self.candidate_index_state.as_cli_status()
            ),
            format!("ghost_count: {}", self.ghost_count),
            "authority: local workspace projection only; not receipt; not evidence; not remote sync".to_string(),
        ]
        .join("\n")
    }
}

fn render_daily_state_report(
    lab_home: &LabHome,
    ghost_list: &GhostList,
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
    let mut lines = vec![
        "# Daily Lab State".to_string(),
        String::new(),
        format!("Generated at: unix-ms-{}", now_unix_millis()),
        format!("Home: {}", lab_home.home().display()),
        String::new(),
        "## Authority".to_string(),
        String::new(),
        "This report is a local workspace projection.".to_string(),
        "It is not official spine, not evidence, not a receipt, and not remote sync.".to_string(),
        String::new(),
        "## Pack/Profile".to_string(),
        String::new(),
        format!("- Pack: {pack_id}"),
        format!("- Profile: {profile_id}"),
        String::new(),
        "## Profile Capability State".to_string(),
        String::new(),
    ];
    if let Some(profile) = profile {
        for capability in profile.capabilities {
            lines.push(format!("- {}: {}", capability.key, capability.state));
        }
    } else {
        lines.push("- unknown profile: unavailable/ghost".to_string());
    }
    lines.extend([
        String::new(),
        "## Counts".to_string(),
        String::new(),
        format!("- Candidates: {}", candidates.len()),
        format!(
            "- Candidate index: {}",
            candidate_index_state.as_cli_status()
        ),
        format!("- Ghosts: {}", ghost_list.ghosts.len()),
        String::new(),
        "## Ghosts".to_string(),
        String::new(),
    ]);
    if ghost_list.ghosts.is_empty() {
        lines.push("- none listed locally".to_string());
    } else {
        for ghost in &ghost_list.ghosts {
            lines.push(format!("- {ghost}"));
        }
    }
    lines.extend([
        String::new(),
        "## Candidate Index".to_string(),
        String::new(),
        format!("- State: {}", candidate_index_state.as_cli_status()),
        format!("- Candidates indexed: {}", candidates.len()),
        "- Authority: local operational index only; not official spine".to_string(),
        String::new(),
        "## Candidate Queue".to_string(),
        String::new(),
    ]);
    if candidates.is_empty() {
        lines.push("- none".to_string());
    } else {
        for candidate in candidates {
            lines.push(format!("- {}", candidate.candidate_id));
        }
    }
    lines.extend([
        String::new(),
        "## Remaining unavailable capabilities".to_string(),
        String::new(),
        "- remote spine".to_string(),
        "- evidence registry".to_string(),
        "- receipt closure".to_string(),
        "- interactive lab surface".to_string(),
        "- LLM translator".to_string(),
        "- YAML Act parser".to_string(),
        String::new(),
    ]);
    lines.join("\n")
}

fn list_report_paths(reports_dir: &std::path::Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(reports_dir) else {
        return Vec::new();
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn now_unix_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
