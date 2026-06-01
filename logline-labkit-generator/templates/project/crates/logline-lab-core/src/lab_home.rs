use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub const LOCAL_DIR: &str = ".logline-lab";
pub const MANIFEST_FILE: &str = "lab.manifest.yaml";
pub const STATUS_FILE: &str = "STATUS.md";
pub const GHOSTS_FILE: &str = "GHOSTS.md";
pub const LOCAL_DIRS: &[&str] = &["candidates", "reports", "ghosts", "profiles", "packs"];
pub const PROJECT_REQUIRED_PATHS: &[&str] = &[
    "schemas/logline-act.schema.json",
    "schemas/lab-manifest.schema.json",
    "examples/acts/minimal.act.json",
    "docs/01-logline-act.md",
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
    pub ghosts: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub home: PathBuf,
    pub failures: Vec<String>,
    pub ghosts: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct LabHomeStatus {
    pub home: PathBuf,
    pub manifest_exists: bool,
    pub local_ready: bool,
    pub candidate_count: usize,
    pub local_candidate_queue_available: bool,
    pub ghosts: Vec<String>,
    pub reports_available: usize,
    pub latest_report: Option<PathBuf>,
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
        fs::create_dir_all(self.local_dir())?;
        write_if_missing(&self.manifest_path(), default_manifest())?;
        write_if_missing(&self.status_path(), default_status())?;
        write_if_missing(&self.ghosts_path(), default_ghosts())?;
        for dir in LOCAL_DIRS {
            let path = self.local_dir().join(dir);
            fs::create_dir_all(&path)?;
            write_if_missing(&path.join(".keep"), "")?;
        }
        Ok(InitReport {
            home: self.home.clone(),
            manifest: self.manifest_path(),
            ghosts: INITIAL_GHOSTS.to_vec(),
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
        DoctorReport {
            home: self.home.clone(),
            failures,
            ghosts: INITIAL_GHOSTS.to_vec(),
        }
    }

    pub fn status(&self) -> LabHomeStatus {
        let doctor = self.doctor();
        LabHomeStatus {
            home: self.home.clone(),
            manifest_exists: self.manifest_path().is_file(),
            local_ready: doctor.failures.is_empty(),
            candidate_count: self.candidate_count(),
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
        }
    }
}

impl InitReport {
    pub fn to_text(&self) -> String {
        let mut lines = vec![
            "initialized local LogLine Lab home".to_string(),
            format!("home: {}", self.home.display()),
            format!("manifest: {}", self.manifest.display()),
            "authority: local workspace only; not official spine; not receipt".to_string(),
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
        if self.is_ok() {
            return [
                "doctor: ok".to_string(),
                format!("home: {}", self.home.display()),
                "scope: local workspace only".to_string(),
                "local candidate queue: available".to_string(),
                "remote spine: ghost remote-spine-unconfigured".to_string(),
            ]
            .join("\n");
        }
        let mut lines = vec![
            "doctor: failed".to_string(),
            format!("home: {}", self.home.display()),
            "scope: local workspace only".to_string(),
            "failures:".to_string(),
        ];
        for failure in &self.failures {
            lines.push(format!("  - {failure}"));
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
            "ghosts:".to_string(),
        ];
        for ghost in &self.ghosts {
            lines.push(format!("  - {ghost}"));
        }
        lines.extend([
            "remote spine status: ghost/unconfigured".to_string(),
            "receipt status: unavailable/unimplemented".to_string(),
            "interactive UX: ghost/unimplemented".to_string(),
            "LLM translator: ghost/unimplemented".to_string(),
            "authority: local workspace only; not official spine".to_string(),
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

fn default_manifest() -> &'static str {
    "manifest_version: 1\nlab:\n  id: local-lab\n  name: Local LogLine Lab\n  kind: logline_lab_instance\nloads:\n  canon: referenced\n  pack: none\n  profile: local-workspace\nrules:\n  act_shape: nine-slot\n  local_home_is_authority: false\n  projections_are_read_models: true\n  llm_is_authority: false\n"
}

fn default_status() -> &'static str {
    "# Local LogLine Lab Status\n\nStatus: local workspace initialized.\n\nAuthority: local workspace only; not official spine; not receipt.\n\nRemote spine: Ghost remote-spine-unconfigured.\nReceipt closure: Ghost receipt-closure-unimplemented.\n"
}

fn default_ghosts() -> &'static str {
    "# Local LogLine Lab Ghosts\n\n- remote-spine-unconfigured\n- evidence-registry-unimplemented\n- receipt-closure-unimplemented\n- interactive-lab-surface-unimplemented\n- llm-translator-unimplemented\n- yaml-act-parser-unimplemented\n"
}
