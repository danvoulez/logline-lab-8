use serde_json::{Map, Value};
use std::{error::Error, fmt};

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

#[derive(Debug, Clone, PartialEq)]
pub struct LogLineAct {
    slots: Map<String, Value>,
}

impl LogLineAct {
    pub fn slots(&self) -> &Map<String, Value> {
        &self.slots
    }

    pub fn status(&self) -> Option<&str> {
        self.slots.get("status").and_then(Value::as_str)
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActValidationError {
    ParseError(String),
    NotObject,
    MissingSlot(String),
    UnknownSlot(String),
    ReservedMetadataSlot(String),
    Multiple(Vec<ActValidationError>),
}

impl ActValidationError {
    pub fn messages(&self) -> Vec<String> {
        match self {
            ActValidationError::Multiple(errors) => errors
                .iter()
                .flat_map(ActValidationError::messages)
                .collect(),
            _ => vec![self.to_string()],
        }
    }
}

impl fmt::Display for ActValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActValidationError::ParseError(message) => {
                write!(f, "invalid JSON syntax: {message}")
            }
            ActValidationError::NotObject => write!(
                f,
                "invalid LogLine Act: top-level JSON value must be an object with exactly the nine canonical slots"
            ),
            ActValidationError::MissingSlot(slot) => {
                write!(f, "missing required LogLine Act slot: {slot}")
            }
            ActValidationError::UnknownSlot(slot) => write!(
                f,
                "unknown top-level LogLine Act slot: {slot}; canonical Acts accept exactly: {}",
                ACT_SLOTS.join(", ")
            ),
            ActValidationError::ReservedMetadataSlot(slot) => {
                write!(f, "{}", reserved_metadata_message(slot))
            }
            ActValidationError::Multiple(errors) => {
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        writeln!(f)?;
                    }
                    write!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for ActValidationError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationReport {
    pub status: &'static str,
    pub slot_count: usize,
    pub act_status: Option<String>,
    pub missing_slots: Vec<String>,
    pub errors: Vec<String>,
}

pub fn parse_act_json(input: &str) -> Result<LogLineAct, ActValidationError> {
    let value = serde_json::from_str::<Value>(input)
        .map_err(|err| ActValidationError::ParseError(err.to_string()))?;
    let object = value.as_object().ok_or(ActValidationError::NotObject)?;

    let mut errors = Vec::new();

    for slot in ACT_SLOTS {
        if !object.contains_key(slot) {
            errors.push(ActValidationError::MissingSlot(slot.to_string()));
        }
    }

    for slot in object.keys() {
        if !ACT_SLOTS.contains(&slot.as_str()) {
            if is_reserved_metadata_slot(slot) {
                errors.push(ActValidationError::ReservedMetadataSlot(slot.clone()));
            } else {
                errors.push(ActValidationError::UnknownSlot(slot.clone()));
            }
        }
    }

    if errors.is_empty() {
        Ok(LogLineAct {
            slots: object.clone(),
        })
    } else if errors.len() == 1 {
        Err(errors.remove(0))
    } else {
        Err(ActValidationError::Multiple(errors))
    }
}

pub fn validate_act_text(input: &str) -> ValidationReport {
    match parse_act_json(input) {
        Ok(act) => ValidationReport {
            status: "implemented",
            slot_count: act.slot_count(),
            act_status: act.status().map(ToOwned::to_owned),
            missing_slots: Vec::new(),
            errors: Vec::new(),
        },
        Err(error) => ValidationReport {
            status: "invalid",
            slot_count: 0,
            act_status: None,
            missing_slots: collect_missing_slots(&error),
            errors: error.messages(),
        },
    }
}

fn collect_missing_slots(error: &ActValidationError) -> Vec<String> {
    match error {
        ActValidationError::MissingSlot(slot) => vec![slot.clone()],
        ActValidationError::Multiple(errors) => {
            errors.iter().flat_map(collect_missing_slots).collect()
        }
        _ => Vec::new(),
    }
}

fn is_reserved_metadata_slot(slot: &str) -> bool {
    matches!(
        slot,
        "selected_branch"
            | "runtime_envelope"
            | "runtime_id"
            | "hash"
            | "hashes"
            | "type_hint"
            | "type_hints"
            | "provenance"
            | "metadata"
    )
}

fn reserved_metadata_message(slot: &str) -> String {
    match slot {
        "selected_branch" => "selected_branch is not a LogLine Act slot; it may exist as metadata/projection/practice output outside the canonical Act.".to_string(),
        "runtime_envelope" => "runtime_envelope is provenance metadata or pack/profile practice; it is not a tenth semantic slot.".to_string(),
        "runtime_id" => "runtime_id is runtime metadata; it is not a LogLine Act slot.".to_string(),
        "hash" | "hashes" => format!("{slot} belongs to metadata/projection/practice material; it is not a LogLine Act slot."),
        "type_hint" | "type_hints" => format!("{slot} belongs to profile/practice interpretation material; it is not a LogLine Act slot."),
        "provenance" | "metadata" => format!("{slot} is metadata outside the canonical Act; it is not one of the nine semantic slots."),
        _ => format!("{slot} is not a LogLine Act slot; keep metadata outside the canonical Act."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_nine_slots() {
        let input = r#"{"who":"dan","did":"tested","this":"act","when":"now","confirmed_by":"stdout","if_ok":"continue","if_doubt":"review","if_not":"reject","status":"candidate"}"#;
        let act = parse_act_json(input).expect("valid act");
        assert_eq!(act.slot_count(), 9);
        assert_eq!(act.status(), Some("candidate"));
    }
}
