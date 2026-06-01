use std::{env, fs, path::PathBuf, process};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let code = dispatch(&args);
    process::exit(code);
}

fn dispatch(args: &[String]) -> i32 {
    match args {
        [] => {
            println!("logline-lab 0.1.0");
            println!("partial: run --help for generated CLI commands");
            0
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("logline-lab 0.1.0");
            0
        }
        [flag] if flag == "--help" || flag == "-h" => {
            print_help();
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
        [scope, rest @ ..] if scope == "report" => dispatch_report(rest),
        [cmd] if cmd == "lab" => {
            eprintln!("Ghost: interactive-lab-surface-unimplemented");
            2
        }
        [cmd] if cmd == "chat" => {
            eprintln!("Ghost: llm-translator-unimplemented");
            2
        }
        [scope, action] if scope == "act" && action == "validate" => {
            println!("partial: provide --file <path> to validate a JSON LogLine Act");
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
            eprintln!("usage: logline-lab candidate <add --file <path>|list|get <candidate_id>> [--home <path>]");
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
            "usage: logline-lab <init|doctor|status|candidate list|candidate get|ghost list|report generate daily-state> [--home <path>]"
                .to_string(),
        ),
    }
}

fn print_help() {
    println!("Usage: logline-lab <command>");
    println!("Commands: init [--home <path>] [--pack <id>] [--profile <id>], doctor [--home <path>], status [--home <path>], candidate add --file <path> [--home <path>], candidate list [--home <path>], candidate get <candidate_id> [--home <path>], ghost list [--home <path>], report generate daily-state [--home <path>], act validate [--file <path>], act emit --file <path>, lab, chat");
}
