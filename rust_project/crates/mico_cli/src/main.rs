use std::env;
use std::fs;
use std::process;

use mico_codegen::{emit_json_ir, emit_sva_skeleton, emit_systemverilog};
use mico_frontend::{ParseError, parse_mico};
use mico_ir::{Design, Diagnostic, Severity, TypedDesign, build_typed_ir, check_design};
use serde_json::{Value, json};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    command: Command,
    path: String,
    format: OutputFormat,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        eprintln!("{}", usage());
        process::exit(2);
    });

    let source = read_source_or_exit(&cli.path, cli.format);
    let design = parse_or_exit(&source, cli.format);

    match cli.command {
        Command::Parse => print_parse_summary(&design, cli.format),
        Command::Check => check_or_exit(&design, true, cli.format),
        Command::Build => {
            let typed = build_or_exit(&design, cli.format);
            print_build_summary(&typed, cli.format);
        }
        Command::DumpIr => {
            let typed = build_or_exit(&design, cli.format);
            print!("{}", emit_json_ir(&typed));
        }
        Command::EmitSv => {
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_systemverilog(&design));
        }
        Command::EmitSva => {
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_sva_skeleton(&design));
        }
        Command::Verify => {
            let typed = build_or_exit(&design, cli.format);
            print_verify_summary(&typed, cli.format);
        }
        Command::Report => report_or_exit(&design, cli.format),
    }
}

fn parse_args(args: &[String]) -> Result<CliArgs, String> {
    let mut format = OutputFormat::Text;
    let mut positional = Vec::new();
    let mut idx = 1;

    while idx < args.len() {
        let arg = &args[idx];
        if arg == "--format" {
            idx += 1;
            let value = args
                .get(idx)
                .ok_or_else(|| "missing value after `--format`".to_string())?;
            format = parse_output_format(value)?;
        } else if let Some(value) = arg.strip_prefix("--format=") {
            format = parse_output_format(value)?;
        } else {
            positional.push(arg.clone());
        }
        idx += 1;
    }

    let (command, path) = match positional.as_slice() {
        [command, path] => parse_simple_command(command).map(|command| (command, path.clone()))?,
        [emit, emit_format, path] if emit == "emit" => {
            parse_emit_format(emit_format).map(|command| (command, path.clone()))?
        }
        _ => return Err("invalid arguments".to_string()),
    };

    Ok(CliArgs {
        command,
        path,
        format,
    })
}

fn parse_output_format(format: &str) -> Result<OutputFormat, String> {
    match format {
        "text" => Ok(OutputFormat::Text),
        "json" => Ok(OutputFormat::Json),
        _ => Err(format!("unknown output format `{format}`")),
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
    "usage: mico [--format text|json] <parse|check|build|dump-ir|emit-sv|emit-sva|verify|report> <file.mico>\n       mico [--format text|json] emit <json|sv|sva> <file.mico>"
}

fn read_source_or_exit(path: &str, format: OutputFormat) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| {
        match format {
            OutputFormat::Text => eprintln!("failed to read `{}`: {}", path, err),
            OutputFormat::Json => print_json(io_error_response_json("read", path, &err)),
        }
        process::exit(1);
    })
}

fn parse_or_exit(source: &str, format: OutputFormat) -> Design {
    match parse_mico(source) {
        Ok(design) => design,
        Err(errors) => {
            match format {
                OutputFormat::Text => print_parse_errors_text(&errors),
                OutputFormat::Json => print_json(parse_error_response_json(&errors)),
            }
            process::exit(1);
        }
    }
}

fn check_or_exit(design: &Design, print_success: bool, format: OutputFormat) {
    let diagnostics = check_design(design);
    if format == OutputFormat::Json {
        print_json(diagnostic_response_json("check", &diagnostics));
        if has_errors(&diagnostics) {
            process::exit(1);
        }
        return;
    }

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

fn build_or_exit(design: &Design, format: OutputFormat) -> TypedDesign {
    let diagnostics = check_design(design);
    if has_errors(&diagnostics) {
        print_phase_diagnostics("check", &diagnostics, format);
        process::exit(1);
    }

    build_typed_ir(design).unwrap_or_else(|diagnostics| {
        print_phase_diagnostics("build", &diagnostics, format);
        process::exit(1);
    })
}

fn report_or_exit(design: &Design, format: OutputFormat) {
    let diagnostics = check_design(design);
    if format == OutputFormat::Json {
        print_json(report_json(design, &diagnostics));
    } else {
        println!("MICO report");
        print_design_summary(design);

        if diagnostics.is_empty() {
            println!("diagnostics: none");
        } else {
            println!("diagnostics:");
            print_diagnostics_stdout(&diagnostics);
        }
    }

    if has_errors(&diagnostics) {
        process::exit(1);
    }
}

fn print_parse_summary(design: &Design, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            println!("MICO parse passed");
            print_design_summary(design);
        }
        OutputFormat::Json => print_json(json!({
            "schema_version": "mico.diagnostics.v0",
            "ok": true,
            "phase": "parse",
            "summary": design_summary_json(design),
            "diagnostics": [],
        })),
    }
}

fn print_build_summary(typed: &TypedDesign, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            println!("MICO build passed");
            println!("clock_domains: {}", typed.clock_domains.len());
            println!("interfaces: {}", typed.interfaces.len());
            println!("modules: {}", typed.modules.len());
            println!("adapters: {}", typed.adapters.len());
            println!("composes: {}", typed.composes.len());
            println!("connections: {}", typed_connection_count(typed));
        }
        OutputFormat::Json => print_json(json!({
            "schema_version": "mico.diagnostics.v0",
            "ok": true,
            "phase": "build",
            "summary": typed_summary_json(typed),
            "diagnostics": [],
        })),
    }
}

fn print_verify_summary(typed: &TypedDesign, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            println!("MICO verify passed");
            println!("compiler_checks: passed");
            println!("typed_ir: passed");
            println!("connections: {}", typed_connection_count(typed));
            println!("eda: not run (Yosys/Verilator flow is added in the EDA milestone)");
        }
        OutputFormat::Json => print_json(json!({
            "schema_version": "mico.diagnostics.v0",
            "ok": true,
            "phase": "verify",
            "summary": typed_summary_json(typed),
            "checks": {
                "compiler_checks": "passed",
                "typed_ir": "passed",
                "eda": "not_run"
            },
            "diagnostics": [],
        })),
    }
}

fn print_design_summary(design: &Design) {
    println!("clock_domains: {}", design.clock_domains.len());
    println!("interfaces: {}", design.interfaces.len());
    println!("modules: {}", design.modules.len());
    println!("adapters: {}", design.adapters.len());
    println!("composes: {}", design.composes.len());
}

fn print_parse_errors_text(errors: &[ParseError]) {
    for e in errors {
        eprintln!(
            "parse error line {} column {} [{}]: {}",
            e.line, e.column, e.code, e.message
        );
    }
}

fn print_diagnostics_stdout(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        println!(
            "{} [{}] {}",
            diagnostic.severity.as_str(),
            diagnostic.code,
            diagnostic.message
        );
        for hint in &diagnostic.hints {
            println!("  hint: {hint}");
        }
    }
}

fn print_diagnostics_stderr(diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        eprintln!(
            "{} [{}] {}",
            diagnostic.severity.as_str(),
            diagnostic.code,
            diagnostic.message
        );
        for hint in &diagnostic.hints {
            eprintln!("  hint: {hint}");
        }
    }
}

fn print_phase_diagnostics(phase: &'static str, diagnostics: &[Diagnostic], format: OutputFormat) {
    match format {
        OutputFormat::Text => print_diagnostics_stderr(diagnostics),
        OutputFormat::Json => print_json(diagnostic_response_json(phase, diagnostics)),
    }
}

fn diagnostic_response_json(phase: &'static str, diagnostics: &[Diagnostic]) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": !has_errors(diagnostics),
        "phase": phase,
        "diagnostics": diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
    })
}

fn diagnostic_json(diagnostic: &Diagnostic) -> Value {
    json!({
        "severity": diagnostic.severity.as_str(),
        "code": diagnostic.code,
        "message": &diagnostic.message,
        "span": Value::Null,
        "labels": [],
        "nodes": [],
        "hints": &diagnostic.hints,
    })
}

fn parse_error_response_json(errors: &[ParseError]) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": false,
        "phase": "parse",
        "diagnostics": errors.iter().map(parse_error_json).collect::<Vec<_>>(),
    })
}

fn parse_error_json(error: &ParseError) -> Value {
    json!({
        "severity": "error",
        "code": error.code,
        "message": &error.message,
        "span": {
            "start": error.span.start,
            "end": error.span.end,
            "line": error.line,
            "column": error.column,
        },
        "labels": [{
            "style": "primary",
            "message": &error.message,
            "span": {
                "start": error.span.start,
                "end": error.span.end,
                "line": error.line,
                "column": error.column,
            }
        }],
        "nodes": [],
        "hints": [],
    })
}

fn io_error_response_json(phase: &'static str, path: &str, err: &std::io::Error) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": false,
        "phase": phase,
        "diagnostics": [{
            "severity": "error",
            "code": "IoError",
            "message": format!("failed to read `{path}`: {err}"),
            "span": Value::Null,
            "labels": [],
            "nodes": [{"kind": "file", "name": path}],
            "hints": ["check that the file exists and is readable"],
        }],
    })
}

fn report_json(design: &Design, diagnostics: &[Diagnostic]) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": !has_errors(diagnostics),
        "phase": "report",
        "summary": design_summary_json(design),
        "diagnostics": diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
    })
}

fn design_summary_json(design: &Design) -> Value {
    json!({
        "clock_domains": design.clock_domains.len(),
        "interfaces": design.interfaces.len(),
        "modules": design.modules.len(),
        "adapters": design.adapters.len(),
        "composes": design.composes.len(),
        "connections": design.composes.iter().map(|compose| compose.connections.len()).sum::<usize>(),
    })
}

fn typed_summary_json(typed: &TypedDesign) -> Value {
    json!({
        "clock_domains": typed.clock_domains.len(),
        "interfaces": typed.interfaces.len(),
        "modules": typed.modules.len(),
        "adapters": typed.adapters.len(),
        "composes": typed.composes.len(),
        "connections": typed_connection_count(typed),
    })
}

fn typed_connection_count(typed: &TypedDesign) -> usize {
    typed
        .composes
        .iter()
        .map(|compose| compose.connections.len())
        .sum::<usize>()
}

fn print_json(value: Value) {
    println!(
        "{}",
        serde_json::to_string_pretty(&value)
            .expect("MICO CLI JSON serialization should be infallible")
    );
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
            CliArgs {
                command: Command::Build,
                path: "input.mico".to_string(),
                format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn parses_emit_alias() {
        let args = strings(&["mico", "emit", "sv", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::EmitSv,
                path: "input.mico".to_string(),
                format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn parses_json_format_before_command() {
        let args = strings(&["mico", "--format", "json", "check", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::Check,
                path: "input.mico".to_string(),
                format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn parses_json_format_after_path() {
        let args = strings(&["mico", "check", "input.mico", "--format=json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::Check,
                path: "input.mico".to_string(),
                format: OutputFormat::Json,
            }
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
