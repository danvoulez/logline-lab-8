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

fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("project root")
        .to_path_buf()
}

#[test]
fn generated_act_schema_documents_exact_nine_slots() {
    let schema_path = project_root().join("schemas/logline-act.schema.json");
    let schema = std::fs::read_to_string(&schema_path).expect("read logline-act schema");
    let value: serde_json::Value = serde_json::from_str(&schema).expect("schema is JSON");
    let required = value
        .get("required")
        .and_then(serde_json::Value::as_array)
        .expect("required array");
    let required_slots = required
        .iter()
        .map(|slot| slot.as_str().expect("slot string"))
        .collect::<Vec<_>>();
    assert_eq!(required_slots, logline_act::ACT_SLOTS.to_vec());
    assert_eq!(
        value.get("additionalProperties"),
        Some(&serde_json::Value::Bool(false))
    );
    assert!(!schema.contains("selected_branch"));
    assert!(!schema.contains("runtime_envelope"));
}

#[test]
fn generated_candidate_schemas_and_fixture_index_exist() {
    let root = project_root();
    for rel in [
        "schemas/candidate-metadata.schema.json",
        "schemas/candidate-index.schema.json",
        "examples/fixtures.index.md",
    ] {
        assert!(root.join(rel).is_file(), "missing {rel}");
    }
    let index = std::fs::read_to_string(root.join("examples/fixtures.index.md"))
        .expect("read fixture index");
    for fixture in [
        "examples/acts/minimal.act.json",
        "examples/candidates/ugly-candidate.json",
        "examples/invalid/extra-selected-branch.json",
        "examples/invalid/extra-runtime-envelope.json",
        "examples/candidates/candidate-metadata.json",
        "examples/candidates/candidate-index.json",
    ] {
        assert!(index.contains(fixture), "fixture index missing {fixture}");
    }
}

#[test]
fn fixture_files_match_rust_validator_expectations() {
    let root = project_root();
    for rel in [
        "examples/acts/minimal.act.json",
        "examples/candidates/ugly-candidate.json",
    ] {
        let text = std::fs::read_to_string(root.join(rel)).expect("read valid fixture");
        parse_act_json(&text).unwrap_or_else(|err| panic!("{rel} should validate: {err}"));
    }
    for rel in [
        "examples/invalid/missing-confirmed-by.json",
        "examples/invalid/extra-selected-branch.json",
        "examples/invalid/extra-runtime-envelope.json",
        "examples/invalid/extra-top-level-field.json",
        "examples/invalid/malformed.json",
    ] {
        let text = std::fs::read_to_string(root.join(rel)).expect("read invalid fixture");
        assert!(parse_act_json(&text).is_err(), "{rel} should be rejected");
    }
}
