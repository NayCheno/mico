#!/usr/bin/env python3
"""Write formal coverage matrix artifacts from benchmark manifests/results."""

from __future__ import annotations

import argparse
import csv
import json
from collections import defaultdict
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover - Docker tool setup covers this
    raise SystemExit("PyYAML is required. Run this through scripts/eda-docker.*.") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]

PROPERTY_ORDER = [
    "ready_valid_stability",
    "fire_implies_transfer",
    "no_combinational_self_loop",
    "no_drop_bounded",
    "no_duplicate_bounded",
    "width_extension_correctness",
    "register_status_visibility",
    "protocol_request_response",
    "telemetry_filter_predicate",
]

PROPERTY_NOTES = {
    "ready_valid_stability": "payload stability while valid is held before ready",
    "fire_implies_transfer": "valid-and-ready fire event advances the modeled transfer",
    "no_combinational_self_loop": "bounded smoke excludes combinational ready/valid self-loop",
    "no_drop_bounded": "FIFO, skid, or pipeline monitor observes no dropped transfer in bound",
    "no_duplicate_bounded": "FIFO, skid, or pipeline monitor observes no duplicate transfer in bound",
    "width_extension_correctness": "width adapter output preserves and zero-extends source payload",
    "register_status_visibility": "command payload becomes visible on status path in bound",
    "protocol_request_response": "request/response or bridge payload refinement is preserved",
    "telemetry_filter_predicate": "filter or accumulator predicate relation is preserved",
}

BASE_OBLIGATIONS = [
    "ready_valid_stability",
    "fire_implies_transfer",
    "no_combinational_self_loop",
]


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{path} must contain a YAML mapping")
    return data


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


def task_obligations(features: set[str]) -> list[str]:
    obligations = list(BASE_OBLIGATIONS)
    if features & {
        "fifo_chain",
        "latency_seed",
        "skid_buffer",
        "pipeline",
        "video_pipeline",
        "filter_chain",
        "streaming_accelerator",
        "processing_chain",
    }:
        obligations.extend(["no_drop_bounded", "no_duplicate_bounded"])
    if features & {
        "width_adapter",
        "width_bridge",
        "explicit_width_adapter",
        "mmio_control_data_path",
    }:
        obligations.append("width_extension_correctness")
    if features & {"register_wrapper", "status_path", "dma_register_map", "bus_wrapper_seed"}:
        obligations.append("register_status_visibility")
    if features & {
        "protocol_bridge",
        "request_response",
        "axi_apb_wrapper",
        "axi_stream_packetizer",
    }:
        obligations.append("protocol_request_response")
    if features & {"telemetry_chain", "multi_ip_subsystem"}:
        obligations.append("telemetry_filter_predicate")
    return [obligation for obligation in PROPERTY_ORDER if obligation in set(obligations)]


def adapter_monitor(features: set[str]) -> str:
    monitors = []
    if "width_adapter" in features or "explicit_width_adapter" in features or "width_bridge" in features:
        monitors.append("width zero-extension/refinement")
    if "skid_buffer" in features:
        monitors.append("ready/valid preservation")
    if "pipeline" in features or "video_pipeline" in features or "fifo_chain" in features:
        monitors.append("order/no-drop/no-duplicate")
    if "register_wrapper" in features or "status_path" in features or "dma_register_map" in features:
        monitors.append("read/status visibility")
    if "cdc_adapter" in features or "explicit_cdc_adapter" in features or "cdc_subsystem" in features:
        monitors.append("structural CDC boundary")
    return "; ".join(monitors) if monitors else "direct ready/valid monitor"


def cdc_task(task: dict[str, Any]) -> bool:
    features = set(task.get("expected_features", []))
    return bool(
        features
        & {
            "cdc_adapter",
            "explicit_cdc_adapter",
            "cdc_subsystem",
            "smoke_only_cdc",
            "unsafe_cdc_without_adapter",
        }
    )


def result_map(path: Path) -> dict[str, dict[str, Any]]:
    data = load_json(path)
    rows = data.get("results", [])
    if not isinstance(rows, list):
        raise ValueError(f"{path} results must be a list")
    return {str(row.get("task_id")): row for row in rows if row.get("task_id")}


def collect_split(split: str, manifest_path: Path, result_path: Path) -> list[dict[str, Any]]:
    manifest = load_yaml(manifest_path)
    results = result_map(result_path)
    rows: list[dict[str, Any]] = []
    for task in manifest.get("tasks", []):
        if task.get("type") != "positive" or not task.get("formal_harness"):
            continue
        task_id = str(task["id"])
        result = results.get(task_id, {})
        formal = result.get("formal_result", {}) if isinstance(result.get("formal_result"), dict) else {}
        features = set(task.get("expected_features", []))
        for obligation in task_obligations(features):
            rows.append(
                {
                    "split": split,
                    "task_id": task_id,
                    "level": task.get("level", ""),
                    "property_class": obligation,
                    "harness": task.get("formal_harness"),
                    "top": task.get("formal_top", formal.get("top", "")),
                    "depth": task.get("formal_depth", formal.get("depth", "")),
                    "bounded": "yes",
                    "clock_scope": "single-clock",
                    "adapter_monitor": adapter_monitor(features),
                    "assumptions": "reset-first-cycle; smoke leaf behavior; bounded depth",
                    "formal_status": result.get("formal_status", "not-run"),
                    "formal_pass": bool(result.get("formal_pass")),
                }
            )
    return rows


def collect_cdc_boundaries(split: str, manifest_path: Path, result_path: Path) -> list[dict[str, Any]]:
    manifest = load_yaml(manifest_path)
    results = result_map(result_path)
    rows: list[dict[str, Any]] = []
    for task in manifest.get("tasks", []):
        if not cdc_task(task):
            continue
        task_id = str(task["id"])
        result = results.get(task_id, {})
        features = set(task.get("expected_features", []))
        is_positive = task.get("type") == "positive"
        rows.append(
            {
                "split": split,
                "task_id": task_id,
                "type": task.get("type", ""),
                "boundary_class": "explicit_adapter" if is_positive else "direct_cdc_rejection",
                "required_check": "declared CDC adapter boundary"
                if is_positive
                else "compiler rejects missing CDC adapter",
                "evidence": "simulation/lint smoke; CDC proof not claimed"
                if is_positive
                else "expected unsafe rejection",
                "result_status": result.get("formal_status", result.get("status", "not-run")),
                "expected_outcome_pass": bool(result.get("expected_outcome_pass")),
                "non_claim": "metastability, MTBF, gray-pointer FIFO correctness, and multi-clock proof",
                "features": ",".join(sorted(features)),
            }
        )
    return rows


def summarize(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    counts: dict[str, dict[str, dict[str, int]]] = defaultdict(
        lambda: defaultdict(lambda: {"pass": 0, "total": 0})
    )
    for row in rows:
        item = counts[row["property_class"]][row["split"]]
        item["total"] += 1
        if row["formal_pass"]:
            item["pass"] += 1

    summary_rows = []
    for property_class in PROPERTY_ORDER:
        if property_class not in counts:
            continue
        out: dict[str, Any] = {
            "property_class": property_class,
            "note": PROPERTY_NOTES[property_class],
        }
        for split in ["public-dev", "held-out", "realism"]:
            item = counts[property_class].get(split, {"pass": 0, "total": 0})
            out[f"{split}_pass"] = item["pass"]
            out[f"{split}_total"] = item["total"]
        summary_rows.append(out)
    return summary_rows


def write_csv(path: Path, rows: list[dict[str, Any]], fieldnames: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=fieldnames, extrasaction="ignore")
        writer.writeheader()
        writer.writerows(rows)


def fraction(row: dict[str, Any], split: str) -> str:
    return f"{row[f'{split}_pass']}/{row[f'{split}_total']}"


def write_tex(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "% Generated by scripts/write-formal-coverage-matrix.py; do not edit.",
        "\\begin{tabular}{lllll}",
        "\\toprule",
        "Property obligation & Public-dev & Held-out & Realism & Boundary \\\\",
        "\\midrule",
    ]
    for row in rows:
        lines.append(
            " & ".join(
                [
                    tex_escape(row["property_class"]),
                    tex_escape(fraction(row, "public-dev")),
                    tex_escape(fraction(row, "held-out")),
                    tex_escape(fraction(row, "realism")),
                    tex_escape(row["note"]),
                ]
            )
            + " \\\\"
        )
    lines.extend(["\\bottomrule", "\\end{tabular}", ""])
    path.write_text("\n".join(lines), encoding="utf-8")


def write_cdc_tex(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    lines = [
        "% Generated by scripts/write-formal-coverage-matrix.py; do not edit.",
        "\\begin{tabular}{lllll}",
        "\\toprule",
        "Split & Task & Boundary & Evidence & Non-claim \\\\",
        "\\midrule",
    ]
    for row in rows:
        lines.append(
            " & ".join(
                [
                    tex_escape(row["split"]),
                    tex_escape(row["task_id"]),
                    tex_escape(row["boundary_class"]),
                    tex_escape(row["evidence"]),
                    tex_escape(row["non_claim"]),
                ]
            )
            + " \\\\"
        )
    lines.extend(["\\bottomrule", "\\end{tabular}", ""])
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--public-manifest", default="benchmarks/module_compose_bench_manifest.yaml")
    parser.add_argument("--public-result", default="build/bench/seed_results.json")
    parser.add_argument("--heldout-manifest", default="benchmarks/module_compose_bench_heldout.yaml")
    parser.add_argument("--heldout-result", default="build/bench/heldout_results.json")
    parser.add_argument("--realism-manifest", default="benchmarks/module_compose_bench_realism.yaml")
    parser.add_argument("--realism-result", default="build/bench/realism_results.json")
    parser.add_argument("--out-dir", default="build/bench/formal_coverage")
    parser.add_argument("--paper-table-dir", default="build/paper_tables/formal")
    args = parser.parse_args()

    task_rows = []
    task_rows.extend(collect_split("public-dev", repo_path(args.public_manifest), repo_path(args.public_result)))
    task_rows.extend(collect_split("held-out", repo_path(args.heldout_manifest), repo_path(args.heldout_result)))
    task_rows.extend(collect_split("realism", repo_path(args.realism_manifest), repo_path(args.realism_result)))
    cdc_rows = []
    cdc_rows.extend(
        collect_cdc_boundaries("public-dev", repo_path(args.public_manifest), repo_path(args.public_result))
    )
    cdc_rows.extend(
        collect_cdc_boundaries("held-out", repo_path(args.heldout_manifest), repo_path(args.heldout_result))
    )
    cdc_rows.extend(
        collect_cdc_boundaries("realism", repo_path(args.realism_manifest), repo_path(args.realism_result))
    )
    summary_rows = summarize(task_rows)

    out_dir = repo_path(args.out_dir)
    table_dir = repo_path(args.paper_table_dir)
    write_csv(
        out_dir / "formal_coverage_tasks.csv",
        task_rows,
        [
            "split",
            "task_id",
            "level",
            "property_class",
            "harness",
            "top",
            "depth",
            "bounded",
            "clock_scope",
            "adapter_monitor",
            "assumptions",
            "formal_status",
            "formal_pass",
        ],
    )
    write_csv(
        out_dir / "formal_coverage_matrix.csv",
        summary_rows,
        [
            "property_class",
            "public-dev_pass",
            "public-dev_total",
            "held-out_pass",
            "held-out_total",
            "realism_pass",
            "realism_total",
            "note",
        ],
    )
    write_csv(
        out_dir / "cdc_structural_boundaries.csv",
        cdc_rows,
        [
            "split",
            "task_id",
            "type",
            "boundary_class",
            "required_check",
            "evidence",
            "result_status",
            "expected_outcome_pass",
            "non_claim",
            "features",
        ],
    )
    write_tex(table_dir / "formal_coverage_matrix.tex", summary_rows)
    write_cdc_tex(table_dir / "cdc_structural_boundaries.tex", cdc_rows)
    print(f"wrote {out_dir.relative_to(REPO_ROOT)} and {table_dir.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
