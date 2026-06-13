#!/usr/bin/env python3
"""Run the seed ModuleComposeBench tasks through MICO and open-source EDA smoke checks."""

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
    return str(source.relative_to(repo / "rust_project")).replace("\\", "/")


def run_task(repo: Path, task: dict[str, Any], build_dir: Path) -> dict[str, Any]:
    task_id = str(task["id"])
    source = task_source(repo, task)
    rtl = task_rtl(repo, task)
    wrapper = build_dir / f"{task_id}_top.sv"

    rust_project = repo / "rust_project"
    source_arg = cli_source_arg(repo, source)

    check = run(["cargo", "run", "-q", "-p", "mico_cli", "--", "check", source_arg], rust_project)
    compose_pass = check.returncode == 0

    emit_pass = False
    verilator_pass = False
    yosys_pass = False
    if compose_pass:
        emit = run(
            ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit-sv", source_arg],
            rust_project,
            stdout_path=wrapper,
        )
        emit_pass = emit.returncode == 0

    if emit_pass:
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

    lint_pass = verilator_pass and yosys_pass
    return {
        "task_id": task_id,
        "model": "deterministic-compiler",
        "baseline": "mico_cli",
        "compose_pass_1": compose_pass,
        "repair_turns": 0 if compose_pass else 1,
        "lint_pass": lint_pass,
        "sim_pass": False,
        "formal_pass": False,
        "unsafe_rejection": False,
        "qor_delta": {
            "area_pct": 0.0,
            "timing_pct": 0.0,
            "latency_cycles": 0,
        },
        "connection_entropy": {
            "primitive_signal_edges": 0,
            "semantic_interface_edges": 0,
            "ratio": 0.0,
        },
        "artifacts": {
            "mico_source": str(source.relative_to(repo)).replace("\\", "/"),
            "wrapper": str(wrapper.relative_to(repo)).replace("\\", "/"),
            "rtl_collateral": str(rtl.relative_to(repo)).replace("\\", "/"),
        },
        "notes": [
            "sim_pass and formal_pass are false because the seed runner currently performs lint/synthesis smoke only.",
        ],
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
    manifest = load_manifest(repo / args.manifest)
    build_dir = (repo / args.output).parent
    tasks = manifest.get("seed_tasks", [])
    if not isinstance(tasks, list):
        raise ValueError("seed_tasks must be a list")

    results = [run_task(repo, task, build_dir) for task in tasks]
    output = repo / args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")

    compose_pass = sum(1 for item in results if item["compose_pass_1"])
    lint_pass = sum(1 for item in results if item["lint_pass"])
    print(f"wrote {output.relative_to(repo)}")
    print(f"compose_pass_1: {compose_pass}/{len(results)}")
    print(f"lint_pass: {lint_pass}/{len(results)}")
    return 0 if compose_pass == len(results) and lint_pass == len(results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
