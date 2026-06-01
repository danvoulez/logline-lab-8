use std::process::Command;

fn run_lab(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(args)
        .output()
        .expect("run cli")
}

#[test]
fn version_contains_binary_name_and_version() {
    let output = run_lab(&["--version"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("logline-lab"));
    assert!(stdout.contains("0.1.0-alpha.0"));
}

#[test]
fn help_mentions_core_commands_and_boundaries() {
    let output = run_lab(&["--help"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for expected in [
        "init --home",
        "doctor --home",
        "status --home",
        "act validate --file",
        "act emit --file",
        "candidate add",
        "candidate list",
        "candidate get",
        "ghost list",
        "report generate daily-state",
        "lab",
        "chat",
        "local-offline works without Supabase",
        "not an official spine",
    ] {
        assert!(
            stdout.contains(expected),
            "missing {expected} in help:\n{stdout}"
        );
    }
}

#[test]
fn command_help_mentions_authority_boundary() {
    let output = run_lab(&["candidate", "add", "--help"]);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: logline-lab candidate add"));
    assert!(stdout.contains("local Candidate only"));
}
