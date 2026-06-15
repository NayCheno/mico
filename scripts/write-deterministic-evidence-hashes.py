#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]


def display(path: Path) -> str:
    try:
        return path.resolve().relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def repo_path(path: str) -> Path:
    raw = Path(path)
    if raw.is_absolute():
        return raw
    return REPO_ROOT / raw


def sha256_file(path: Path) -> str | None:
    if not path.exists() or not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def file_entry(path: Path) -> dict[str, Any]:
    digest = sha256_file(path)
    entry: dict[str, Any] = {
        "path": display(path),
        "exists": digest is not None,
        "sha256": digest,
    }
    if digest is not None:
        entry["bytes"] = path.stat().st_size
    return entry


def collect_files(paths: list[str]) -> list[dict[str, Any]]:
    return [file_entry(repo_path(path)) for path in paths]


def collect_tree(path: str, suffixes: tuple[str, ...] | None = None) -> list[dict[str, Any]]:
    root = repo_path(path)
    if not root.exists():
        return []
    files = sorted(item for item in root.rglob("*") if item.is_file())
    if suffixes is not None:
        files = [item for item in files if item.suffix.lower() in suffixes]
    return [file_entry(item) for item in files]


def git_text(args: list[str]) -> str | None:
    proc = subprocess.run(
        ["git", *args],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if proc.returncode != 0:
        return None
    return proc.stdout.strip()


def first_line(cmd: list[str]) -> str | None:
    proc = subprocess.run(
        cmd,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    text = proc.stdout.strip()
    if not text:
        return None
    return text.splitlines()[0]


def missing_entries(groups: dict[str, list[dict[str, Any]]]) -> list[str]:
    missing: list[str] = []
    for entries in groups.values():
        for entry in entries:
            if not entry.get("exists"):
                missing.append(str(entry["path"]))
    return missing


def build_payload(full_check_manifest: str) -> dict[str, Any]:
    manifest_files = [
        "benchmarks/module_compose_bench_manifest.yaml",
        "benchmarks/module_compose_bench_heldout.yaml",
        "benchmarks/manifest_schema.json",
    ]
    deterministic_results = [
        "build/bench/seed_results.json",
        "build/bench/heldout_results.json",
    ]
    aggregate_results = [
        "build/bench/aggregate_results.json",
        "build/bench/aggregate_heldout_results.json",
    ]
    release_files = [
        full_check_manifest,
    ]
    table_roots = [
        "build/paper_tables",
        "build/bench/heldout_tables",
    ]
    bench_table_patterns = collect_tree("build/bench", (".csv", ".tex"))
    table_entries = []
    for root in table_roots:
        table_entries.extend(collect_tree(root, (".csv", ".tex")))
    table_entries.extend(bench_table_patterns)
    table_entries = sorted({entry["path"]: entry for entry in table_entries}.values(), key=lambda item: item["path"])

    groups = {
        "benchmark_manifests": collect_files(manifest_files),
        "deterministic_results": collect_files(deterministic_results),
        "aggregate_results": collect_files(aggregate_results),
        "release_files": collect_files(release_files),
    }
    missing = missing_entries(groups)

    payload: dict[str, Any] = {
        "schema_version": "mico.release.deterministic_evidence_hashes.v0",
        "source_commit_hash": git_text(["rev-parse", "HEAD"]),
        "source_branch": git_text(["branch", "--show-current"]),
        "worktree_status_short": (git_text(["status", "--short"]) or "").splitlines(),
        "tool_versions": {
            "rustc": first_line(["rustc", "--version"]),
            "cargo": first_line(["cargo", "--version"]),
            "python3": first_line(["python3", "--version"]),
            "verilator": first_line(["verilator", "--version"]),
            "iverilog": first_line(["iverilog", "-V"]),
            "yosys": first_line(["yosys", "-V"]),
            "sby": first_line(["bash", "-lc", "sby --version 2>&1 || command -v sby"]),
            "z3": first_line(["z3", "--version"]),
        },
        "evidence": {
            **groups,
            "generated_tables": table_entries,
        },
        "missing_required": missing,
    }
    payload["status"] = "complete" if not missing else "incomplete"
    return payload


def main() -> int:
    parser = argparse.ArgumentParser(description="Write deterministic MICO evidence hash sidecar.")
    parser.add_argument("--output", default="build/release/deterministic_evidence_hashes.json")
    parser.add_argument("--full-check-manifest", default="build/release/full_check_manifest.json")
    args = parser.parse_args()

    payload = build_payload(args.full_check_manifest)
    output = repo_path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {display(output)}")
    if payload["missing_required"]:
        for item in payload["missing_required"]:
            print(f"missing required deterministic evidence: {item}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
