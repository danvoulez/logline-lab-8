use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    thread,
    time::Duration,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_home(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time available")
        .as_nanos();
    std::env::temp_dir().join(format!("logline-lab-setup-{name}-{nonce}"))
}

fn run_setup(home: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args([
            "setup",
            "--yes",
            "--home",
            home.to_str().expect("utf-8 temp path"),
            "--pack",
            "santo-andre",
            "--profile",
            "local-offline",
        ])
        .output()
        .expect("run setup")
}

#[test]
fn setup_creates_a_ready_lab_for_a_human_operator() {
    let home = temp_home("ready");
    let output = run_setup(&home);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("LogLine Lab is ready."));
    assert!(stdout.contains("Open your Lab:"));
    assert!(stdout.contains("not official spine"));
    assert!(stdout.contains("not receipt"));
    assert!(stdout.contains("not evidence"));
    assert!(stdout.contains("no LLM authority"));
    assert!(home.join(".logline-lab/lab.manifest.yaml").is_file());
    assert!(home.join(".logline-lab/reports/daily-state.md").is_file());
    assert!(home.join(".logline-lab/projections/local-summary.md").is_file());
    let candidate_dirs = fs::read_dir(home.join(".logline-lab/candidates"))
        .expect("read candidates")
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .count();
    assert_eq!(candidate_dirs, 1);
    let _ = fs::remove_dir_all(home);
}

#[test]
fn setup_help_explains_the_wizard() {
    let output = Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(["setup", "--help"])
        .output()
        .expect("run setup help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: logline-lab setup"));
    assert!(stdout.contains("Interactive first-run setup"));
}

#[test]
fn serve_help_explains_browser_product() {
    let output = Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(["serve", "--help"])
        .output()
        .expect("run serve help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: logline-lab serve"));
    assert!(stdout.contains("local browser product"));
}

#[test]
fn serve_exposes_local_browser_ui() {
    let port = 19000 + (unique_millis() % 1000) as u16;
    let mut child = Command::new(env!("CARGO_BIN_EXE_logline-lab"))
        .args(["serve", "--port", &port.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("start server");
    wait_for_server(port, &mut child);
    let response = http_get(port, "/");
    assert!(response.contains("LogLine Lab"));
    assert!(response.contains("Create first Lab"));
    assert!(response.contains("Run cycle"));
    assert!(response.contains("Load"));
    assert!(response.contains("Declare"));
    assert!(response.contains("Observe"));
    assert!(response.contains("Emit"));
    assert!(response.contains("Project"));
    assert!(response.contains("Learn"));
    assert!(response.contains("Not official spine") || response.contains("not official spine"));
    let _ = child.kill();
    let _ = child.wait();
}

fn unique_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time available")
        .as_millis()
}

fn wait_for_server(port: u16, child: &mut Child) {
    for _ in 0..40 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        if let Ok(Some(status)) = child.try_wait() {
            panic!("server exited before accepting connections: {status}");
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!("server did not accept connections on port {port}");
}

fn http_get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
    )
    .expect("write request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    response
}
