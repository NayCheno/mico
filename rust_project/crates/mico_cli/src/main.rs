use std::env;
use std::fs;
use std::process;

use mico_codegen::{emit_json_ir, emit_sva_skeleton, emit_systemverilog};
use mico_frontend::parse_mico;
use mico_ir::{Severity, check_design};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: mico <check|dump-ir|emit-sv|emit-sva> <file.mico>");
        process::exit(2);
    }

    let command = &args[1];
    let path = &args[2];
    let source = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("failed to read `{}`: {}", path, err);
        process::exit(1);
    });

    let design = match parse_mico(&source) {
        Ok(design) => design,
        Err(errors) => {
            for e in errors {
                eprintln!(
                    "parse error line {} column {} [{}]: {}",
                    e.line, e.column, e.code, e.message
                );
            }
            process::exit(1);
        }
    };

    let diagnostics = check_design(&design);
    let has_errors = diagnostics.iter().any(|d| d.severity == Severity::Error);

    match command.as_str() {
        "check" => {
            if diagnostics.is_empty() {
                println!("MICO check passed");
            } else {
                for d in &diagnostics {
                    println!("{:?} [{}] {}", d.severity, d.code, d.message);
                    for h in &d.hints {
                        println!("  hint: {}", h);
                    }
                }
            }
            if has_errors {
                process::exit(1);
            }
        }
        "dump-ir" => {
            if has_errors {
                print_diags_and_exit(&diagnostics);
            }
            print!("{}", emit_json_ir(&design));
        }
        "emit-sv" => {
            if has_errors {
                print_diags_and_exit(&diagnostics);
            }
            print!("{}", emit_systemverilog(&design));
        }
        "emit-sva" => {
            if has_errors {
                print_diags_and_exit(&diagnostics);
            }
            print!("{}", emit_sva_skeleton(&design));
        }
        _ => {
            eprintln!("unknown command `{}`", command);
            process::exit(2);
        }
    }
}

fn print_diags_and_exit(diags: &[mico_ir::Diagnostic]) -> ! {
    for d in diags {
        eprintln!("{:?} [{}] {}", d.severity, d.code, d.message);
        for h in &d.hints {
            eprintln!("  hint: {}", h);
        }
    }
    process::exit(1)
}
