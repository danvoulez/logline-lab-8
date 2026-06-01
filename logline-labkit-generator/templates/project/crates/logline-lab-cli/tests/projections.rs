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
    std::env::temp_dir().join(format!("logline-lab-projections-{name}-{nonce}"))
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
fn init_creates_projection_directory_and_index() {
    let home = temp_home("init");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    assert!(home.join(".logline-lab/projections").is_dir());
    let index = home.join(".logline-lab/projections/projection-index.json");
    assert!(index.is_file());
    let text = fs::read_to_string(index).expect("read projection index");
    assert!(text.contains("local_projection_index_not_official_spine"));
    assert!(text.contains("\"projections\": []"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_list_fails_before_init() {
    let home = temp_home("before-init");
    fs::create_dir_all(&home).expect("create empty temp dir");

    let output = run_lab_with_path(&["projection", "list", "--home"], &home, &[]);

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("Lab home not initialized"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_list_after_init_shows_available_local_summary_kind() {
    let home = temp_home("list");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(&["projection", "list", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("projections: 0"));
    assert!(stdout.contains("available projection kinds:"));
    assert!(stdout.contains("- local-summary"));
    assert!(stdout.contains("authority: local read models only"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_generate_local_summary_writes_markdown_with_zero_candidates() {
    let home = temp_home("generate-zero");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(
        &["projection", "generate", "local-summary", "--home"],
        &home,
        &[],
    );

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("local-summary projection generated"));
    assert!(stdout.contains(
        "authority: local read model only; not truth; not receipt; not evidence; not remote sync"
    ));
    let projection = home.join(".logline-lab/projections/local-summary.md");
    assert!(projection.is_file());
    let text = fs::read_to_string(projection).expect("read projection");
    assert!(text.contains("# Local Summary Projection"));
    assert!(text.contains("This projection is a local read model over workspace state."));
    assert!(text.contains("It can be regenerated."));
    assert!(text.contains("- Candidates: 0"));
    assert!(text.contains("- Index state: available"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_generate_local_summary_counts_candidate_after_add() {
    let home = temp_home("generate-one");
    let act = write_temp_act(&home, "valid.act.json");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["candidate", "add", "--home"],
        &home,
        &["--file", act.to_str().unwrap()],
    )
    .status
    .success());

    let output = run_lab_with_path(
        &["projection", "generate", "local-summary", "--home"],
        &home,
        &[],
    );

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = fs::read_to_string(home.join(".logline-lab/projections/local-summary.md"))
        .expect("read projection");
    assert!(text.contains("- Candidates: 1"));
    assert!(text.contains("- Candidates indexed: 1"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_list_after_generate_shows_path_and_available_state() {
    let home = temp_home("list-generated");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["projection", "generate", "local-summary", "--home"],
        &home,
        &[],
    )
    .status
    .success());

    let output = run_lab_with_path(&["projection", "list", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("projections: 1"));
    assert!(stdout.contains(
        "- local-summary path=.logline-lab/projections/local-summary.md state=available"
    ));
    assert!(
        stdout.contains("authority: local read models only; not truth; not receipt; not evidence")
    );
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_includes_projection_count_and_latest_path_after_generation() {
    let home = temp_home("status");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["projection", "generate", "local-summary", "--home"],
        &home,
        &[],
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
    assert!(stdout.contains("projections_available: 1"));
    assert!(stdout.contains("latest_projection: "));
    assert!(stdout.contains("local-summary.md"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_warns_on_malformed_projection_index_without_requiring_projection_file() {
    let home = temp_home("doctor-index");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    fs::write(
        home.join(".logline-lab/projections/projection-index.json"),
        "not-json",
    )
    .expect("break projection index");

    let output = run_lab_with_path(&["doctor", "--home"], &home, &[]);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("projection index: malformed"));
    assert!(stdout.contains("warnings:"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn unknown_projection_kind_fails() {
    let home = temp_home("unknown");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());

    let output = run_lab_with_path(
        &["projection", "generate", "something-else", "--home"],
        &home,
        &[],
    );

    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown projection: something-else"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn projection_flow_does_not_create_forbidden_named_paths() {
    let home = temp_home("names");
    assert!(run_lab_with_path(&["init", "--home"], &home, &[])
        .status
        .success());
    assert!(run_lab_with_path(
        &["projection", "generate", "local-summary", "--home"],
        &home,
        &[],
    )
    .status
    .success());

    for forbidden in ["ledger", "artifacts", "official", "truth"] {
        assert!(!home.join(forbidden).exists());
        assert!(!home.join(".logline-lab").join(forbidden).exists());
        assert!(!home
            .join(".logline-lab/projections")
            .join(forbidden)
            .exists());
    }
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_before_projection_keeps_latest_projection_optional() {
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
    assert!(stdout.contains("projections_available: 0"));
    assert!(stdout.contains("latest_projection: none"));
    let _ = fs::remove_dir_all(home);
}
