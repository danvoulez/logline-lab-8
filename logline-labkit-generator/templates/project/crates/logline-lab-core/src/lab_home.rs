use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub const LOCAL_DIR: &str = ".logline-lab";
pub const MANIFEST_FILE: &str = "lab.manifest.yaml";
pub const STATUS_FILE: &str = "STATUS.md";
pub const GHOSTS_FILE: &str = "GHOSTS.md";
pub const LOCAL_DIRS: &[&str] = &[
    "candidates",
    "reports",
    "ghosts",
    "profiles",
    "packs",
    "projections",
];
pub const PROJECT_REQUIRED_PATHS: &[&str] = &[
    "schemas/logline-act.schema.json",
    "schemas/lab-manifest.schema.json",
    "schemas/candidate-metadata.schema.json",
    "schemas/candidate-index.schema.json",
    "schemas/pack-manifest.schema.json",
    "schemas/profile.schema.json",
    "examples/acts/minimal.act.json",
    "examples/fixtures.index.md",
    "docs/01-logline-act.md",
    "docs/09-schemas-and-fixtures.md",
];
pub const INITIAL_GHOSTS: &[&str] = &[
    "remote-spine-unconfigured",
    "evidence-registry-unimplemented",
    "receipt-closure-unimplemented",
    "interactive-lab-surface-unimplemented",
    "llm-translator-unimplemented",
    "yaml-act-parser-unimplemented",
];

#[derive(Debug, Clone)]
pub struct LabHome {
    home: PathBuf,
}

#[derive(Debug, Clone)]
pub struct InitReport {
    pub home: PathBuf,
    pub manifest: PathBuf,
    pub pack_id: String,
    pub profile_id: String,
    pub authority: String,
    pub ghosts: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub home: PathBuf,
    pub pack_id: Option<String>,
    pub profile_id: Option<String>,
    pub profile_capabilities: Vec<crate::catalog::ProfileCapability>,
    pub authority: String,
    pub failures: Vec<String>,
    pub warnings: Vec<String>,
    pub candidate_index_state: crate::candidates::CandidateIndexState,
    pub candidate_index_entries: usize,
    pub candidate_directories: usize,
    pub ghosts: Vec<&'static str>,
    pub projection_index_state: crate::projections::ProjectionIndexState,
}

#[derive(Debug, Clone)]
pub struct LabHomeStatus {
    pub home: PathBuf,
    pub pack_id: Option<String>,
    pub profile_id: Option<String>,
    pub profile_capabilities: Vec<crate::catalog::ProfileCapability>,
    pub authority: String,
    pub manifest_exists: bool,
    pub local_ready: bool,
    pub candidate_count: usize,
    pub candidate_index_state: crate::candidates::CandidateIndexState,
    pub local_candidate_queue_available: bool,
    pub ghosts: Vec<String>,
    pub reports_available: usize,
    pub latest_report: Option<PathBuf>,
    pub projections_available: usize,
    pub latest_projection: Option<PathBuf>,
}

impl LabHome {
    pub fn new(home: impl Into<PathBuf>) -> Self {
        Self { home: home.into() }
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn local_dir(&self) -> PathBuf {
        self.home.join(LOCAL_DIR)
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.local_dir().join(MANIFEST_FILE)
    }

    pub fn ghosts_path(&self) -> PathBuf {
        self.local_dir().join(GHOSTS_FILE)
    }

    pub fn status_path(&self) -> PathBuf {
        self.local_dir().join(STATUS_FILE)
    }

    pub fn init(&self) -> io::Result<InitReport> {
        let selection = crate::catalog::default_selection();
        self.init_with_selection(selection.pack_id, selection.profile_id)
    }

    pub fn init_with_selection(&self, pack_id: &str, profile_id: &str) -> io::Result<InitReport> {
        let selection = crate::catalog::validate_selection(pack_id, profile_id)
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;
        let profile =
            crate::catalog::known_profile(selection.profile_id).expect("validated profile");
        let ghosts = crate::catalog::selection_ghosts(&selection);
        fs::create_dir_all(self.local_dir())?;
        write_if_missing(
            &self.manifest_path(),
            &default_manifest(&selection, profile),
        )?;
        write_if_missing(&self.status_path(), &default_status(&selection, profile))?;
        write_if_missing(&self.ghosts_path(), &default_ghosts(&ghosts))?;
        for dir in LOCAL_DIRS {
            let path = self.local_dir().join(dir);
            fs::create_dir_all(&path)?;
            write_if_missing(&path.join(".keep"), "")?;
        }
        self.initialize_candidate_index_if_missing()?;
        self.initialize_projection_index_if_missing()?;
        Ok(InitReport {
            home: self.home.clone(),
            manifest: self.manifest_path(),
            pack_id: selection.pack_id.to_string(),
            profile_id: selection.profile_id.to_string(),
            authority: profile.authority_summary.to_string(),
            ghosts,
        })
    }

    pub fn doctor(&self) -> DoctorReport {
        let mut failures = Vec::new();
        require_dir(&mut failures, &self.home, "lab home");
        require_dir(&mut failures, &self.local_dir(), ".logline-lab/");
        require_file(
            &mut failures,
            &self.manifest_path(),
            ".logline-lab/lab.manifest.yaml",
        );
        require_file(&mut failures, &self.ghosts_path(), ".logline-lab/GHOSTS.md");
        require_file(&mut failures, &self.status_path(), ".logline-lab/STATUS.md");
        for dir in LOCAL_DIRS {
            require_dir(
                &mut failures,
                &self.local_dir().join(dir),
                &format!(".logline-lab/{dir}/"),
            );
        }
        check_candidate_records(&mut failures, &self.local_dir().join("candidates"));
        let candidate_index_inspection = self.candidate_index_inspection();
        failures.extend(candidate_index_inspection.failures.clone());
        let mut warnings = candidate_index_inspection.warnings.clone();
        let projection_index_state = self.projection_index_state();
        if projection_index_state == crate::projections::ProjectionIndexState::Malformed {
            warnings.push("projection index: malformed".to_string());
        }
        match find_project_root() {
            Some(root) => {
                for required in PROJECT_REQUIRED_PATHS {
                    let path = root.join(required);
                    if !path.exists() {
                        failures.push(format!("missing generated project path: {required}"));
                    }
                }
            }
            None => failures.push(
                "missing generated project root: required docs/examples/schemas not found"
                    .to_string(),
            ),
        }

        let selected = read_manifest_selection(&self.manifest_path());
        let mut pack_id = selected.as_ref().map(|selection| selection.0.clone());
        let mut profile_id = selected.as_ref().map(|selection| selection.1.clone());
        let mut profile_capabilities = Vec::new();
        let mut authority = "local workspace only; not official spine; not receipt".to_string();
        let mut ghosts = INITIAL_GHOSTS.to_vec();

        if let Some((pack, profile)) = selected {
            if crate::catalog::known_pack(&pack).is_none() {
                failures.push(format!("unknown pack: {pack}"));
            }
            match crate::catalog::known_profile(&profile) {
                Some(profile_manifest) => {
                    profile_capabilities = profile_manifest.capabilities.to_vec();
                    authority = profile_manifest.authority_summary.to_string();
                }
                None => failures.push(format!("unknown profile: {profile}")),
            }
            if let Ok(selection) = crate::catalog::validate_selection(&pack, &profile) {
                ghosts = crate::catalog::selection_ghosts(&selection);
                pack_id = Some(selection.pack_id.to_string());
                profile_id = Some(selection.profile_id.to_string());
            }
        }

        DoctorReport {
            home: self.home.clone(),
            pack_id,
            profile_id,
            profile_capabilities,
            authority,
            failures,
            warnings,
            candidate_index_state: candidate_index_inspection.state,
            candidate_index_entries: candidate_index_inspection.entries,
            candidate_directories: candidate_index_inspection.directories,
            ghosts,
            projection_index_state,
        }
    }

    pub fn selected_pack_profile(&self) -> Option<(String, String)> {
        read_manifest_selection(&self.manifest_path())
    }

    pub fn status(&self) -> LabHomeStatus {
        let doctor = self.doctor();
        let candidate_listing = self.candidate_listing_for_status();
        LabHomeStatus {
            home: self.home.clone(),
            pack_id: doctor.pack_id.clone(),
            profile_id: doctor.profile_id.clone(),
            profile_capabilities: doctor.profile_capabilities.clone(),
            authority: doctor.authority.clone(),
            manifest_exists: self.manifest_path().is_file(),
            local_ready: doctor.failures.is_empty(),
            candidate_count: candidate_listing
                .as_ref()
                .map(|list| list.records.len())
                .unwrap_or(0),
            candidate_index_state: candidate_listing
                .as_ref()
                .map(|list| list.index_status)
                .unwrap_or(crate::candidates::CandidateIndexState::Malformed),
            local_candidate_queue_available: self.local_dir().join("candidates").is_dir(),
            ghosts: crate::ghosts::read_ghost_keys_from_markdown(&self.ghosts_path())
                .unwrap_or_else(|_| {
                    INITIAL_GHOSTS
                        .iter()
                        .map(|ghost| (*ghost).to_string())
                        .collect()
                }),
            reports_available: self.report_count(),
            latest_report: self.latest_report_path(),
            projections_available: self.projection_count(),
            latest_projection: self.latest_projection_path(),
        }
    }

    fn candidate_listing_for_status(
        &self,
    ) -> Result<crate::candidates::CandidateList, crate::candidates::CandidateError> {
        if !self.local_dir().join("candidates").is_dir() {
            return Ok(crate::candidates::CandidateList {
                home: self.home.clone(),
                records: Vec::new(),
                index_status: crate::candidates::CandidateIndexState::Missing,
            });
        }
        self.list_candidates()
    }
}

impl InitReport {
    pub fn to_text(&self) -> String {
        let mut lines = vec![
            "initialized local LogLine Lab home".to_string(),
            format!("home: {}", self.home.display()),
            format!("manifest: {}", self.manifest.display()),
            format!("pack: {}", self.pack_id),
            format!("profile: {}", self.profile_id),
            format!("authority: {}", self.authority),
            "ghosts:".to_string(),
        ];
        for ghost in &self.ghosts {
            lines.push(format!("  - {ghost}"));
        }
        lines.join("\n")
    }
}

impl DoctorReport {
    pub fn is_ok(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn to_text(&self) -> String {
        let mut lines = vec![
            if self.is_ok() {
                "doctor: ok"
            } else {
                "doctor: failed"
            }
            .to_string(),
            format!("home: {}", self.home.display()),
            format!("scope: {}", self.authority),
        ];
        if let Some(pack_id) = &self.pack_id {
            if crate::catalog::known_pack(pack_id).is_some() {
                lines.push(format!("pack: {pack_id} known"));
            } else {
                lines.push(format!("unknown pack: {pack_id}"));
            }
        }
        if let Some(profile_id) = &self.profile_id {
            if crate::catalog::known_profile(profile_id).is_some() {
                lines.push(format!("profile: {profile_id} known"));
            } else {
                lines.push(format!("unknown profile: {profile_id}"));
            }
        }
        lines.push("local candidate queue: available".to_string());
        lines.push(format!(
            "candidate index: {}",
            self.candidate_index_state.as_cli_status()
        ));
        lines.push(format!(
            "candidate index entries: {}",
            self.candidate_index_entries
        ));
        lines.push(format!(
            "candidate directories: {}",
            self.candidate_directories
        ));
        lines.push(format!(
            "projection index: {}",
            self.projection_index_state.as_cli_status()
        ));
        lines.push(format!(
            "candidate index consistency: {}",
            if self
                .failures
                .iter()
                .any(|failure| failure.contains("candidate index"))
            {
                "failed"
            } else {
                "ok"
            }
        ));
        for capability in &self.profile_capabilities {
            if capability.key == "remote_spine" {
                lines.push(format!("remote spine: {}", capability.state));
            }
        }
        if self.profile_capabilities.is_empty() {
            lines.push("remote spine: ghost remote-spine-unconfigured".to_string());
        } else if self.profile_id.as_deref() == Some(crate::catalog::DEFAULT_PROFILE_ID) {
            lines.push("remote spine: ghost remote-spine-unconfigured".to_string());
        }
        if !self.warnings.is_empty() {
            lines.push("warnings:".to_string());
            for warning in &self.warnings {
                lines.push(format!("  - {warning}"));
            }
        }
        if !self.is_ok() {
            lines.push("failures:".to_string());
            for failure in &self.failures {
                lines.push(format!("  - {failure}"));
            }
        }
        lines.push("ghosts:".to_string());
        for ghost in &self.ghosts {
            lines.push(format!("  - {ghost}"));
        }
        lines.join("\n")
    }
}

impl LabHomeStatus {
    pub fn to_text(&self) -> String {
        let mut lines = vec![
            "status: local LogLine Lab workspace".to_string(),
            format!("home: {}", self.home.display()),
            format!("pack: {}", self.pack_id.as_deref().unwrap_or("unknown")),
            format!(
                "profile: {}",
                self.profile_id.as_deref().unwrap_or("unknown")
            ),
            format!("manifest exists: {}", yes_no(self.manifest_exists)),
            format!(
                "local workspace status: {}",
                if self.local_ready {
                    "ready"
                } else {
                    "missing required local structure"
                }
            ),
            format!("candidate_count: {}", self.candidate_count),
            format!(
                "candidate_index: {}",
                self.candidate_index_state.as_cli_status()
            ),
            format!(
                "local_candidate_queue: {}",
                if self.local_candidate_queue_available {
                    "available"
                } else {
                    "missing"
                }
            ),
            format!("ghost count: {}", self.ghosts.len()),
            format!("reports_available: {}", self.reports_available),
            format!(
                "latest_report: {}",
                self.latest_report
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
            format!("projections_available: {}", self.projections_available),
            format!(
                "latest_projection: {}",
                self.latest_projection
                    .as_ref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
            "ghosts:".to_string(),
        ];
        for ghost in &self.ghosts {
            lines.push(format!("  - {ghost}"));
        }
        lines.push("profile capability state:".to_string());
        for capability in &self.profile_capabilities {
            lines.push(format!("  - {}: {}", capability.key, capability.state));
        }
        let remote_state = self
            .profile_capabilities
            .iter()
            .find(|capability| capability.key == "remote_spine")
            .map(|capability| capability.state)
            .unwrap_or("ghost/unconfigured");
        lines.extend([
            format!("remote_spine: {remote_state}"),
            "remote spine status: ghost/unconfigured".to_string(),
            "receipt status: unavailable/unimplemented".to_string(),
            "interactive UX: ghost/unimplemented".to_string(),
            "LLM translator: ghost/unimplemented".to_string(),
            format!("authority: {}", self.authority),
        ]);
        lines.join("\n")
    }
}

pub fn find_project_root() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;
    loop {
        if PROJECT_REQUIRED_PATHS
            .iter()
            .all(|required| dir.join(required).exists())
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn check_candidate_records(failures: &mut Vec<String>, candidates_dir: &Path) {
    if !candidates_dir.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(candidates_dir) else {
        failures.push("unable to read .logline-lab/candidates/".to_string());
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let metadata = path.join("metadata.json");
            let candidate = path.join("candidate.json");
            if !metadata.is_file() {
                failures.push(format!(
                    "missing candidate metadata: {}",
                    metadata.display()
                ));
            }
            if !candidate.is_file() {
                failures.push(format!(
                    "missing candidate content: {}",
                    candidate.display()
                ));
            }
        }
    }
}

fn require_dir(failures: &mut Vec<String>, path: &Path, label: &str) {
    if !path.is_dir() {
        failures.push(format!("missing directory: {label}"));
    }
}

fn require_file(failures: &mut Vec<String>, path: &Path, label: &str) {
    if !path.is_file() {
        failures.push(format!("missing file: {label}"));
    }
}

fn write_if_missing(path: &Path, content: &str) -> io::Result<()> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn read_manifest_selection(path: &Path) -> Option<(String, String)> {
    let text = fs::read_to_string(path).ok()?;
    let mut in_selected = false;
    let mut pack = None;
    let mut profile = None;
    for line in text.lines() {
        if line.trim() == "selected:" {
            in_selected = true;
            continue;
        }
        if in_selected && !line.starts_with("  ") && !line.trim().is_empty() {
            in_selected = false;
        }
        if in_selected {
            let trimmed = line.trim();
            if let Some(value) = trimmed.strip_prefix("pack:") {
                pack = Some(value.trim().to_string());
            } else if let Some(value) = trimmed.strip_prefix("profile:") {
                profile = Some(value.trim().to_string());
            }
        }
    }
    Some((pack?, profile?))
}

fn default_manifest(
    selection: &crate::catalog::SelectedPackProfile,
    profile: &crate::catalog::ProfileManifest,
) -> String {
    format!(
        "manifest_version: 1\nlab:\n  id: local-lab\n  name: Local LogLine Lab\n  kind: logline_lab_instance\nselected:\n  pack: {}\n  profile: {}\nloads:\n  canon: referenced\n  pack: selected local practice, not canon\n  profile: selected capability declaration\nauthority:\n  summary: {}\n  canon_amendment: false\n  local_workspace_only: true\n  receipt_closure: false\nrules:\n  act_shape: nine-slot\n  local_home_is_authority: false\n  projections_are_read_models: true\n  llm_is_authority: false\n",
        selection.pack_id, selection.profile_id, profile.authority_summary
    )
}

fn default_status(
    selection: &crate::catalog::SelectedPackProfile,
    profile: &crate::catalog::ProfileManifest,
) -> String {
    format!(
        "# Local LogLine Lab Status\n\nStatus: local workspace initialized.\n\nPack: {}.\nProfile: {}.\n\nAuthority: {}.\n\nRemote spine: Ghost remote-spine-unconfigured unless selected profile only declares it unimplemented/unconfigured.\nReceipt closure: Ghost receipt-closure-unimplemented.\n",
        selection.pack_id, selection.profile_id, profile.authority_summary
    )
}

fn default_ghosts(ghosts: &[&str]) -> String {
    let mut lines = vec!["# Local LogLine Lab Ghosts".to_string(), String::new()];
    for ghost in ghosts {
        lines.push(format!("- {ghost}"));
    }
    lines.push(String::new());
    lines.join("\n")
}
