use std::env;
use std::fs;
use std::process;

use mico_codegen::{emit_json_ir, emit_sva_skeleton, emit_systemverilog, emit_traceability_report};
use mico_frontend::{ParseError, parse_mico};
use mico_ir::{
    AstDocument, Design, Diagnostic, DiagnosticLabel, DiagnosticNode, LabelStyle, RepairAction,
    RepairPatch, Severity, SourceSpan, TypedDesign, apply_repair_patch_to_ast, build_typed_ir,
    check_design,
};
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Parse,
    Check,
    Build,
    DumpAst,
    DumpIr,
    EmitSv,
    EmitSva,
    EmitTrace,
    CheckJson,
    BuildJson,
    DumpJsonIr,
    EmitJsonSv,
    EmitJsonSva,
    EmitJsonTrace,
    RepairJson,
    Verify,
    Report,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RepairMode {
    DryRun,
    Apply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    command: Command,
    path: String,
    patch_path: Option<String>,
    format: OutputFormat,
    repair_mode: RepairMode,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        eprintln!("{}", usage());
        process::exit(2);
    });

    let source = read_source_or_exit(&cli.path, cli.format);

    match cli.command {
        Command::Parse => {
            let design = parse_or_exit(&source, cli.format);
            print_parse_summary(&design, cli.format);
        }
        Command::Check => {
            let design = parse_or_exit(&source, cli.format);
            check_or_exit(&design, true, cli.format);
        }
        Command::Build => {
            let design = parse_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print_build_summary(&typed, cli.format);
        }
        Command::DumpAst => {
            let design = parse_or_exit(&source, cli.format);
            print!("{}", emit_ast_json(&design));
        }
        Command::DumpIr => {
            let design = parse_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print!("{}", emit_json_ir(&typed));
        }
        Command::EmitSv => {
            let design = parse_or_exit(&source, cli.format);
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_systemverilog(&design));
        }
        Command::EmitSva => {
            let design = parse_or_exit(&source, cli.format);
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_sva_skeleton(&design));
        }
        Command::EmitTrace => {
            let design = parse_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print!("{}", emit_traceability_report(&typed));
        }
        Command::CheckJson => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            check_or_exit(&design, true, cli.format);
        }
        Command::BuildJson => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print_build_summary(&typed, cli.format);
        }
        Command::DumpJsonIr => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print!("{}", emit_json_ir(&typed));
        }
        Command::EmitJsonSv => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_systemverilog(&design));
        }
        Command::EmitJsonSva => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            let _typed = build_or_exit(&design, cli.format);
            print!("{}", emit_sva_skeleton(&design));
        }
        Command::EmitJsonTrace => {
            let design = parse_json_ast_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print!("{}", emit_traceability_report(&typed));
        }
        Command::RepairJson => {
            let patch_path = cli.patch_path.as_deref().unwrap_or_else(|| {
                eprintln!("missing patch path for repair-json");
                process::exit(2);
            });
            repair_json_or_exit(&cli.path, &source, patch_path, cli.repair_mode, cli.format);
        }
        Command::Verify => {
            let design = parse_or_exit(&source, cli.format);
            let typed = build_or_exit(&design, cli.format);
            print_verify_summary(&typed, cli.format);
        }
        Command::Report => {
            let design = parse_or_exit(&source, cli.format);
            report_or_exit(&design, cli.format);
        }
    }
}

fn parse_args(args: &[String]) -> Result<CliArgs, String> {
    let mut format = OutputFormat::Text;
    let mut repair_mode = RepairMode::DryRun;
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
        } else if arg == "--json" {
            format = OutputFormat::Json;
        } else if arg == "--dry-run" {
            repair_mode = RepairMode::DryRun;
        } else if arg == "--apply" {
            repair_mode = RepairMode::Apply;
        } else {
            positional.push(arg.clone());
        }
        idx += 1;
    }

    let (command, path, patch_path) = match positional.as_slice() {
        [command, path] => {
            let command = parse_simple_command(command)?;
            if command == Command::RepairJson {
                return Err("repair-json requires <ast.json> <patch.json>".to_string());
            }
            (command, path.clone(), None)
        }
        [command, path, patch] if command == "repair-json" || command == "apply-repair-json" => {
            (Command::RepairJson, path.clone(), Some(patch.clone()))
        }
        [emit, emit_format, path] if emit == "emit" => {
            parse_emit_format(emit_format).map(|command| (command, path.clone(), None))?
        }
        _ => return Err("invalid arguments".to_string()),
    };

    if command != Command::RepairJson && repair_mode != RepairMode::DryRun {
        return Err("--apply is only valid with repair-json".to_string());
    }

    Ok(CliArgs {
        command,
        path,
        patch_path,
        format,
        repair_mode,
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
        "dump-ast" | "dump-ast-json" | "emit-ast-json" => Ok(Command::DumpAst),
        "dump-ir" | "emit-json" => Ok(Command::DumpIr),
        "emit-sv" => Ok(Command::EmitSv),
        "emit-sva" => Ok(Command::EmitSva),
        "emit-trace" | "trace" => Ok(Command::EmitTrace),
        "check-json" => Ok(Command::CheckJson),
        "build-json" => Ok(Command::BuildJson),
        "dump-json-ir" => Ok(Command::DumpJsonIr),
        "emit-json-sv" => Ok(Command::EmitJsonSv),
        "emit-json-sva" => Ok(Command::EmitJsonSva),
        "emit-json-trace" => Ok(Command::EmitJsonTrace),
        "repair-json" | "apply-repair-json" => Ok(Command::RepairJson),
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
        "trace" | "traceability" => Ok(Command::EmitTrace),
        _ => Err(format!("unknown emit format `{format}`")),
    }
}

fn usage() -> &'static str {
    "usage: mico [--format text|json|--json] <parse|check|build|dump-ast-json|dump-ir|emit-sv|emit-sva|emit-trace|check-json|build-json|dump-json-ir|emit-json-sv|emit-json-sva|emit-json-trace|verify|report> <file>\n       mico [--format text|json|--json] emit <json|sv|sva|trace> <file.mico>\n       mico [--format text|json|--json] repair-json [--dry-run|--apply] <ast.json> <patch.json>"
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

fn parse_json_ast_or_exit(source: &str, format: OutputFormat) -> Design {
    match parse_json_ast(source) {
        Ok(design) => design,
        Err(diagnostics) => {
            print_phase_diagnostics("parse", &diagnostics, format);
            process::exit(1);
        }
    }
}

fn parse_json_ast(source: &str) -> Result<Design, Vec<Diagnostic>> {
    parse_json_ast_document(source)?.into_design()
}

fn parse_json_ast_document(source: &str) -> Result<AstDocument, Vec<Diagnostic>> {
    let document = serde_json::from_str::<AstDocument>(source).map_err(|err| {
        vec![
            Diagnostic::error("JsonSchemaError", format!("invalid MICO JSON AST: {err}"))
                .with_label(
                    LabelStyle::Primary,
                    "JSON AST does not match schemas/mico_ast.schema.json",
                )
                .with_repair(RepairAction::FixSyntax),
        ]
    })?;
    if document.schema_version != mico_ir::MICO_AST_SCHEMA_VERSION || document.kind != "design" {
        document.clone().into_design()?;
    }
    Ok(document)
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

fn repair_json_or_exit(
    ast_path: &str,
    ast_source: &str,
    patch_path: &str,
    mode: RepairMode,
    format: OutputFormat,
) {
    let document = parse_json_ast_document(ast_source).unwrap_or_else(|diagnostics| {
        print_phase_diagnostics("parse", &diagnostics, format);
        process::exit(1);
    });
    let patch_source = read_source_or_exit(patch_path, format);
    let patch = serde_json::from_str::<RepairPatch>(&patch_source).unwrap_or_else(|err| {
        let diagnostics = vec![
            Diagnostic::error(
                "RepairPatchError",
                format!("invalid repair patch JSON: {err}"),
            )
            .with_label(
                LabelStyle::Primary,
                "repair patch does not match schemas/mico_repair_patch.schema.json",
            )
            .with_repair(RepairAction::FixSyntax),
        ];
        print_phase_diagnostics("parse", &diagnostics, format);
        process::exit(1);
    });
    let patched = apply_repair_patch_to_ast(&document, &patch).unwrap_or_else(|diagnostic| {
        let diagnostics = vec![diagnostic];
        print_phase_diagnostics("parse", &diagnostics, format);
        process::exit(1);
    });
    let design = patched.clone().into_design().unwrap_or_else(|diagnostics| {
        print_phase_diagnostics("parse", &diagnostics, format);
        process::exit(1);
    });
    let diagnostics = check_design(&design);

    if mode == RepairMode::Apply {
        let mut out = serde_json::to_string_pretty(&patched)
            .expect("MICO AST JSON serialization should be infallible");
        out.push('\n');
        fs::write(ast_path, out).unwrap_or_else(|err| {
            match format {
                OutputFormat::Text => eprintln!("failed to write `{}`: {}", ast_path, err),
                OutputFormat::Json => print_json(io_error_response_json("parse", ast_path, &err)),
            }
            process::exit(1);
        });
    }

    match format {
        OutputFormat::Text => {
            match mode {
                RepairMode::DryRun => println!("MICO repair patch dry run passed"),
                RepairMode::Apply => println!("MICO repair patch applied"),
            }
            if diagnostics.is_empty() {
                println!("MICO check passed after repair");
            } else {
                print_diagnostics_stdout(&diagnostics);
            }
        }
        OutputFormat::Json => print_json(repair_response_json(mode, &patch, &diagnostics)),
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
        OutputFormat::Json => print_json(parse_summary_response_json(design)),
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
        OutputFormat::Json => print_json(build_summary_response_json(typed)),
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

fn emit_ast_json(design: &Design) -> String {
    let mut out = serde_json::to_string_pretty(&AstDocument::from_design(design))
        .expect("MICO AST JSON serialization should be infallible");
    out.push('\n');
    out
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

fn parse_summary_response_json(design: &Design) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": true,
        "phase": "parse",
        "summary": design_summary_json(design),
        "diagnostics": [],
    })
}

fn build_summary_response_json(typed: &TypedDesign) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": true,
        "phase": "build",
        "summary": typed_summary_json(typed),
        "diagnostics": [],
    })
}

fn repair_response_json(
    mode: RepairMode,
    patch: &RepairPatch,
    diagnostics: &[Diagnostic],
) -> Value {
    json!({
        "schema_version": "mico.diagnostics.v0",
        "ok": !has_errors(diagnostics),
        "phase": "check",
        "summary": {
            "patch_operations": patch.operations.len(),
        },
        "checks": {
            "repair_mode": match mode {
                RepairMode::DryRun => "dry_run",
                RepairMode::Apply => "apply",
            },
            "patch_application": "passed",
            "recheck": if has_errors(diagnostics) { "failed" } else { "passed" },
        },
        "diagnostics": diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
    })
}

fn diagnostic_json(diagnostic: &Diagnostic) -> Value {
    json!({
        "severity": diagnostic.severity.as_str(),
        "code": diagnostic.code,
        "message": &diagnostic.message,
        "span": span_json(diagnostic.span),
        "labels": diagnostic.labels.iter().map(label_json).collect::<Vec<_>>(),
        "nodes": diagnostic.nodes.iter().map(node_json).collect::<Vec<_>>(),
        "hints": &diagnostic.hints,
        "repair_action": diagnostic.repair_action.map(|action| action.as_str()),
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
        "repair_action": RepairAction::FixSyntax.as_str(),
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
            "repair_action": RepairAction::CheckFile.as_str(),
        }],
    })
}

fn span_json(span: Option<SourceSpan>) -> Value {
    match span {
        Some(span) => json!({
            "start": span.start,
            "end": span.end,
            "line": span.line,
            "column": span.column,
        }),
        None => Value::Null,
    }
}

fn label_json(label: &DiagnosticLabel) -> Value {
    json!({
        "style": label.style.as_str(),
        "message": &label.message,
        "span": span_json(label.span),
    })
}

fn node_json(node: &DiagnosticNode) -> Value {
    json!({
        "kind": node.kind,
        "name": &node.name,
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
                patch_path: None,
                format: OutputFormat::Text,
                repair_mode: RepairMode::DryRun,
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
                patch_path: None,
                format: OutputFormat::Text,
                repair_mode: RepairMode::DryRun,
            }
        );
    }

    #[test]
    fn parses_trace_emit_alias() {
        let args = strings(&["mico", "emit", "trace", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::EmitTrace,
                path: "input.mico".to_string(),
                patch_path: None,
                format: OutputFormat::Text,
                repair_mode: RepairMode::DryRun,
            }
        );
    }

    #[test]
    fn parses_json_ast_commands() {
        let args = strings(&["mico", "check-json", "input.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::CheckJson,
                path: "input.json".to_string(),
                patch_path: None,
                format: OutputFormat::Text,
                repair_mode: RepairMode::DryRun,
            }
        );

        let args = strings(&["mico", "emit-json-sv", "input.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::EmitJsonSv,
                path: "input.json".to_string(),
                patch_path: None,
                format: OutputFormat::Text,
                repair_mode: RepairMode::DryRun,
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
                patch_path: None,
                format: OutputFormat::Json,
                repair_mode: RepairMode::DryRun,
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
                patch_path: None,
                format: OutputFormat::Json,
                repair_mode: RepairMode::DryRun,
            }
        );
    }

    #[test]
    fn parses_json_alias() {
        let args = strings(&["mico", "--json", "check", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::Check,
                path: "input.mico".to_string(),
                patch_path: None,
                format: OutputFormat::Json,
                repair_mode: RepairMode::DryRun,
            }
        );
    }

    #[test]
    fn parses_repair_json_command() {
        let args = strings(&["mico", "--json", "repair-json", "input.json", "patch.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::RepairJson,
                path: "input.json".to_string(),
                patch_path: Some("patch.json".to_string()),
                format: OutputFormat::Json,
                repair_mode: RepairMode::DryRun,
            }
        );

        let args = strings(&["mico", "repair-json", "--apply", "input.json", "patch.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::RepairJson,
                path: "input.json".to_string(),
                patch_path: Some("patch.json".to_string()),
                format: OutputFormat::Text,
                repair_mode: RepairMode::Apply,
            }
        );
    }

    #[test]
    fn rejects_bad_usage() {
        let args = strings(&["mico", "emit", "bad", "input.mico"]);
        assert!(parse_args(&args).is_err());
        let args = strings(&["mico", "--apply", "check", "input.mico"]);
        assert!(parse_args(&args).is_err());
        let args = strings(&["mico", "repair-json", "input.json"]);
        assert!(parse_args(&args).is_err());
    }

    #[test]
    fn diagnostics_json_matches_golden_fixtures() {
        assert_parse_fixture(
            include_str!("../../../examples/stream_fifo.mico"),
            include_str!("../tests/fixtures/diagnostics/valid_parse.json"),
        );
        assert_check_fixture(
            include_str!("../../../examples/stream_fifo.mico"),
            include_str!("../tests/fixtures/diagnostics/valid_check.json"),
        );
        assert_build_fixture(
            include_str!("../../../examples/stream_fifo.mico"),
            include_str!("../tests/fixtures/diagnostics/valid_build.json"),
        );
        assert_check_fixture(
            include_str!("../../../examples/invalid_width.mico"),
            include_str!("../tests/fixtures/diagnostics/invalid_width.json"),
        );
        assert_check_fixture(
            include_str!(
                "../../../../benchmarks/tasks/T006_direct_cdc_without_adapter/invalid.mico"
            ),
            include_str!("../tests/fixtures/diagnostics/direct_cdc_without_adapter.json"),
        );
        assert_check_fixture(
            include_str!("../../../../benchmarks/tasks/T007_reversed_direction/invalid.mico"),
            include_str!("../tests/fixtures/diagnostics/reversed_direction.json"),
        );
        assert_check_fixture(
            include_str!("../tests/fixtures/diagnostics/unknown_adapter_kind.mico"),
            include_str!("../tests/fixtures/diagnostics/unknown_adapter_kind.json"),
        );
        assert_check_fixture(
            include_str!("../tests/fixtures/diagnostics/contract_violation.mico"),
            include_str!("../tests/fixtures/diagnostics/contract_violation.json"),
        );
    }

    #[test]
    fn json_ast_round_trip_preserves_typed_ir() {
        let design = parse_mico(include_str!("../../../examples/stream_fifo.mico")).unwrap();
        let ast_json = emit_ast_json(&design);
        let json_design = parse_json_ast(&ast_json).unwrap();

        let dsl_typed = build_typed_ir(&design).unwrap();
        let json_typed = build_typed_ir(&json_design).unwrap();
        assert_eq!(emit_json_ir(&dsl_typed), emit_json_ir(&json_typed));
    }

    #[test]
    fn invalid_json_ast_returns_schema_diagnostic() {
        let diagnostics = parse_json_ast("{\"schema_version\":\"mico.ast.v0\"}").unwrap_err();
        assert_eq!(diagnostics[0].code, "JsonSchemaError");
        assert_eq!(diagnostics[0].repair_action, Some(RepairAction::FixSyntax));
    }

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    fn assert_parse_fixture(source: &str, fixture: &str) {
        let design = parse_mico(source).unwrap();
        assert_eq!(parse_summary_response_json(&design), fixture_json(fixture));
    }

    fn assert_check_fixture(source: &str, fixture: &str) {
        let design = parse_mico(source).unwrap();
        let diagnostics = check_design(&design);
        assert_eq!(
            diagnostic_response_json("check", &diagnostics),
            fixture_json(fixture)
        );
    }

    fn assert_build_fixture(source: &str, fixture: &str) {
        let design = parse_mico(source).unwrap();
        let typed = build_typed_ir(&design).unwrap();
        assert_eq!(build_summary_response_json(&typed), fixture_json(fixture));
    }

    fn fixture_json(fixture: &str) -> Value {
        serde_json::from_str(fixture).unwrap()
    }
}
