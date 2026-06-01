use std::{env, fs, path::PathBuf, process};

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
        [scope, action, flag, path] if scope == "act" && action == "emit" && flag == "--file" => {
            match fs::read_to_string(path) {
                Ok(input) => match logline_lab_core::emit_preview_result(&input) {
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
        _ => {
            eprintln!("ghost: unknown-command");
            print_help();
            1
        }
    }
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
            println!("Usage: logline-lab act emit --file <path>");
            println!("Preview-only command; no remote write and no receipt closure.");
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
    println!("  act emit --file <path>                              Preview only; no remote write or receipt");
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
