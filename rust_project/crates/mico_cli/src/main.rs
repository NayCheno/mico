use std::env;
use std::fs;
use std::process;

use mico_codegen::{emit_json_ir, emit_sva_skeleton, emit_systemverilog};
use mico_frontend::parse_mico;
use mico_ir::{Design, Diagnostic, Severity, TypedDesign, build_typed_ir, check_design};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Parse,
    Check,
    Build,
    DumpIr,
    EmitSv,
    EmitSva,
    Verify,
    Report,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (command, path) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        eprintln!("{}", usage());
        process::exit(2);
    });

    let source = fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("failed to read `{}`: {}", path, err);
        process::exit(1);
    });

    let design = parse_or_exit(&source);

    match command {
        Command::Parse => print_parse_summary(&design),
        Command::Check => check_or_exit(&design, true),
        Command::Build => {
            let typed = build_or_exit(&design);
            print_build_summary(&typed);
        }
        Command::DumpIr => {
            check_or_exit(&design, false);
            print!("{}", emit_json_ir(&design));
        }
        Command::EmitSv => {
            check_or_exit(&design, false);
            print!("{}", emit_systemverilog(&design));
        }
        Command::EmitSva => {
            check_or_exit(&design, false);
            print!("{}", emit_sva_skeleton(&design));
        }
        Command::Verify => {
            let typed = build_or_exit(&design);
            print_verify_summary(&typed);
        }
        Command::Report => report_or_exit(&design),
    }
}

fn parse_args(args: &[String]) -> Result<(Command, String), String> {
    match args {
        [_, command, path] => parse_simple_command(command).map(|command| (command, path.clone())),
        [_, emit, format, path] if emit == "emit" => {
            parse_emit_format(format).map(|command| (command, path.clone()))
        }
        _ => Err("invalid arguments".to_string()),
    }
}

fn parse_simple_command(command: &str) -> Result<Command, String> {
    match command {
        "parse" => Ok(Command::Parse),
        "check" => Ok(Command::Check),
        "build" => Ok(Command::Build),
        "dump-ir" | "emit-json" => Ok(Command::DumpIr),
        "emit-sv" => Ok(Command::EmitSv),
        "emit-sva" => Ok(Command::EmitSva),
        "verify" => Ok(Command::Verify),
        "report" => Ok(Command::Report),
        _ => Err(format!("unknown command `{command}`")),
    }
}

fn parse_emit_format(format: &str) -> Result<Command, String> {
    match format {
        "json" | "ir" => Ok(Command::DumpIr),
        "sv" | "systemverilog" => Ok(Command::EmitSv),
        "sva" => Ok(Command::EmitSva),
        _ => Err(format!("unknown emit format `{format}`")),
    }
}

fn usage() -> &'static str {
    "usage: mico <parse|check|build|dump-ir|emit-sv|emit-sva|verify|report> <file.mico>\n       mico emit <json|sv|sva> <file.mico>"
}

fn parse_or_exit(source: &str) -> Design {
    match parse_mico(source) {
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
    }
}

fn check_or_exit(design: &Design, print_success: bool) {
    let diagnostics = check_design(design);
    if diagnostics.is_empty() {
        if print_success {
            println!("MICO check passed");
        }
        return;
    }

    print_diagnostics_stdout(&diagnostics);
    if has_errors(&diagnostics) {
        process::exit(1);
    }
}

fn build_or_exit(design: &Design) -> TypedDesign {
    let diagnostics = check_design(design);
    if has_errors(&diagnostics) {
        print_diagnostics_stderr(&diagnostics);
        process::exit(1);
    }

    build_typed_ir(design).unwrap_or_else(|diagnostics| {
        print_diagnostics_stderr(&diagnostics);
        process::exit(1);
    })
}

fn report_or_exit(design: &Design) {
    let diagnostics = check_design(design);
    println!("MICO report");
    print_design_summary(design);

    if diagnostics.is_empty() {
        println!("diagnostics: none");
    } else {
        println!("diagnostics:");
        print_diagnostics_stdout(&diagnostics);
    }

    if has_errors(&diagnostics) {
        process::exit(1);
    }
}

fn print_parse_summary(design: &Design) {
    println!("MICO parse passed");
    print_design_summary(design);
}

fn print_build_summary(typed: &TypedDesign) {
    println!("MICO build passed");
    println!("clock_domains: {}", typed.clock_domains.len());
    println!("interfaces: {}", typed.interfaces.len());
    println!("modules: {}", typed.modules.len());
    println!("adapters: {}", typed.adapters.len());
    println!("composes: {}", typed.composes.len());
    println!(
        "connections: {}",
        typed
            .composes
            .iter()
            .map(|compose| compose.connections.len())
            .sum::<usize>()
    );
}

fn print_verify_summary(typed: &TypedDesign) {
    println!("MICO verify passed");
    println!("compiler_checks: passed");
    println!("typed_ir: passed");
    println!(
        "connections: {}",
        typed
            .composes
            .iter()
            .map(|compose| compose.connections.len())
            .sum::<usize>()
    );
    println!("eda: not run (Yosys/Verilator flow is added in the EDA milestone)");
}

fn print_design_summary(design: &Design) {
    println!("clock_domains: {}", design.clock_domains.len());
    println!("interfaces: {}", design.interfaces.len());
    println!("modules: {}", design.modules.len());
    println!("adapters: {}", design.adapters.len());
    println!("composes: {}", design.composes.len());
}

fn print_diagnostics_stdout(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        println!(
            "{:?} [{}] {}",
            diagnostic.severity, diagnostic.code, diagnostic.message
        );
        for hint in &diagnostic.hints {
            println!("  hint: {hint}");
        }
    }
}

fn print_diagnostics_stderr(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{:?} [{}] {}",
            diagnostic.severity, diagnostic.code, diagnostic.message
        );
        for hint in &diagnostic.hints {
            eprintln!("  hint: {hint}");
        }
    }
}

fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_command() {
        let args = strings(&["mico", "build", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            (Command::Build, "input.mico".to_string())
        );
    }

    #[test]
    fn parses_emit_alias() {
        let args = strings(&["mico", "emit", "sv", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            (Command::EmitSv, "input.mico".to_string())
        );
    }

    #[test]
    fn rejects_bad_usage() {
        let args = strings(&["mico", "emit", "bad", "input.mico"]);
        assert!(parse_args(&args).is_err());
    }

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }
}
