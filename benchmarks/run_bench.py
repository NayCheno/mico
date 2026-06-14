#!/usr/bin/env python3
"""Run ModuleComposeBench tasks through MICO and open-source EDA smoke checks."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import yaml


def run(cmd: list[str], cwd: Path, stdout_path: Path | None = None) -> subprocess.CompletedProcess[str]:
    stdout = subprocess.PIPE
    handle = None
    if stdout_path is not None:
        stdout_path.parent.mkdir(parents=True, exist_ok=True)
        handle = stdout_path.open("w", encoding="utf-8")
        stdout = handle
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            stdout=stdout,
            stderr=subprocess.PIPE,
            text=True,
            check=False,
        )
    finally:
        if handle is not None:
            handle.close()
    if result.returncode != 0 and result.stderr:
        print(result.stderr, file=sys.stderr, end="")
    return result


def load_manifest(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError("benchmark manifest must be a YAML mapping")
    return data


def manifest_tasks(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    tasks = manifest.get("tasks")
    if isinstance(tasks, list):
        return tasks

    # Backward compatibility for older manifests.
    seed_tasks = manifest.get("seed_tasks", [])
    if isinstance(seed_tasks, list) and all(isinstance(task, dict) for task in seed_tasks):
        return seed_tasks
    raise ValueError("manifest must contain a tasks list")


def task_source(repo: Path, task: dict[str, Any]) -> Path:
    source = task.get("mico_source")
    if not isinstance(source, str) or not source:
        raise ValueError(f"task {task.get('id', '<unknown>')} is missing mico_source")
    return repo / source


def task_rtl(repo: Path, task: dict[str, Any]) -> Path:
    collateral = task.get("rtl_collateral", "rtl/examples/mico_example_leafs.sv")
    if not isinstance(collateral, str) or not collateral:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid rtl_collateral")
    return repo / collateral


def cli_source_arg(repo: Path, source: Path) -> str:
    rust_project = repo / "rust_project"
    try:
        return str(source.relative_to(rust_project)).replace("\\", "/")
    except ValueError:
        return str(source)


def expected_compose_pass(task: dict[str, Any]) -> bool:
    expected = task.get("expected", {})
    if isinstance(expected, dict) and "compose_pass" in expected:
        return bool(expected["compose_pass"])
    return task.get("type", "positive") != "negative"


def expected_diagnostics(task: dict[str, Any]) -> list[str]:
    expected = task.get("expected", {})
    diagnostics = expected.get("diagnostics", []) if isinstance(expected, dict) else []
    if not isinstance(diagnostics, list):
        return []
    return [str(item) for item in diagnostics]


def parse_json_stdout(result: subprocess.CompletedProcess[str]) -> dict[str, Any]:
    if not result.stdout:
        return {}
    try:
        parsed = json.loads(result.stdout)
    except json.JSONDecodeError:
        return {}
    return parsed if isinstance(parsed, dict) else {}


def diagnostic_codes(diagnostics_json: dict[str, Any]) -> list[str]:
    diagnostics = diagnostics_json.get("diagnostics", [])
    if not isinstance(diagnostics, list):
        return []
    codes = []
    for diagnostic in diagnostics:
        if isinstance(diagnostic, dict) and isinstance(diagnostic.get("code"), str):
            codes.append(diagnostic["code"])
    return codes


def run_task(repo: Path, task: dict[str, Any], build_dir: Path) -> dict[str, Any]:
    task_id = str(task["id"])
    task_type = str(task.get("type", "positive"))
    source = task_source(repo, task)
    rtl = task_rtl(repo, task)
    task_build_dir = build_dir / task_id
    wrapper = task_build_dir / "top.sv"
    sva = task_build_dir / "top_sva.sv"
    trace = task_build_dir / "traceability.json"
    ast_json = task_build_dir / "ast.json"
    dsl_ir = task_build_dir / "typed_ir_dsl.json"
    json_ir = task_build_dir / "typed_ir_json.json"
    vvp = task_build_dir / "top.vvp"

    rust_project = repo / "rust_project"
    source_arg = cli_source_arg(repo, source)
    expected_accept = expected_compose_pass(task)

    check = run(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "check", "--format", "json", source_arg],
        rust_project,
    )
    check_json = parse_json_stdout(check)
    codes = diagnostic_codes(check_json)
    compose_pass = check.returncode == 0
    expected_codes = expected_diagnostics(task)
    expected_diagnostic_match = all(code in codes for code in expected_codes)
    expected_outcome_pass = compose_pass == expected_accept and (
        expected_accept or expected_diagnostic_match
    )
    unsafe_rejection = task_type == "negative" and not compose_pass

    dump_ast = run(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "dump-ast-json", source_arg],
        rust_project,
        stdout_path=ast_json,
    )
    dump_ast_pass = dump_ast.returncode == 0
    json_ast_check_pass = False
    json_ast_expected_diagnostic_match = False
    json_ast_expected_outcome_pass = False
    json_ast_codes: list[str] = []
    typed_ir_match = False
    if dump_ast_pass:
        check_json = run(
            [
                "cargo",
                "run",
                "-q",
                "-p",
                "mico_cli",
                "--",
                "check-json",
                "--format",
                "json",
                str(ast_json),
            ],
            rust_project,
        )
        check_json_payload = parse_json_stdout(check_json)
        json_ast_codes = diagnostic_codes(check_json_payload)
        json_ast_check_pass = check_json.returncode == 0
        json_ast_expected_diagnostic_match = all(code in json_ast_codes for code in expected_codes)
        json_ast_expected_outcome_pass = json_ast_check_pass == expected_accept and (
            expected_accept or json_ast_expected_diagnostic_match
        )

        if expected_accept and compose_pass and json_ast_check_pass:
            dump_dsl_ir = run(
                ["cargo", "run", "-q", "-p", "mico_cli", "--", "dump-ir", source_arg],
                rust_project,
                stdout_path=dsl_ir,
            )
            dump_json_ir = run(
                ["cargo", "run", "-q", "-p", "mico_cli", "--", "dump-json-ir", str(ast_json)],
                rust_project,
                stdout_path=json_ir,
            )
            typed_ir_match = (
                dump_dsl_ir.returncode == 0
                and dump_json_ir.returncode == 0
                and dsl_ir.read_text(encoding="utf-8") == json_ir.read_text(encoding="utf-8")
            )

    emit_sv_pass = False
    emit_sva_pass = False
    emit_trace_pass = False
    verilator_pass = False
    sva_lint_pass = False
    iverilog_pass = False
    yosys_pass = False
    if compose_pass and expected_accept:
        emit_sv = run(
            ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit-sv", source_arg],
            rust_project,
            stdout_path=wrapper,
        )
        emit_sv_pass = emit_sv.returncode == 0
        emit_sva = run(
            ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit-sva", source_arg],
            rust_project,
            stdout_path=sva,
        )
        emit_sva_pass = emit_sva.returncode == 0
        emit_trace = run(
            ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit", "trace", source_arg],
            rust_project,
            stdout_path=trace,
        )
        emit_trace_pass = emit_trace.returncode == 0

    if emit_sv_pass:
        verilator = run(
            [
                "verilator",
                "--lint-only",
                "-Wall",
                "-Wno-DECLFILENAME",
                "-Wno-UNUSEDSIGNAL",
                "--top-module",
                "Top",
                str(rtl),
                str(wrapper),
            ],
            repo,
        )
        verilator_pass = verilator.returncode == 0

        iverilog = run(
            ["iverilog", "-g2012", "-s", "Top", "-o", str(vvp), str(rtl), str(wrapper)],
            repo,
        )
        iverilog_pass = iverilog.returncode == 0

        yosys = run(
            [
                "yosys",
                "-q",
                "-p",
                f"read_verilog -sv {rtl} {wrapper}; hierarchy -check -top Top; proc; opt; stat",
            ],
            repo,
        )
        yosys_pass = yosys.returncode == 0

    if emit_sva_pass:
        sva_lint = run(
            [
                "verilator",
                "--lint-only",
                "-Wall",
                "-Wno-DECLFILENAME",
                "-Wno-UNUSEDSIGNAL",
                "--top-module",
                "mico_sva_Top",
                str(sva),
            ],
            repo,
        )
        sva_lint_pass = sva_lint.returncode == 0

    lint_pass = verilator_pass and sva_lint_pass and iverilog_pass and yosys_pass
    return {
        "task_id": task_id,
        "level": str(task.get("level", "")),
        "type": task_type,
        "model": "deterministic-compiler",
        "baseline": "mico_cli",
        "expected_compose_pass": expected_accept,
        "expected_outcome_pass": expected_outcome_pass,
        "compose_pass_1": compose_pass,
        "compiler_result": {
            "exit_code": check.returncode,
            "diagnostic_codes": codes,
            "expected_diagnostics": expected_codes,
            "expected_diagnostic_match": expected_diagnostic_match,
        },
        "json_ast_result": {
            "dump_ast_pass": dump_ast_pass,
            "check_pass": json_ast_check_pass,
            "expected_outcome_pass": json_ast_expected_outcome_pass,
            "diagnostic_codes": json_ast_codes,
            "expected_diagnostics": expected_codes,
            "expected_diagnostic_match": json_ast_expected_diagnostic_match,
            "typed_ir_match": typed_ir_match,
        },
        "repair_turns": 0,
        "emit_sv_pass": emit_sv_pass,
        "emit_sva_pass": emit_sva_pass,
        "emit_trace_pass": emit_trace_pass,
        "lint_pass": lint_pass,
        "eda": {
            "verilator_lint_pass": verilator_pass,
            "sva_lint_pass": sva_lint_pass,
            "iverilog_elab_pass": iverilog_pass,
            "yosys_elab_pass": yosys_pass,
        },
        "sim_pass": False,
        "sim_status": "not_run",
        "formal_pass": False,
        "formal_status": "not_run",
        "unsafe_rejection": unsafe_rejection,
        "qor_delta": {
            "area_pct": 0.0,
            "timing_pct": 0.0,
            "latency_cycles": 0,
        },
        "qor": {
            "available": False,
            "source": "not_run",
            "area_cells": None,
            "timing_ns": None,
        },
        "connection_entropy": {
            "primitive_signal_edges": 0,
            "semantic_interface_edges": 0,
            "ratio": 0.0,
        },
        "artifacts": {
            "mico_source": str(source.relative_to(repo)).replace("\\", "/"),
            "wrapper": str(wrapper.relative_to(repo)).replace("\\", "/"),
            "sva": str(sva.relative_to(repo)).replace("\\", "/"),
            "traceability": str(trace.relative_to(repo)).replace("\\", "/"),
            "ast_json": str(ast_json.relative_to(repo)).replace("\\", "/"),
            "typed_ir_dsl": str(dsl_ir.relative_to(repo)).replace("\\", "/"),
            "typed_ir_json": str(json_ir.relative_to(repo)).replace("\\", "/"),
            "rtl_collateral": str(rtl.relative_to(repo)).replace("\\", "/"),
        },
        "notes": [
            "sim_pass and formal_pass are false because per-task simulation and formal harnesses are not implemented yet.",
            "qor is not available until a synthesis QoR parser is added.",
        ],
    }


def aggregate_results(results: list[dict[str, Any]]) -> dict[str, Any]:
    total = len(results)
    positives = [item for item in results if item["expected_compose_pass"]]
    negatives = [item for item in results if not item["expected_compose_pass"]]

    def count(items: list[dict[str, Any]], key: str) -> int:
        return sum(1 for item in items if item.get(key) is True)

    expected_outcomes = count(results, "expected_outcome_pass")
    positive_lint = count(positives, "lint_pass")
    unsafe_rejections = count(negatives, "unsafe_rejection")
    json_ast_expected = sum(
        1 for item in results if item["json_ast_result"]["expected_outcome_pass"] is True
    )
    return {
        "total_tasks": total,
        "positive_tasks": len(positives),
        "negative_tasks": len(negatives),
        "expected_outcome_pass": {
            "passed": expected_outcomes,
            "total": total,
            "rate": expected_outcomes / total if total else 0.0,
        },
        "compose_pass_1": {
            "passed": count(positives, "compose_pass_1"),
            "total": len(positives),
            "rate": count(positives, "compose_pass_1") / len(positives) if positives else 0.0,
        },
        "lint_pass": {
            "passed": positive_lint,
            "total": len(positives),
            "rate": positive_lint / len(positives) if positives else 0.0,
        },
        "unsafe_rejection": {
            "passed": unsafe_rejections,
            "total": len(negatives),
            "rate": unsafe_rejections / len(negatives) if negatives else 0.0,
        },
        "json_ast_path": {
            "passed": json_ast_expected,
            "total": total,
            "rate": json_ast_expected / total if total else 0.0,
        },
        "sim_pass": {
            "passed": count(positives, "sim_pass"),
            "total": len(positives),
            "rate": 0.0,
            "status": "not_run",
        },
        "formal_pass": {
            "passed": count(positives, "formal_pass"),
            "total": len(positives),
            "rate": 0.0,
            "status": "not_run",
        },
        "qor": {
            "available_tasks": sum(1 for item in results if item["qor"]["available"]),
            "status": "not_run",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--manifest",
        default="benchmarks/module_compose_bench_manifest.yaml",
        help="ModuleComposeBench manifest",
    )
    parser.add_argument(
        "--output",
        default="build/bench/seed_results.json",
        help="JSON result path",
    )
    args = parser.parse_args()

    repo = Path(__file__).resolve().parents[1]
    manifest_path = repo / args.manifest
    manifest = load_manifest(manifest_path)
    build_dir = (repo / args.output).parent
    tasks = manifest_tasks(manifest)

    results = [run_task(repo, task, build_dir) for task in tasks]
    summary = aggregate_results(results)
    output_payload = {
        "schema_version": "mico.bench.results.v0",
        "benchmark": {
            "name": manifest.get("name", "ModuleComposeBench"),
            "version": manifest.get("version", "0.0.0"),
            "manifest": str(manifest_path.relative_to(repo)).replace("\\", "/"),
        },
        "summary": summary,
        "results": results,
    }

    output = repo / args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(output_payload, indent=2) + "\n", encoding="utf-8")

    print(f"wrote {output.relative_to(repo)}")
    print(
        "expected_outcome_pass: "
        f"{summary['expected_outcome_pass']['passed']}/{summary['expected_outcome_pass']['total']}"
    )
    print(f"compose_pass_1: {summary['compose_pass_1']['passed']}/{summary['compose_pass_1']['total']}")
    print(f"lint_pass: {summary['lint_pass']['passed']}/{summary['lint_pass']['total']}")
    print(
        "unsafe_rejection: "
        f"{summary['unsafe_rejection']['passed']}/{summary['unsafe_rejection']['total']}"
    )
    print(
        "json_ast_path: "
        f"{summary['json_ast_path']['passed']}/{summary['json_ast_path']['total']}"
    )

    return (
        0
        if summary["expected_outcome_pass"]["passed"] == summary["expected_outcome_pass"]["total"]
        and summary["lint_pass"]["passed"] == summary["lint_pass"]["total"]
        else 1
    )


if __name__ == "__main__":
    raise SystemExit(main())
