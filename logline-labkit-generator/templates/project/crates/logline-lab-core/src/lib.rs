pub mod candidates;
pub mod catalog;
pub mod ghosts;
pub mod lab_home;
pub mod projections;
pub mod reports;
pub mod supabase;

use std::path::Path;

pub use lab_home::LabHome;
use logline_act::{parse_act_json, ACT_SLOTS};

pub fn init_lab_home(home: impl AsRef<Path>) -> std::io::Result<lab_home::InitReport> {
    LabHome::new(home.as_ref()).init()
}

pub fn init_lab_home_with_selection(
    home: impl AsRef<Path>,
    pack_id: &str,
    profile_id: &str,
) -> std::io::Result<lab_home::InitReport> {
    LabHome::new(home.as_ref()).init_with_selection(pack_id, profile_id)
}

pub fn doctor_report_for(home: impl AsRef<Path>) -> lab_home::DoctorReport {
    LabHome::new(home.as_ref()).doctor()
}

pub fn status_report_for(home: impl AsRef<Path>) -> lab_home::LabHomeStatus {
    LabHome::new(home.as_ref()).status()
}

pub fn capture_candidate(
    home: impl AsRef<Path>,
    source_file: impl AsRef<Path>,
) -> Result<candidates::CaptureReport, candidates::CandidateError> {
    LabHome::new(home.as_ref()).capture_candidate(source_file.as_ref())
}

pub fn list_candidates(
    home: impl AsRef<Path>,
) -> Result<candidates::CandidateList, candidates::CandidateError> {
    LabHome::new(home.as_ref()).list_candidates()
}

pub fn get_candidate(
    home: impl AsRef<Path>,
    candidate_id: &str,
) -> Result<candidates::CandidateRecord, candidates::CandidateError> {
    LabHome::new(home.as_ref()).get_candidate(candidate_id)
}

pub fn list_ghosts(home: impl AsRef<Path>) -> Result<ghosts::GhostList, ghosts::GhostError> {
    LabHome::new(home.as_ref()).list_ghosts()
}

pub fn generate_daily_state_report(
    home: impl AsRef<Path>,
) -> Result<reports::DailyStateReport, reports::ReportError> {
    LabHome::new(home.as_ref()).generate_daily_state_report()
}

pub fn list_projections(
    home: impl AsRef<Path>,
) -> Result<projections::ProjectionList, projections::ProjectionError> {
    LabHome::new(home.as_ref()).list_projections()
}

pub fn generate_projection(
    home: impl AsRef<Path>,
    kind: projections::ProjectionKind,
) -> Result<projections::LocalSummaryProjection, projections::ProjectionError> {
    LabHome::new(home.as_ref()).generate_projection(kind)
}

pub fn doctor_report() -> String {
    doctor_report_for(".").to_text()
}

pub fn status_report() -> String {
    status_report_for(".").to_text()
}

pub fn validate_text(input: &str) -> String {
    match validate_text_result(input) {
        Ok(message) | Err(message) => message,
    }
}

pub fn validate_text_result(input: &str) -> Result<String, String> {
    match parse_act_json(input) {
        Ok(act) => {
            let status = act.status().unwrap_or("not-a-string");
            Ok(format!(
                "valid LogLine Act\nslots: {}/{}\nstatus: {}",
                act.slot_count(),
                ACT_SLOTS.len(),
                status
            ))
        }
        Err(error) => Err(format!("invalid LogLine Act\n{}", error)),
    }
}

pub fn emit_preview_result(input: &str) -> Result<String, String> {
    validate_text_result(input).map(|validation| {
        format!(
            "{}\npartial: act validated; emit preview only; no storage, no receipt, no remote spine write",
            validation
        )
    })
}

pub fn supabase_check_result() -> Result<String, String> {
    supabase::check_from_env()
        .map(|report| report.to_text())
        .map_err(|err| err.to_string())
}

pub fn emit_remote_result(input: &str) -> Result<String, String> {
    supabase::emit_act_from_env(input)
        .map(|report| report.to_text())
        .map_err(|err| err.to_string())
}
