use logline_act::{parse_act_json, ActValidationError};

fn valid_minimal() -> &'static str {
    r#"{
      "who": "dan",
      "did": "record_decision",
      "this": {},
      "when": "2026-06-01T00:00:00Z",
      "confirmed_by": {},
      "if_ok": {},
      "if_doubt": {},
      "if_not": {},
      "status": "candidate"
    }"#
}

#[test]
fn valid_minimal_act_passes() {
    let act = parse_act_json(valid_minimal()).expect("minimal act validates");
    assert_eq!(act.slot_count(), 9);
    assert_eq!(act.status(), Some("candidate"));
}

#[test]
fn valid_ugly_candidate_passes() {
    let input = r#"{
      "who": "dan",
      "did": "dump_thought",
      "this": {
        "raw": "messy half-formed thought about LogLine packs",
        "quality": "ugly but captured"
      },
      "when": "2026-06-01T00:00:00Z",
      "confirmed_by": { "source": "operator_text" },
      "if_ok": { "project": ["planning_corpus"] },
      "if_doubt": { "open_ghost": "candidate-needs-qualification" },
      "if_not": { "reject": "not-operator-confirmed" },
      "status": "candidate"
    }"#;

    let act = parse_act_json(input).expect("ugly candidate validates");
    assert_eq!(act.slot_count(), 9);
}

#[test]
fn missing_confirmed_by_fails() {
    let input = r#"{
      "who": "dan",
      "did": "record_decision",
      "this": {},
      "when": "2026-06-01T00:00:00Z",
      "if_ok": {},
      "if_doubt": {},
      "if_not": {},
      "status": "candidate"
    }"#;

    let error = parse_act_json(input).expect_err("missing slot rejected");
    assert!(error
        .to_string()
        .contains("missing required LogLine Act slot: confirmed_by"));
}

#[test]
fn extra_selected_branch_fails_with_specific_error() {
    let input = valid_minimal().replace("\n    }", ",\n      \"selected_branch\": \"ok\"\n    }");

    let error = parse_act_json(&input).expect_err("selected branch rejected");
    assert!(error
        .to_string()
        .contains("selected_branch is not a LogLine Act slot"));
    assert!(error
        .to_string()
        .contains("metadata/projection/practice output"));
}

#[test]
fn extra_runtime_envelope_fails_with_specific_error() {
    let input = valid_minimal().replace("\n    }", ",\n      \"runtime_envelope\": {}\n    }");

    let error = parse_act_json(&input).expect_err("runtime_envelope rejected");
    assert!(error
        .to_string()
        .contains("runtime_envelope is provenance metadata"));
    assert!(error.to_string().contains("not a tenth semantic slot"));
}

#[test]
fn malformed_json_fails() {
    let error =
        parse_act_json(r#"{ "who": "dan", "did": "record" "#).expect_err("malformed JSON rejected");
    assert!(matches!(error, ActValidationError::ParseError(_)));
    assert!(error.to_string().contains("invalid JSON syntax"));
}

#[test]
fn arbitrary_extra_top_level_field_fails() {
    let input = valid_minimal().replace(
        "\n    }",
        ",\n      \"operator_note\": \"outside canonical act\"\n    }",
    );

    let error = parse_act_json(&input).expect_err("unknown top-level field rejected");
    assert!(error
        .to_string()
        .contains("unknown top-level LogLine Act slot: operator_note"));
}

#[test]
fn top_level_exactly_nine_slots_passes() {
    let act = parse_act_json(valid_minimal()).expect("exactly nine slots validate");
    let keys = act.slots().keys().map(String::as_str).collect::<Vec<_>>();
    assert_eq!(keys.len(), 9);
}
