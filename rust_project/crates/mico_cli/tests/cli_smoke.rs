use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("mico_cli crate should live under rust_project/crates")
        .to_path_buf()
}

fn mico(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mico"))
        .current_dir(workspace_root())
        .args(args)
        .output()
        .expect("mico command should run")
}

fn assert_success(args: &[&str]) -> Output {
    let output = mico(args);
    assert!(
        output.status.success(),
        "mico {args:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    output
}

fn assert_failure_contains(args: &[&str], expected: &str) {
    let output = mico(args);
    assert!(
        !output.status.success(),
        "mico {args:?} unexpectedly passed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains(expected),
        "mico {args:?} did not contain {expected:?}\noutput:\n{combined}",
    );
}

fn temp_dir(test_name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mico_cli_{test_name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create CLI smoke temp dir");
    dir
}

fn write(path: &Path, contents: &str) {
    fs::write(path, contents).expect("write CLI smoke fixture")
}

#[test]
fn smoke_covers_source_cli_commands() {
    let source = "examples/stream_fifo.mico";
    for args in [
        &["parse", source][..],
        &["check", source][..],
        &["build", source][..],
        &["dump-ir", source][..],
        &["emit-sv", source][..],
        &["emit-sva", source][..],
        &["emit-trace", source][..],
        &["--json", "verify", source][..],
        &["--json", "report", source][..],
    ] {
        assert_success(args);
    }
}

#[test]
fn smoke_covers_json_ast_cli_commands() {
    let temp = temp_dir("json_ast_commands");
    let ast_path = temp.join("width_adapter.ast.json");
    let ast = assert_success(&["dump-ast-json", "examples/width_adapter.mico"]);
    fs::write(&ast_path, ast.stdout).expect("write emitted AST JSON");
    let ast_arg = ast_path.to_str().expect("temp AST path should be UTF-8");

    for args in [
        &["check-json", ast_arg][..],
        &["build-json", ast_arg][..],
        &["dump-json-ir", ast_arg][..],
        &["emit-json-sv", ast_arg][..],
        &["emit-json-sva", ast_arg][..],
        &["emit-json-trace", ast_arg][..],
    ] {
        assert_success(args);
    }
}

#[test]
fn repair_json_smoke_and_negative_patches() {
    let temp = temp_dir("repair_json");
    let ast_path = temp.join("width_adapter.ast.json");
    let ast = assert_success(&["dump-ast-json", "examples/width_adapter.mico"]);
    fs::write(&ast_path, ast.stdout).expect("write emitted AST JSON");

    let valid_patch = temp.join("valid.patch.json");
    write(
        &valid_patch,
        r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "update_contract_attribute",
      "adapter": "Widen32To64",
      "value": "preserves_ready_valid"
    }
  ]
}
"#,
    );

    let ast_arg = ast_path.to_str().expect("temp AST path should be UTF-8");
    let valid_arg = valid_patch
        .to_str()
        .expect("temp patch path should be UTF-8");
    assert_success(&["--json", "repair-json", ast_arg, valid_arg]);

    let negative_patches = [
        (
            "empty.patch.json",
            r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": []
}
"#,
            "RepairPatchError",
        ),
        (
            "unknown-compose.patch.json",
            r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {"op": "remove_instance", "compose": "Missing", "name": "s"}
  ]
}
"#,
            "unknown compose",
        ),
        (
            "unknown-adapter.patch.json",
            r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "update_contract_attribute",
      "adapter": "MissingAdapter",
      "value": "preserves_ready_valid"
    }
  ]
}
"#,
            "unknown adapter",
        ),
        (
            "invalid-endpoint.patch.json",
            r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "change_endpoint",
      "compose": "Top",
      "from": {"instance": "s", "port": "tx"},
      "to": {"instance": "t", "port": "rx"},
      "side": "to",
      "endpoint": {"instance": "t", "port": "missing"}
    }
  ]
}
"#,
            "UnknownPort",
        ),
        (
            "post-check-fail.patch.json",
            r#"{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "replace_connection",
      "compose": "Top",
      "from": {"instance": "s", "port": "tx"},
      "to": {"instance": "t", "port": "rx"},
      "connection": {
        "from": {"instance": "s", "port": "tx"},
        "to": {"instance": "t", "port": "rx"},
        "adapter": null
      }
    }
  ]
}
"#,
            "InterfaceMismatch",
        ),
    ];

    for (name, contents, expected) in negative_patches {
        let patch = temp.join(name);
        write(&patch, contents);
        let patch_arg = patch.to_str().expect("temp patch path should be UTF-8");
        assert_failure_contains(&["--json", "repair-json", ast_arg, patch_arg], expected);
    }
}
