#!/usr/bin/env python3
"""Write sanitized hash metadata for authenticated LLM evidence files."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def display(path: Path) -> str:
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def git_text(args: list[str]) -> str | None:
    try:
        return subprocess.check_output(["git", *args], cwd=REPO_ROOT, text=True, stderr=subprocess.DEVNULL).strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{display(path)} must contain a JSON object")
    return data


def llm_bench_summary(data: dict[str, Any]) -> dict[str, Any]:
    run = data.get("run", {}) if isinstance(data.get("run"), dict) else {}
    summary = data.get("summary", {}) if isinstance(data.get("summary"), dict) else {}
    attempts = data.get("results", [])
    if not isinstance(attempts, list):
        attempts = []
    profiles = run.get("profiles")
    if not isinstance(profiles, list):
        profiles = sorted({str(item.get("profile")) for item in attempts if item.get("profile")})
    baselines = run.get("baselines")
    if not isinstance(baselines, list):
        baselines = sorted({str(item.get("baseline")) for item in attempts if item.get("baseline")})
    return {
        "schema_version": data.get("schema_version"),
        "run_id": run.get("id"),
        "mode": run.get("mode"),
        "sdk": run.get("sdk"),
        "manifest_path": run.get("manifest"),
        "manifest_sha256": run.get("manifest_sha256"),
        "profiles": profiles,
        "baselines": baselines,
        "attempts": summary.get("attempts", len(attempts)),
        "positive_attempts": summary.get("positive_attempts"),
        "negative_attempts": summary.get("negative_attempts"),
        "provider_requests": summary.get("provider_requests"),
        "cache_hits": summary.get("cache_hits"),
        "response_json_valid": summary.get("response_json_valid"),
        "mico_compiler_pass": summary.get("mico_compiler_pass"),
        "sv_lint_pass": summary.get("sv_lint_pass"),
        "unsafe_rejection": summary.get("unsafe_rejection"),
    }


def aggregate_summary(data: dict[str, Any]) -> dict[str, Any]:
    return {
        "schema_version": data.get("schema_version"),
        "inputs": data.get("inputs"),
        "llm_summary_rows": len(data.get("llm_summary", [])),
        "llm_compact_rows": len(data.get("llm_compact_summary", [])),
        "llm_paired_comparison_rows": len(data.get("llm_paired_comparisons", [])),
        "llm_failure_taxonomy_rows": len(data.get("llm_failure_taxonomy", [])),
    }


def collect_artifact(path: Path) -> dict[str, Any]:
    data = load_json(path)
    rel = display(path)
    summary = aggregate_summary(data) if rel.endswith("aggregate_llm_v3.json") else llm_bench_summary(data)
    return {
        "path": rel,
        "sha256": sha256_file(path),
        "bytes": path.stat().st_size,
        "summary": summary,
    }


def build_payload(paths: list[str], require: bool) -> dict[str, Any]:
    resolved = [repo_path(path) for path in paths]
    missing = [display(path) for path in resolved if not path.exists()]
    artifacts = [collect_artifact(path) for path in resolved if path.exists()]
    return {
        "schema_version": "mico.release.llm_evidence_hashes.v0",
        "source_commit_hash": git_text(["rev-parse", "HEAD"]),
        "source_branch": git_text(["branch", "--show-current"]),
        "required_artifacts": paths,
        "require_artifacts": require,
        "artifacts": artifacts,
        "missing_required": missing if require else [],
        "missing_optional": [] if require else missing,
        "status": "complete" if not missing else ("incomplete" if require else "missing_optional"),
        "redaction_policy": {
            "api_keys": "never stored",
            "local_provider_config": "not included",
            "raw_provider_caches": "not included",
            "bundle_scope": "sanitized v3 execute records and aggregate metadata only",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", default="build/release/llm_evidence_hashes.json")
    parser.add_argument(
        "--require",
        action="store_true",
        help="Fail if any configured authenticated LLM evidence artifact is missing.",
    )
    parser.add_argument(
        "--artifact",
        action="append",
        default=[
            "build/llm/bench_execute_public_dev_v3.json",
            "build/llm/bench_execute_heldout_v3.json",
            "build/bench/aggregate_llm_v3.json",
        ],
    )
    args = parser.parse_args()

    output = repo_path(args.output)
    payload = build_payload(args.artifact, args.require)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {display(output)}")
    if payload["missing_required"]:
        for item in payload["missing_required"]:
            print(f"missing required LLM evidence: {item}")
        return 1
    if payload["missing_optional"]:
        for item in payload["missing_optional"]:
            print(f"missing optional LLM evidence: {item}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
