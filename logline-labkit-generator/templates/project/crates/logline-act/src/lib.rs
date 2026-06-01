pub const ACT_SLOTS: [&str; 9] = [
    "who",
    "did",
    "this",
    "when",
    "confirmed_by",
    "if_ok",
    "if_doubt",
    "if_not",
    "status",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub status: &'static str,
    pub missing_slots: Vec<&'static str>,
}

pub fn validate_act_text(input: &str) -> ValidationReport {
    let missing_slots = ACT_SLOTS
        .iter()
        .copied()
        .filter(|slot| !contains_slot(input, slot))
        .collect::<Vec<_>>();

    ValidationReport {
        status: if missing_slots.is_empty() { "implemented" } else { "partial" },
        missing_slots,
    }
}

fn contains_slot(input: &str, slot: &str) -> bool {
    let json_key = format!("\"{}\"", slot);
    let yaml_key = format!("{}:", slot);
    input.contains(&json_key) || input.contains(&yaml_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_nine_slots() {
        let input = r#"{"who":"dan","did":"tested","this":"act","when":"now","confirmed_by":"stdout","if_ok":"continue","if_doubt":"review","if_not":"reject","status":"candidate"}"#;
        assert_eq!(validate_act_text(input).status, "implemented");
    }
}
