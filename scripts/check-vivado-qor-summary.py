#!/usr/bin/env python3
"""Check and summarize the host-Vivado QoR subset report."""

from __future__ import annotations

import argparse
import json
import statistics
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return data


def tex_escape(value: Any) -> str:
    text = str(value)
    return (
        text.replace("\\", "\\textbackslash{}")
        .replace("&", "\\&")
        .replace("%", "\\%")
        .replace("_", "\\_")
        .replace("#", "\\#")
    )


def by_task_kind(records: list[dict[str, Any]]) -> dict[tuple[str, str], dict[str, Any]]:
    return {(str(row.get("task")), str(row.get("kind"))): row for row in records}


def numeric(value: Any) -> float | None:
    if value in (None, "null", ""):
        return None
    return float(value)


def pct_delta(generated: float, reference: float) -> float | None:
    if reference == 0:
        return 0.0 if generated == 0 else None
    return (generated - reference) * 100.0 / reference


def format_number(value: float | None, suffix: str = "") -> str:
    if value is None:
        return "n/a"
    return f"{value:.3f}{suffix}"


def build_summary(data: dict[str, Any], max_median_lut_delta_pct: float) -> dict[str, Any]:
    records = data.get("records", [])
    if not isinstance(records, list):
        raise ValueError("records must be a list")
    split_coverage = data.get("split_coverage", [])
    if not isinstance(split_coverage, list):
        raise ValueError("split_coverage must be a list when present")
    coverage_summary = data.get("coverage_summary", {})
    if coverage_summary and not isinstance(coverage_summary, dict):
        raise ValueError("coverage_summary must be an object when present")
    index = by_task_kind(records)
    tasks = sorted({str(row.get("task")) for row in records if row.get("task")})

    task_rows = []
    lut_deltas = []
    generated_wns = []
    reference_wns = []
    failures: list[str] = []
    if not data.get("vivado_version"):
        failures.append("source report is missing vivado_version")
    if not data.get("constraint_assumptions"):
        failures.append("source report is missing constraint_assumptions")
    split_rows = len(split_coverage)
    unique_split_tasks = sorted({str(row.get("task")) for row in split_coverage if row.get("task")})
    if coverage_summary:
        expected_split_rows = coverage_summary.get("total_reference_enabled_rows")
        expected_pairs = coverage_summary.get("unique_vivado_task_pairs")
        expected_unique_tasks = coverage_summary.get("unique_covered_tasks")
        if expected_split_rows != split_rows:
            failures.append(
                f"coverage_summary total_reference_enabled_rows={expected_split_rows} "
                f"does not match split_coverage rows={split_rows}"
            )
        if expected_pairs != len(tasks):
            failures.append(
                f"coverage_summary unique_vivado_task_pairs={expected_pairs} "
                f"does not match report task pairs={len(tasks)}"
            )
        if expected_unique_tasks != len(unique_split_tasks):
            failures.append(
                f"coverage_summary unique_covered_tasks={expected_unique_tasks} "
                f"does not match split unique tasks={len(unique_split_tasks)}"
            )
    if split_coverage and set(unique_split_tasks) != set(tasks):
        missing = sorted(set(unique_split_tasks) - set(tasks))
        extra = sorted(set(tasks) - set(unique_split_tasks))
        if missing:
            failures.append(f"split coverage tasks missing Vivado records: {', '.join(missing)}")
        if extra:
            failures.append(f"Vivado records not referenced by split coverage: {', '.join(extra)}")
    for task in tasks:
        gen = index.get((task, "generated"))
        ref = index.get((task, "reference"))
        if gen is None or ref is None:
            failures.append(f"{task}: missing generated/reference pair")
            continue
        if gen.get("status") != "pass":
            failures.append(f"{task}: generated status is {gen.get('status')}")
        if ref.get("status") != "pass":
            failures.append(f"{task}: reference status is {ref.get('status')}")
        if gen.get("timing_pass") is not True:
            failures.append(f"{task}: generated timing did not pass")
        if ref.get("timing_pass") is not True:
            failures.append(f"{task}: reference timing did not pass")
        for kind, row in (("generated", gen), ("reference", ref)):
            elapsed = numeric(row.get("elapsed_seconds"))
            if elapsed is None or elapsed <= 0.0:
                failures.append(f"{task}: {kind} elapsed_seconds is missing or nonpositive")
            if not row.get("vivado_version"):
                failures.append(f"{task}: {kind} vivado_version is missing")
            if row.get("error") not in (None, "null", ""):
                failures.append(f"{task}: {kind} reported error text")
        gen_lut = float(gen.get("lut", 0))
        ref_lut = float(ref.get("lut", 0))
        delta = pct_delta(gen_lut, ref_lut)
        if delta is None:
            failures.append(f"{task}: reference LUT is zero while generated LUT is nonzero")
        else:
            lut_deltas.append(delta)
        gen_wns = numeric(gen.get("wns"))
        ref_wns = numeric(ref.get("wns"))
        if gen_wns is not None:
            generated_wns.append(gen_wns)
            if gen_wns < 0.0:
                failures.append(f"{task}: generated WNS is negative")
        if ref_wns is not None:
            reference_wns.append(ref_wns)
            if ref_wns < 0.0:
                failures.append(f"{task}: reference WNS is negative")
        task_rows.append(
            {
                "task": task,
                "generated_lut": int(gen.get("lut", 0)),
                "reference_lut": int(ref.get("lut", 0)),
                "lut_delta_pct": delta,
                "generated_wns": gen_wns,
                "reference_wns": ref_wns,
                "generated_elapsed_seconds": numeric(gen.get("elapsed_seconds")),
                "reference_elapsed_seconds": numeric(ref.get("elapsed_seconds")),
                "generated_timing_pass": gen.get("timing_pass") is True,
                "reference_timing_pass": ref.get("timing_pass") is True,
            }
        )

    median_lut_delta = statistics.median(lut_deltas) if lut_deltas else None
    max_abs_lut_delta = max((abs(value) for value in lut_deltas), default=None)
    if median_lut_delta is None or abs(median_lut_delta) > max_median_lut_delta_pct:
        failures.append(
            f"median LUT delta {format_number(median_lut_delta, '%')} exceeds "
            f"{max_median_lut_delta_pct:.3f}%"
        )

    return {
        "schema_version": "mico.vivado_qor_thresholds.v0",
        "source_report": {
            "schema_version": data.get("schema_version"),
            "vivado_version": data.get("vivado_version"),
            "vivado_part": data.get("vivado_part"),
            "vivado_flow": data.get("vivado_flow"),
            "clock_period_ns": data.get("clock_period_ns"),
            "run_elapsed_seconds": data.get("run_elapsed_seconds"),
            "constraint_assumptions": data.get("constraint_assumptions"),
        },
        "thresholds": {
            "max_median_lut_delta_pct": max_median_lut_delta_pct,
            "require_nonnegative_wns": True,
            "require_generated_and_reference_status_pass": True,
        },
        "summary": {
            "task_count": len(tasks),
            "task_pairs_checked": len(task_rows),
            "reference_enabled_split_rows": split_rows if split_coverage else None,
            "unique_covered_tasks": len(unique_split_tasks) if split_coverage else None,
            "median_lut_delta_pct": median_lut_delta,
            "max_abs_lut_delta_pct": max_abs_lut_delta,
            "min_generated_wns_ns": min(generated_wns) if generated_wns else None,
            "min_reference_wns_ns": min(reference_wns) if reference_wns else None,
            "all_generated_timing_nonnegative": all(value >= 0.0 for value in generated_wns),
            "all_reference_timing_nonnegative": all(value >= 0.0 for value in reference_wns),
            "status": "pass" if not failures else "fail",
            "failures": failures,
        },
        "tasks": task_rows,
    }


def write_tex(path: Path, payload: dict[str, Any]) -> None:
    summary = payload["summary"]
    thresholds = payload["thresholds"]
    split_rows = summary.get("reference_enabled_split_rows")
    split_row_text = (
        f"{split_rows}/{split_rows}" if isinstance(split_rows, int) and split_rows > 0 else "n/a"
    )
    rows = [
        ("Reference-enabled split rows", split_row_text),
        ("Task pairs", f"{summary['task_pairs_checked']}/{summary['task_count']}"),
        ("Median LUT delta", format_number(summary["median_lut_delta_pct"], "%")),
        ("Max absolute LUT delta", format_number(summary["max_abs_lut_delta_pct"], "%")),
        ("Minimum generated WNS", format_number(summary["min_generated_wns_ns"], " ns")),
        ("Minimum reference WNS", format_number(summary["min_reference_wns_ns"], " ns")),
        ("Threshold", f"median LUT delta at most {thresholds['max_median_lut_delta_pct']:.1f}%, nonnegative WNS"),
        ("Status", summary["status"]),
    ]
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "% Generated by scripts/check-vivado-qor-summary.py; do not edit.",
        "\\begin{tabular}{ll}",
        "\\toprule",
        "Metric & Value \\\\",
        "\\midrule",
    ]
    for key, value in rows:
        lines.append(f"{tex_escape(key)} & {tex_escape(value)} \\\\")
    lines.extend(["\\bottomrule", "\\end{tabular}", ""])
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", default="build/reports/vivado-host/vivado_qor_subset_summary.json")
    parser.add_argument("--output", default="build/reports/vivado-host/vivado_qor_thresholds.json")
    parser.add_argument("--tex", default="build/reports/vivado-host/vivado_qor_thresholds.tex")
    parser.add_argument("--paper-tex")
    parser.add_argument("--max-median-lut-delta-pct", type=float, default=5.0)
    args = parser.parse_args()

    payload = build_summary(load_json(repo_path(args.input)), args.max_median_lut_delta_pct)
    output = repo_path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    write_tex(repo_path(args.tex), payload)
    if args.paper_tex:
        write_tex(repo_path(args.paper_tex), payload)
    print(f"wrote {output.relative_to(REPO_ROOT)}")
    print(f"status={payload['summary']['status']}")
    if payload["summary"]["failures"]:
        for failure in payload["summary"]["failures"]:
            print(f"failure: {failure}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
