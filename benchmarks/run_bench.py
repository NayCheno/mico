#!/usr/bin/env python3
"""Run ModuleComposeBench tasks through MICO and open-source EDA smoke checks."""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

import yaml


def run(
    cmd: list[str],
    cwd: Path,
    stdout_path: Path | None = None,
    stderr_path: Path | None = None,
) -> subprocess.CompletedProcess[str]:
    stdout = subprocess.PIPE
    stderr = subprocess.PIPE
    stdout_handle = None
    stderr_handle = None
    if stdout_path is not None:
        stdout_path.parent.mkdir(parents=True, exist_ok=True)
        stdout_handle = stdout_path.open("w", encoding="utf-8")
        stdout = stdout_handle
    if stderr_path is not None:
        stderr_path.parent.mkdir(parents=True, exist_ok=True)
        stderr_handle = stderr_path.open("w", encoding="utf-8")
        stderr = stderr_handle
    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            stdout=stdout,
            stderr=stderr,
            text=True,
            check=False,
        )
    finally:
        if stdout_handle is not None:
            stdout_handle.close()
        if stderr_handle is not None:
            stderr_handle.close()
    if result.returncode != 0 and result.stderr and stderr_path is None:
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
        validated = [validate_task_metadata(task) for task in tasks]
        minimum_tasks = manifest.get("minimum_tasks")
        if isinstance(minimum_tasks, int) and len(validated) < minimum_tasks:
            raise ValueError(
                f"manifest declares minimum_tasks={minimum_tasks}, found {len(validated)}"
            )
        return validated

    # Backward compatibility for older manifests.
    seed_tasks = manifest.get("seed_tasks", [])
    if isinstance(seed_tasks, list) and all(isinstance(task, dict) for task in seed_tasks):
        return [validate_task_metadata(task) for task in seed_tasks]
    raise ValueError("manifest must contain a tasks list")


def validate_task_metadata(task: Any) -> dict[str, Any]:
    if not isinstance(task, dict):
        raise ValueError("benchmark task must be a YAML mapping")
    task_id = task.get("id", "<unknown>")
    required_strings = ["id", "level", "type", "path", "request", "mico_source", "rtl_collateral"]
    for key in required_strings:
        value = task.get(key)
        if not isinstance(value, str) or not value:
            raise ValueError(f"task {task_id} is missing non-empty string field {key}")

    if task["level"] not in {"L1", "L2", "L3", "L4", "L5", "L6"}:
        raise ValueError(f"task {task_id} has invalid level {task['level']}")
    if task["type"] not in {"positive", "negative"}:
        raise ValueError(f"task {task_id} has invalid type {task['type']}")

    for key in ["module_inventory", "interface_inventory", "adapter_inventory"]:
        value = task.get(key)
        if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
            raise ValueError(f"task {task_id} must define {key} as a string list")

    expected = task.get("expected")
    if not isinstance(expected, dict):
        raise ValueError(f"task {task_id} is missing expected metadata")
    for key in ["compose_pass", "lint_pass", "diagnostics"]:
        if key not in expected:
            raise ValueError(f"task {task_id} expected metadata is missing {key}")
    if not isinstance(expected["compose_pass"], bool):
        raise ValueError(f"task {task_id} expected.compose_pass must be boolean")
    if not isinstance(expected["lint_pass"], bool):
        raise ValueError(f"task {task_id} expected.lint_pass must be boolean")
    diagnostics = expected["diagnostics"]
    if not isinstance(diagnostics, list) or not all(
        isinstance(item, str) for item in diagnostics
    ):
        raise ValueError(f"task {task_id} expected.diagnostics must be a string list")
    return task


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


def task_sim_testbench(repo: Path, task: dict[str, Any]) -> Path | None:
    testbench = task.get("sim_testbench")
    if testbench is None:
        return None
    if not isinstance(testbench, str) or not testbench:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid sim_testbench")
    return repo / testbench


def task_sim_top(task: dict[str, Any]) -> str | None:
    sim_top = task.get("sim_top")
    if sim_top is None:
        return None
    if not isinstance(sim_top, str) or not sim_top:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid sim_top")
    return sim_top


def task_formal_harness(repo: Path, task: dict[str, Any]) -> Path | None:
    harness = task.get("formal_harness")
    if harness is None:
        return None
    if not isinstance(harness, str) or not harness:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid formal_harness")
    return repo / harness


def task_formal_top(task: dict[str, Any]) -> str | None:
    formal_top = task.get("formal_top")
    if formal_top is None:
        return None
    if not isinstance(formal_top, str) or not formal_top:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid formal_top")
    return formal_top


def task_formal_depth(task: dict[str, Any]) -> int:
    depth = task.get("formal_depth", 4)
    if not isinstance(depth, int) or depth <= 0:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid formal_depth")
    return depth


def write_sby_job(
    sby_path: Path,
    rtl: Path,
    wrapper: Path,
    harness: Path,
    formal_top: str,
    depth: int,
) -> None:
    files = [rtl, wrapper, harness]
    read_files = " ".join(path.name for path in files)
    listed_files = "\n".join(path.as_posix() for path in files)
    sby_path.write_text(
        f"""[options]
mode prove
depth {depth}

[engines]
smtbmc z3

[script]
read -formal -sv {read_files}
prep -top {formal_top}

[files]
{listed_files}
""",
        encoding="utf-8",
    )


def sv_safe_name(value: str) -> str:
    normalized = [ch if ch.isalnum() or ch == "_" else "_" for ch in value]
    out = "".join(normalized)
    if not out or out[0].isdigit():
        out = f"_{out}"
    return out


def is_reset_port(name: str) -> bool:
    lowered = name.lower()
    return "rst" in lowered or "reset" in lowered


def reset_active_value(name: str) -> str:
    lowered = name.lower()
    return "1'b0" if lowered.endswith("_n") or lowered in {"rstn", "resetn"} else "1'b1"


def reset_inactive_value(name: str) -> str:
    return "1'b1" if reset_active_value(name) == "1'b0" else "1'b0"


def trace_signals(trace: dict[str, Any]) -> list[dict[str, Any]]:
    signals: dict[str, dict[str, Any]] = {}
    for compose in trace.get("composes", []):
        if not isinstance(compose, dict):
            continue
        for connection in compose.get("connections", []):
            if not isinstance(connection, dict):
                continue
            for binding in connection.get("field_bindings", []):
                if not isinstance(binding, dict):
                    continue
                signal = binding.get("signal")
                if isinstance(signal, str) and signal:
                    signals.setdefault(signal, binding)
    return [signals[name] for name in sorted(signals)]


def trace_first_compose(trace: dict[str, Any]) -> dict[str, Any]:
    composes = trace.get("composes", [])
    compose = composes[0] if composes and isinstance(composes[0], dict) else {}
    return compose


def trace_clock_reset_ports(trace: dict[str, Any]) -> tuple[list[str], list[str]]:
    compose = trace_first_compose(trace)
    ports = [str(port) for port in compose.get("clock_reset_ports", []) if isinstance(port, str)]
    clocks = [port for port in ports if not is_reset_port(port)]
    resets = [port for port in ports if is_reset_port(port)]
    return clocks, resets


def sv_width_decl(width_bits: Any) -> str:
    width = int(width_bits) if isinstance(width_bits, int) and width_bits > 0 else 1
    return f"[{width - 1}:0] " if width > 1 else ""


def sv_expr_list(signals: list[str]) -> str:
    if len(signals) == 1:
        return signals[0]
    return "{" + ", ".join(signals) + "}"


def ready_valid_properties(compose: dict[str, Any]) -> list[dict[str, str]]:
    properties: list[dict[str, str]] = []
    for item in compose.get("sva_properties", []):
        if not isinstance(item, dict):
            continue
        if item.get("kind") != "ready_valid_stable_payload":
            continue
        payload = item.get("payload")
        valid = item.get("valid")
        ready = item.get("ready")
        if isinstance(payload, str) and isinstance(valid, str) and isinstance(ready, str):
            properties.append({"payload": payload, "valid": valid, "ready": ready})
    return properties


def write_autogen_sim_testbench(tb_path: Path, task_id: str, trace_path: Path) -> str:
    trace = json.loads(trace_path.read_text(encoding="utf-8"))
    compose = trace_first_compose(trace)
    ports = [str(port) for port in compose.get("clock_reset_ports", []) if isinstance(port, str)]
    clocks = [port for port in ports if not is_reset_port(port)]
    resets = [port for port in ports if is_reset_port(port)]
    if not clocks:
        raise ValueError(f"trace for {task_id} does not list a clock port")
    signals = trace_signals(trace)
    if not signals:
        raise ValueError(f"trace for {task_id} does not contain field bindings")

    top = f"tb_{sv_safe_name(task_id).lower()}_autogen"
    lines = [
        "`timescale 1ns/1ps",
        "",
        f"module {top};",
    ]
    for clock in clocks:
        lines.append(f"  logic {clock} = 1'b0;")
    for reset in resets:
        lines.append(f"  logic {reset} = {reset_active_value(reset)};")
    lines.extend(["", "  Top dut ("])
    for index, port in enumerate(ports):
        comma = "," if index + 1 < len(ports) else ""
        lines.append(f"    .{port}({port}){comma}")
    lines.extend(["  );", ""])
    for index, clock in enumerate(clocks):
        period = 5 + (index * 2)
        lines.append(f"  always #{period} {clock} = ~{clock};")
    lines.extend(
        [
            "",
            "  initial begin",
            f"    repeat (2) @(posedge {clocks[0]});",
        ]
    )
    for reset in resets:
        lines.append(f"    {reset} = {reset_inactive_value(reset)};")
    lines.extend(
        [
            f"    repeat (4) @(posedge {clocks[0]});",
            "    #1;",
            "",
        ]
    )
    for binding in signals:
        signal = str(binding["signal"])
        field = str(binding.get("field", ""))
        lines.append(f"    if (^dut.{signal} === 1'bx) begin")
        lines.append(f"      $fatal(1, \"autogen sim saw X on {signal}\");")
        lines.append("    end")
        if field in {"valid", "ready"}:
            lines.append(f"    if (dut.{signal} !== 1'b1) begin")
            lines.append(f"      $fatal(1, \"autogen sim expected {signal} asserted\");")
            lines.append("    end")
    lines.extend(
        [
            "",
            f"    $display(\"SIM PASS {task_id} autogen\");",
            "    $finish;",
            "  end",
            "endmodule",
            "",
        ]
    )
    tb_path.parent.mkdir(parents=True, exist_ok=True)
    tb_path.write_text("\n".join(lines), encoding="utf-8")
    return top


def write_autogen_formal_harness(harness_path: Path, task_id: str, trace_path: Path) -> str:
    trace = json.loads(trace_path.read_text(encoding="utf-8"))
    compose = trace_first_compose(trace)
    clocks, resets = trace_clock_reset_ports(trace)
    if len(clocks) != 1:
        raise ValueError("auto formal requires exactly one clock domain")
    if len(resets) > 1:
        raise ValueError("auto formal requires at most one reset")
    signals = trace_signals(trace)
    if not signals:
        raise ValueError(f"trace for {task_id} does not contain field bindings")
    valid_ready_signals = [
        str(binding["signal"])
        for binding in signals
        if str(binding.get("field", "")) in {"valid", "ready"}
    ]
    if not valid_ready_signals:
        raise ValueError("auto formal requires ready/valid field bindings")

    clock = clocks[0]
    reset = resets[0] if resets else None
    monitor = f"tb_{sv_safe_name(task_id).lower()}_formal_autogen"
    reset_inactive = reset_inactive_value(reset) if reset is not None else None
    reset_guard = f"{reset} == {reset_inactive}" if reset is not None else "1'b1"
    reset_past_guard = reset_guard
    properties = ready_valid_properties(compose)

    port_decls = [f"input wire {clock}"]
    if reset is not None:
        port_decls.append(f"input wire {reset}")
    for binding in signals:
        signal = str(binding["signal"])
        port_decls.append(f"input wire {sv_width_decl(binding.get('width_bits'))}{signal}")

    lines = [
        "`default_nettype none",
        "",
        f"module {monitor} (",
    ]
    for index, decl in enumerate(port_decls):
        comma = "," if index + 1 < len(port_decls) else ""
        lines.append(f"  {decl}{comma}")
    lines.extend(
        [
            ");",
            "  reg past_valid = 1'b0;",
            "",
            f"  always @(posedge {clock}) begin",
        ]
    )
    if reset is not None:
        lines.extend(
            [
                "    if (!past_valid) begin",
                f"      assume ({reset} == {reset_active_value(reset)});",
                "    end else begin",
                f"      assume ({reset} == {reset_inactive});",
                "    end",
            ]
        )
    lines.extend(
        [
            "    past_valid <= 1'b1;",
            "",
            f"    assert (!$isunknown({sv_expr_list([str(binding['signal']) for binding in signals])}));",
            "",
            f"    if (past_valid && {reset_guard}) begin",
        ]
    )
    for signal in valid_ready_signals:
        lines.append(f"      assert ({signal} == 1'b1);")
    lines.append("    end")

    for prop in properties:
        lines.extend(
            [
                (
                    "    if (past_valid && "
                    f"$past({reset_past_guard} && {prop['valid']} && !{prop['ready']})) begin"
                ),
                f"      assert ({prop['payload']} == $past({prop['payload']}));",
                "    end",
            ]
        )

    bind_ports = [clock]
    if reset is not None:
        bind_ports.append(reset)
    bind_ports.extend(str(binding["signal"]) for binding in signals)

    lines.extend(
        [
            "  end",
            "endmodule",
            "",
            f"bind Top {monitor} mico_{sv_safe_name(task_id).lower()}_formal_autogen (",
        ]
    )
    for index, port in enumerate(bind_ports):
        comma = "," if index + 1 < len(bind_ports) else ""
        lines.append(f"  .{port}({port}){comma}")
    lines.extend(
        [
            ");",
            "",
            "`default_nettype wire",
            "",
        ]
    )
    harness_path.parent.mkdir(parents=True, exist_ok=True)
    harness_path.write_text("\n".join(lines), encoding="utf-8")
    return "Top"


def task_qor_reference(repo: Path, task: dict[str, Any]) -> Path | None:
    reference = task.get("qor_reference")
    if reference is None:
        return None
    if not isinstance(reference, str) or not reference:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid qor_reference")
    return repo / reference


def task_top_name(task: dict[str, Any], key: str, default: str) -> str:
    top = task.get(key, default)
    if not isinstance(top, str) or not top:
        raise ValueError(f"task {task.get('id', '<unknown>')} has invalid {key}")
    return top


def run_yosys_structural_stat(
    repo: Path,
    rtl: Path,
    design: Path,
    top: str,
    stat_json: Path,
    stdout_path: Path,
    stderr_path: Path,
) -> subprocess.CompletedProcess[str]:
    stat_json.parent.mkdir(parents=True, exist_ok=True)
    script = (
        f"read_verilog -sv {rtl.as_posix()} {design.as_posix()}; "
        f"hierarchy -check -top {top}; proc; "
        f"tee -q -o {stat_json.as_posix()} stat -json"
    )
    return run(
        ["yosys", "-q", "-p", script],
        repo,
        stdout_path=stdout_path,
        stderr_path=stderr_path,
    )


def load_yosys_metrics(stat_json: Path, top: str) -> dict[str, Any]:
    data = json.loads(stat_json.read_text(encoding="utf-8"))
    modules = data.get("modules", {})
    if not isinstance(modules, dict):
        raise ValueError(f"{stat_json} does not contain Yosys module metrics")
    top_key = top if top.startswith("\\") else f"\\{top}"
    top_metrics = modules.get(top_key) or modules.get(top)
    if not isinstance(top_metrics, dict):
        raise ValueError(f"{stat_json} does not contain metrics for top `{top}`")
    design_metrics = data.get("design", {})
    if not isinstance(design_metrics, dict):
        design_metrics = {}
    return {
        "top_cells": int(top_metrics.get("num_cells", 0)),
        "top_wires": int(top_metrics.get("num_wires", 0)),
        "top_wire_bits": int(top_metrics.get("num_wire_bits", 0)),
        "primitive_cells": int(design_metrics.get("num_cells", 0)),
        "cell_types": normalize_cell_types(top_metrics.get("num_cells_by_type", {})),
    }


def normalize_cell_types(cell_types: Any) -> dict[str, int]:
    if not isinstance(cell_types, dict):
        return {}
    normalized = {}
    for key, value in cell_types.items():
        if isinstance(key, str) and isinstance(value, int):
            normalized[key.lstrip("\\")] = value
    return normalized


def pct_delta(generated: int | float | None, reference: int | float | None) -> float:
    if generated is None or reference is None or reference == 0:
        return 0.0
    return ((float(generated) - float(reference)) / float(reference)) * 100.0


def write_qor_tables(csv_path: Path, tex_path: Path, results: list[dict[str, Any]]) -> None:
    csv_path.parent.mkdir(parents=True, exist_ok=True)
    rows = []
    for item in results:
        qor = item["qor"]
        if not qor["available"]:
            continue
        rows.append(
            {
                "task_id": item["task_id"],
                "level": item["level"],
                "area_cells": qor["area_cells"],
                "reference_area_cells": qor["reference_area_cells"],
                "area_delta_pct": item["qor_delta"]["area_pct"],
                "wire_count": qor["wire_count"],
                "reference_wire_count": qor["reference_wire_count"],
                "wire_delta_pct": item["qor_delta"]["wire_pct"],
                "primitive_cells": qor["primitive_cells"],
            }
        )

    fieldnames = [
        "task_id",
        "level",
        "area_cells",
        "reference_area_cells",
        "area_delta_pct",
        "wire_count",
        "reference_wire_count",
        "wire_delta_pct",
        "primitive_cells",
    ]
    with csv_path.open("w", encoding="utf-8", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)

    lines = [
        "% Generated by benchmarks/run_bench.py; do not edit.",
        "\\begin{tabular}{llrrrr}",
        "\\toprule",
        "Task & Level & Area & Area $\\Delta$ & Wires & Wire $\\Delta$ \\\\",
        "\\midrule",
    ]
    for row in rows:
        task = row["task_id"].replace("_", "\\_")
        lines.append(
            f"{task} & {row['level']} & {row['area_cells']} & "
            f"{row['area_delta_pct']:.1f}\\% & {row['wire_count']} & "
            f"{row['wire_delta_pct']:.1f}\\% \\\\"
        )
    lines.extend(["\\bottomrule", "\\end{tabular}", ""])
    tex_path.write_text("\n".join(lines), encoding="utf-8")


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
    sim_testbench = task_sim_testbench(repo, task)
    sim_top = task_sim_top(task)
    formal_harness = task_formal_harness(repo, task)
    formal_top = task_formal_top(task)
    if (formal_harness is None) != (formal_top is None):
        raise ValueError(f"task {task_id} must define formal_harness and formal_top together")
    formal_depth = task_formal_depth(task)
    qor_reference = task_qor_reference(repo, task)
    qor_top = task_top_name(task, "qor_top", "Top")
    qor_reference_top = task_top_name(task, "qor_reference_top", qor_top)
    task_build_dir = build_dir / task_id
    wrapper = task_build_dir / "top.sv"
    sva = task_build_dir / "top_sva.sv"
    trace = task_build_dir / "traceability.json"
    ast_json = task_build_dir / "ast.json"
    dsl_ir = task_build_dir / "typed_ir_dsl.json"
    json_ir = task_build_dir / "typed_ir_json.json"
    vvp = task_build_dir / "top.vvp"
    sim_vvp = task_build_dir / "sim.vvp"
    autogen_sim_tb = task_build_dir / "sim_autogen.sv"
    sim_stdout = task_build_dir / "sim.stdout.txt"
    sim_stderr = task_build_dir / "sim.stderr.txt"
    formal_dir = task_build_dir / "formal"
    autogen_formal_harness = task_build_dir / "formal_autogen.sv"
    formal_sby = formal_dir / "task.sby"
    formal_stdout = formal_dir / "sby.stdout.txt"
    formal_stderr = formal_dir / "sby.stderr.txt"
    qor_generated_stat = task_build_dir / "qor.generated.yosys_stat.json"
    qor_generated_stdout = task_build_dir / "qor.generated.yosys.stdout.txt"
    qor_generated_stderr = task_build_dir / "qor.generated.yosys.stderr.txt"
    qor_reference_stat = task_build_dir / "qor.reference.yosys_stat.json"
    qor_reference_stdout = task_build_dir / "qor.reference.yosys.stdout.txt"
    qor_reference_stderr = task_build_dir / "qor.reference.yosys.stderr.txt"

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
    sim_enabled = expected_accept
    sim_compile_pass = False
    sim_run_pass = False
    sim_pass = False
    sim_status = "blocked" if expected_accept else "rejected"
    sim_mode = (
        "rejected"
        if not expected_accept
        else "declared"
        if sim_testbench is not None and sim_top is not None
        else "autogen"
    )
    formal_enabled = formal_harness is not None and formal_top is not None
    formal_mode = (
        "rejected"
        if not expected_accept
        else "declared"
        if formal_enabled
        else "not_run"
    )
    formal_pass = False
    formal_status = "blocked" if expected_accept else "rejected"
    qor_enabled = qor_reference is not None
    qor_generated_pass = False
    qor_reference_pass = False
    qor_available = False
    qor_status = "blocked" if expected_accept and qor_enabled else "not_run"
    if not expected_accept:
        qor_status = "rejected"
    qor_generated_metrics: dict[str, Any] | None = None
    qor_reference_metrics: dict[str, Any] | None = None
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

        yosys = run_yosys_structural_stat(
            repo,
            rtl,
            wrapper,
            qor_top,
            qor_generated_stat,
            qor_generated_stdout,
            qor_generated_stderr,
        )
        yosys_pass = yosys.returncode == 0
        qor_generated_pass = yosys_pass

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

    if emit_sv_pass and expected_accept and sim_enabled:
        if sim_testbench is None or sim_top is None:
            sim_testbench = autogen_sim_tb
            sim_top = write_autogen_sim_testbench(autogen_sim_tb, task_id, trace)
            sim_mode = "autogen"
        sim_compile = run(
            [
                "iverilog",
                "-g2012",
                "-s",
                sim_top,
                "-o",
                str(sim_vvp),
                str(rtl),
                str(wrapper),
                str(sim_testbench),
            ],
            repo,
            stdout_path=sim_stdout,
            stderr_path=sim_stderr,
        )
        sim_compile_pass = sim_compile.returncode == 0
        if sim_compile_pass:
            sim_run = run(
                ["vvp", str(sim_vvp)],
                repo,
                stdout_path=sim_stdout,
                stderr_path=sim_stderr,
            )
            sim_run_pass = sim_run.returncode == 0
        sim_pass = sim_compile_pass and sim_run_pass
        sim_status = "pass" if sim_pass else "failed"

    if emit_sv_pass and emit_trace_pass and expected_accept and not formal_enabled:
        try:
            formal_top = write_autogen_formal_harness(autogen_formal_harness, task_id, trace)
            formal_harness = autogen_formal_harness
            formal_enabled = True
            formal_mode = "autogen"
        except ValueError as exc:
            formal_status = f"not_run: {exc}"

    if emit_sv_pass and expected_accept and formal_enabled:
        formal_dir.mkdir(parents=True, exist_ok=True)
        assert formal_harness is not None
        assert formal_top is not None
        write_sby_job(formal_sby, rtl, wrapper, formal_harness, formal_top, formal_depth)
        formal = run(
            ["sby", "-f", formal_sby.name],
            formal_dir,
            stdout_path=formal_stdout,
            stderr_path=formal_stderr,
        )
        formal_pass = formal.returncode == 0
        formal_status = "proved" if formal_pass else "failed"

    if emit_sv_pass and expected_accept and qor_enabled and qor_generated_pass:
        assert qor_reference is not None
        reference_qor = run_yosys_structural_stat(
            repo,
            rtl,
            qor_reference,
            qor_reference_top,
            qor_reference_stat,
            qor_reference_stdout,
            qor_reference_stderr,
        )
        qor_reference_pass = reference_qor.returncode == 0
        if qor_reference_pass:
            try:
                qor_generated_metrics = load_yosys_metrics(qor_generated_stat, qor_top)
                qor_reference_metrics = load_yosys_metrics(qor_reference_stat, qor_reference_top)
                qor_available = True
                qor_status = "available"
            except (OSError, json.JSONDecodeError, ValueError) as exc:
                qor_status = f"parse_failed: {exc}"
        else:
            qor_status = "reference_failed"

    lint_pass = verilator_pass and sva_lint_pass and iverilog_pass and yosys_pass
    if expected_accept and not formal_enabled:
        notes = ["formal_pass is not run because this task is outside the formal smoke denominator."]
    else:
        notes = []
    if expected_accept and sim_mode == "autogen":
        notes.append("simulation uses an auto-generated ready/valid smoke harness.")
    if expected_accept and formal_mode == "autogen":
        notes.append("formal uses an auto-generated single-clock ready/valid smoke harness.")
    if expected_accept and not qor_enabled:
        notes.append("qor is not run because this task has no QoR reference wrapper.")
    if qor_available:
        notes.append("QoR timing is not reported by the current structural Yosys stat flow.")

    area_cells = qor_generated_metrics["top_cells"] if qor_generated_metrics is not None else None
    reference_area_cells = (
        qor_reference_metrics["top_cells"] if qor_reference_metrics is not None else None
    )
    wire_count = qor_generated_metrics["top_wires"] if qor_generated_metrics is not None else None
    reference_wire_count = (
        qor_reference_metrics["top_wires"] if qor_reference_metrics is not None else None
    )
    wire_bits = qor_generated_metrics["top_wire_bits"] if qor_generated_metrics is not None else None
    reference_wire_bits = (
        qor_reference_metrics["top_wire_bits"] if qor_reference_metrics is not None else None
    )
    primitive_cells = (
        qor_generated_metrics["primitive_cells"] if qor_generated_metrics is not None else None
    )
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
        "sim_pass": sim_pass,
        "sim_status": sim_status,
        "sim_result": {
            "enabled": sim_enabled,
            "mode": sim_mode,
            "compile_pass": sim_compile_pass,
            "run_pass": sim_run_pass,
            "testbench": str(sim_testbench.relative_to(repo)).replace("\\", "/")
            if sim_testbench is not None
            else None,
            "top": sim_top,
            "stdout": str(sim_stdout.relative_to(repo)).replace("\\", "/"),
            "stderr": str(sim_stderr.relative_to(repo)).replace("\\", "/"),
        },
        "formal_pass": formal_pass,
        "formal_status": formal_status,
        "formal_result": {
            "enabled": formal_enabled,
            "mode": formal_mode,
            "prove_pass": formal_pass,
            "harness": str(formal_harness.relative_to(repo)).replace("\\", "/")
            if formal_harness is not None
            else None,
            "top": formal_top,
            "depth": formal_depth,
            "sby": str(formal_sby.relative_to(repo)).replace("\\", "/"),
            "stdout": str(formal_stdout.relative_to(repo)).replace("\\", "/"),
            "stderr": str(formal_stderr.relative_to(repo)).replace("\\", "/"),
        },
        "unsafe_rejection": unsafe_rejection,
        "qor_delta": {
            "area_pct": pct_delta(area_cells, reference_area_cells),
            "wire_pct": pct_delta(wire_count, reference_wire_count),
            "timing_pct": 0.0,
            "latency_cycles": 0,
        },
        "qor": {
            "available": qor_available,
            "source": "yosys_structural_stat" if qor_available else qor_status,
            "area_cells": area_cells,
            "reference_area_cells": reference_area_cells,
            "wire_count": wire_count,
            "reference_wire_count": reference_wire_count,
            "wire_bits": wire_bits,
            "reference_wire_bits": reference_wire_bits,
            "primitive_cells": primitive_cells,
            "timing_ns": None,
            "cell_types": qor_generated_metrics["cell_types"] if qor_generated_metrics else {},
            "reference_cell_types": qor_reference_metrics["cell_types"]
            if qor_reference_metrics
            else {},
        },
        "qor_result": {
            "enabled": qor_enabled,
            "generated_pass": qor_generated_pass,
            "reference_pass": qor_reference_pass,
            "reference": str(qor_reference.relative_to(repo)).replace("\\", "/")
            if qor_reference is not None
            else None,
            "top": qor_top,
            "reference_top": qor_reference_top,
            "generated_stat": str(qor_generated_stat.relative_to(repo)).replace("\\", "/"),
            "reference_stat": str(qor_reference_stat.relative_to(repo)).replace("\\", "/"),
            "generated_stdout": str(qor_generated_stdout.relative_to(repo)).replace("\\", "/"),
            "generated_stderr": str(qor_generated_stderr.relative_to(repo)).replace("\\", "/"),
            "reference_stdout": str(qor_reference_stdout.relative_to(repo)).replace("\\", "/"),
            "reference_stderr": str(qor_reference_stderr.relative_to(repo)).replace("\\", "/"),
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
            "sim_vvp": str(sim_vvp.relative_to(repo)).replace("\\", "/"),
            "sim_stdout": str(sim_stdout.relative_to(repo)).replace("\\", "/"),
            "sim_stderr": str(sim_stderr.relative_to(repo)).replace("\\", "/"),
            "formal_sby": str(formal_sby.relative_to(repo)).replace("\\", "/"),
            "formal_stdout": str(formal_stdout.relative_to(repo)).replace("\\", "/"),
            "formal_stderr": str(formal_stderr.relative_to(repo)).replace("\\", "/"),
            "qor_generated_stat": str(qor_generated_stat.relative_to(repo)).replace("\\", "/"),
            "qor_reference_stat": str(qor_reference_stat.relative_to(repo)).replace("\\", "/"),
            "rtl_collateral": str(rtl.relative_to(repo)).replace("\\", "/"),
        },
        "notes": notes,
    }


def aggregate_results(results: list[dict[str, Any]]) -> dict[str, Any]:
    total = len(results)
    positives = [item for item in results if item["expected_compose_pass"]]
    negatives = [item for item in results if not item["expected_compose_pass"]]
    sim_enabled = [item for item in positives if item["sim_result"]["enabled"]]
    formal_enabled = [item for item in positives if item["formal_result"]["enabled"]]
    qor_enabled = [item for item in positives if item["qor_result"]["enabled"]]
    qor_available = [item for item in qor_enabled if item["qor"]["available"]]

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
        "sim_pass": aggregate_with_status(sim_enabled, count(sim_enabled, "sim_pass")),
        "formal_pass": aggregate_with_status(formal_enabled, count(formal_enabled, "formal_pass")),
        "qor": {
            "available_tasks": len(qor_available),
            "total": len(qor_enabled),
            "status": aggregate_with_status(qor_enabled, len(qor_available))["status"],
            "avg_area_delta_pct": average_delta(qor_available, "area_pct"),
            "avg_wire_delta_pct": average_delta(qor_available, "wire_pct"),
        },
    }


def average_delta(items: list[dict[str, Any]], key: str) -> float:
    if not items:
        return 0.0
    return sum(float(item["qor_delta"][key]) for item in items) / len(items)


def aggregate_with_status(items: list[dict[str, Any]], passed: int) -> dict[str, Any]:
    total = len(items)
    if total == 0:
        status = "not_run"
        rate = 0.0
    else:
        rate = passed / total
        status = "pass" if passed == total else "partial"
    return {
        "passed": passed,
        "total": total,
        "rate": rate,
        "status": status,
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
    qor_csv = build_dir / "qor_summary.csv"
    qor_tex = build_dir / "qor_summary.tex"
    write_qor_tables(qor_csv, qor_tex, results)
    output_payload = {
        "schema_version": "mico.bench.results.v0",
        "benchmark": {
            "name": manifest.get("name", "ModuleComposeBench"),
            "version": manifest.get("version", "0.0.0"),
            "manifest": str(manifest_path.relative_to(repo)).replace("\\", "/"),
        },
        "generated_tables": {
            "qor_csv": str(qor_csv.relative_to(repo)).replace("\\", "/"),
            "qor_tex": str(qor_tex.relative_to(repo)).replace("\\", "/"),
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
    print(f"sim_pass: {summary['sim_pass']['passed']}/{summary['sim_pass']['total']}")
    print(f"formal_pass: {summary['formal_pass']['passed']}/{summary['formal_pass']['total']}")
    print(f"qor_available: {summary['qor']['available_tasks']}/{summary['qor']['total']}")
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
        and summary["sim_pass"]["passed"] == summary["sim_pass"]["total"]
        and summary["formal_pass"]["passed"] == summary["formal_pass"]["total"]
        and summary["qor"]["available_tasks"] == summary["qor"]["total"]
        else 1
    )


if __name__ == "__main__":
    raise SystemExit(main())
