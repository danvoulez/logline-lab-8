use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_home(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time available")
        .as_nanos();
    std::env::temp_dir().join(format!("logline-lab-candidate-index-{name}-{nonce}"))
}

fn write_temp_act(home: &Path, name: &str, content: &str) -> PathBuf {
    fs::create_dir_all(home).expect("create temp dir");
    let path = home.join(name);
    fs::write(&path, content).expect("write act");
    path
}

fn run_lab(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(args)
        .output()
        .expect("run cli")
}

fn run_lab_with_path(args: &[&str], path: &Path, tail: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(args)
        .arg(path)
        .args(tail)
        .output()
        .expect("run cli")
}

fn valid_act() -> &'static str {
    r#"{"who":"dan","did":"capture","this":{"note":"candidate index"},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate"}"#
}

fn add_candidate(home: &Path) -> String {
    let act = write_temp_act(home, "valid.act.json", valid_act());
    let output = run_lab_with_path(
        &["candidate", "add", "--home"],
        home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("candidate id")
        .to_string()
}

#[test]
fn init_creates_empty_candidate_index() {
    let home = temp_home("init");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let index_path = home.join(".logline-lab/candidates/index.json");
    assert!(index_path.is_file());
    let index = fs::read_to_string(index_path).expect("read index");
    assert!(index.contains("\"index_version\": 1"));
    assert!(index.contains("local_candidate_queue_index_not_official_spine"));
    assert!(index.contains("\"candidates\": ["));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_add_updates_index_entry() {
    let home = temp_home("add");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    let index =
        fs::read_to_string(home.join(".logline-lab/candidates/index.json")).expect("read index");
    assert!(index.contains(&format!("\"candidate_id\": \"{id}\"")));
    assert!(index.contains(&format!("\"path\": \"{id}/candidate.json\"")));
    assert!(index.contains(&format!("\"metadata_path\": \"{id}/metadata.json\"")));
    assert_eq!(index.matches(&id).count(), 3);
    let _ = fs::remove_dir_all(home);
}

#[test]
fn invalid_candidate_does_not_update_index() {
    let home = temp_home("invalid");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let before = fs::read_to_string(home.join(".logline-lab/candidates/index.json"))
        .expect("read index before");
    let invalid = r#"{"who":"dan","did":"capture","this":{},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate","selected_branch":"no"}"#;
    let act = write_temp_act(&home, "invalid.act.json", invalid);
    let output = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("validation failed"));
    assert!(stderr.contains("index: unchanged"));
    let after = fs::read_to_string(home.join(".logline-lab/candidates/index.json"))
        .expect("read index after");
    assert_eq!(before, after);
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_list_reads_available_index() {
    let home = temp_home("list");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    let output = run_lab_with_path(&["candidate", "list", "--home"], &home, &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candidates: 1"));
    assert!(stdout.contains("index: available"));
    assert!(stdout.contains(&id));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_reports_candidate_count_and_index_state() {
    let home = temp_home("status");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    add_candidate(&home);
    let output = run_lab_with_path(&["status", "--home"], &home, &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candidate_count: 1"));
    assert!(stdout.contains("candidate_index: available"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn daily_state_report_includes_candidate_index_section() {
    let home = temp_home("report");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    let output = run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[]);
    assert!(output.status.success());
    let report =
        fs::read_to_string(home.join(".logline-lab/reports/daily-state.md")).expect("read report");
    assert!(report.contains("## Candidate Index"));
    assert!(report.contains("- State: available"));
    assert!(report.contains("- Candidates indexed: 1"));
    assert!(report.contains("local operational index only; not official spine"));
    assert!(report.contains(&id));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn malformed_index_causes_doctor_failure() {
    let home = temp_home("malformed");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    fs::write(home.join(".logline-lab/candidates/index.json"), "not json")
        .expect("write malformed index");
    let output = run_lab_with_path(&["doctor", "--home"], &home, &[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("doctor: failed"));
    assert!(stderr.contains("candidate index: malformed"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn missing_index_is_handled_explicitly_and_rebuilds_on_list() {
    let home = temp_home("missing");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    fs::remove_file(home.join(".logline-lab/candidates/index.json")).expect("remove index");

    let doctor = run_lab_with_path(&["doctor", "--home"], &home, &[]);
    assert!(doctor.status.success());
    let doctor_stdout = String::from_utf8_lossy(&doctor.stdout);
    assert!(doctor_stdout.contains("candidate index: missing"));
    assert!(doctor_stdout.contains("candidate index missing"));

    let list = run_lab_with_path(&["candidate", "list", "--home"], &home, &[]);
    assert!(list.status.success());
    let stdout = String::from_utf8_lossy(&list.stdout);
    assert!(stdout.contains("index: rebuilt"));
    assert!(stdout.contains(&id));
    assert!(home.join(".logline-lab/candidates/index.json").is_file());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_reports_index_entries_point_to_existing_files() {
    let home = temp_home("missing-file");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    fs::remove_file(
        home.join(".logline-lab/candidates")
            .join(&id)
            .join("candidate.json"),
    )
    .expect("remove candidate file");
    let output = run_lab_with_path(&["doctor", "--home"], &home, &[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("candidate index entry missing candidate file"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_index_rebuild_command_repairs_missing_index() {
    let home = temp_home("rebuild");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let id = add_candidate(&home);
    fs::remove_file(home.join(".logline-lab/candidates/index.json")).expect("remove index");
    let output = run_lab(&[
        "candidate",
        "index",
        "rebuild",
        "--home",
        home.to_str().unwrap(),
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candidate index rebuilt"));
    assert!(stdout.contains("index: rebuilt"));
    let index = fs::read_to_string(home.join(".logline-lab/candidates/index.json"))
        .expect("read rebuilt index");
    assert!(index.contains(&id));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_index_flow_does_not_create_forbidden_named_paths() {
    let home = temp_home("names");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    add_candidate(&home);
    for forbidden in ["ledger", "artifacts", "official", "truth"] {
        assert!(!home.join(forbidden).exists());
        assert!(!home.join(".logline-lab").join(forbidden).exists());
    }
    let _ = fs::remove_dir_all(home);
}
