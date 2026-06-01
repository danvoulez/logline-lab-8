use std::{fs, path::PathBuf};

use crate::lab_home::{LabHome, GHOSTS_FILE, INITIAL_GHOSTS, MANIFEST_FILE};

#[derive(Debug, Clone)]
pub struct GhostList {
    pub home: PathBuf,
    pub ghosts: Vec<String>,
}

#[derive(Debug)]
pub enum GhostError {
    LabHomeNotInitialized(PathBuf),
    Io(String),
}

impl std::fmt::Display for GhostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GhostError::LabHomeNotInitialized(home) => {
                write!(f, "Lab home not initialized: {}", home.display())
            }
            GhostError::Io(message) => write!(f, "ghost I/O error: {message}"),
        }
    }
}

impl std::error::Error for GhostError {}

impl LabHome {
    pub fn require_initialized_for_local_reads(&self) -> Result<(), GhostError> {
        if !self.local_dir().is_dir() || !self.local_dir().join(MANIFEST_FILE).is_file() {
            return Err(GhostError::LabHomeNotInitialized(self.home().to_path_buf()));
        }
        Ok(())
    }

    pub fn list_ghosts(&self) -> Result<GhostList, GhostError> {
        self.require_initialized_for_local_reads()?;
        let mut ghosts = read_ghost_keys_from_markdown(&self.local_dir().join(GHOSTS_FILE))?;
        let ghosts_dir = self.local_dir().join("ghosts");
        if ghosts_dir.is_dir() {
            for entry in fs::read_dir(&ghosts_dir).map_err(|err| GhostError::Io(err.to_string()))? {
                let entry = entry.map_err(|err| GhostError::Io(err.to_string()))?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|value| value.to_str()) {
                        if stem != ".keep" && !stem.is_empty() {
                            ghosts.push(stem.to_string());
                        }
                    }
                }
            }
        }
        if ghosts.is_empty() {
            ghosts = INITIAL_GHOSTS
                .iter()
                .map(|ghost| (*ghost).to_string())
                .collect();
        }
        ghosts.sort();
        ghosts.dedup();
        Ok(GhostList {
            home: self.home().to_path_buf(),
            ghosts,
        })
    }
}

impl GhostList {
    pub fn to_text(&self) -> String {
        let mut lines = vec![format!("ghosts: {}", self.ghosts.len())];
        for ghost in &self.ghosts {
            lines.push(format!("- {ghost}"));
        }
        lines.push("authority: local workspace Ghost list only".to_string());
        lines.join("\n")
    }
}

pub fn read_ghost_keys_from_markdown(path: &std::path::Path) -> Result<Vec<String>, GhostError> {
    let text = fs::read_to_string(path).map_err(|err| {
        GhostError::Io(format!(
            "unable to read local Ghost list {}: {err}",
            path.display()
        ))
    })?;
    let ghosts = text
        .lines()
        .map(str::trim)
        .filter_map(|line| {
            line.strip_prefix("- Ghost: ")
                .or_else(|| line.strip_prefix("- ghost: "))
                .or_else(|| line.strip_prefix("- "))
        })
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.contains(':'))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    Ok(ghosts)
}
