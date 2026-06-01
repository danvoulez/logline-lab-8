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
    std::env::temp_dir().join(format!("logline-lab-pack-profile-{name}-{nonce}"))
}

fn run_lab(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(args)
        .output()
        .expect("run cli")
}

fn init(home: &Path, pack: &str, profile: &str) -> Output {
    run_lab(&[
        "init",
        "--home",
        home.to_str().unwrap(),
        "--pack",
        pack,
        "--profile",
        profile,
    ])
}

#[test]
fn init_with_santo_andre_local_offline_writes_manifest_selection() {
    let home = temp_home("santo-local");
    let output = init(&home, "santo-andre", "local-offline");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pack: santo-andre"));
    assert!(stdout.contains("profile: local-offline"));
    let manifest =
        fs::read_to_string(home.join(".logline-lab/lab.manifest.yaml")).expect("read manifest");
    assert!(manifest.contains("selected:"));
    assert!(manifest.contains("  pack: santo-andre"));
    assert!(manifest.contains("  profile: local-offline"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn status_after_init_shows_pack_and_profile() {
    let home = temp_home("status");
    assert!(init(&home, "santo-andre", "local-offline").status.success());
    let output = run_lab(&["status", "--home", home.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pack: santo-andre"));
    assert!(stdout.contains("profile: local-offline"));
    assert!(stdout.contains("remote_spine: unavailable/ghost"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_after_init_reports_known_pack_and_profile() {
    let home = temp_home("doctor");
    assert!(init(&home, "santo-andre", "local-offline").status.success());
    let output = run_lab(&["doctor", "--home", home.to_str().unwrap()]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("doctor: ok"));
    assert!(stdout.contains("pack: santo-andre known"));
    assert!(stdout.contains("profile: local-offline known"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn init_with_personal_offline_local_offline_works() {
    let home = temp_home("personal");
    let output = init(&home, "personal-offline", "local-offline");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pack: personal-offline"));
    let ghosts = fs::read_to_string(home.join(".logline-lab/GHOSTS.md")).expect("read ghosts");
    assert!(ghosts.contains("passkey-checkpointing-unimplemented"));
    assert!(ghosts.contains("personal-adapters-unimplemented"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn init_with_unknown_pack_fails_without_manifest() {
    let home = temp_home("unknown-pack");
    let output = init(&home, "missing-pack", "local-offline");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown pack: missing-pack"));
    assert!(!home.join(".logline-lab/lab.manifest.yaml").exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn init_with_unknown_profile_fails_without_manifest() {
    let home = temp_home("unknown-profile");
    let output = init(&home, "santo-andre", "missing-profile");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown profile: missing-profile"));
    assert!(!home.join(".logline-lab/lab.manifest.yaml").exists());
    let _ = fs::remove_dir_all(home);
}

#[test]
fn supabase_profile_initializes_but_reports_unconfigured_ghosts() {
    let home = temp_home("supabase");
    let output = init(&home, "santo-andre", "supabase");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let doctor = run_lab(&["doctor", "--home", home.to_str().unwrap()]);
    assert!(
        doctor.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&doctor.stderr)
    );
    let stdout = String::from_utf8_lossy(&doctor.stdout);
    assert!(stdout.contains("profile: supabase known"));
    assert!(stdout.contains("remote spine: declared_not_implemented/unconfigured"));
    assert!(stdout.contains("supabase-ingest-unimplemented"));
    assert!(stdout.contains("supabase-env-unverified"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn daily_state_report_includes_pack_profile_and_capabilities() {
    let home = temp_home("daily");
    assert!(init(&home, "santo-andre", "local-offline").status.success());
    let output = run_lab(&[
        "report",
        "generate",
        "daily-state",
        "--home",
        home.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = fs::read_to_string(home.join(".logline-lab/reports/daily-state.md"))
        .expect("read daily-state");
    assert!(report.contains("## Pack/Profile"));
    assert!(report.contains("- Pack: santo-andre"));
    assert!(report.contains("- Profile: local-offline"));
    assert!(report.contains("## Profile Capability State"));
    assert!(report.contains("- remote_spine: unavailable/ghost"));
    let _ = fs::remove_dir_all(home);
}

#[test]
fn created_paths_do_not_use_forbidden_names() {
    let home = temp_home("names");
    assert!(init(&home, "personal-offline", "local-offline")
        .status
        .success());
    for forbidden in ["ledger", "artifacts", "official", "truth"] {
        assert!(!home.join(forbidden).exists());
        assert!(!home.join(".logline-lab").join(forbidden).exists());
    }
    let _ = fs::remove_dir_all(home);
}

#[test]
fn doctor_fails_unknown_pack_in_manifest() {
    let home = temp_home("manifest-unknown-pack");
    assert!(init(&home, "santo-andre", "local-offline").status.success());
    let manifest_path = home.join(".logline-lab/lab.manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read manifest");
    fs::write(
        &manifest_path,
        manifest.replace("pack: santo-andre", "pack: mystery-pack"),
    )
    .expect("write manifest");
    let output = run_lab(&["doctor", "--home", home.to_str().unwrap()]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("doctor: failed"));
    assert!(stderr.contains("unknown pack: mystery-pack"));
    let _ = fs::remove_dir_all(home);
}
