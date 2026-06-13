use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Finding {
    level: Level,
    message: String,
}

#[derive(Debug, Clone, Copy)]
enum Level {
    Error,
    Warning,
    Note,
}

fn main() {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("current directory"));
    let root = root.canonicalize().unwrap_or(root);
    let rust_root = root.join("rust_project");

    let mut findings = Vec::new();
    require_file(&rust_root.join("Cargo.toml"), &mut findings);
    require_file(&rust_root.join("README.md"), &mut findings);

    for krate in ["mico_ir", "mico_frontend", "mico_codegen", "mico_cli"] {
        let crate_dir = rust_root.join("crates").join(krate);
        require_file(&crate_dir.join("Cargo.toml"), &mut findings);
        let has_lib = crate_dir.join("src").join("lib.rs").exists();
        let has_main = crate_dir.join("src").join("main.rs").exists();
        if !has_lib && !has_main {
            findings.push(error(format!(
                "crate `{}` has neither src/lib.rs nor src/main.rs",
                krate
            )));
        }
    }

    if let Ok(manifest) = fs::read_to_string(rust_root.join("Cargo.toml")) {
        require_contains(
            &manifest,
            "[workspace]",
            "workspace manifest",
            &mut findings,
        );
        require_contains(
            &manifest,
            "unsafe_code = \"forbid\"",
            "workspace unsafe_code policy",
            &mut findings,
        );
    }

    if let Ok(files) = rust_files(&rust_root) {
        for file in files {
            audit_rust_file(&file, &mut findings);
        }
    }

    println!("MICO Rust skill audit");
    println!("root: {}", root.display());
    if findings.is_empty() {
        println!("ok: no findings");
        return;
    }

    let mut error_count = 0usize;
    for finding in &findings {
        match finding.level {
            Level::Error => error_count += 1,
            Level::Warning | Level::Note => {}
        }
        println!("{:?}: {}", finding.level, finding.message);
    }

    if error_count > 0 {
        std::process::exit(1);
    }
}

fn require_file(path: &Path, findings: &mut Vec<Finding>) {
    if !path.exists() {
        findings.push(error(format!("missing required file `{}`", path.display())));
    }
}

fn require_contains(text: &str, needle: &str, label: &str, findings: &mut Vec<Finding>) {
    if !text.contains(needle) {
        findings.push(error(format!("missing {} marker `{}`", label, needle)));
    }
}

fn rust_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    visit(root, &mut out)?;
    Ok(out)
}

fn visit(path: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if path
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|name| matches!(name, "target" | ".git"))
    {
        return Ok(());
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            visit(&entry?.path(), out)?;
        }
    } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
        out.push(path.to_path_buf());
    }
    Ok(())
}

fn audit_rust_file(path: &Path, findings: &mut Vec<Finding>) {
    let Ok(text) = fs::read_to_string(path) else {
        findings.push(warning(format!("could not read `{}`", path.display())));
        return;
    };

    let production = production_region(&text);

    if production.contains("unsafe ") || production.contains("unsafe{") {
        findings.push(error(format!(
            "unsafe code appears in `{}`",
            path.display()
        )));
    }

    if !is_test_path(path) && production.contains(".unwrap()") {
        findings.push(warning(format!(
            "production file `{}` contains `.unwrap()`; prefer diagnostics or controlled CLI exits",
            path.display()
        )));
    }

    if !is_test_path(path) && production.contains(".expect(") {
        findings.push(note(format!(
            "production file `{}` contains `.expect(...)`; verify it protects an internal invariant",
            path.display()
        )));
    }
}

fn production_region(text: &str) -> &str {
    text.split_once("#[cfg(test)]")
        .map(|(production, _)| production)
        .unwrap_or(text)
}

fn is_test_path(path: &Path) -> bool {
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .is_some_and(|part| matches!(part, "tests" | "benches"))
    }) || path
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|name| name.ends_with("_test.rs"))
}

fn error(message: String) -> Finding {
    Finding {
        level: Level::Error,
        message,
    }
}

fn warning(message: String) -> Finding {
    Finding {
        level: Level::Warning,
        message,
    }
}

fn note(message: String) -> Finding {
    Finding {
        level: Level::Note,
        message,
    }
}
