use std::{env, fs, process};

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
        [cmd] if cmd == "init" => {
            println!("partial: lab manifest template is available in manifests/lab.manifest.example.yaml");
            0
        }
        [cmd] if cmd == "doctor" => {
            println!("{}", logline_lab_core::doctor_report());
            0
        }
        [cmd] if cmd == "status" => {
            println!("{}", logline_lab_core::status_report());
            0
        }
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

fn print_help() {
    println!("Usage: logline-lab <command>");
    println!("Commands: init, doctor, status, act validate [--file <path>], act emit --file <path>, lab, chat");
}
