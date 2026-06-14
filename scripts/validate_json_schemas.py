#!/usr/bin/env python3
"""Validate MICO JSON artifacts against their repository schemas."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import yaml
from jsonschema import validators
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012


REPO_ROOT = Path(__file__).resolve().parents[1]

SCHEMA_FILES = {
    "bench_manifest": REPO_ROOT / "benchmarks" / "manifest_schema.json",
    "diagnostics": REPO_ROOT / "schemas" / "diagnostics.schema.json",
    "ast": REPO_ROOT / "schemas" / "mico_ast.schema.json",
    "ir": REPO_ROOT / "schemas" / "mico_ir.schema.json",
    "traceability": REPO_ROOT / "schemas" / "traceability.schema.json",
    "repair_patch": REPO_ROOT / "schemas" / "mico_repair_patch.schema.json",
    "bench": REPO_ROOT / "benchmarks" / "scoring_schema.json",
    "llm_run": REPO_ROOT / "schemas" / "llm_run.schema.json",
    "llm_bench": REPO_ROOT / "schemas" / "llm_bench.schema.json",
    "aggregate": REPO_ROOT / "schemas" / "aggregate_results.schema.json",
}


def repo_path(path: str | Path) -> Path:
    value = Path(path)
    if value.is_absolute():
        return value
    return REPO_ROOT / value


def display(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def load_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return yaml.safe_load(fh)


def load_schemas() -> Registry:
    registry = Registry()
    for path in SCHEMA_FILES.values():
        schema = load_json(path)
        resource = Resource.from_contents(schema, default_specification=DRAFT202012)
        schema_id = schema.get("$id")
        if isinstance(schema_id, str):
            registry = registry.with_resource(schema_id, resource)
        file_uri = path.resolve().as_uri()
        registry = registry.with_resource(file_uri, resource)
    return registry


def make_validator(schema: dict[str, Any], registry: Registry) -> validators.Validator:
    validator_cls = validators.validator_for(schema)
    validator_cls.check_schema(schema)
    return validator_cls(schema, registry=registry)


def validate_instance(
    name: str,
    schema_name: str,
    artifact: Path,
    registry: Registry,
) -> None:
    schema = load_json(SCHEMA_FILES[schema_name])
    instance = load_json(artifact)
    validator = make_validator(schema, registry)
    errors = sorted(validator.iter_errors(instance), key=lambda err: list(err.path))
    if errors:
        print(f"ERROR: {name} failed {schema_name} schema: {display(artifact)}", file=sys.stderr)
        for error in errors[:20]:
            location = "/".join(str(part) for part in error.path) or "<root>"
            print(f"  - {location}: {error.message}", file=sys.stderr)
        if len(errors) > 20:
            print(f"  ... {len(errors) - 20} more errors", file=sys.stderr)
        raise SystemExit(1)
    print(f"OK {name}: {display(artifact)}")


def validate_yaml_instance(
    name: str,
    schema_name: str,
    artifact: Path,
    registry: Registry,
) -> None:
    schema = load_json(SCHEMA_FILES[schema_name])
    instance = load_yaml(artifact)
    validator = make_validator(schema, registry)
    errors = sorted(validator.iter_errors(instance), key=lambda err: list(err.path))
    if errors:
        print(f"ERROR: {name} failed {schema_name} schema: {display(artifact)}", file=sys.stderr)
        for error in errors[:20]:
            location = "/".join(str(part) for part in error.path) or "<root>"
            print(f"  - {location}: {error.message}", file=sys.stderr)
        if len(errors) > 20:
            print(f"  ... {len(errors) - 20} more errors", file=sys.stderr)
        raise SystemExit(1)
    print(f"OK {name}: {display(artifact)}")


def run_capture(cmd: list[str], cwd: Path, stdout_path: Path) -> None:
    stdout_path.parent.mkdir(parents=True, exist_ok=True)
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        print(f"ERROR: command failed in {display(cwd)}: {' '.join(cmd)}", file=sys.stderr)
        if proc.stdout:
            print(proc.stdout, file=sys.stderr, end="")
        if proc.stderr:
            print(proc.stderr, file=sys.stderr, end="")
        raise SystemExit(proc.returncode)
    stdout_path.write_text(proc.stdout, encoding="utf-8")


def run_step(cmd: list[str], cwd: Path) -> None:
    proc = subprocess.run(cmd, cwd=cwd, text=True, check=False)
    if proc.returncode != 0:
        print(f"ERROR: command failed in {display(cwd)}: {' '.join(cmd)}", file=sys.stderr)
        raise SystemExit(proc.returncode)


def write_repair_patch_sample(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": "mico.repair_patch.v0",
        "kind": "repair_patch",
        "operations": [
            {
                "op": "update_contract_attribute",
                "adapter": "widen",
                "value": "preserves_ready_valid",
            }
        ],
    }
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def generate_smoke_records(output_dir: Path, llm_config: Path) -> dict[str, list[Path]]:
    rust_project = REPO_ROOT / "rust_project"
    output_dir.mkdir(parents=True, exist_ok=True)

    diagnostics = output_dir / "diagnostics.check.json"
    ast = output_dir / "stream_fifo.ast.json"
    ir = output_dir / "stream_fifo.ir.json"
    trace = output_dir / "stream_fifo.trace.json"
    repair_patch = output_dir / "repair_patch.sample.json"
    llm_run = output_dir / "llm_provider_validate.json"
    llm_bench = output_dir / "llm_bench_validate.json"

    run_capture(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mico_cli",
            "--",
            "check",
            "--format",
            "json",
            "examples/stream_fifo.mico",
        ],
        rust_project,
        diagnostics,
    )
    run_capture(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "dump-ast-json", "examples/stream_fifo.mico"],
        rust_project,
        ast,
    )
    run_capture(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "dump-ir", "examples/stream_fifo.mico"],
        rust_project,
        ir,
    )
    run_capture(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit", "trace", "examples/stream_fifo.mico"],
        rust_project,
        trace,
    )
    write_repair_patch_sample(repair_patch)

    run_step(
        [
            "python3",
            "scripts/llm-provider-smoke.py",
            "--config",
            display(llm_config),
            "--profile",
            "smoke",
            "--validate-only",
            "--output",
            display(llm_run),
        ],
        REPO_ROOT,
    )
    run_step(
        [
            "python3",
            "scripts/run_llm_bench.py",
            "--config",
            display(llm_config),
            "--profiles",
            "smoke",
            "--baselines",
            "mico_source",
            "--task-id",
            "T004_direct_stream",
            "--output",
            display(llm_bench),
        ],
        REPO_ROOT,
    )

    return {
        "diagnostics": [diagnostics],
        "ast": [ast],
        "ir": [ir],
        "traceability": [trace],
        "repair_patch": [repair_patch],
        "llm_run": [llm_run],
        "llm_bench": [llm_bench],
    }


def append_paths(target: dict[str, list[Path]], key: str, values: list[str]) -> None:
    target.setdefault(key, [])
    target[key].extend(repo_path(value) for value in values)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output-dir",
        default="build/schema-validation",
        help="Ignored directory for generated smoke artifacts.",
    )
    parser.add_argument(
        "--llm-config",
        default="config/llm-provider.example.yaml",
        help="Config used only for validate-only generated LLM schema records.",
    )
    parser.add_argument("--no-generate-smoke", action="store_true")
    parser.add_argument("--bench-result", action="append", default=[])
    parser.add_argument(
        "--bench-manifest",
        default="benchmarks/module_compose_bench_manifest.yaml",
        help="ModuleComposeBench YAML manifest to validate against benchmarks/manifest_schema.json.",
    )
    parser.add_argument("--llm-run", action="append", default=[])
    parser.add_argument("--llm-bench", action="append", default=[])
    parser.add_argument("--aggregate-result", action="append", default=[])
    args = parser.parse_args()

    registry = load_schemas()
    for name, schema_path in SCHEMA_FILES.items():
        make_validator(load_json(schema_path), registry)
        print(f"OK schema: {name} ({display(schema_path)})")

    artifacts: dict[str, list[Path]] = {}
    if not args.no_generate_smoke:
        generated = generate_smoke_records(repo_path(args.output_dir), repo_path(args.llm_config))
        for key, values in generated.items():
            artifacts.setdefault(key, []).extend(values)

    if args.bench_manifest:
        manifest_path = repo_path(args.bench_manifest)
        if not manifest_path.exists():
            print(f"ERROR: missing bench manifest: {display(manifest_path)}", file=sys.stderr)
            return 1
        validate_yaml_instance("bench_manifest", "bench_manifest", manifest_path, registry)

    append_paths(artifacts, "bench", args.bench_result)
    append_paths(artifacts, "llm_run", args.llm_run)
    append_paths(artifacts, "llm_bench", args.llm_bench)
    append_paths(artifacts, "aggregate", args.aggregate_result)

    for schema_name, paths in artifacts.items():
        for path in paths:
            if not path.exists():
                print(f"ERROR: missing artifact for {schema_name}: {display(path)}", file=sys.stderr)
                return 1
            validate_instance(schema_name, schema_name, path, registry)

    print("MICO JSON schema validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
