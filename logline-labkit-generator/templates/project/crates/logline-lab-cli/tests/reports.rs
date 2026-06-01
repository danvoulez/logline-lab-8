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
    std::env::temp_dir().join(format!("logline-lab-reports-{name}-{nonce}"))
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

fn write_temp_act(home: &Path, name: &str) -> PathBuf {
    fs::create_dir_all(home).expect("create temp dir");
    let path = home.join(name);
    fs::write(&path, valid_act()).expect("write act");
    path
}

fn valid_act() -> &'static str {
    r#"{"who":"dan","did":"capture","this":{"note":"candidate content"},"when":"2026-06-01T00:00:00Z","confirmed_by":{},"if_ok":{},"if_doubt":{},"if_not":{},"status":"candidate"}"#
}

#[test]
fn ghost_list_before_init_fails() {
    let home = temp_home("ghost-before-init");
    fs::create_dir_all(&home).expect("create empty temp dir");

    let output = run_lab_with_path(&["ghost", "list", "--home"], &home, &[]);

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("Lab home not initialized"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn ghost_list_after_init_lists_expected_ghosts() {
    let home = temp_home("ghost-list");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(&["ghost", "list", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ghosts: 6"));
    assert!(stdout.contains("- remote-spine-unconfigured"));
    assert!(stdout.contains("- evidence-registry-unimplemented"));
    assert!(stdout.contains("- receipt-closure-unimplemented"));
    assert!(stdout.contains("- interactive-lab-surface-unimplemented"));
    assert!(stdout.contains("- llm-translator-unimplemented"));
    assert!(stdout.contains("- yaml-act-parser-unimplemented"));
    assert!(stdout.contains("authority: local workspace Ghost list only"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn daily_state_report_before_init_fails() {
    let home = temp_home("report-before-init");
    fs::create_dir_all(&home).expect("create empty temp dir");

    let output = run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[]);

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("Lab home not initialized"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn daily_state_report_writes_markdown_with_authority_and_zero_candidates() {
    let home = temp_home("report-zero");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("daily-state report generated"));
    assert!(stdout.contains(
        "authority: local workspace projection only; not receipt; not evidence; not remote sync"
    ));
    let path = home.join(".logline-lab/reports/daily-state.md");
    assert!(path.is_file());
    let report = fs::read_to_string(path).expect("read report");
    assert!(report.contains("# Daily Lab State"));
    assert!(report.contains("This report is a local workspace projection."));
    assert!(report
        .contains("It is not official spine, not evidence, not a receipt, and not remote sync."));
    assert!(report.contains("- Candidates: 0"));
    assert!(report.contains("- Candidate index: available"));
    assert!(report.contains("## Candidate Index"));
    assert!(report.contains("- State: available"));
    assert!(report.contains("- Ghosts: 6"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn daily_state_report_counts_candidate_after_add() {
    let home = temp_home("report-one");
    let act = write_temp_act(&home, "valid.act.json");
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

    let output = run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report =
        fs::read_to_string(home.join(".logline-lab/reports/daily-state.md")).expect("read report");
    assert!(report.contains("- Candidates: 1"));
    assert!(report.contains("- Candidate index: available"));
    assert!(report.contains("- Candidates indexed: 1"));
    assert!(report.contains("- cand_"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_includes_report_count_and_latest_path_after_generation() {
    let home = temp_home("status-report");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(
        run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[],)
            .status
            .success()
    );

    let output = run_lab_with_path(&["status", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("reports_available: 1"));
    assert!(stdout.contains("latest_report: "));
    assert!(stdout.contains("daily-state.md"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn local_report_flow_does_not_create_forbidden_named_paths() {
    let home = temp_home("names");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(
        run_lab_with_path(&["report", "generate", "daily-state", "--home"], &home, &[],)
            .status
            .success()
    );

    for forbidden in ["ledger", "artifacts", "official", "truth"] {
        assert!(!home.join(forbidden).exists());
        assert!(!home.join(".logline-lab").join(forbidden).exists());
    }
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_before_report_keeps_latest_report_optional() {
    let home = temp_home("status-none");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab(&["status", "--home", home.to_str().unwrap()]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("reports_available: 0"));
    assert!(stdout.contains("latest_report: none"));
    let _ = fs::remove_dir_all(home);
}
