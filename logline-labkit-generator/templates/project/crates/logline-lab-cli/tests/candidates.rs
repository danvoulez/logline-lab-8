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
    std::env::temp_dir().join(format!("logline-lab-candidate-{name}-{nonce}"))
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
    r#"{"who":"dan","did":"capture","this":{"note":"candidate content"},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate"}"#
}

#[test]
fn candidate_add_after_init_creates_candidate_files() {
    let home = temp_home("add");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candidate captured"));
    assert!(stdout.contains("index: available"));
    assert!(stdout.contains(
        "authority: local capture only; not official spine; not receipt; not remote synced"
    ));
    let id = stdout
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("id line");
    assert!(home
        .join(".logline-lab/candidates")
        .join(id)
        .join("candidate.json")
        .is_file());
    assert!(home
        .join(".logline-lab/candidates")
        .join(id)
        .join("metadata.json")
        .is_file());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_add_before_init_fails() {
    let home = temp_home("before-init");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    let output = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("lab home is not initialized"));
    assert!(!home.join(".logline-lab/candidates").exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_add_invalid_selected_branch_fails_without_candidate() {
    let home = temp_home("invalid");
    let invalid = r#"{"who":"dan","did":"capture","this":{},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate","selected_branch":"no"}"#;
    let act = write_temp_act(&home, "invalid.act.json", invalid);
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("selected_branch is not a LogLine Act slot"));
    let entries = fs::read_dir(home.join(".logline-lab/candidates"))
        .expect("read candidates")
        .count();
    assert_eq!(entries, 2, "only .keep and index.json should exist");
    let index_text =
        fs::read_to_string(home.join(".logline-lab/candidates/index.json")).expect("read index");
    assert!(index_text.contains("\"candidates\": ["));
    assert!(!index_text.contains("cand_"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_list_after_one_add_shows_count_and_id() {
    let home = temp_home("list");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let add = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(add.status.success());
    let add_stdout = String::from_utf8_lossy(&add.stdout);
    let id = add_stdout
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("id line");

    let list = run_lab_with_path(&["candidate", "list", "--home"], &home, &[]);
    assert!(
        list.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&list.stderr)
    );
    let stdout = String::from_utf8_lossy(&list.stdout);
    assert!(stdout.contains("candidates: 1"));
    assert!(stdout.contains("index: available"));
    assert!(stdout.contains(id));
    assert!(stdout.contains("candidate captured_at="));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_get_returns_metadata_and_captured_content() {
    let home = temp_home("get");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let add = run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    );
    assert!(add.status.success());
    let add_stdout = String::from_utf8_lossy(&add.stdout);
    let id = add_stdout
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .expect("id line");

    let get = run_lab(&["candidate", "get", id, "--home", home.to_str().unwrap()]);
    assert!(
        get.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&get.stderr)
    );
    let stdout = String::from_utf8_lossy(&get.stdout);
    assert!(stdout.contains("metadata:"));
    assert!(stdout.contains("candidate:"));
    assert!(stdout.contains("candidate content"));
    assert!(stdout.contains("local workspace record only; not official spine; not receipt"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_get_missing_id_returns_non_zero() {
    let home = temp_home("missing");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    let output = run_lab(&[
        "candidate",
        "get",
        "cand_missing",
        "--home",
        home.to_str().unwrap(),
    ]);
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("candidate not found: cand_missing"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_includes_candidate_count() {
    let home = temp_home("status");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()]
    )
    .status
    .success());

    let output = run_lab_with_path(&["status", "--home"], &home, &[]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("candidate_count: 1"));
    assert!(stdout.contains("candidate_index: available"));
    assert!(stdout.contains("local_candidate_queue: available"));
    assert!(stdout.contains("authority: local workspace only; not official spine"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn candidate_flow_does_not_create_forbidden_named_paths() {
    let home = temp_home("names");
    let act = write_temp_act(&home, "valid.act.json", valid_act());
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()]
    )
    .status
    .success());

    for forbidden in ["ledger", "artifacts", "official", "truth"] {
        assert!(!home.join(forbidden).exists());
        assert!(!home.join(".logline-lab").join(forbidden).exists());
    }
    let _ = fs::remove_dir_all(home);
}
