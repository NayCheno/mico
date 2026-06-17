use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerifyMode {
    Compiler,
    Eda,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    command: Command,
    path: String,
    patch_path: Option<String>,
    format: OutputFormat,
    repair_mode: RepairMode,
    verify_mode: VerifyMode,
    artifact_dir: Option<String>,
    schema_path: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args
        .iter()
        .skip(1)
        .any(|arg| arg == "--help" || arg == "-h")
    {
        println!("{}", usage());
        return;
    }

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
            verify_or_exit(&design, &typed, &cli);
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
    let mut verify_mode = VerifyMode::Compiler;
    let mut artifact_dir = None;
    let mut schema_path = None;
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
        } else if arg == "--compiler" {
            verify_mode = VerifyMode::Compiler;
        } else if arg == "--eda" {
            verify_mode = VerifyMode::Eda;
        } else if arg == "--artifact-dir" {
            idx += 1;
            artifact_dir = Some(
                args.get(idx)
                    .ok_or_else(|| "missing value after `--artifact-dir`".to_string())?
                    .clone(),
            );
        } else if let Some(value) = arg.strip_prefix("--artifact-dir=") {
            artifact_dir = Some(value.to_string());
        } else if arg == "--schema-path" {
            idx += 1;
            schema_path = Some(
                args.get(idx)
                    .ok_or_else(|| "missing value after `--schema-path`".to_string())?
                    .clone(),
            );
        } else if let Some(value) = arg.strip_prefix("--schema-path=") {
            schema_path = Some(value.to_string());
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
    if command != Command::Verify
        && (verify_mode != VerifyMode::Compiler || artifact_dir.is_some() || schema_path.is_some())
    {
        return Err(
            "--eda, --compiler, --artifact-dir, and --schema-path are only valid with verify"
                .to_string(),
        );
    }

    Ok(CliArgs {
        command,
        path,
        patch_path,
        format,
        repair_mode,
        verify_mode,
        artifact_dir,
        schema_path,
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
    "usage: mico [--format text|json|--json] <parse|check|build|dump-ast-json|dump-ir|emit-sv|emit-sva|emit-trace|check-json|build-json|dump-json-ir|emit-json-sv|emit-json-sva|emit-json-trace|verify|report> <file>\n       mico [--format text|json|--json] emit <json|sv|sva|trace> <file.mico>\n       mico [--format text|json|--json] repair-json [--dry-run|--apply] <ast.json> <patch.json>\n       mico [--format text|json|--json] verify [--compiler|--eda] [--artifact-dir DIR] [--schema-path PATH] <file.mico>"
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

#[derive(Debug, Clone)]
struct VerifyReport {
    artifact_dir: Option<PathBuf>,
    schema_path: Option<String>,
    eda_checks: Vec<EdaCheck>,
}

impl VerifyReport {
    fn eda_passed(&self) -> bool {
        self.eda_checks.iter().all(|check| check.passed)
    }
}

#[derive(Debug, Clone)]
struct EdaCheck {
    name: &'static str,
    command: String,
    passed: bool,
    exit_code: Option<i32>,
    stdout: PathBuf,
    stderr: PathBuf,
}

fn verify_or_exit(design: &Design, typed: &TypedDesign, cli: &CliArgs) {
    let report = match cli.verify_mode {
        VerifyMode::Compiler => VerifyReport {
            artifact_dir: None,
            schema_path: cli.schema_path.clone(),
            eda_checks: Vec::new(),
        },
        VerifyMode::Eda => {
            let mut report = run_eda_verify(design, &cli.path, cli.artifact_dir.as_deref())
                .unwrap_or_else(|diagnostic| {
                    let diagnostics = vec![diagnostic];
                    print_phase_diagnostics("verify", &diagnostics, cli.format);
                    process::exit(1);
                });
            report.schema_path = cli.schema_path.clone();
            report
        }
    };
    print_verify_summary(typed, &report, cli.format);
    if !report.eda_checks.is_empty() && !report.eda_passed() {
        process::exit(1);
    }
}

fn print_verify_summary(typed: &TypedDesign, report: &VerifyReport, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            if report.eda_checks.is_empty() || report.eda_passed() {
                println!("MICO verify passed");
            } else {
                println!("MICO verify failed");
            }
            println!("compiler_checks: passed");
            println!("typed_ir: passed");
            println!("connections: {}", typed_connection_count(typed));
            if let Some(path) = &report.artifact_dir {
                println!("artifact_dir: {}", path.display());
            }
            if report.eda_checks.is_empty() {
                println!("eda: not run");
            } else {
                for check in &report.eda_checks {
                    println!(
                        "{}: {}",
                        check.name,
                        if check.passed { "passed" } else { "failed" }
                    );
                }
            }
        }
        OutputFormat::Json => print_json(json!({
            "schema_version": "mico.diagnostics.v0",
            "ok": report.eda_checks.is_empty() || report.eda_passed(),
            "phase": "verify",
            "summary": typed_summary_json(typed),
            "checks": verify_checks_json(report),
            "diagnostics": [],
        })),
    }
}

fn verify_checks_json(report: &VerifyReport) -> Value {
    let mut checks = serde_json::Map::new();
    checks.insert(
        "compiler_checks".to_string(),
        Value::String("passed".to_string()),
    );
    checks.insert("typed_ir".to_string(), Value::String("passed".to_string()));
    checks.insert(
        "eda".to_string(),
        Value::String(if report.eda_checks.is_empty() {
            "not_run".to_string()
        } else if report.eda_passed() {
            "passed".to_string()
        } else {
            "failed".to_string()
        }),
    );
    if let Some(path) = &report.artifact_dir {
        checks.insert(
            "artifact_dir".to_string(),
            Value::String(path.display().to_string()),
        );
    }
    if let Some(path) = &report.schema_path {
        checks.insert("schema_path".to_string(), Value::String(path.clone()));
    }
    for check in &report.eda_checks {
        checks.insert(
            check.name.to_string(),
            Value::String(if check.passed { "passed" } else { "failed" }.to_string()),
        );
        checks.insert(
            format!("{}_stdout", check.name),
            Value::String(check.stdout.display().to_string()),
        );
        checks.insert(
            format!("{}_stderr", check.name),
            Value::String(check.stderr.display().to_string()),
        );
        checks.insert(
            format!("{}_command", check.name),
            Value::String(check.command.clone()),
        );
        checks.insert(
            format!("{}_exit_code", check.name),
            Value::String(
                check
                    .exit_code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "terminated_by_signal".to_string()),
            ),
        );
    }
    Value::Object(checks)
}

fn run_eda_verify(
    design: &Design,
    input_path: &str,
    artifact_dir: Option<&str>,
) -> Result<VerifyReport, Diagnostic> {
    let repo_root = find_repo_root().ok_or_else(|| {
        Diagnostic::error(
            "VerifyEdaError",
            "could not locate repository root with rtl/examples/mico_example_leafs.sv",
        )
        .with_label(
            LabelStyle::Primary,
            "verify --eda requires repository RTL collateral",
        )
        .with_hint("run verify --eda from the repository or rust_project directory")
        .with_repair(RepairAction::CheckFile)
    })?;
    let leafs = repo_root.join("rtl/examples/mico_example_leafs.sv");
    let artifact_dir = match artifact_dir {
        Some(path) => {
            let path = PathBuf::from(path);
            if path.is_absolute() {
                path
            } else {
                env::current_dir()
                    .map_err(|err| {
                        Diagnostic::error(
                            "VerifyEdaError",
                            format!("failed to resolve current directory: {err}"),
                        )
                        .with_label(
                            LabelStyle::Primary,
                            "artifact directory could not be resolved",
                        )
                        .with_repair(RepairAction::CheckFile)
                    })?
                    .join(path)
            }
        }
        None => repo_root
            .join("build/mico-verify")
            .join(input_stem(input_path)),
    };
    fs::create_dir_all(&artifact_dir).map_err(|err| {
        Diagnostic::error(
            "VerifyEdaError",
            format!(
                "failed to create artifact dir `{}`: {err}",
                artifact_dir.display()
            ),
        )
        .with_label(LabelStyle::Primary, "artifact directory is not writable")
        .with_repair(RepairAction::CheckFile)
    })?;

    let wrapper = artifact_dir.join("Top.sv");
    let sva = artifact_dir.join("Top.sva.sv");
    let vvp = artifact_dir.join("Top.vvp");
    fs::write(&wrapper, emit_systemverilog(design)).map_err(|err| {
        Diagnostic::error(
            "VerifyEdaError",
            format!("failed to write `{}`: {err}", wrapper.display()),
        )
        .with_label(
            LabelStyle::Primary,
            "SystemVerilog wrapper could not be written",
        )
        .with_repair(RepairAction::CheckFile)
    })?;
    fs::write(&sva, emit_sva_skeleton(design)).map_err(|err| {
        Diagnostic::error(
            "VerifyEdaError",
            format!("failed to write `{}`: {err}", sva.display()),
        )
        .with_label(LabelStyle::Primary, "SVA skeleton could not be written")
        .with_repair(RepairAction::CheckFile)
    })?;

    let checks = vec![
        run_tool_check(
            "verilator_wrapper",
            &repo_root,
            &artifact_dir,
            "verilator",
            vec![
                "--lint-only".to_string(),
                "-Wall".to_string(),
                "-Wno-DECLFILENAME".to_string(),
                "-Wno-UNUSEDSIGNAL".to_string(),
                "--top-module".to_string(),
                "Top".to_string(),
                path_arg(&leafs),
                path_arg(&wrapper),
            ],
        ),
        run_tool_check(
            "verilator_sva",
            &repo_root,
            &artifact_dir,
            "verilator",
            vec![
                "--lint-only".to_string(),
                "-Wall".to_string(),
                "-Wno-DECLFILENAME".to_string(),
                "-Wno-UNUSEDSIGNAL".to_string(),
                "--top-module".to_string(),
                "mico_sva_Top".to_string(),
                path_arg(&sva),
            ],
        ),
        run_tool_check(
            "iverilog_elab",
            &repo_root,
            &artifact_dir,
            "iverilog",
            vec![
                "-g2012".to_string(),
                "-s".to_string(),
                "Top".to_string(),
                "-o".to_string(),
                path_arg(&vvp),
                path_arg(&leafs),
                path_arg(&wrapper),
            ],
        ),
        run_tool_check(
            "yosys_hierarchy",
            &repo_root,
            &artifact_dir,
            "yosys",
            vec![
                "-q".to_string(),
                "-p".to_string(),
                format!(
                    "read_verilog -sv {} {}; hierarchy -check -top Top; proc; opt; stat",
                    path_arg(&leafs),
                    path_arg(&wrapper)
                ),
            ],
        ),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?;

    Ok(VerifyReport {
        artifact_dir: Some(artifact_dir),
        schema_path: None,
        eda_checks: checks,
    })
}

fn run_tool_check(
    name: &'static str,
    cwd: &Path,
    artifact_dir: &Path,
    program: &str,
    args: Vec<String>,
) -> Result<EdaCheck, Diagnostic> {
    let stdout = artifact_dir.join(format!("{name}.stdout.txt"));
    let stderr = artifact_dir.join(format!("{name}.stderr.txt"));
    let output = process::Command::new(program)
        .args(&args)
        .current_dir(cwd)
        .output()
        .map_err(|err| {
            Diagnostic::error(
                "VerifyEdaError",
                format!("failed to run `{program}` for {name}: {err}"),
            )
            .with_label(LabelStyle::Primary, "EDA tool invocation failed")
            .with_repair(RepairAction::CheckFile)
        })?;
    fs::write(&stdout, &output.stdout).map_err(|err| {
        Diagnostic::error(
            "VerifyEdaError",
            format!("failed to write `{}`: {err}", stdout.display()),
        )
        .with_label(
            LabelStyle::Primary,
            "EDA stdout artifact could not be written",
        )
        .with_repair(RepairAction::CheckFile)
    })?;
    fs::write(&stderr, &output.stderr).map_err(|err| {
        Diagnostic::error(
            "VerifyEdaError",
            format!("failed to write `{}`: {err}", stderr.display()),
        )
        .with_label(
            LabelStyle::Primary,
            "EDA stderr artifact could not be written",
        )
        .with_repair(RepairAction::CheckFile)
    })?;
    Ok(EdaCheck {
        name,
        command: format!("{} {}", program, args.join(" ")),
        passed: output.status.success(),
        exit_code: output.status.code(),
        stdout,
        stderr,
    })
}

fn find_repo_root() -> Option<PathBuf> {
    let mut current = env::current_dir().ok()?;
    loop {
        if current.join("rtl/examples/mico_example_leafs.sv").is_file() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn input_stem(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("input")
        .to_string()
}

fn path_arg(path: &Path) -> String {
    path.display().to_string()
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
            expected_cli(Command::Build, "input.mico", OutputFormat::Text)
        );
    }

    #[test]
    fn parses_emit_alias() {
        let args = strings(&["mico", "emit", "sv", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::EmitSv, "input.mico", OutputFormat::Text)
        );
    }

    #[test]
    fn parses_trace_emit_alias() {
        let args = strings(&["mico", "emit", "trace", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::EmitTrace, "input.mico", OutputFormat::Text)
        );
    }

    #[test]
    fn parses_json_ast_commands() {
        let args = strings(&["mico", "check-json", "input.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::CheckJson, "input.json", OutputFormat::Text)
        );

        let args = strings(&["mico", "emit-json-sv", "input.json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::EmitJsonSv, "input.json", OutputFormat::Text)
        );
    }

    #[test]
    fn parses_json_format_before_command() {
        let args = strings(&["mico", "--format", "json", "check", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::Check, "input.mico", OutputFormat::Json)
        );
    }

    #[test]
    fn parses_json_format_after_path() {
        let args = strings(&["mico", "check", "input.mico", "--format=json"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::Check, "input.mico", OutputFormat::Json)
        );
    }

    #[test]
    fn parses_json_alias() {
        let args = strings(&["mico", "--json", "check", "input.mico"]);
        assert_eq!(
            parse_args(&args).unwrap(),
            expected_cli(Command::Check, "input.mico", OutputFormat::Json)
        );
    }

    #[test]
    fn parses_verify_eda_command() {
        let args = strings(&[
            "mico",
            "--json",
            "verify",
            "--eda",
            "--artifact-dir",
            "build/verify",
            "--schema-path=schemas",
            "input.mico",
        ]);
        assert_eq!(
            parse_args(&args).unwrap(),
            CliArgs {
                command: Command::Verify,
                path: "input.mico".to_string(),
                patch_path: None,
                format: OutputFormat::Json,
                repair_mode: RepairMode::DryRun,
                verify_mode: VerifyMode::Eda,
                artifact_dir: Some("build/verify".to_string()),
                schema_path: Some("schemas".to_string()),
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
                verify_mode: VerifyMode::Compiler,
                artifact_dir: None,
                schema_path: None,
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
                verify_mode: VerifyMode::Compiler,
                artifact_dir: None,
                schema_path: None,
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
        let args = strings(&["mico", "--eda", "check", "input.mico"]);
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

    fn expected_cli(command: Command, path: &str, format: OutputFormat) -> CliArgs {
        CliArgs {
            command,
            path: path.to_string(),
            patch_path: None,
            format,
            repair_mode: RepairMode::DryRun,
            verify_mode: VerifyMode::Compiler,
            artifact_dir: None,
            schema_path: None,
        }
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
