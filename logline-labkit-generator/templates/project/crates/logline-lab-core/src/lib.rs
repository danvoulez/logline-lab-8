use logline_act::{parse_act_json, ACT_SLOTS};

pub fn doctor_report() -> &'static str {
    "implemented: core loaded; profile checks are partial; no external provider required"
}

pub fn status_report() -> &'static str {
    "partial: generated lab kit is present; runtime execution surfaces are ghosts"
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
