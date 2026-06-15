#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover - covered by Docker tool setup
    raise SystemExit("ERROR: PyYAML is required; run this script in scripts/eda-docker.ps1") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]


def read_text(rel: str) -> str:
    return (REPO_ROOT / rel).read_text(encoding="utf-8")


def load_manifest(rel: str) -> dict[str, Any]:
    with (REPO_ROOT / rel).open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise AssertionError(f"{rel} is not a YAML mapping")
    tasks = data.get("tasks")
    if not isinstance(tasks, list):
        raise AssertionError(f"{rel} does not contain a tasks list")
    return data


def task_summary(manifest: dict[str, Any]) -> dict[str, Any]:
    tasks = manifest["tasks"]
    positives = [task for task in tasks if task.get("type") == "positive"]
    negatives = [task for task in tasks if task.get("type") == "negative"]
    return {
        "total": len(tasks),
        "positive": len(positives),
        "negative": len(negatives),
        "levels": dict(Counter(str(task.get("level")) for task in tasks)),
        "declared_sim": sum(1 for task in positives if task.get("sim_testbench")),
        "declared_formal": sum(1 for task in positives if task.get("formal_harness")),
        "qor_reference": sum(1 for task in positives if task.get("qor_reference")),
    }


def expect(label: str, actual: Any, expected: Any) -> list[str]:
    if actual == expected:
        return []
    return [f"{label}: expected {expected!r}, got {actual!r}"]


def scan_required_references() -> list[str]:
    errors: list[str] = []
    required_docs = [
        "README.md",
        "PROJECT_MANIFEST.md",
        "docs/claim_boundary.md",
        "docs/current_status.md",
        "docs/dac2027_submission_plan.md",
        "docs/13_architecture_audit.md",
        "docs/14_reproduction_workflow.md",
    ]
    for rel in required_docs:
        if "release_claim_table.md" not in read_text(rel):
            errors.append(f"{rel}: missing docs/release_claim_table.md reference")

    numeric_claim_re = re.compile(
        r"\b(62|36/36|31/31|26/26|20-task|20/20|10/10|32|24|nine-task)\b"
    )
    for path in sorted((REPO_ROOT / "paper" / "sections").glob("*.tex")):
        text = path.read_text(encoding="utf-8")
        if numeric_claim_re.search(text) and "release\\_claim\\_table.md" not in text:
            errors.append(f"{path.relative_to(REPO_ROOT).as_posix()}: numeric claims need release_claim_table mapping")
    return errors


def scan_stale_claims() -> list[str]:
    errors: list[str] = []
    checks: list[tuple[str, str, str]] = [
        ("PROJECT_MANIFEST.md", r"\b60-task\b", "public benchmark is now 62 tasks"),
        ("docs/13_architecture_audit.md", r"\b60-task\b", "public benchmark is now 62 tasks"),
        ("docs/14_reproduction_workflow.md", r"\b60-task\b", "public benchmark is now 62 tasks"),
        ("docs/13_architecture_audit.md", r"Twenty tasks,", "public directed simulations are now 32"),
        ("docs/14_reproduction_workflow.md", r"\b20 use committed directed\b", "public directed simulations are now 32"),
        ("docs/13_architecture_audit.md", r"\b20 use committed directed\b", "public directed simulations are now 32"),
        ("docs/13_architecture_audit.md", r"\bfourteen committed\b", "public directed formal monitors are now 24"),
        ("docs/14_reproduction_workflow.md", r"\b14 use committed\b", "public directed formal monitors are now 24"),
        ("docs/13_architecture_audit.md", r"\b14 use committed\b", "public directed formal monitors are now 24"),
        ("docs/14_reproduction_workflow.md", r"\b12/12\b", "held-out split is now 20/20"),
        ("docs/14_reproduction_workflow.md", r"\b6/6\b", "held-out positives/unsafe are now 10/10"),
        ("docs/14_reproduction_workflow.md", r"\b5/5\b", "held-out formal is now 9/9"),
        ("docs/13_architecture_audit.md", r"\bfour-task\b", "Vivado subset is now nine tasks"),
        ("docs/14_reproduction_workflow.md", r"\bfour-task\b", "Vivado subset is now nine tasks"),
        ("docs/13_architecture_audit.md", r"\bfour representative tasks\b", "Vivado subset is now nine tasks"),
        ("docs/14_reproduction_workflow.md", r"\bfour wrappers\b", "Vivado subset is now nine tasks"),
        ("docs/13_architecture_audit.md", r"\bnegative authenticated low-cost LLM matrix\b", "v2 bounded Branch A candidate must be described"),
        ("rust_project/Cargo.toml", r"example\.com", "Cargo repository metadata must not be a placeholder"),
    ]
    for rel, pattern, reason in checks:
        text = read_text(rel)
        for match in re.finditer(pattern, text, flags=re.IGNORECASE):
            line = text.count("\n", 0, match.start()) + 1
            errors.append(f"{rel}:{line}: stale claim ({reason}): {match.group(0)!r}")
    return errors


def check_claim_table() -> list[str]:
    text = read_text("docs/release_claim_table.md")
    required_tokens = [
        "62 total, 36 positive, 26 negative",
        "32 declared, 4 generated",
        "24 declared, 7 generated",
        "20 total, 10 positive, 10 negative",
        "7 declared, 3 generated",
        "6 declared, 3 generated",
        "9 representative tasks",
        "build/release/full_check_manifest.json",
        "mico.bench.results.v0",
        "mico.aggregate.results.v0",
        "mico.llm.bench.v0",
    ]
    return [f"docs/release_claim_table.md: missing {token!r}" for token in required_tokens if token not in text]


def check_manifests() -> list[str]:
    errors: list[str] = []
    public = task_summary(load_manifest("benchmarks/module_compose_bench_manifest.yaml"))
    heldout = task_summary(load_manifest("benchmarks/module_compose_bench_heldout.yaml"))

    errors += expect("public total", public["total"], 62)
    errors += expect("public positives", public["positive"], 36)
    errors += expect("public negatives", public["negative"], 26)
    errors += expect("public levels", public["levels"], {"L1": 10, "L2": 13, "L3": 10, "L4": 10, "L5": 10, "L6": 9})
    errors += expect("public declared simulations", public["declared_sim"], 32)
    errors += expect("public declared formal monitors", public["declared_formal"], 24)
    errors += expect("public QoR references", public["qor_reference"], 9)

    errors += expect("held-out total", heldout["total"], 20)
    errors += expect("held-out positives", heldout["positive"], 10)
    errors += expect("held-out negatives", heldout["negative"], 10)
    errors += expect("held-out declared simulations", heldout["declared_sim"], 7)
    errors += expect("held-out declared formal monitors", heldout["declared_formal"], 6)
    errors += expect("held-out QoR references", heldout["qor_reference"], 3)
    return errors


def main() -> int:
    errors: list[str] = []
    errors.extend(check_manifests())
    errors.extend(check_claim_table())
    errors.extend(scan_required_references())
    errors.extend(scan_stale_claims())

    if errors:
        print("Documentation claim check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("documentation claim check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
