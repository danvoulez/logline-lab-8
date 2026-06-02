use std::{
    env, fs,
    io::{self, Write},
    net::SocketAddr,
    path::PathBuf,
    process,
};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

const VERSION: &str = include_str!("../../../VERSION");

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let code = dispatch(&args);
    process::exit(code);
}

fn dispatch(args: &[String]) -> i32 {
    match args {
        [] => {
            print_help();
            0
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("logline-lab {}", VERSION.trim());
            0
        }
        [flag] if flag == "--help" || flag == "-h" => {
            print_help();
            0
        }
        [cmd, flag] if flag == "--help" || flag == "-h" => {
            print_command_help(cmd);
            0
        }
        [cmd, rest @ ..] if cmd == "setup" => dispatch_setup(rest),
        [cmd, rest @ ..] if cmd == "serve" => dispatch_serve(rest),
        [scope, rest @ ..] if scope == "supabase" => dispatch_supabase(rest),
        [scope, action, flag] if flag == "--help" || flag == "-h" => {
            print_nested_command_help(scope, action);
            0
        }
        [cmd, rest @ ..] if cmd == "init" => match init_args(rest) {
            Ok((home, pack_id, profile_id)) => {
                match logline_lab_core::init_lab_home_with_selection(&home, &pack_id, &profile_id) {
                    Ok(report) => {
                        println!("{}", report.to_text());
                        0
                    }
                    Err(err) => {
                        eprintln!("init: failed");
                        eprintln!("home: {}", home.display());
                        eprintln!("error: {err}");
                        1
                    }
                }
            }
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [cmd, rest @ ..] if cmd == "doctor" => match home_from_args(rest) {
            Ok(home) => {
                let report = logline_lab_core::doctor_report_for(&home);
                if report.is_ok() {
                    println!("{}", report.to_text());
                    0
                } else {
                    eprintln!("{}", report.to_text());
                    1
                }
            }
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [cmd, rest @ ..] if cmd == "status" => match home_from_args(rest) {
            Ok(home) => {
                println!("{}", logline_lab_core::status_report_for(&home).to_text());
                0
            }
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [scope, rest @ ..] if scope == "candidate" => dispatch_candidate(rest),
        [scope, rest @ ..] if scope == "ghost" => dispatch_ghost(rest),
        [scope, rest @ ..] if scope == "projection" => dispatch_projection(rest),
        [scope, rest @ ..] if scope == "report" => dispatch_report(rest),
        [cmd] if cmd == "lab" => {
            eprintln!("Ghost: interactive-lab-surface-unimplemented");
            eprintln!("authority: no interactive surface is implemented by this CLI-first kit");
            2
        }
        [cmd] if cmd == "chat" => {
            eprintln!("Ghost: llm-translator-unimplemented");
            eprintln!("authority: no LLM provider is configured or authoritative");
            2
        }
        [scope, action] if scope == "act" && action == "validate" => {
            println!("usage: logline-lab act validate --file <path>");
            println!(
                "authority: validates local JSON Act shape only; no remote write and no receipt"
            );
            0
        }
        [scope, action, flag, path]
            if scope == "act" && action == "validate" && flag == "--file" =>
        {
            match fs::read_to_string(path) {
                Ok(input) => match logline_lab_core::validate_text_result(&input) {
                    Ok(message) => {
                        println!("{message}");
                        0
                    }
                    Err(message) => {
                        eprintln!("{message}");
                        1
                    }
                },
                Err(err) => {
                    eprintln!("ghost: unable-to-read-act-file: {err}");
                    1
                }
            }
        }
        [scope, action, rest @ ..] if scope == "act" && action == "emit" => {
            dispatch_act_emit(rest)
        }
        _ => {
            eprintln!("ghost: unknown-command");
            print_help();
            1
        }
    }
}

fn dispatch_supabase(args: &[String]) -> i32 {
    match args {
        [action] if action == "check" => match logline_lab_core::supabase_check_result() {
            Ok(message) => {
                println!("{message}");
                0
            }
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [action, flag] if action == "check" && (flag == "--help" || flag == "-h") => {
            print_nested_command_help("supabase", "check");
            0
        }
        _ => {
            eprintln!("usage: logline-lab supabase check");
            1
        }
    }
}

fn dispatch_act_emit(args: &[String]) -> i32 {
    match act_emit_args(args) {
        Ok((file, remote)) => match fs::read_to_string(&file) {
            Ok(input) => {
                let result = if remote {
                    logline_lab_core::emit_remote_result(&input)
                } else {
                    logline_lab_core::emit_preview_result(&input)
                };
                match result {
                    Ok(message) => {
                        println!("{message}");
                        0
                    }
                    Err(message) => {
                        eprintln!("{message}");
                        1
                    }
                }
            }
            Err(err) => {
                eprintln!("ghost: unable-to-read-act-file: {err}");
                1
            }
        },
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn dispatch_serve(args: &[String]) -> i32 {
    match serve_args(args) {
        Ok((host, port)) => {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(err) => {
                    eprintln!("serve: failed to start runtime: {err}");
                    return 1;
                }
            };
            runtime.block_on(async move {
                match serve_http(&host, port).await {
                    Ok(()) => 0,
                    Err(err) => {
                        eprintln!("{err}");
                        1
                    }
                }
            })
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn dispatch_setup(args: &[String]) -> i32 {
    match setup_args(args) {
        Ok(mut options) => {
            if !options.yes {
                println!("LogLine Lab setup");
                println!("This creates a local Lab workspace. It is not a remote spine, receipt, evidence proof, or LLM runtime.");
                match prompt_default("Lab home", &options.home.display().to_string()) {
                    Ok(value) => options.home = PathBuf::from(value),
                    Err(err) => {
                        eprintln!("setup: failed to read Lab home: {err}");
                        return 1;
                    }
                }
                match prompt_default("Pack", &options.pack_id) {
                    Ok(value) => options.pack_id = value,
                    Err(err) => {
                        eprintln!("setup: failed to read pack: {err}");
                        return 1;
                    }
                }
                match prompt_default("Profile", &options.profile_id) {
                    Ok(value) => options.profile_id = value,
                    Err(err) => {
                        eprintln!("setup: failed to read profile: {err}");
                        return 1;
                    }
                }
                println!();
            }
            match run_setup(&options.home, &options.pack_id, &options.profile_id) {
                Ok(output) => {
                    println!("{output}");
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            }
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

async fn serve_http(host: &str, port: u16) -> Result<(), String> {
    let address = format!("{host}:{port}")
        .parse::<SocketAddr>()
        .map_err(|err| format!("serve: invalid address {host}:{port}: {err}"))?;
    let app = Router::new()
        .route("/", get(app_home))
        .route("/api/setup", post(api_setup))
        .route("/api/status", get(api_status));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|err| format!("serve: unable to bind http listener: {err}"))?;
    println!("LogLine Lab running at http://{address}");
    println!("Authority boundary: local workspace only; no remote spine, receipt, evidence proof, sync, or LLM authority.");
    axum::serve(listener, app)
        .await
        .map_err(|err| format!("serve: http server failed: {err}"))
}

async fn app_home() -> Html<&'static str> {
    Html(APP_HTML)
}

async fn api_setup(Json(input): Json<SetupRequest>) -> impl IntoResponse {
    let home = PathBuf::from(default_if_empty(input.home, "./demo-lab"));
    let pack = default_if_empty(
        input.pack,
        logline_lab_core::catalog::DEFAULT_PACK_ID,
    );
    let profile = default_if_empty(
        input.profile,
        logline_lab_core::catalog::DEFAULT_PROFILE_ID,
    );
    match run_setup(&home, &pack, &profile) {
        Ok(output) => (StatusCode::OK, output).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err).into_response(),
    }
}

async fn api_status(Query(query): Query<HomeQuery>) -> impl IntoResponse {
    let home = PathBuf::from(default_if_empty(query.home, "./demo-lab"));
    let report = logline_lab_core::status_report_for(&home).to_text();
    (StatusCode::OK, report)
}

#[derive(Debug, Deserialize)]
struct SetupRequest {
    home: Option<String>,
    pack: Option<String>,
    profile: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HomeQuery {
    home: Option<String>,
}

fn default_if_empty(value: Option<String>, default: &str) -> String {
    match value {
        Some(value) if !value.trim().is_empty() => value,
        _ => default.to_string(),
    }
}

fn serve_args(args: &[String]) -> Result<(String, u16), String> {
    let mut host = "127.0.0.1".to_string();
    let mut port = 8787;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--host" => {
                index += 1;
                host = args
                    .get(index)
                    .ok_or_else(|| "missing value for --host".to_string())?
                    .to_string();
            }
            "--port" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --port".to_string())?;
                port = value
                    .parse::<u16>()
                    .map_err(|err| format!("invalid --port value: {err}"))?;
            }
            value => return Err(format!("unexpected serve argument: {value}")),
        }
        index += 1;
    }
    Ok((host, port))
}

fn dispatch_candidate(args: &[String]) -> i32 {
    match args {
        [action, rest @ ..] if action == "add" => match candidate_add_args(rest) {
            Ok((home, file)) => match logline_lab_core::capture_candidate(&home, &file) {
                Ok(report) => {
                    println!("{}", report.to_text());
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            },
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [action, rest @ ..] if action == "list" => match home_from_args(rest) {
            Ok(home) => match logline_lab_core::list_candidates(&home) {
                Ok(list) => {
                    println!("{}", list.to_text());
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            },
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [action, subaction, rest @ ..] if action == "index" && subaction == "rebuild" => {
            match home_from_args(rest) {
                Ok(home) => match logline_lab_core::LabHome::new(&home).rebuild_candidate_index() {
                    Ok(list) => {
                        println!("candidate index rebuilt");
                        println!("home: {}", home.display());
                        println!("candidates: {}", list.records.len());
                        println!("index: {}", list.index_status.as_cli_status());
                        println!("authority: local capture queue index only; not official spine; not receipt; not remote synced");
                        0
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        1
                    }
                },
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            }
        }
        [action, candidate_id, rest @ ..] if action == "get" => match home_from_args(rest) {
            Ok(home) => match logline_lab_core::get_candidate(&home, candidate_id) {
                Ok(record) => {
                    println!("{}", record.to_text());
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            },
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        _ => {
            eprintln!("usage: logline-lab candidate <add --file <path>|list|get <candidate_id>|index rebuild> [--home <path>]");
            1
        }
    }
}

fn dispatch_ghost(args: &[String]) -> i32 {
    match args {
        [action, rest @ ..] if action == "list" => match home_from_args(rest) {
            Ok(home) => match logline_lab_core::list_ghosts(&home) {
                Ok(list) => {
                    println!("{}", list.to_text());
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            },
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        _ => {
            eprintln!("usage: logline-lab ghost list [--home <path>]");
            1
        }
    }
}

fn dispatch_projection(args: &[String]) -> i32 {
    match args {
        [action, rest @ ..] if action == "list" => match home_from_args(rest) {
            Ok(home) => match logline_lab_core::list_projections(&home) {
                Ok(list) => {
                    println!("{}", list.to_text());
                    0
                }
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            },
            Err(message) => {
                eprintln!("{message}");
                1
            }
        },
        [action, kind, rest @ ..] if action == "generate" => {
            let projection_kind = match logline_lab_core::projections::ProjectionKind::parse(kind) {
                Ok(kind) => kind,
                Err(err) => {
                    eprintln!("{err}");
                    return 1;
                }
            };
            match home_from_args(rest) {
                Ok(home) => match logline_lab_core::generate_projection(&home, projection_kind) {
                    Ok(projection) => {
                        println!("{}", projection.to_text());
                        0
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        1
                    }
                },
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            }
        }
        [action] if action == "generate" => {
            eprintln!("usage: logline-lab projection generate local-summary [--home <path>]");
            1
        }
        _ => {
            eprintln!(
                "usage: logline-lab projection <list|generate local-summary> [--home <path>]"
            );
            1
        }
    }
}

fn dispatch_report(args: &[String]) -> i32 {
    match args {
        [action, kind, rest @ ..] if action == "generate" && kind == "daily-state" => {
            match home_from_args(rest) {
                Ok(home) => match logline_lab_core::generate_daily_state_report(&home) {
                    Ok(report) => {
                        println!("{}", report.to_text());
                        0
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        1
                    }
                },
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            }
        }
        _ => {
            eprintln!("usage: logline-lab report generate daily-state [--home <path>]");
            1
        }
    }
}

#[derive(Debug)]
struct SetupOptions {
    home: PathBuf,
    pack_id: String,
    profile_id: String,
    yes: bool,
}

fn setup_args(args: &[String]) -> Result<SetupOptions, String> {
    let mut options = SetupOptions {
        home: PathBuf::from("./demo-lab"),
        pack_id: logline_lab_core::catalog::DEFAULT_PACK_ID.to_string(),
        profile_id: logline_lab_core::catalog::DEFAULT_PROFILE_ID.to_string(),
        yes: false,
    };
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--home" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --home".to_string())?;
                options.home = PathBuf::from(value);
            }
            "--pack" => {
                index += 1;
                options.pack_id = args
                    .get(index)
                    .ok_or_else(|| "missing value for --pack".to_string())?
                    .to_string();
            }
            "--profile" => {
                index += 1;
                options.profile_id = args
                    .get(index)
                    .ok_or_else(|| "missing value for --profile".to_string())?
                    .to_string();
            }
            "--yes" | "-y" => options.yes = true,
            value => return Err(format!("unexpected setup argument: {value}")),
        }
        index += 1;
    }
    Ok(options)
}

fn prompt_default(label: &str, default: &str) -> io::Result<String> {
    print!("{label} [{default}]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value = input.trim();
    if value.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(value.to_string())
    }
}

fn run_setup(home: &PathBuf, pack_id: &str, profile_id: &str) -> Result<String, String> {
    let project_root = logline_lab_core::lab_home::find_project_root()
        .ok_or_else(|| "setup: generated project root not found".to_string())?;
    let act = project_root.join("examples/acts/minimal.act.json");
    let input = fs::read_to_string(&act)
        .map_err(|err| format!("setup: unable to read starter Act {}: {err}", act.display()))?;

    let init = logline_lab_core::init_lab_home_with_selection(home, pack_id, profile_id)
        .map_err(|err| format!("setup: init failed: {err}"))?;
    let doctor = logline_lab_core::doctor_report_for(home);
    if !doctor.is_ok() {
        return Err(format!("setup: doctor failed\n{}", doctor.to_text()));
    }
    let validation = logline_lab_core::validate_text_result(&input)
        .map_err(|err| format!("setup: starter Act validation failed\n{err}"))?;
    let candidate = logline_lab_core::capture_candidate(home, &act)
        .map_err(|err| format!("setup: starter Candidate capture failed\n{err}"))?;
    let report = logline_lab_core::generate_daily_state_report(home)
        .map_err(|err| format!("setup: Daily State report failed\n{err}"))?;
    let projection = logline_lab_core::generate_projection(
        home,
        logline_lab_core::projections::ProjectionKind::LocalSummary,
    )
    .map_err(|err| format!("setup: local-summary projection failed\n{err}"))?;
    let ghosts = logline_lab_core::list_ghosts(home)
        .map_err(|err| format!("setup: Ghost list failed\n{err}"))?;
    let status = logline_lab_core::status_report_for(home);

    Ok([
        "LogLine Lab is ready.".to_string(),
        String::new(),
        init.to_text(),
        String::new(),
        doctor.to_text(),
        String::new(),
        validation,
        String::new(),
        candidate.to_text(),
        String::new(),
        report.to_text(),
        projection.to_text(),
        String::new(),
        ghosts.to_text(),
        String::new(),
        status.to_text(),
        String::new(),
        "Open your Lab:".to_string(),
        format!("  logline-lab status --home {}", home.display()),
        format!("  logline-lab candidate list --home {}", home.display()),
        format!("  logline-lab report generate daily-state --home {}", home.display()),
        String::new(),
        "Authority boundary: local workspace only; not official spine; not receipt; not evidence; not remote sync; no LLM authority.".to_string(),
    ]
    .join("\n"))
}

fn init_args(args: &[String]) -> Result<(PathBuf, String, String), String> {
    let mut home = PathBuf::from(".");
    let mut pack_id = logline_lab_core::catalog::DEFAULT_PACK_ID.to_string();
    let mut profile_id = logline_lab_core::catalog::DEFAULT_PROFILE_ID.to_string();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--home" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --home".to_string())?;
                home = PathBuf::from(value);
            }
            "--pack" => {
                index += 1;
                pack_id = args
                    .get(index)
                    .ok_or_else(|| "missing value for --pack".to_string())?
                    .to_string();
            }
            "--profile" => {
                index += 1;
                profile_id = args
                    .get(index)
                    .ok_or_else(|| "missing value for --profile".to_string())?
                    .to_string();
            }
            value => return Err(format!("unexpected init argument: {value}")),
        }
        index += 1;
    }
    Ok((home, pack_id, profile_id))
}

fn candidate_add_args(args: &[String]) -> Result<(PathBuf, PathBuf), String> {
    let mut home = PathBuf::from(".");
    let mut file = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--home" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --home".to_string())?;
                home = PathBuf::from(value);
            }
            "--file" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --file".to_string())?;
                file = Some(PathBuf::from(value));
            }
            value => return Err(format!("unexpected candidate add argument: {value}")),
        }
        index += 1;
    }
    let file = file.ok_or_else(|| {
        "usage: logline-lab candidate add --file <path> [--home <path>]".to_string()
    })?;
    Ok((home, file))
}

fn act_emit_args(args: &[String]) -> Result<(PathBuf, bool), String> {
    let mut file = None;
    let mut remote = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--file" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --file".to_string())?;
                file = Some(PathBuf::from(value));
            }
            "--remote" => remote = true,
            value => return Err(format!("unexpected act emit argument: {value}")),
        }
        index += 1;
    }
    let file = file.ok_or_else(|| {
        "usage: logline-lab act emit --file <path> [--remote]".to_string()
    })?;
    Ok((file, remote))
}

fn home_from_args(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, path] if flag == "--home" => Ok(PathBuf::from(path)),
        [flag] if flag == "--home" => Err("missing value for --home".to_string()),
        [path] => Ok(PathBuf::from(path)),
        _ => Err(
            "usage: logline-lab <init|doctor|status|candidate list|candidate get|candidate index rebuild|ghost list|projection list|projection generate local-summary|report generate daily-state> [--home <path>]"
                .to_string(),
        ),
    }
}

fn print_command_help(command: &str) {
    match command {
        "init" => {
            println!("Usage: logline-lab init --home <path> [--pack <id>] [--profile <id>]");
            println!("Initializes local workspace files only; no remote spine, receipt, or external service.");
        }
        "setup" => {
            println!("Usage: logline-lab setup [--home <path>] [--pack <id>] [--profile <id>] [--yes]");
            println!("Interactive first-run setup for a human operator. Creates a local Lab, captures a starter Candidate, writes a report, and generates a projection.");
        }
        "serve" => {
            println!("Usage: logline-lab serve [--host <ip>] [--port <port>]");
            println!("Runs the local browser product for human operators. It creates and reads local Lab state only.");
        }
        "supabase" => {
            println!("Usage: logline-lab supabase check");
            println!("Checks configured Supabase REST access to ops.logline_acts; does not write.");
        }
        "doctor" => {
            println!("Usage: logline-lab doctor --home <path>");
            println!("Checks local generated-project and Lab home structure; Ghosts may remain expected.");
        }
        "status" => {
            println!("Usage: logline-lab status --home <path>");
            println!("Reports local workspace state only; it is not an official spine or receipt.");
        }
        "lab" => {
            println!("Usage: logline-lab lab");
            println!("Ghost: interactive-lab-surface-unimplemented. The kit remains CLI-first.");
        }
        "chat" => {
            println!("Usage: logline-lab chat");
            println!("Ghost: llm-translator-unimplemented. No LLM is configured or authoritative.");
        }
        _ => print_help(),
    }
}

fn print_nested_command_help(scope: &str, action: &str) {
    match (scope, action) {
        ("act", "validate") => {
            println!("Usage: logline-lab act validate --file <path>");
            println!("Validates JSON LogLine Act shape only; no remote write and no receipt.");
        }
        ("act", "emit") => {
            println!("Usage: logline-lab act emit --file <path> [--remote]");
            println!("Without --remote this is preview-only. With --remote it writes through ops.ingest_logline_act into ops.logline_acts when Supabase is configured.");
            println!("Remote emit is not a receipt and not evidence.");
        }
        ("supabase", "check") => {
            println!("Usage: logline-lab supabase check");
            println!("Checks Supabase URL/key presence and ops.logline_acts visibility; no write.");
        }
        ("candidate", "add") => {
            println!("Usage: logline-lab candidate add --home <path> --file <path>");
            println!("Captures a local Candidate only; it is not an official spine write, receipt, or remote sync.");
        }
        ("candidate", "list") => {
            println!("Usage: logline-lab candidate list --home <path>");
            println!("Lists local Candidates only.");
        }
        ("candidate", "get") => {
            println!("Usage: logline-lab candidate get <candidate_id> --home <path>");
            println!("Shows local Candidate content and metadata only.");
        }
        ("candidate", "index") => {
            println!("Usage: logline-lab candidate index rebuild --home <path>");
            println!("Rebuilds local Candidate index workspace metadata only.");
        }
        ("ghost", "list") => {
            println!("Usage: logline-lab ghost list --home <path>");
            println!("Lists unresolved local Ghosts; this is not evidence proof.");
        }
        ("report", "generate") => {
            println!("Usage: logline-lab report generate daily-state --home <path>");
            println!("Generates a local workspace projection only; it is not a receipt.");
        }
        ("projection", "list") => {
            println!("Usage: logline-lab projection list --home <path>");
            println!(
                "Lists local projection read models only; not truth, not receipt, not evidence."
            );
        }
        ("projection", "generate") => {
            println!("Usage: logline-lab projection generate local-summary --home <path>");
            println!("Generates a local read model summary only; not truth, not receipt, not evidence, not remote sync.");
        }
        _ => print_help(),
    }
}

fn print_help() {
    println!("logline-lab {}", VERSION.trim());
    println!("CLI-first local LogLine Lab Kit.");
    println!();
    println!("Usage: logline-lab <command>");
    println!();
    println!("Local workspace commands:");
    println!("  setup [--home <path>] [--pack <id>] [--profile <id>]  Install-time wizard for a local Lab");
    println!("  serve [--host <ip>] [--port <port>]                 Run local browser product");
    println!("  supabase check                                      Check Supabase spine configuration");
    println!("  init --home <path> [--pack <id>] [--profile <id>]  Initialize a local Lab home");
    println!(
        "  doctor --home <path>                                Check local Lab home structure"
    );
    println!(
        "  status --home <path>                                Show local Lab workspace status"
    );
    println!();
    println!("Act commands:");
    println!(
        "  act validate --file <path>                          Validate JSON LogLine Act shape"
    );
    println!("  act emit --file <path> [--remote]                   Preview locally or write to ops.logline_acts");
    println!();
    println!("Candidate commands:");
    println!(
        "  candidate add --home <path> --file <path>           Capture a validated local Candidate"
    );
    println!("  candidate list --home <path>                        List local Candidates");
    println!("  candidate get <candidate_id> --home <path>          Show one local Candidate");
    println!("  candidate index rebuild --home <path>               Rebuild local Candidate index");
    println!();
    println!("Ghost and report commands:");
    println!("  ghost list --home <path>                            List unresolved local Ghosts");
    println!("  report generate daily-state --home <path>           Write a local Daily State projection");
    println!(
        "  projection list --home <path>                       List local projection read models"
    );
    println!("  projection generate local-summary --home <path>     Generate the local-summary read model");
    println!();
    println!("Ghost commands:");
    println!("  lab                                                 Ghost: interactive-lab-surface-unimplemented");
    println!(
        "  chat                                                Ghost: llm-translator-unimplemented"
    );
    println!();
    println!("Authority boundaries:");
    println!("  local-offline works without Supabase or external services.");
    println!("  Local workspace state is not an official spine, not a receipt store, and not evidence proof.");
    println!("  LLM/TUI/Supabase/receipts remain Ghosted unless explicitly implemented.");
}

const APP_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>LogLine Lab</title>
  <style>
    :root {
      color-scheme: light;
      --paper: #f4f2ee;
      --surface: #fffdf8;
      --ink: #1b1d21;
      --muted: #68707b;
      --line: #d8d1c4;
      --green: #155f47;
      --green-soft: #e2eee8;
      --amber: #9a6a13;
      --amber-soft: #f4ead3;
      --blue: #234b73;
      --blue-soft: #e1e9f2;
      --red: #8d2e2e;
      --red-soft: #f1dfdb;
      font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }
    * { box-sizing: border-box; }
    body { margin: 0; background: var(--paper); color: var(--ink); }
    main { max-width: 1280px; margin: 0 auto; padding: 24px; }
    header { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 20px; align-items: start; margin-bottom: 18px; border-bottom: 1px solid var(--line); padding-bottom: 18px; }
    h1 { font-size: clamp(30px, 4vw, 52px); line-height: .95; margin: 0 0 10px; letter-spacing: 0; }
    h2 { font-size: 17px; margin: 0 0 14px; }
    h3 { font-size: 13px; margin: 0; text-transform: uppercase; color: var(--muted); }
    p { line-height: 1.5; margin: 0; }
    .lede { max-width: 760px; color: #3e4651; font-size: 17px; }
    .shell { display: grid; grid-template-columns: 340px minmax(330px, .92fr) minmax(420px, 1.18fr); gap: 14px; align-items: stretch; }
    .panel { background: var(--surface); border: 1px solid var(--line); border-radius: 8px; padding: 18px; box-shadow: 0 1px 0 rgba(20, 25, 35, .04); }
    .identity { min-width: 230px; border: 1px solid var(--line); border-radius: 8px; background: var(--surface); padding: 12px; font-size: 13px; }
    .identity-row { display: flex; justify-content: space-between; gap: 12px; padding: 6px 0; border-bottom: 1px solid #ece5da; }
    .identity-row:last-child { border-bottom: 0; }
    .label { color: var(--muted); }
    .value { font-weight: 700; text-align: right; }
    label { display: block; font-size: 12px; font-weight: 750; color: #333942; margin: 14px 0 6px; text-transform: uppercase; }
    input { width: 100%; font: inherit; padding: 12px; border: 1px solid #b9b2a5; border-radius: 6px; background: #fff; color: var(--ink); }
    input:focus { outline: 2px solid #b8d8ca; border-color: var(--green); }
    button { width: 100%; font: inherit; font-weight: 800; padding: 13px 14px; border: 0; border-radius: 6px; color: #fff; background: var(--green); cursor: pointer; }
    button:hover { background: #104d3a; }
    button:disabled { opacity: .65; cursor: wait; }
    .form-actions { display: grid; gap: 10px; margin-top: 18px; }
    .hint { color: var(--muted); font-size: 13px; margin-top: 10px; }
    .boundary { color: #4f442f; background: var(--amber-soft); border: 1px solid #dec995; border-radius: 6px; font-size: 13px; padding: 11px; margin-top: 16px; }
    .cycle { display: grid; gap: 8px; }
    .step { display: grid; grid-template-columns: 18px 1fr auto; gap: 10px; align-items: start; padding: 10px; border: 1px solid #e4ded3; border-radius: 7px; background: #fffaf0; }
    .dot { width: 10px; height: 10px; border-radius: 999px; margin-top: 5px; background: #b6b0a5; }
    .step[data-state="active"] .dot { background: var(--amber); box-shadow: 0 0 0 4px rgba(154, 106, 19, .14); }
    .step[data-state="done"] .dot { background: var(--green); }
    .step[data-state="error"] .dot { background: var(--red); }
    .step-title { font-weight: 800; }
    .step-note { color: var(--muted); font-size: 13px; margin-top: 2px; }
    .step-state { font-size: 11px; color: var(--muted); text-transform: uppercase; padding-top: 2px; }
    .result-head { display: flex; justify-content: space-between; align-items: center; gap: 12px; margin-bottom: 12px; }
    .pill { display: inline-flex; align-items: center; gap: 7px; border-radius: 999px; padding: 7px 10px; font-size: 12px; font-weight: 800; background: var(--blue-soft); color: var(--blue); white-space: nowrap; }
    .pill.ready { background: var(--green-soft); color: var(--green); }
    .pill.error { background: var(--red-soft); color: var(--red); }
    .summary { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 8px; margin-bottom: 12px; }
    .metric { border: 1px solid #e4ded3; border-radius: 7px; padding: 10px; background: #fffaf0; min-height: 64px; }
    .metric strong { display: block; font-size: 18px; }
    .metric span { color: var(--muted); font-size: 12px; }
    pre { min-height: 430px; max-height: 58vh; margin: 0; white-space: pre-wrap; overflow: auto; font: 13px/1.45 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; background: #111820; color: #edf3f1; border-radius: 7px; padding: 15px; border: 1px solid #27313b; }
    .next { display: grid; gap: 8px; margin-top: 12px; }
    .cmd { background: #edf2ee; color: #17221e; border: 1px solid #cbd9d0; border-radius: 6px; padding: 9px 10px; font: 12px/1.35 ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; overflow-wrap: anywhere; }
    @media (max-width: 1100px) { .shell { grid-template-columns: 340px 1fr; } .output-panel { grid-column: 1 / -1; } }
    @media (max-width: 760px) { main { padding: 16px; } header, .shell, .summary { grid-template-columns: 1fr; } .identity { min-width: 0; } }
  </style>
</head>
<body>
  <main>
    <header>
      <div>
        <h1>LogLine Lab</h1>
        <p class="lede">First-run operator for a local Lab: create the workspace, validate the starter Act, capture a Candidate, generate reports, and leave a clear boundary around what is still not official authority.</p>
      </div>
      <div class="identity" aria-label="Runtime identity">
        <div class="identity-row"><span class="label">Profile</span><span class="value">local-offline</span></div>
        <div class="identity-row"><span class="label">Spine</span><span class="value">unconfigured</span></div>
        <div class="identity-row"><span class="label">Receipts</span><span class="value">not claimed</span></div>
      </div>
    </header>
    <div class="shell">
      <section class="panel">
        <h2>Create first Lab</h2>
        <p class="hint">Defaults are ready for a first local run. Change them only when you know the pack/profile you want.</p>
        <label for="home">Lab home</label>
        <input id="home" value="./demo-lab">
        <label for="pack">Pack</label>
        <input id="pack" value="santo-andre">
        <label for="profile">Profile</label>
        <input id="profile" value="local-offline">
        <div class="form-actions">
          <button id="setup">Create first Lab</button>
        </div>
        <p class="boundary">Local workspace only. Not official spine, not receipt, not evidence proof, not remote sync, and no LLM authority.</p>
      </section>
      <section class="panel">
        <h2>Run cycle</h2>
        <div class="cycle" id="cycle">
          <div class="step" data-step="load" data-state="idle"><span class="dot"></span><div><div class="step-title">Load</div><div class="step-note">Read core, examples, pack, and profile.</div></div><span class="step-state">idle</span></div>
          <div class="step" data-step="declare" data-state="idle"><span class="dot"></span><div><div class="step-title">Declare</div><div class="step-note">Create the Lab home and manifest.</div></div><span class="step-state">idle</span></div>
          <div class="step" data-step="observe" data-state="idle"><span class="dot"></span><div><div class="step-title">Observe</div><div class="step-note">Run doctor and read local status.</div></div><span class="step-state">idle</span></div>
          <div class="step" data-step="emit" data-state="idle"><span class="dot"></span><div><div class="step-title">Emit</div><div class="step-note">Validate the starter Act and capture one Candidate.</div></div><span class="step-state">idle</span></div>
          <div class="step" data-step="project" data-state="idle"><span class="dot"></span><div><div class="step-title">Project</div><div class="step-note">Write Daily State and local-summary.</div></div><span class="step-state">idle</span></div>
          <div class="step" data-step="learn" data-state="idle"><span class="dot"></span><div><div class="step-title">Learn</div><div class="step-note">List Ghosts and print next commands.</div></div><span class="step-state">idle</span></div>
        </div>
      </section>
      <section class="panel output-panel">
        <div class="result-head">
          <h2>Lab output</h2>
          <span class="pill" id="result-pill">ready</span>
        </div>
        <div class="summary">
          <div class="metric"><strong id="candidate-count">0</strong><span>Candidates</span></div>
          <div class="metric"><strong id="ghost-count">-</strong><span>Ghosts listed</span></div>
          <div class="metric"><strong id="projection-count">0</strong><span>Projections</span></div>
        </div>
        <pre id="output">Ready. Create the first Lab to run Load, Declare, Observe, Emit, Project, and Learn.</pre>
        <div class="next" id="next-commands" hidden></div>
      </section>
    </div>
  </main>
  <script>
    const button = document.getElementById('setup');
    const output = document.getElementById('output');
    const pill = document.getElementById('result-pill');
    const nextCommands = document.getElementById('next-commands');
    const steps = Array.from(document.querySelectorAll('.step'));
    const candidateCount = document.getElementById('candidate-count');
    const ghostCount = document.getElementById('ghost-count');
    const projectionCount = document.getElementById('projection-count');

    function setAllSteps(state) {
      steps.forEach((step) => setStep(step, state));
    }

    function setStep(step, state) {
      step.dataset.state = state;
      step.querySelector('.step-state').textContent = state;
    }

    function setRunning() {
      pill.textContent = 'running';
      pill.className = 'pill';
      setAllSteps('active');
      nextCommands.hidden = true;
      nextCommands.innerHTML = '';
    }

    function setComplete(text, ok) {
      pill.textContent = ok ? 'ready' : 'error';
      pill.className = ok ? 'pill ready' : 'pill error';
      setAllSteps(ok ? 'done' : 'error');
      candidateCount.textContent = matchValue(text, /candidate_count:\s*(\d+)/) || (text.includes('candidate captured') ? '1' : '0');
      ghostCount.textContent = matchValue(text, /ghosts:\s*(\d+)/) || matchValue(text, /ghost_count:\s*(\d+)/) || '-';
      projectionCount.textContent = matchValue(text, /projections_available:\s*(\d+)/) || (text.includes('local-summary projection generated') ? '1' : '0');
      renderNextCommands(text);
    }

    function matchValue(text, regex) {
      const match = text.match(regex);
      return match ? match[1] : null;
    }

    function renderNextCommands(text) {
      const lines = text.split('\n').filter((line) => line.trim().startsWith('logline-lab '));
      nextCommands.innerHTML = '';
      if (!lines.length) {
        nextCommands.hidden = true;
        return;
      }
      lines.slice(0, 3).forEach((line) => {
        const item = document.createElement('div');
        item.className = 'cmd';
        item.textContent = line.trim();
        nextCommands.appendChild(item);
      });
      nextCommands.hidden = false;
    }

    button.addEventListener('click', async () => {
      button.disabled = true;
      setRunning();
      output.textContent = 'Creating Lab...';
      const payload = {
        home: document.getElementById('home').value,
        pack: document.getElementById('pack').value,
        profile: document.getElementById('profile').value
      };
      try {
        const response = await fetch('/api/setup', {
          method: 'POST',
          headers: { 'content-type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const text = await response.text();
        output.textContent = text;
        setComplete(text, response.ok);
      } catch (error) {
        const text = String(error);
        output.textContent = text;
        setComplete(text, false);
      } finally {
        button.disabled = false;
      }
    });
  </script>
</body>
</html>
"#;
