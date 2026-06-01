use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn write_temp_act(name: &str, content: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time available")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("logline-lab-{name}-{nonce}.json"));
    fs::write(&path, content).expect("write temp act");
    path
}

#[test]
fn validate_file_accepts_valid_json_act() {
    let path = write_temp_act(
        "valid",
        r#"{"who":"dan","did":"record_decision","this":{},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate"}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(["act", "validate", "--file"])
        .arg(&path)
        .output()
        .expect("run cli");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("valid LogLine Act"));
    assert!(stdout.contains("slots: 9/9"));
    assert!(stdout.contains("status: candidate"));

    let _ = fs::remove_file(path);
}

#[test]
fn emit_file_rejects_invalid_json_act_before_preview() {
    let path = write_temp_act(
        "invalid",
        r#"{"who":"dan","did":"record_decision","this":{},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate","selected_branch":"ok"}"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(["act", "emit", "--file"])
        .arg(&path)
        .output()
        .expect("run cli");

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("selected_branch is not a LogLine Act slot"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("emit preview only"));

    let _ = fs::remove_file(path);
}
