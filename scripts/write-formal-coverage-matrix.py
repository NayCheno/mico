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
    "direct_ready_valid_payload",
    "fifo_pipeline_ordering",
    "width_payload_relation",
    "register_status_visibility",
    "protocol_request_response",
    "telemetry_filter_predicate",
]

PROPERTY_NOTES = {
    "direct_ready_valid_payload": "payload stability and ready/valid after reset",
    "fifo_pipeline_ordering": "bounded no-drop/no-duplicate proxy and payload ordering",
    "width_payload_relation": "widening/packing relation and handshake coupling",
    "register_status_visibility": "command-to-status visibility and payload relation",
    "protocol_request_response": "request/response or bridge payload refinement",
    "telemetry_filter_predicate": "filter/accumulator payload predicate preservation",
}


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


def classify(features: set[str], task_id: str) -> str:
    if features & {"width_adapter", "width_bridge", "explicit_width_adapter", "mmio_control_data_path"}:
        return "width_payload_relation"
    if features & {"fifo_chain", "latency_seed", "skid_buffer", "streaming_accelerator"}:
        return "fifo_pipeline_ordering"
    if features & {"register_wrapper", "status_path", "dma_register_map"}:
        return "register_status_visibility"
    if features & {"protocol_bridge", "request_response", "axi_apb_wrapper", "axi_stream_packetizer"}:
        return "protocol_request_response"
    if features & {"telemetry_chain", "multi_ip_subsystem"}:
        return "telemetry_filter_predicate"
    if "direct" in task_id or features & {"direct_connect", "renamed_instances", "monitor_contract"}:
        return "direct_ready_valid_payload"
    return "direct_ready_valid_payload"


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
        rows.append(
            {
                "split": split,
                "task_id": task_id,
                "level": task.get("level", ""),
                "property_class": classify(features, task_id),
                "harness": task.get("formal_harness"),
                "top": task.get("formal_top", formal.get("top", "")),
                "depth": task.get("formal_depth", formal.get("depth", "")),
                "bounded": "yes",
                "clock_scope": "single-clock",
                "assumptions": "reset-first-cycle; smoke leaf behavior; bounded depth",
                "formal_status": result.get("formal_status", "not-run"),
                "formal_pass": bool(result.get("formal_pass")),
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
        "Property class & Public-dev & Held-out & Realism & Boundary \\\\",
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
    write_tex(table_dir / "formal_coverage_matrix.tex", summary_rows)
    print(f"wrote {out_dir.relative_to(REPO_ROOT)} and {table_dir.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
