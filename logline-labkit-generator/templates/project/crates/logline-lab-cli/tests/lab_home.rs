use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_home(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time available")
        .as_nanos();
    std::env::temp_dir().join(format!("logline-lab-home-{name}-{nonce}"))
}

fn run_lab(args: &[&str], home: &Path) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_logline-lab"));
    command
        .args(args)
        .arg("--home")
        .arg(home)
        .output()
        .expect("run cli")
}

#[test]
fn init_home_creates_required_local_files() {
    let home = temp_home("init");
    let output = run_lab(&["init"], &home);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(home.join(".logline-lab").is_dir());
    assert!(home.join(".logline-lab/lab.manifest.yaml").is_file());
    assert!(home.join(".logline-lab/GHOSTS.md").is_file());
    assert!(home.join(".logline-lab/STATUS.md").is_file());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("initialized local LogLine Lab home"));
    assert!(stdout.contains("authority: local workspace only; not official spine; not receipt"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_passes_after_init() {
    let home = temp_home("doctor-ok");
    assert!(run_lab(&["init"], &home).status.success());
    let output = run_lab(&["doctor"], &home);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("doctor: ok"));
    assert!(stdout.contains("remote spine: ghost remote-spine-unconfigured"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_fails_before_init() {
    let home = temp_home("doctor-fail");
    fs::create_dir_all(&home).expect("create empty temp home");
    let output = run_lab(&["doctor"], &home);
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("doctor: failed"));
    assert!(stderr.contains("missing directory: .logline-lab/"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_reports_remote_spine_as_ghost_after_init() {
    let home = temp_home("status");
    assert!(run_lab(&["init"], &home).status.success());
    let output = run_lab(&["status"], &home);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("remote spine status: ghost/unconfigured"));
    assert!(stdout.contains("receipt status: unavailable/unimplemented"));
    assert!(stdout.contains("llm-translator-unimplemented"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn init_is_idempotent_and_preserves_manifest() {
    let home = temp_home("idempotent");
    assert!(run_lab(&["init"], &home).status.success());
    let manifest = home.join(".logline-lab/lab.manifest.yaml");
    fs::write(&manifest, "manifest_version: 1\nlab:\n  id: kept\n").expect("edit manifest");
    assert!(run_lab(&["init"], &home).status.success());
    let manifest_text = fs::read_to_string(&manifest).expect("read manifest");
    assert!(manifest_text.contains("id: kept"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn init_does_not_create_forbidden_named_paths() {
    let home = temp_home("names");
    assert!(run_lab(&["init"], &home).status.success());
    let forbidden_names = ["ledger", "artifacts"];
    for name in forbidden_names {
        assert!(!home.join(name).exists());
        assert!(!home.join(".logline-lab").join(name).exists());
    }
    let _ = fs::remove_dir_all(home);
}
