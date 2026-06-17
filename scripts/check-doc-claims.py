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

UNSUPPORTED_CLAIM_PATTERNS: list[tuple[str, str]] = [
    (r"\barbitrary[- ]model\b|\barbitrary models\b", "arbitrary-model LLM generalization"),
    (r"\buntested models\b", "untested-model LLM generalization"),
    (r"\bbroad free[- ]form repair\b|\bfree[- ]form model repair\b", "broad free-form repair"),
    (r"\bautonomous semantic repair\b|\bautonomous model repair\b", "autonomous semantic repair"),
    (r"\bgeneral LLM repair\b|\bmodel-general repair\b", "general repair reliability"),
    (r"\bexhaustive formal\b|\bfull formal proof\b|\bfull per-task formal proof\b", "exhaustive/full formal proof"),
    (r"\barbitrary LTL\b", "arbitrary LTL support"),
    (r"\bCDC correctness proof\b|\bCDC proof\b|\bformal CDC proof\b", "CDC correctness proof"),
    (r"\brouted timing closure\b|\bfull timing closure\b|\btiming closure\b", "timing closure"),
    (r"\bboard-level implementation\b|\bbitstream generation\b", "board-level implementation or bitstream generation"),
]

CLAIM_SCAN_FILES = [
    "README.md",
    "PROJECT_MANIFEST.md",
    "docs/current_status.md",
    "docs/dac2027_submission_plan.md",
    "docs/artifact_quickstart.md",
    "docs/13_architecture_audit.md",
    "docs/14_reproduction_workflow.md",
    "docs/20_paper_dac_ready.md",
    "docs/dac2027_full_check_baseline_2026-06-15.md",
    "docs/final_claim_freeze.md",
    "paper/main.tex",
]

DISCLAIMER_MARKERS = [
    "not claim",
    "does not claim",
    "do not claim",
    "must not claim",
    "not support",
    "does not support",
    "not ",
    "not as evidence",
    "unsupported",
    "unclaimed",
    "non-claim",
    "not yet implemented",
    "outside the claim",
    "outside the current claim",
    "outside the scope",
    "no ",
    "not a ",
    "not routed",
    "not arbitrary",
    "not exhaustive",
    "not unrestricted",
    "not generalized",
    "rather than broad",
    "rather than unrestricted",
    "limited to",
    "bounded",
    "limitation",
    "limitations",
    "threats",
    "claim boundary",
    "frozen non-claims",
    "known limitations",
    "use vivado only",
    "host-vivado",
    "remains unsupported",
    "remain unsupported",
    "remains outside",
    "remain outside",
    "must not",
    "never",
]


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


def is_disclaimed_context(lines: list[str], index: int) -> bool:
    start = max(0, index - 8)
    end = min(len(lines), index + 9)
    window = " ".join(lines[start:end]).lower()
    return any(marker in window for marker in DISCLAIMER_MARKERS)


def scan_unsupported_affirmative_claims() -> list[str]:
    errors: list[str] = []
    scan_paths = [REPO_ROOT / rel for rel in CLAIM_SCAN_FILES]
    scan_paths.extend(sorted((REPO_ROOT / "paper" / "sections").glob("*.tex")))
    for path in scan_paths:
        if not path.exists():
            errors.append(f"{path.relative_to(REPO_ROOT).as_posix()}: missing claim-scan target")
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        for index, line in enumerate(lines):
            for pattern, reason in UNSUPPORTED_CLAIM_PATTERNS:
                if re.search(pattern, line, flags=re.IGNORECASE) and not is_disclaimed_context(lines, index):
                    rel = path.relative_to(REPO_ROOT).as_posix()
                    errors.append(
                        f"{rel}:{index + 1}: unsupported affirmative claim ({reason}): {line.strip()!r}"
                    )
    return errors


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
        r"\b(83|46/46|40/40|37/37|40-task|40/40|20/20|30-task|30/30|15/15|12-task|32|24|nine-task)\b"
    )
    for path in sorted((REPO_ROOT / "paper" / "sections").glob("*.tex")):
        text = path.read_text(encoding="utf-8")
        if numeric_claim_re.search(text) and "release\\_claim\\_table.md" not in text:
            errors.append(f"{path.relative_to(REPO_ROOT).as_posix()}: numeric claims need release_claim_table mapping")
    return errors


def scan_stale_claims() -> list[str]:
    errors: list[str] = []
    checks: list[tuple[str, str, str]] = [
        ("PROJECT_MANIFEST.md", r"\b60-task\b", "public benchmark is now 83 tasks"),
        ("docs/13_architecture_audit.md", r"\b60-task\b", "public benchmark is now 83 tasks"),
        ("docs/14_reproduction_workflow.md", r"\b60-task\b", "public benchmark is now 83 tasks"),
        ("docs/13_architecture_audit.md", r"Twenty tasks,", "public directed simulations are now 36"),
        ("docs/14_reproduction_workflow.md", r"\b20 use committed directed\b", "public directed simulations are now 36"),
        ("docs/13_architecture_audit.md", r"\b20 use committed directed\b", "public directed simulations are now 36"),
        ("docs/14_reproduction_workflow.md", r"\b32 use committed directed\b", "public directed simulations are now 36"),
        ("docs/13_architecture_audit.md", r"\bThirty-two\s+tasks\b", "public directed simulations are now 36"),
        ("docs/07_module_compose_bench.md", r"\{declared:\s*32,\s*autogen:\s*4\}", "public simulations are now all declared"),
        ("docs/18_directed_verification_hardening.md", r"\{declared:\s*32,\s*autogen:\s*4\}", "public simulations are now all declared"),
        ("docs/18_directed_verification_hardening.md", r"\b4 generated ready/valid simulation\b", "public simulations are now all declared"),
        ("docs/23_heldout_benchmark_hardening.md", r"\{declared:\s*7,\s*autogen:\s*3\}", "held-out simulations are now all declared"),
        ("docs/14_reproduction_workflow.md", r"\b7 declared and 3 generated simulations\b", "held-out simulations are now all declared"),
        ("docs/13_architecture_audit.md", r"\bfourteen committed\b", "public directed formal monitors are now 31"),
        ("docs/14_reproduction_workflow.md", r"\b14 use committed\b", "public directed formal monitors are now 31"),
        ("docs/13_architecture_audit.md", r"\b14 use committed\b", "public directed formal monitors are now 31"),
        ("docs/13_architecture_audit.md", r"\btwenty-four committed\b", "public directed formal monitors are now 31"),
        ("docs/14_reproduction_workflow.md", r"\b24 use committed\b", "public directed formal monitors are now 31"),
        ("docs/07_module_compose_bench.md", r"\{declared:\s*24,\s*autogen:\s*7\}", "public formal checks are now all declared"),
        ("docs/18_directed_verification_hardening.md", r"\{declared:\s*24,\s*autogen:\s*7\}", "public formal checks are now all declared"),
        ("docs/18_directed_verification_hardening.md", r"\b7 generated ready/valid formal\b", "public formal checks are now all declared"),
        ("docs/23_heldout_benchmark_hardening.md", r"\{declared:\s*6,\s*autogen:\s*3\}", "held-out formal checks are now all declared"),
        ("docs/14_reproduction_workflow.md", r"\b6 declared and 3\s+generated single-clock formal checks\b", "held-out formal checks are now all declared"),
        ("docs/14_reproduction_workflow.md", r"\b12/12\b", "held-out split is now 20/20"),
        ("docs/14_reproduction_workflow.md", r"\b5/5\b", "held-out formal is now 9/9"),
        ("docs/13_architecture_audit.md", r"\bfour-task\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/14_reproduction_workflow.md", r"\bfour-task\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/13_architecture_audit.md", r"\bfour representative tasks\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/14_reproduction_workflow.md", r"\bfour wrappers\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("README.md", r"\bnine-task\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/13_architecture_audit.md", r"\bnine representative tasks\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/14_reproduction_workflow.md", r"\bnine representative tasks\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/dac2027_submission_plan.md", r"\b[Nn]ine-task\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/dac2027_submission_plan.md", r"\bnine representative\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/19_vivado_qor_subset.md", r"\bnine representative\b", "Vivado subset is now 21 split rows / 12 task pairs"),
        ("docs/release_claim_table.md", r"\b9 representative tasks\b", "Vivado subset is now 21 split rows / 12 task pairs"),
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
        "83 total, 46 positive, 37 negative",
        "46 declared, 0 generated",
        "40 declared, 0 generated",
        "40 total, 20 positive, 20 negative",
        "20 declared, 0 generated",
        "17 declared, 0 generated",
        "30 total, 15 positive, 15 negative",
        "15 declared, 0 generated",
        "13 declared, 0 generated",
        "21/21 reference-enabled split rows",
        "build/release/full_check_manifest.json",
        "build/release/deterministic_evidence_hashes.json",
        "mico.bench.results.v0",
        "mico.aggregate.results.v0",
        "mico.llm.bench.v0",
    ]
    return [f"docs/release_claim_table.md: missing {token!r}" for token in required_tokens if token not in text]


def check_manifests() -> list[str]:
    errors: list[str] = []
    public = task_summary(load_manifest("benchmarks/module_compose_bench_manifest.yaml"))
    heldout = task_summary(load_manifest("benchmarks/module_compose_bench_heldout.yaml"))
    realism = task_summary(load_manifest("benchmarks/module_compose_bench_realism.yaml"))

    errors += expect("public total", public["total"], 83)
    errors += expect("public positives", public["positive"], 46)
    errors += expect("public negatives", public["negative"], 37)
    errors += expect("public levels", public["levels"], {"L1": 11, "L2": 13, "L3": 10, "L4": 12, "L5": 18, "L6": 19})
    errors += expect("public declared simulations", public["declared_sim"], 46)
    errors += expect("public declared formal monitors", public["declared_formal"], 40)
    errors += expect("public QoR references", public["qor_reference"], 11)

    errors += expect("held-out total", heldout["total"], 40)
    errors += expect("held-out positives", heldout["positive"], 20)
    errors += expect("held-out negatives", heldout["negative"], 20)
    errors += expect("held-out declared simulations", heldout["declared_sim"], 20)
    errors += expect("held-out declared formal monitors", heldout["declared_formal"], 17)
    errors += expect("held-out QoR references", heldout["qor_reference"], 6)

    errors += expect("realism total", realism["total"], 30)
    errors += expect("realism positives", realism["positive"], 15)
    errors += expect("realism negatives", realism["negative"], 15)
    errors += expect("realism levels", realism["levels"], {"L1": 4, "L2": 5, "L3": 4, "L4": 5, "L5": 5, "L6": 7})
    errors += expect("realism declared simulations", realism["declared_sim"], 15)
    errors += expect("realism declared formal monitors", realism["declared_formal"], 13)
    errors += expect("realism QoR references", realism["qor_reference"], 4)
    return errors


def main() -> int:
    errors: list[str] = []
    errors.extend(check_manifests())
    errors.extend(check_claim_table())
    errors.extend(scan_required_references())
    errors.extend(scan_stale_claims())
    errors.extend(scan_unsupported_affirmative_claims())

    if errors:
        print("Documentation claim check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("documentation claim check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
