pub mod candidates;
pub mod lab_home;

use std::path::Path;

use lab_home::LabHome;
use logline_act::{parse_act_json, ACT_SLOTS};

pub fn init_lab_home(home: impl AsRef<Path>) -> std::io::Result<lab_home::InitReport> {
    LabHome::new(home.as_ref()).init()
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
