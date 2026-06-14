#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

llm_config="${MICO_LLM_CONFIG:-config/llm-provider.local.yaml}"
profiles="${MICO_LLM_PROFILES:-smoke,low_cost_crosscheck}"
baselines="${MICO_LLM_BASELINES:-direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair}"
provider_profile="${MICO_LLM_PROVIDER_PROFILE:-smoke}"
release_manifest="${MICO_RELEASE_MANIFEST:-build/release/full_check_manifest.json}"

usage() {
    cat <<'EOF'
Usage: scripts/full-check.sh [options]

Runs the full MICO release-candidate gate inside the Docker EDA environment.

Options:
  --llm-config PATH       LLM provider config. Default: config/llm-provider.local.yaml
  --profiles LIST         Comma-separated LLM benchmark profiles.
  --baselines LIST        Comma-separated LLM benchmark baselines.
  --provider-profile NAME LLM provider smoke profile. Default: smoke
  --manifest PATH         Release manifest output. Default: build/release/full_check_manifest.json
  -h, --help              Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --llm-config)
            llm_config="$2"
            shift 2
            ;;
        --profiles)
            profiles="$2"
            shift 2
            ;;
        --baselines)
            baselines="$2"
            shift 2
            ;;
        --provider-profile)
            provider_profile="$2"
            shift 2
            ;;
        --manifest)
            release_manifest="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "ERROR: unknown argument: $1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ "${MICO_ALLOW_HOST_FULL_CHECK:-0}" != "1" && ! -f /.dockerenv ]]; then
    echo "ERROR: scripts/full-check.sh must run through scripts/eda-docker.* unless MICO_ALLOW_HOST_FULL_CHECK=1 is set." >&2
    exit 1
fi

if [[ ! -f "${llm_config}" ]]; then
    echo "ERROR: missing LLM config: ${llm_config}" >&2
    echo "Create an ignored local config from config/llm-provider.example.yaml before running the release gate." >&2
    exit 1
fi

run_step() {
    local label="$1"
    shift
    echo
    echo "== ${label} =="
    "$@"
}

run_step "Docker tool verification" mico-verify-tools
run_step "Rust fmt/check/test" bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
run_step "EDA smoke" bash scripts/eda-smoke.sh
run_step "Deterministic benchmark" python3 benchmarks/run_bench.py --output build/bench/seed_results.json
run_step "LLM provider validate-only" python3 scripts/llm-provider-smoke.py \
    --config "${llm_config}" \
    --profile "${provider_profile}" \
    --validate-only \
    --output build/llm/provider_validate.json
run_step "LLM batch validate-only" python3 scripts/run_llm_bench.py \
    --config "${llm_config}" \
    --profiles "${profiles}" \
    --baselines "${baselines}" \
    --output build/llm/bench_validate.json
run_step "Aggregate benchmark and LLM records" python3 benchmarks/aggregate_results.py \
    --bench-result build/bench/seed_results.json \
    --llm-result build/llm/bench_validate.json \
    --out-json build/bench/aggregate_results.json
run_step "JSON schema validation" python3 scripts/validate_json_schemas.py \
    --bench-result build/bench/seed_results.json \
    --llm-run build/llm/provider_validate.json \
    --llm-bench build/llm/bench_validate.json \
    --aggregate-result build/bench/aggregate_results.json

tracked_generated="$(
    git ls-files -- \
        build out reports target rust_project/target \
        'paper/*.pdf' 'config/*.local.yaml' \
        '*.log' '*.jou' '*.str' '*.wdb' '*.vcd' '*.fst' '*.fsdb' '*.dcp' '*.bit'
)"
if [[ -n "${tracked_generated}" ]]; then
    echo "ERROR: generated or local-only files are tracked by Git:" >&2
    echo "${tracked_generated}" >&2
    exit 1
fi

export MICO_FULL_CHECK_LLM_CONFIG="${llm_config}"
export MICO_FULL_CHECK_PROFILES="${profiles}"
export MICO_FULL_CHECK_BASELINES="${baselines}"
export MICO_FULL_CHECK_PROVIDER_PROFILE="${provider_profile}"
export MICO_RELEASE_MANIFEST="${release_manifest}"

run_step "Release manifest" python3 - <<'PY'
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import subprocess
from typing import Any

try:
    import yaml
except ImportError:  # pragma: no cover - checked by Docker tool verification
    yaml = None


repo = Path.cwd()


def display(path: Path) -> str:
    try:
        return path.relative_to(repo).as_posix()
    except ValueError:
        return str(path)


def run_text(cmd: list[str]) -> str:
    proc = subprocess.run(
        cmd,
        cwd=repo,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    return proc.stdout.strip()


def first_line(cmd: list[str]) -> str | None:
    text = run_text(cmd)
    if not text:
        return None
    return text.splitlines()[0]


def sha256_file(path: Path) -> str | None:
    if not path.exists() or not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def git_text(args: list[str]) -> str | None:
    try:
        proc = subprocess.run(
            ["git", *args],
            cwd=repo,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            check=False,
        )
    except FileNotFoundError:
        return None
    if proc.returncode != 0:
        return None
    return proc.stdout.strip()


def prompt_hashes() -> list[dict[str, str]]:
    root = repo / "prompts"
    rows = []
    if root.exists():
        for path in sorted(item for item in root.iterdir() if item.is_file()):
            digest = sha256_file(path)
            if digest is not None:
                rows.append({"path": display(path), "sha256": digest})
    return rows


def load_llm_metadata(path: Path) -> dict[str, Any]:
    metadata: dict[str, Any] = {
        "path": display(path),
        "exists": path.exists(),
        "provider": None,
        "policy": None,
        "selected_profiles": os.environ.get("MICO_FULL_CHECK_PROFILES", ""),
        "selected_baselines": os.environ.get("MICO_FULL_CHECK_BASELINES", ""),
        "provider_profile": os.environ.get("MICO_FULL_CHECK_PROVIDER_PROFILE", ""),
        "profiles": [],
    }
    if not path.exists() or yaml is None:
        return metadata
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        return metadata
    provider = data.get("provider") if isinstance(data.get("provider"), dict) else {}
    key_env = str(provider.get("api_key_env", "OPENCODE_GO_API_KEY"))
    metadata["provider"] = {
        "name": provider.get("name"),
        "api": provider.get("api"),
        "base_url": provider.get("base_url"),
        "api_key_env": key_env,
        "api_key_present": bool(provider.get("api_key") or os.environ.get(key_env)),
        "api_key_redacted": True,
    }
    policy = data.get("policy") if isinstance(data.get("policy"), dict) else {}
    metadata["policy"] = {
        "default_profile": policy.get("default_profile"),
        "escalation_order": policy.get("escalation_order", []),
    }
    profiles = data.get("profiles") if isinstance(data.get("profiles"), dict) else {}
    selected = [item.strip() for item in metadata["selected_profiles"].split(",") if item.strip()]
    for name in selected:
        profile = profiles.get(name)
        if isinstance(profile, dict):
            metadata["profiles"].append(
                {
                    "name": name,
                    "model": profile.get("model"),
                    "tier": profile.get("tier"),
                    "purpose": profile.get("purpose"),
                }
            )
    return metadata


result_paths = [
    "build/bench/seed_results.json",
    "build/llm/provider_validate.json",
    "build/llm/bench_validate.json",
    "build/bench/aggregate_results.json",
]

payload = {
    "schema_version": "mico.release.full_check.v0",
    "source_commit_hash": git_text(["rev-parse", "HEAD"]),
    "source_branch": git_text(["branch", "--show-current"]),
    "paper_commit_hash": git_text(["log", "-1", "--format=%H", "--", "paper"]),
    "worktree_status_short": (git_text(["status", "--short"]) or "").splitlines(),
    "environment": {
        "host_docker_version": os.environ.get("MICO_HOST_DOCKER_VERSION"),
        "container_os_release": first_line(["bash", "-lc", "source /etc/os-release && echo \"$PRETTY_NAME\""]),
    },
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
    "prompts": prompt_hashes(),
    "llm": load_llm_metadata(repo / os.environ["MICO_FULL_CHECK_LLM_CONFIG"]),
    "benchmark_manifest": {
        "path": "benchmarks/module_compose_bench_manifest.yaml",
        "sha256": sha256_file(repo / "benchmarks/module_compose_bench_manifest.yaml"),
    },
    "result_json_hashes": [
        {"path": rel, "sha256": sha256_file(repo / rel)}
        for rel in result_paths
    ],
    "generated_output_policy": {
        "tracked_generated_files": (git_text([
            "ls-files",
            "--",
            "build",
            "out",
            "reports",
            "target",
            "rust_project/target",
            "paper/*.pdf",
            "config/*.local.yaml",
        ]) or "").splitlines(),
    },
}

out = repo / os.environ["MICO_RELEASE_MANIFEST"]
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"wrote {display(out)}")
PY

echo
echo "MICO full release-candidate check passed"
