use logline_act::validate_act_text;

pub fn doctor_report() -> &'static str {
    "implemented: core loaded; profile checks are partial; no external provider required"
}

pub fn status_report() -> &'static str {
    "partial: generated lab kit is present; runtime execution surfaces are ghosts"
}

pub fn validate_text(input: &str) -> String {
    let report = validate_act_text(input);
    if report.missing_slots.is_empty() {
        "implemented: act contains the nine canonical slots".to_string()
    } else {
        format!("partial: missing slots: {}", report.missing_slots.join(", "))
    }
}
