#!/usr/bin/env python3
"""Aggregate deterministic and LLM benchmark results into paper-ready tables."""

from __future__ import annotations

import argparse
import csv
from collections import Counter, defaultdict
import json
import math
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("PyYAML is required. Run this through scripts/eda-docker.*.") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]
AGGREGATE_SCHEMA = "mico.aggregate.results.v0"


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def display_path(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT)).replace("\\", "/")
    except ValueError:
        return str(path)


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{display_path(path)} must contain a JSON object")
    return data


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{display_path(path)} must contain a YAML mapping")
    return data


def wilson_interval(passed: int, total: int, z: float = 1.96) -> tuple[float, float]:
    if total == 0:
        return 0.0, 0.0
    phat = passed / total
    denom = 1 + (z * z / total)
    center = (phat + (z * z) / (2 * total)) / denom
    margin = (z * math.sqrt((phat * (1 - phat) / total) + (z * z) / (4 * total * total))) / denom
    return max(0.0, center - margin), min(1.0, center + margin)


def pct(value: float) -> str:
    return f"{value * 100.0:.1f}\\%"


def count_rate(items: list[dict[str, Any]], key: str) -> dict[str, Any]:
    total = len(items)
    passed = sum(1 for item in items if item.get(key) is True)
    low, high = wilson_interval(passed, total)
    return {
        "passed": passed,
        "total": total,
        "rate": passed / total if total else 0.0,
        "ci95_low": low,
        "ci95_high": high,
    }


def deterministic_per_level(results: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for level in sorted({str(item.get("level", "")) for item in results}):
        items = [item for item in results if item.get("level") == level]
        positives = [item for item in items if item.get("type") == "positive"]
        negatives = [item for item in items if item.get("type") == "negative"]
        expected = count_rate(items, "expected_outcome_pass")
        compose = count_rate(positives, "compose_pass_1")
        lint = count_rate(positives, "lint_pass")
        unsafe = count_rate(negatives, "unsafe_rejection")
        rows.append(
            {
                "level": level,
                "tasks": len(items),
                "positive_tasks": len(positives),
                "negative_tasks": len(negatives),
                "expected_passed": expected["passed"],
                "expected_total": expected["total"],
                "expected_rate": expected["rate"],
                "expected_ci95_low": expected["ci95_low"],
                "expected_ci95_high": expected["ci95_high"],
                "compose_passed": compose["passed"],
                "compose_total": compose["total"],
                "compose_rate": compose["rate"],
                "lint_passed": lint["passed"],
                "lint_total": lint["total"],
                "lint_rate": lint["rate"],
                "unsafe_passed": unsafe["passed"],
                "unsafe_total": unsafe["total"],
                "unsafe_rate": unsafe["rate"],
            }
        )
    return rows


def diagnostic_taxonomy(results: list[dict[str, Any]]) -> list[dict[str, Any]]:
    counts: Counter[str] = Counter()
    tasks_by_code: dict[str, list[str]] = defaultdict(list)
    for item in results:
        if item.get("type") != "negative":
            continue
        compiler = item.get("compiler_result", {})
        for code in compiler.get("expected_diagnostics", []):
            counts[str(code)] += 1
            tasks_by_code[str(code)].append(str(item.get("task_id")))
    return [
        {
            "diagnostic": code,
            "tasks": counts[code],
            "task_ids": tasks_by_code[code],
        }
        for code in sorted(counts)
    ]


def ablation_rows(results: list[dict[str, Any]], manifest: dict[str, Any]) -> list[dict[str, Any]]:
    by_id = {str(task.get("id")): task for task in manifest.get("tasks", []) if isinstance(task, dict)}
    negatives = [item for item in results if item.get("type") == "negative"]
    definitions = [
        (
            "no_contract_checks",
            "ContractViolation",
            lambda item: "ContractViolation" in expected_codes(item),
        ),
        (
            "no_clock_domain_checks",
            "ClockDomainMismatch or domain adapter misuse",
            lambda item: "ClockDomainMismatch" in expected_codes(item)
            or "cross_domain" in str(item.get("task_id"))
            or "cdc" in str(item.get("task_id")),
        ),
        (
            "no_adapter_library",
            "adapter legality and availability checks",
            lambda item: any(
                code in expected_codes(item)
                for code in ["UnknownAdapterKind", "UnknownAdapter", "AdapterMismatch", "WidthMismatch"]
            ),
        ),
        (
            "no_structured_diagnostics",
            "negative tasks lose machine-actionable repair codes",
            lambda item: bool(expected_codes(item)),
        ),
    ]
    rows = []
    for name, removed_guard, predicate in definitions:
        affected = [item for item in negatives if predicate(item)]
        rejected = sum(1 for item in affected if item.get("unsafe_rejection") is True)
        low, high = wilson_interval(rejected, len(affected))
        rows.append(
            {
                "ablation": name,
                "removed_guard": removed_guard,
                "affected_tasks": len(affected),
                "currently_rejected": rejected,
                "current_rejection_rate": rejected / len(affected) if affected else 0.0,
                "ci95_low": low,
                "ci95_high": high,
                "task_ids": [str(item.get("task_id")) for item in affected],
            }
        )
    json_ast_total = len(results)
    json_ast_passed = sum(
        1
        for item in results
        if item.get("json_ast_result", {}).get("expected_outcome_pass") is True
    )
    low, high = wilson_interval(json_ast_passed, json_ast_total)
    rows.append(
        {
            "ablation": "dsl_vs_json_ast",
            "removed_guard": "source-to-AST equivalence path",
            "affected_tasks": json_ast_total,
            "currently_rejected": json_ast_passed,
            "current_rejection_rate": json_ast_passed / json_ast_total if json_ast_total else 0.0,
            "ci95_low": low,
            "ci95_high": high,
            "task_ids": sorted(by_id),
        }
    )
    return rows


def expected_codes(item: dict[str, Any]) -> list[str]:
    compiler = item.get("compiler_result", {})
    codes = compiler.get("expected_diagnostics", [])
    return [str(code) for code in codes] if isinstance(codes, list) else []


def llm_summary(llm_payloads: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for payload in llm_payloads:
        run = payload.get("run", {})
        results = payload.get("results", [])
        groups: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
        for item in results if isinstance(results, list) else []:
            if isinstance(item, dict):
                profile = item.get("profile", {})
                groups[(str(profile.get("name", "")), str(item.get("baseline", "")))].append(item)
        for (profile, baseline), items in sorted(groups.items()):
            scored = [item for item in items if is_scored_llm_result(item)]
            positive = [item for item in scored if item.get("task_type") == "positive"]
            negative = [item for item in scored if item.get("task_type") == "negative"]
            compiler_positive = [
                item
                for item in positive
                if item.get("baseline") in {"mico_source", "mico_json_ast", "mico_json_ast_repair"}
            ]
            compiler_pass = sum(
                1 for item in compiler_positive if item.get("compiler_result", {}).get("check_pass") is True
            )
            unsafe_total = len(
                [
                    item
                    for item in negative
                    if item.get("baseline") in {"mico_source", "mico_json_ast", "mico_json_ast_repair"}
                ]
            )
            unsafe_pass = sum(
                1
                for item in negative
                if item.get("baseline") in {"mico_source", "mico_json_ast", "mico_json_ast_repair"}
                and item.get("compiler_result", {}).get("unsafe_rejection") is True
            )
            lint_pass = sum(
                1 for item in positive if item.get("eda_result", {}).get("lint_pass") is True
            )
            requested = sum(1 for item in items if item.get("response", {}).get("requested") is True)
            json_valid = sum(1 for item in items if item.get("response", {}).get("json_valid") is True)
            repair_turns = [int(item.get("repair", {}).get("turns", 0)) for item in items]
            compiler_low, compiler_high = wilson_interval(compiler_pass, len(compiler_positive))
            lint_low, lint_high = wilson_interval(lint_pass, len(positive))
            unsafe_low, unsafe_high = wilson_interval(unsafe_pass, unsafe_total)
            rows.append(
                {
                    "run_id": run.get("id"),
                    "mode": run.get("mode"),
                    "profile": profile,
                    "baseline": baseline,
                    "attempts": len(items),
                    "scored_attempts": len(scored),
                    "provider_requests": requested,
                    "json_valid": json_valid,
                    "json_valid_rate": json_valid / requested if requested else 0.0,
                    "compiler_passed": compiler_pass,
                    "compiler_total": len(compiler_positive),
                    "compiler_rate": compiler_pass / len(compiler_positive)
                    if compiler_positive
                    else 0.0,
                    "compiler_ci95_low": compiler_low,
                    "compiler_ci95_high": compiler_high,
                    "lint_passed": lint_pass,
                    "lint_total": len(positive),
                    "lint_rate": lint_pass / len(positive) if positive else 0.0,
                    "lint_ci95_low": lint_low,
                    "lint_ci95_high": lint_high,
                    "unsafe_passed": unsafe_pass,
                    "unsafe_total": unsafe_total,
                    "unsafe_rate": unsafe_pass / unsafe_total if unsafe_total else 0.0,
                    "unsafe_ci95_low": unsafe_low,
                    "unsafe_ci95_high": unsafe_high,
                    "repair_turns_avg": sum(repair_turns) / len(repair_turns) if repair_turns else 0.0,
                    "token_total": sum_tokens(items),
                    "estimated_cost_usd": sum_cost(items),
                }
            )
    return rows


def is_scored_llm_result(item: dict[str, Any]) -> bool:
    response = item.get("response", {})
    compiler = item.get("compiler_result", {})
    return not (response.get("requested") is False and compiler.get("reason") == "validate_only")


def repair_turn_distribution(llm_payloads: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for payload in llm_payloads:
        run = payload.get("run", {})
        results = payload.get("results", [])
        groups: dict[tuple[str, str], Counter[int]] = defaultdict(Counter)
        totals: Counter[tuple[str, str]] = Counter()
        for item in results if isinstance(results, list) else []:
            if not isinstance(item, dict):
                continue
            if not is_scored_llm_result(item):
                continue
            profile = item.get("profile", {})
            key = (str(profile.get("name", "")), str(item.get("baseline", "")))
            turns = int(item.get("repair", {}).get("turns", 0))
            groups[key][turns] += 1
            totals[key] += 1
        for (profile, baseline), counts in sorted(groups.items()):
            for turns, count in sorted(counts.items()):
                total = totals[(profile, baseline)]
                rows.append(
                    {
                        "run_id": run.get("id"),
                        "mode": run.get("mode"),
                        "profile": profile,
                        "baseline": baseline,
                        "repair_turns": turns,
                        "attempts": count,
                        "total_attempts": total,
                        "share": count / total if total else 0.0,
                    }
                )
    return rows


def cost_token_rows(llm_payloads: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for payload in llm_payloads:
        run = payload.get("run", {})
        results = payload.get("results", [])
        groups: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
        for item in results if isinstance(results, list) else []:
            if not isinstance(item, dict):
                continue
            profile = item.get("profile", {})
            groups[(str(profile.get("name", "")), str(item.get("baseline", "")))].append(item)
        for (profile, baseline), items in sorted(groups.items()):
            estimated = sum_cost(items)
            rows.append(
                {
                    "run_id": run.get("id"),
                    "mode": run.get("mode"),
                    "profile": profile,
                    "baseline": baseline,
                    "attempts": len(items),
                    "provider_requests": sum(
                        1 for item in items if item.get("response", {}).get("requested") is True
                    ),
                    "prompt_tokens": sum_token_field(items, "prompt_tokens"),
                    "completion_tokens": sum_token_field(items, "completion_tokens"),
                    "total_tokens": sum_token_field(items, "total_tokens"),
                    "estimated_cost_usd": round(estimated, 6) if estimated is not None else "",
                    "cost_status": "configured" if estimated is not None else "not_configured",
                }
            )
    return rows


def sum_tokens(items: list[dict[str, Any]]) -> int:
    return sum_token_field(items, "total_tokens")


def sum_token_field(items: list[dict[str, Any]], field: str) -> int:
    total = 0
    for item in items:
        usage = item.get("usage", {})
        value = usage.get(field) if isinstance(usage, dict) else None
        if isinstance(value, int):
            total += value
    return total


def sum_cost(items: list[dict[str, Any]]) -> float | None:
    total = 0.0
    seen = False
    for item in items:
        cost = item.get("cost", {})
        value = cost.get("estimated_usd") if isinstance(cost, dict) else None
        if isinstance(value, (int, float)):
            total += float(value)
            seen = True
    return total if seen else None


def paired_comparisons(llm_payloads: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for payload in llm_payloads:
        run = payload.get("run", {})
        results = payload.get("results", [])
        by_profile_task: dict[tuple[str, str], dict[str, dict[str, Any]]] = defaultdict(dict)
        for item in results if isinstance(results, list) else []:
            if not isinstance(item, dict):
                continue
            profile = item.get("profile", {})
            key = (str(profile.get("name", "")), str(item.get("task_id", "")))
            by_profile_task[key][str(item.get("baseline", ""))] = item
        target = "mico_json_ast_repair"
        for baseline in ["direct_verilog", "sv_interface", "mico_source", "mico_json_ast"]:
            comparable = 0
            target_wins = 0
            baseline_wins = 0
            ties = 0
            for baseline_map in by_profile_task.values():
                if target not in baseline_map or baseline not in baseline_map:
                    continue
                target_pass = final_pass(baseline_map[target])
                baseline_pass = final_pass(baseline_map[baseline])
                if target_pass is None or baseline_pass is None:
                    continue
                comparable += 1
                if target_pass and not baseline_pass:
                    target_wins += 1
                elif baseline_pass and not target_pass:
                    baseline_wins += 1
                else:
                    ties += 1
            rows.append(
                {
                    "run_id": run.get("id"),
                    "mode": run.get("mode"),
                    "comparison": f"{target}_vs_{baseline}",
                    "comparable_tasks": comparable,
                    "target_wins": target_wins,
                    "baseline_wins": baseline_wins,
                    "ties": ties,
                }
            )
    return rows


def llm_failure_taxonomy(llm_payloads: list[dict[str, Any]]) -> list[dict[str, Any]]:
    counts: Counter[tuple[str, str, str]] = Counter()
    for payload in llm_payloads:
        results = payload.get("results", [])
        for item in results if isinstance(results, list) else []:
            if not isinstance(item, dict):
                continue
            profile = item.get("profile", {})
            profile_name = str(profile.get("name", ""))
            baseline = str(item.get("baseline", ""))
            for category in classify_llm_result(item):
                counts[(profile_name, baseline, category)] += 1
    return [
        {
            "profile": profile,
            "baseline": baseline,
            "category": category,
            "attempts": count,
        }
        for (profile, baseline, category), count in sorted(counts.items())
    ]


def classify_llm_result(item: dict[str, Any]) -> list[str]:
    response = item.get("response", {})
    compiler = item.get("compiler_result", {})
    eda = item.get("eda_result", {})
    if response.get("requested") is False and compiler.get("reason") == "validate_only":
        return ["validate_only_not_scored"]
    if response.get("json_valid") is False or compiler.get("reason") == "response_json_invalid":
        return ["response_json_invalid"]
    if compiler.get("reason") in {
        "model_rejected",
        "missing_systemverilog",
        "missing_mico_source",
        "missing_mico_ast",
    }:
        return [str(compiler.get("reason"))]
    categories = []
    if item.get("task_type") == "negative":
        categories.append(
            "unsafe_rejected"
            if compiler.get("unsafe_rejection") is True
            else "unsafe_not_rejected"
        )
    elif item.get("baseline") in {"direct_verilog", "sv_interface"}:
        categories.append("sv_lint_pass" if eda.get("lint_pass") is True else "sv_lint_fail")
    else:
        if compiler.get("check_pass") is True:
            categories.append("compiler_pass")
            categories.append("sv_lint_pass" if eda.get("lint_pass") is True else "sv_lint_fail")
        else:
            codes = compiler.get("diagnostic_codes", [])
            if isinstance(codes, list) and codes:
                categories.extend(f"compiler:{code}" for code in codes)
            else:
                categories.append(str(compiler.get("reason") or "compiler_rejected"))
    return categories


def final_pass(item: dict[str, Any]) -> bool | None:
    response = item.get("response", {})
    if response.get("requested") is False and item.get("compiler_result", {}).get("reason") == "validate_only":
        return None
    if item.get("task_type") == "negative":
        compiler = item.get("compiler_result", {})
        return bool(compiler.get("unsafe_rejection"))
    if item.get("baseline") in {"direct_verilog", "sv_interface"}:
        return bool(item.get("eda_result", {}).get("lint_pass"))
    compiler = item.get("compiler_result", {})
    eda = item.get("eda_result", {})
    return bool(compiler.get("check_pass")) and bool(eda.get("lint_pass"))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = sorted({key for row in rows for key in row.keys()})
    if not fieldnames:
        path.write_text("", encoding="utf-8")
        return
    with path.open("w", encoding="utf-8", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def tex_escape(value: Any) -> str:
    text = str(value)
    return (
        text.replace("\\", "\\textbackslash{}")
        .replace("_", "\\_")
        .replace("%", "\\%")
        .replace("&", "\\&")
    )


def write_tex_table(path: Path, columns: list[tuple[str, str]], rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    align = "l" * len(columns)
    lines = [
        "% Generated by benchmarks/aggregate_results.py; do not edit.",
        f"\\begin{{tabular}}{{{align}}}",
        "\\toprule",
        " & ".join(tex_escape(label) for _, label in columns) + " \\\\",
        "\\midrule",
    ]
    for row in rows:
        values = []
        for key, _ in columns:
            value = row.get(key, "")
            if isinstance(value, float):
                value = f"{value:.3f}"
            values.append(tex_escape(value))
        lines.append(" & ".join(values) + " \\\\")
    lines.extend(["\\bottomrule", "\\end{tabular}", ""])
    path.write_text("\n".join(lines), encoding="utf-8")


def qor_rows(results: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for item in results:
        if item.get("type") != "positive":
            continue
        qor_result = item.get("qor_result", {})
        if not qor_result.get("enabled"):
            continue
        qor = item.get("qor", {})
        delta = item.get("qor_delta", {})
        rows.append(
            {
                "task_id": item.get("task_id"),
                "level": item.get("level"),
                "status": "available" if qor.get("available") is True else qor.get("source", "missing"),
                "area_cells": qor.get("area_cells"),
                "reference_area_cells": qor.get("reference_area_cells"),
                "area_delta_pct": delta.get("area_pct"),
                "wire_count": qor.get("wire_count"),
                "reference_wire_count": qor.get("reference_wire_count"),
                "wire_delta_pct": delta.get("wire_pct"),
                "wire_bits": qor.get("wire_bits"),
                "reference_wire_bits": qor.get("reference_wire_bits"),
                "timing_ns": qor.get("timing_ns"),
                "latency_cycles": delta.get("latency_cycles"),
            }
        )
    return rows


def deterministic_summary_rows(summary: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for key, label in [
        ("expected_outcome_pass", "Expected outcome"),
        ("compose_pass_1", "Compose pass"),
        ("lint_pass", "Lint/elab pass"),
        ("unsafe_rejection", "Unsafe rejection"),
        ("json_ast_path", "JSON AST path"),
        ("sim_pass", "Simulation pass"),
        ("formal_pass", "Selected formal pass"),
    ]:
        item = summary.get(key, {})
        rows.append(
            {
                "metric": label,
                "passed": item.get("passed", 0),
                "total": item.get("total", 0),
                "rate": item.get("rate", 0.0),
            }
        )
    qor = summary.get("qor", {})
    rows.append(
        {
            "metric": "Structural QoR",
            "passed": qor.get("available_tasks", 0),
            "total": qor.get("total", 0),
            "rate": (qor.get("available_tasks", 0) / qor.get("total", 1))
            if qor.get("total", 0)
            else 0.0,
        }
    )
    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bench-result", default="build/bench/seed_results.json")
    parser.add_argument("--manifest", default="benchmarks/module_compose_bench_manifest.yaml")
    parser.add_argument("--llm-result", action="append", default=[])
    parser.add_argument("--out-json", default="build/bench/aggregate_results.json")
    parser.add_argument("--out-dir", default="build/bench")
    parser.add_argument("--paper-table-dir", default="build/paper_tables")
    args = parser.parse_args()

    bench_path = repo_path(args.bench_result)
    manifest_path = repo_path(args.manifest)
    bench = load_json(bench_path)
    manifest = load_yaml(manifest_path)
    results = bench.get("results", [])
    if not isinstance(results, list):
        raise ValueError("benchmark results must contain a results list")
    llm_payloads = [load_json(repo_path(path)) for path in args.llm_result]

    summary_rows = deterministic_summary_rows(bench.get("summary", {}))
    per_level = deterministic_per_level(results)
    taxonomy = diagnostic_taxonomy(results)
    ablations = ablation_rows(results, manifest)
    qor = qor_rows(results)
    llm_rows = llm_summary(llm_payloads)
    repair_rows = repair_turn_distribution(llm_payloads)
    cost_rows = cost_token_rows(llm_payloads)
    paired = paired_comparisons(llm_payloads)
    llm_failures = llm_failure_taxonomy(llm_payloads)

    out_dir = repo_path(args.out_dir)
    paper_dir = repo_path(args.paper_table_dir)
    write_csv(out_dir / "deterministic_summary.csv", summary_rows)
    write_csv(out_dir / "deterministic_per_level.csv", per_level)
    write_csv(out_dir / "unsafe_diagnostics.csv", taxonomy)
    write_csv(out_dir / "ablation_counterfactual.csv", ablations)
    write_csv(out_dir / "qor_structural.csv", qor)
    if llm_rows:
        write_csv(out_dir / "llm_summary.csv", llm_rows)
    if repair_rows:
        write_csv(out_dir / "llm_repair_turns.csv", repair_rows)
    if cost_rows:
        write_csv(out_dir / "llm_cost_tokens.csv", cost_rows)
    if paired:
        write_csv(out_dir / "llm_paired_comparisons.csv", paired)
    if llm_failures:
        write_csv(out_dir / "llm_failure_taxonomy.csv", llm_failures)

    write_tex_table(
        paper_dir / "deterministic_summary.tex",
        [("metric", "Metric"), ("passed", "Pass"), ("total", "Total"), ("rate", "Rate")],
        summary_rows,
    )
    write_tex_table(
        paper_dir / "deterministic_per_level.tex",
        [
            ("level", "Level"),
            ("tasks", "Tasks"),
            ("positive_tasks", "Pos"),
            ("negative_tasks", "Neg"),
            ("expected_rate", "Expected"),
            ("compose_rate", "Compose"),
            ("unsafe_rate", "Unsafe"),
        ],
        per_level,
    )
    write_tex_table(
        paper_dir / "unsafe_taxonomy.tex",
        [("diagnostic", "Diagnostic"), ("tasks", "Tasks")],
        taxonomy,
    )
    write_tex_table(
        paper_dir / "ablation_counterfactual.tex",
        [
            ("ablation", "Ablation"),
            ("affected_tasks", "Tasks"),
            ("currently_rejected", "Guarded"),
            ("current_rejection_rate", "Rate"),
        ],
        ablations,
    )
    write_tex_table(
        paper_dir / "qor_structural.tex",
        [
            ("task_id", "Task"),
            ("level", "Level"),
            ("status", "Status"),
            ("area_cells", "Cells"),
            ("reference_area_cells", "Ref Cells"),
            ("area_delta_pct", "Cell Delta"),
            ("wire_count", "Wires"),
            ("reference_wire_count", "Ref Wires"),
            ("wire_delta_pct", "Wire Delta"),
        ],
        qor,
    )
    if llm_rows:
        write_tex_table(
            paper_dir / "llm_summary.tex",
            [
                ("mode", "Mode"),
                ("profile", "Profile"),
                ("baseline", "Baseline"),
                ("attempts", "Attempts"),
                ("provider_requests", "Requests"),
                ("compiler_passed", "Compiler"),
                ("compiler_total", "Compiler Total"),
                ("unsafe_passed", "Unsafe"),
                ("unsafe_total", "Unsafe Total"),
            ],
            llm_rows,
        )
    if repair_rows:
        write_tex_table(
            paper_dir / "llm_repair_turns.tex",
            [
                ("mode", "Mode"),
                ("profile", "Profile"),
                ("baseline", "Baseline"),
                ("repair_turns", "Turns"),
                ("attempts", "Attempts"),
                ("total_attempts", "Total"),
                ("share", "Share"),
            ],
            repair_rows,
        )
    if cost_rows:
        write_tex_table(
            paper_dir / "llm_cost_tokens.tex",
            [
                ("mode", "Mode"),
                ("profile", "Profile"),
                ("baseline", "Baseline"),
                ("provider_requests", "Requests"),
                ("total_tokens", "Tokens"),
                ("estimated_cost_usd", "Cost USD"),
                ("cost_status", "Cost Status"),
            ],
            cost_rows,
        )
    if paired:
        write_tex_table(
            paper_dir / "llm_paired_comparisons.tex",
            [
                ("mode", "Mode"),
                ("comparison", "Comparison"),
                ("comparable_tasks", "Tasks"),
                ("target_wins", "Repair Wins"),
                ("baseline_wins", "Baseline Wins"),
                ("ties", "Ties"),
            ],
            paired,
        )
    if llm_failures:
        write_tex_table(
            paper_dir / "llm_failure_taxonomy.tex",
            [
                ("profile", "Profile"),
                ("baseline", "Baseline"),
                ("category", "Category"),
                ("attempts", "Attempts"),
            ],
            llm_failures,
        )

    aggregate = {
        "schema_version": AGGREGATE_SCHEMA,
        "inputs": {
            "bench_result": display_path(bench_path),
            "manifest": display_path(manifest_path),
            "llm_results": [display_path(repo_path(path)) for path in args.llm_result],
        },
        "generated_tables": {
            "out_dir": display_path(out_dir),
            "paper_table_dir": display_path(paper_dir),
        },
        "deterministic_summary": summary_rows,
        "deterministic_per_level": per_level,
        "unsafe_diagnostics": taxonomy,
        "ablation_counterfactual": ablations,
        "qor_structural": qor,
        "llm_summary": llm_rows,
        "llm_repair_turns": repair_rows,
        "llm_cost_tokens": cost_rows,
        "llm_paired_comparisons": paired,
        "llm_failure_taxonomy": llm_failures,
    }
    out_json = repo_path(args.out_json)
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_json.write_text(json.dumps(aggregate, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {display_path(out_json)}")
    print(f"deterministic_levels={len(per_level)}")
    print(f"diagnostics={len(taxonomy)}")
    print(f"ablations={len(ablations)}")
    print(f"qor_rows={len(qor)}")
    print(f"llm_rows={len(llm_rows)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
