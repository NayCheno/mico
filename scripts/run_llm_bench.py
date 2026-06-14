#!/usr/bin/env python3
"""Run or validate LLM baselines for ModuleComposeBench."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
from datetime import UTC, datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
from typing import Any

try:
    import yaml
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("PyYAML is required. Run this through scripts/eda-docker.*.") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]
SCHEMA_VERSION = "mico.llm.bench.v0"
DEFAULT_BASELINES = [
    "direct_verilog",
    "sv_interface",
    "mico_source",
    "mico_json_ast",
    "mico_json_ast_repair",
]
MICO_BASELINES = {"mico_source", "mico_json_ast", "mico_json_ast_repair"}
SV_BASELINES = {"direct_verilog", "sv_interface"}
DEFAULT_BASELINE_PROMPTS = REPO_ROOT / "prompts" / "llm_bench_baselines.yaml"


@dataclass(frozen=True)
class Profile:
    name: str
    data: dict[str, Any]


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def display_path(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT)).replace("\\", "/")
    except ValueError:
        return str(path)


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def sha256_text(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError(f"{display_path(path)} must be a YAML mapping")
    return data


def require_mapping(data: dict[str, Any], key: str) -> dict[str, Any]:
    value = data.get(key)
    if not isinstance(value, dict):
        raise ValueError(f"missing or invalid mapping: {key}")
    return value


def validate_provider(provider: dict[str, Any]) -> str:
    if provider.get("api") != "openai-chat-completions":
        raise ValueError("provider.api must be openai-chat-completions")
    base_url = str(provider.get("base_url", "")).strip().rstrip("/")
    if not base_url:
        raise ValueError("provider.base_url is required")
    if base_url.endswith("/chat/completions") or base_url.endswith("/models"):
        raise ValueError("provider.base_url must be the API root, not an endpoint path")
    return base_url


def resolve_api_key(provider: dict[str, Any]) -> tuple[str | None, str]:
    literal = provider.get("api_key")
    if isinstance(literal, str) and literal.strip():
        return literal.strip(), "provider.api_key"
    env_name = str(provider.get("api_key_env", "OPENCODE_GO_API_KEY")).strip()
    if env_name:
        value = os.environ.get(env_name)
        if value:
            return value, env_name
    return None, env_name or "provider.api_key"


def parse_csv(value: str | None, default: list[str]) -> list[str]:
    if value is None or not value.strip():
        return list(default)
    return [item.strip() for item in value.split(",") if item.strip()]


def resolve_profiles(config: dict[str, Any], names: list[str]) -> list[Profile]:
    profiles = require_mapping(config, "profiles")
    resolved = []
    for name in names:
        profile = profiles.get(name)
        if not isinstance(profile, dict):
            available = ", ".join(sorted(str(item) for item in profiles))
            raise ValueError(f"unknown profile '{name}'. Available profiles: {available}")
        if not profile.get("model"):
            raise ValueError(f"profile '{name}' must set model")
        resolved.append(Profile(name=name, data=profile))
    return resolved


def load_tasks(manifest_path: Path, task_ids: set[str] | None, limit: int | None) -> list[dict[str, Any]]:
    manifest = load_yaml(manifest_path)
    tasks = manifest.get("tasks")
    if not isinstance(tasks, list):
        raise ValueError("benchmark manifest must contain a tasks list")
    selected = []
    for task in tasks:
        if not isinstance(task, dict):
            raise ValueError("benchmark task must be a mapping")
        task_id = str(task.get("id", ""))
        if task_ids is not None and task_id not in task_ids:
            continue
        validate_task(task)
        selected.append(task)
        if limit is not None and len(selected) >= limit:
            break
    return selected


def validate_task(task: dict[str, Any]) -> None:
    task_id = task.get("id", "<unknown>")
    for key in [
        "id",
        "level",
        "type",
        "request",
        "mico_source",
        "rtl_collateral",
        "module_inventory",
        "interface_inventory",
        "adapter_inventory",
        "expected",
    ]:
        if key not in task:
            raise ValueError(f"task {task_id} is missing {key}")
    if task["type"] not in {"positive", "negative"}:
        raise ValueError(f"task {task_id} has invalid type")
    expected = task["expected"]
    if not isinstance(expected, dict) or "diagnostics" not in expected:
        raise ValueError(f"task {task_id} must include expected diagnostics")


def run(
    cmd: list[str],
    cwd: Path,
    stdout_path: Path | None = None,
    stderr_path: Path | None = None,
) -> subprocess.CompletedProcess[str]:
    stdout: int | Any = subprocess.PIPE
    stderr: int | Any = subprocess.PIPE
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
        return subprocess.run(cmd, cwd=cwd, stdout=stdout, stderr=stderr, text=True, check=False)
    finally:
        if stdout_handle is not None:
            stdout_handle.close()
        if stderr_handle is not None:
            stderr_handle.close()


def strip_compose(source: str) -> str:
    marker = "\ncompose "
    idx = source.find(marker)
    if idx == -1:
        return source.strip()
    return source[:idx].strip()


def task_source_text(task: dict[str, Any]) -> str:
    return read_text(repo_path(str(task["mico_source"])))


def load_baseline_instructions(path: Path) -> dict[str, str]:
    data = load_yaml(path)
    instructions = {}
    for key, value in data.items():
        if isinstance(key, str) and isinstance(value, str) and value.strip():
            instructions[key] = value.strip()
    missing = [baseline for baseline in DEFAULT_BASELINES if baseline not in instructions]
    if missing:
        raise ValueError(f"{display_path(path)} is missing baselines: {', '.join(missing)}")
    return instructions


def build_prompt(
    task: dict[str, Any],
    baseline: str,
    system_prompt: str,
    baseline_instructions: dict[str, str],
) -> str:
    source = task_source_text(task)
    inventory = strip_compose(source)
    prompt = {
        "task_id": task["id"],
        "level": task["level"],
        "request": task["request"],
        "module_inventory": task["module_inventory"],
        "interface_inventory": task["interface_inventory"],
        "adapter_inventory": task["adapter_inventory"],
        "interface_and_module_declarations": inventory,
        "baseline": baseline,
        "baseline_instruction": baseline_instructions[baseline],
        "required_top": "Top",
    }
    return (
        system_prompt.strip()
        + "\n\n## Baseline task\n"
        + json.dumps(prompt, indent=2, ensure_ascii=False)
    )


def parse_json_content(content: str) -> tuple[bool, Any, str | None]:
    text = content.strip()
    if text.startswith("```"):
        lines = text.splitlines()
        if lines and lines[0].startswith("```"):
            lines = lines[1:]
        if lines and lines[-1].strip().startswith("```"):
            lines = lines[:-1]
        text = "\n".join(lines).strip()
    try:
        return True, json.loads(text), None
    except json.JSONDecodeError as exc:
        extracted = extract_first_json_object(text)
        if extracted is not None and extracted != text:
            try:
                return True, json.loads(extracted), None
            except json.JSONDecodeError:
                pass
        return False, None, str(exc)


def extract_first_json_object(text: str) -> str | None:
    start = min((idx for idx in [text.find("{"), text.find("[")] if idx >= 0), default=-1)
    if start < 0:
        return None
    stack: list[str] = []
    in_string = False
    escape = False
    for idx in range(start, len(text)):
        char = text[idx]
        if in_string:
            if escape:
                escape = False
            elif char == "\\":
                escape = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char in "{[":
            stack.append("}" if char == "{" else "]")
        elif char in "}]":
            if not stack or stack[-1] != char:
                return None
            stack.pop()
            if not stack:
                return text[start : idx + 1]
    return None


def usage_dict(response: Any) -> dict[str, int | None]:
    usage = getattr(response, "usage", None)
    return {
        "prompt_tokens": getattr(usage, "prompt_tokens", None),
        "completion_tokens": getattr(usage, "completion_tokens", None),
        "total_tokens": getattr(usage, "total_tokens", None),
    }


def empty_usage() -> dict[str, int | None]:
    return {"prompt_tokens": None, "completion_tokens": None, "total_tokens": None}


def optional_float(value: Any) -> float | None:
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str) and value.strip():
        return float(value)
    return None


def estimate_cost(profile: dict[str, Any], usage: dict[str, int | None]) -> dict[str, Any]:
    cost_config = profile.get("cost") if isinstance(profile.get("cost"), dict) else {}
    input_rate = optional_float(cost_config.get("input_usd_per_1m_tokens"))
    output_rate = optional_float(cost_config.get("output_usd_per_1m_tokens"))
    prompt_tokens = usage.get("prompt_tokens")
    completion_tokens = usage.get("completion_tokens")
    estimated_usd = None
    source = "not_configured"
    if input_rate is not None and output_rate is not None:
        source = "profile.cost"
        if prompt_tokens is not None and completion_tokens is not None:
            estimated_usd = round(
                ((prompt_tokens * input_rate) + (completion_tokens * output_rate)) / 1_000_000,
                8,
            )
    return {
        "currency": "USD",
        "estimated_usd": estimated_usd,
        "source": source,
        "input_usd_per_1m_tokens": input_rate,
        "output_usd_per_1m_tokens": output_rate,
        "prompt_tokens": prompt_tokens,
        "completion_tokens": completion_tokens,
        "total_tokens": usage.get("total_tokens"),
    }


def cache_key(
    provider: dict[str, Any],
    base_url: str,
    profile: Profile,
    baseline: str,
    task_id: str,
    prompt: str,
) -> str:
    payload = {
        "provider": provider.get("name"),
        "base_url": base_url,
        "model": profile.data["model"],
        "temperature": float(profile.data.get("temperature", 0.1)),
        "max_tokens": int(profile.data.get("max_tokens", 1024)),
        "baseline": baseline,
        "task_id": task_id,
        "prompt_sha256": sha256_text(prompt),
    }
    return sha256_text(json.dumps(payload, sort_keys=True))[:24]


def request_model(
    *,
    api_key: str,
    base_url: str,
    profile: Profile,
    prompt: str,
    cache_path: Path,
    use_cache: bool,
) -> tuple[dict[str, Any], dict[str, int | None], bool]:
    if use_cache and cache_path.exists():
        cached = json.loads(read_text(cache_path))
        return cached["response"], cached["usage"], True

    try:
        from openai import OpenAI
    except ImportError as exc:  # pragma: no cover - environment guidance
        raise SystemExit("openai Python SDK is required. Run this through scripts/eda-docker.*.") from exc

    client = OpenAI(
        api_key=api_key,
        base_url=base_url,
        default_headers={"User-Agent": "MICO LLM benchmark runner"},
    )
    response = client.chat.completions.create(
        model=str(profile.data["model"]),
        messages=[
            {
                "role": "system",
                "content": "Return only valid JSON. Never include API keys or credentials.",
            },
            {"role": "user", "content": prompt},
        ],
        temperature=float(profile.data.get("temperature", 0.1)),
        max_tokens=int(profile.data.get("max_tokens", 1024)),
        stream=False,
    )
    content = response.choices[0].message.content if response.choices else ""
    json_valid, parsed, parse_error = parse_json_content(content or "")
    response_payload = {
        "requested": True,
        "content_sha256": sha256_text(content or ""),
        "json_valid": json_valid,
        "json": parsed,
        "parse_error": parse_error,
    }
    usage = usage_dict(response)
    cache_path.parent.mkdir(parents=True, exist_ok=True)
    cache_path.write_text(
        json.dumps({"response": response_payload, "usage": usage}, indent=2) + "\n",
        encoding="utf-8",
    )
    return response_payload, usage, False


def offline_response(task: dict[str, Any], baseline: str, artifact_dir: Path) -> tuple[dict[str, Any], dict[str, int | None]]:
    source = task_source_text(task)
    payload: dict[str, Any]
    if baseline in {"direct_verilog", "sv_interface"}:
        if task["type"] == "negative":
            payload = {"reject": True, "reason": "offline fixture rejects negative task"}
        else:
            sv_path = artifact_dir / "offline_top.sv"
            emit_source_sv(task, sv_path)
            payload = {"systemverilog": read_text(sv_path)}
    elif baseline == "mico_source":
        payload = {"mico_source": source}
    else:
        ast_path = artifact_dir / "offline_ast.json"
        dump_ast_from_source(task, ast_path)
        payload = {"mico_ast": json.loads(read_text(ast_path))}
    return (
        {
            "requested": False,
            "content_sha256": sha256_text(json.dumps(payload, sort_keys=True)),
            "json_valid": True,
            "json": payload,
            "parse_error": None,
        },
        empty_usage(),
    )


def source_arg(path: Path) -> str:
    rust_project = REPO_ROOT / "rust_project"
    try:
        return str(path.relative_to(rust_project)).replace("\\", "/")
    except ValueError:
        return str(path)


def dump_ast_from_source(task: dict[str, Any], output: Path) -> None:
    result = run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mico_cli",
            "--",
            "dump-ast-json",
            source_arg(repo_path(str(task["mico_source"]))),
        ],
        REPO_ROOT / "rust_project",
        stdout_path=output,
    )
    if result.returncode != 0:
        raise RuntimeError(f"dump-ast-json failed for {task['id']}")


def emit_source_sv(task: dict[str, Any], output: Path) -> None:
    result = run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mico_cli",
            "--",
            "emit-sv",
            source_arg(repo_path(str(task["mico_source"]))),
        ],
        REPO_ROOT / "rust_project",
        stdout_path=output,
    )
    if result.returncode != 0:
        raise RuntimeError(f"emit-sv failed for {task['id']}")


def diagnostic_codes(payload: dict[str, Any]) -> list[str]:
    diagnostics = payload.get("diagnostics", [])
    if not isinstance(diagnostics, list):
        return []
    return [
        item["code"]
        for item in diagnostics
        if isinstance(item, dict) and isinstance(item.get("code"), str)
    ]


def parse_json_file(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {}
    try:
        data = json.loads(read_text(path))
    except json.JSONDecodeError:
        return {}
    return data if isinstance(data, dict) else {}


def expected_diagnostics(task: dict[str, Any]) -> list[str]:
    expected = task.get("expected", {})
    diagnostics = expected.get("diagnostics", []) if isinstance(expected, dict) else []
    return [str(item) for item in diagnostics] if isinstance(diagnostics, list) else []


def evaluate_response(
    task: dict[str, Any],
    baseline: str,
    response: dict[str, Any],
    artifact_dir: Path,
    max_repair_turns: int,
    provider_call,
) -> tuple[dict[str, Any], dict[str, Any], list[dict[str, Any]]]:
    payload = response.get("json")
    if not isinstance(payload, dict):
        return not_run_compiler("response_json_invalid"), not_run_eda("response_json_invalid"), []
    if payload.get("reject") is True:
        compiler = not_run_compiler("model_rejected")
        compiler["unsafe_rejection"] = task["type"] == "negative"
        return compiler, not_run_eda("model_rejected"), []
    if baseline in SV_BASELINES:
        sv = payload.get("systemverilog") or payload.get("verilog")
        if not isinstance(sv, str) or not sv.strip():
            return not_run_compiler("missing_systemverilog"), not_run_eda("missing_systemverilog"), []
        sv_path = artifact_dir / "candidate.sv"
        sv_path.write_text(sv, encoding="utf-8")
        eda = evaluate_sv(task, sv_path, artifact_dir)
        return not_run_compiler("sv_baseline"), eda, []
    if baseline == "mico_source":
        mico = payload.get("mico_source") or payload.get("source")
        if not isinstance(mico, str) or not mico.strip():
            return not_run_compiler("missing_mico_source"), not_run_eda("missing_mico_source"), []
        source_path = artifact_dir / "candidate.mico"
        source_path.write_text(mico, encoding="utf-8")
        compiler = evaluate_mico_source(task, source_path, artifact_dir)
        eda = evaluate_mico_sv_if_possible(task, source_path, None, compiler, artifact_dir)
        return compiler, eda, []

    ast = payload.get("mico_ast") if isinstance(payload.get("mico_ast"), dict) else payload
    if not isinstance(ast, dict):
        return not_run_compiler("missing_mico_ast"), not_run_eda("missing_mico_ast"), []
    ast_path = artifact_dir / "candidate.ast.json"
    ast_path.write_text(json.dumps(ast, indent=2) + "\n", encoding="utf-8")
    compiler = evaluate_mico_json(task, ast_path, artifact_dir)
    repairs: list[dict[str, Any]] = []
    if baseline == "mico_json_ast_repair" and task["type"] == "positive" and not compiler["check_pass"]:
        current_ast = ast
        current_compiler = compiler
        for turn in range(1, max_repair_turns + 1):
            repair_prompt = build_repair_prompt(task, current_ast, current_compiler)
            repair_response, repair_usage, cache_hit = provider_call(repair_prompt, f"repair{turn}")
            repair_payload = repair_response.get("json")
            repair_record = {
                "turn": turn,
                "prompt_sha256": sha256_text(repair_prompt),
                "cache_hit": cache_hit,
                "usage": repair_usage,
                "response": repair_response,
                "applied": False,
                "compiler_result": None,
            }
            if isinstance(repair_payload, dict):
                patched = apply_repair_patch(current_ast, repair_payload, artifact_dir, turn)
                if patched is not None:
                    current_ast = patched
                    patched_path = artifact_dir / f"candidate.repair{turn}.ast.json"
                    patched_path.write_text(
                        json.dumps(current_ast, indent=2) + "\n",
                        encoding="utf-8",
                    )
                    current_compiler = evaluate_mico_json(task, patched_path, artifact_dir)
                    repair_record["applied"] = True
                    repair_record["compiler_result"] = current_compiler
                    compiler = current_compiler
                    ast_path = patched_path
                    if compiler["check_pass"]:
                        repairs.append(repair_record)
                        break
            repairs.append(repair_record)
    eda = evaluate_mico_sv_if_possible(task, None, ast_path, compiler, artifact_dir)
    return compiler, eda, repairs


def not_run_compiler(reason: str) -> dict[str, Any]:
    return {
        "status": "not_run",
        "reason": reason,
        "check_pass": False,
        "exit_code": None,
        "diagnostic_codes": [],
        "expected_diagnostics": [],
        "expected_diagnostic_match": False,
        "unsafe_rejection": False,
        "stdout": None,
        "stderr": None,
    }


def not_run_eda(reason: str) -> dict[str, Any]:
    return {
        "status": "not_run",
        "reason": reason,
        "lint_pass": False,
        "verilator_lint_pass": False,
        "iverilog_elab_pass": False,
        "yosys_elab_pass": False,
        "artifacts": {},
    }


def evaluate_mico_source(task: dict[str, Any], source: Path, artifact_dir: Path) -> dict[str, Any]:
    stdout = artifact_dir / "check.stdout.json"
    stderr = artifact_dir / "check.stderr.txt"
    result = run(
        ["cargo", "run", "-q", "-p", "mico_cli", "--", "check", "--format", "json", str(source)],
        REPO_ROOT / "rust_project",
        stdout_path=stdout,
        stderr_path=stderr,
    )
    return compiler_result_from_check(task, result.returncode, stdout, stderr)


def evaluate_mico_json(task: dict[str, Any], ast_path: Path, artifact_dir: Path) -> dict[str, Any]:
    stdout = artifact_dir / f"{ast_path.stem}.check.stdout.json"
    stderr = artifact_dir / f"{ast_path.stem}.check.stderr.txt"
    result = run(
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
            str(ast_path),
        ],
        REPO_ROOT / "rust_project",
        stdout_path=stdout,
        stderr_path=stderr,
    )
    return compiler_result_from_check(task, result.returncode, stdout, stderr)


def compiler_result_from_check(
    task: dict[str, Any],
    exit_code: int,
    stdout: Path,
    stderr: Path,
) -> dict[str, Any]:
    payload = parse_json_file(stdout)
    codes = diagnostic_codes(payload)
    expected = expected_diagnostics(task)
    expected_match = all(code in codes for code in expected)
    check_pass = exit_code == 0
    unsafe_rejection = task["type"] == "negative" and not check_pass and expected_match
    return {
        "status": "checked",
        "reason": None,
        "check_pass": check_pass,
        "exit_code": exit_code,
        "diagnostic_codes": codes,
        "expected_diagnostics": expected,
        "expected_diagnostic_match": expected_match,
        "unsafe_rejection": unsafe_rejection,
        "stdout": display_path(stdout),
        "stderr": display_path(stderr),
    }


def evaluate_mico_sv_if_possible(
    task: dict[str, Any],
    source_path: Path | None,
    ast_path: Path | None,
    compiler: dict[str, Any],
    artifact_dir: Path,
) -> dict[str, Any]:
    if task["type"] != "positive":
        return not_run_eda("negative_task")
    if not compiler.get("check_pass"):
        return not_run_eda("compiler_failed")
    wrapper = artifact_dir / "candidate.top.sv"
    if source_path is not None:
        cmd = ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit-sv", str(source_path)]
    elif ast_path is not None:
        cmd = ["cargo", "run", "-q", "-p", "mico_cli", "--", "emit-json-sv", str(ast_path)]
    else:
        return not_run_eda("missing_source")
    emit = run(cmd, REPO_ROOT / "rust_project", stdout_path=wrapper)
    if emit.returncode != 0:
        return not_run_eda("emit_sv_failed")
    return evaluate_sv(task, wrapper, artifact_dir)


def evaluate_sv(task: dict[str, Any], wrapper: Path, artifact_dir: Path) -> dict[str, Any]:
    if task["type"] != "positive":
        return not_run_eda("negative_task")
    rtl = repo_path(str(task["rtl_collateral"]))
    vvp = artifact_dir / "candidate.vvp"
    verilator_stdout = artifact_dir / "verilator.stdout.txt"
    verilator_stderr = artifact_dir / "verilator.stderr.txt"
    iverilog_stdout = artifact_dir / "iverilog.stdout.txt"
    iverilog_stderr = artifact_dir / "iverilog.stderr.txt"
    yosys_stdout = artifact_dir / "yosys.stdout.txt"
    yosys_stderr = artifact_dir / "yosys.stderr.txt"
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
        REPO_ROOT,
        stdout_path=verilator_stdout,
        stderr_path=verilator_stderr,
    )
    iverilog = run(
        ["iverilog", "-g2012", "-s", "Top", "-o", str(vvp), str(rtl), str(wrapper)],
        REPO_ROOT,
        stdout_path=iverilog_stdout,
        stderr_path=iverilog_stderr,
    )
    yosys_script = (
        f"read_verilog -sv {rtl.as_posix()} {wrapper.as_posix()}; "
        "hierarchy -check -top Top; proc; stat"
    )
    yosys = run(
        ["yosys", "-q", "-p", yosys_script],
        REPO_ROOT,
        stdout_path=yosys_stdout,
        stderr_path=yosys_stderr,
    )
    return {
        "status": "checked",
        "reason": None,
        "lint_pass": verilator.returncode == 0 and iverilog.returncode == 0 and yosys.returncode == 0,
        "verilator_lint_pass": verilator.returncode == 0,
        "iverilog_elab_pass": iverilog.returncode == 0,
        "yosys_elab_pass": yosys.returncode == 0,
        "artifacts": {
            "wrapper": display_path(wrapper),
            "rtl_collateral": display_path(rtl),
            "verilator_stdout": display_path(verilator_stdout),
            "verilator_stderr": display_path(verilator_stderr),
            "iverilog_stdout": display_path(iverilog_stdout),
            "iverilog_stderr": display_path(iverilog_stderr),
            "yosys_stdout": display_path(yosys_stdout),
            "yosys_stderr": display_path(yosys_stderr),
        },
    }


def build_repair_prompt(task: dict[str, Any], current_ast: dict[str, Any], compiler: dict[str, Any]) -> str:
    template = read_text(REPO_ROOT / "prompts" / "repair_prompt_template.md")
    replacements = {
        "{{task_description}}": str(task["request"]),
        "{{module_inventory}}": json.dumps(task["module_inventory"], indent=2),
        "{{interface_library}}": strip_compose(task_source_text(task)),
        "{{current_ast}}": json.dumps(current_ast, indent=2),
        "{{diagnostics}}": json.dumps(compiler, indent=2),
    }
    for key, value in replacements.items():
        template = template.replace(key, value)
    return template


def apply_repair_patch(
    ast: dict[str, Any],
    patch: dict[str, Any],
    artifact_dir: Path,
    turn: int,
) -> dict[str, Any] | None:
    ast_path = artifact_dir / f"repair{turn}.input.ast.json"
    patch_path = artifact_dir / f"repair{turn}.patch.json"
    stdout_path = artifact_dir / f"repair{turn}.apply.stdout.json"
    stderr_path = artifact_dir / f"repair{turn}.apply.stderr.txt"
    artifact_dir.mkdir(parents=True, exist_ok=True)
    ast_path.write_text(json.dumps(ast, indent=2) + "\n", encoding="utf-8")
    patch_path.write_text(json.dumps(patch, indent=2) + "\n", encoding="utf-8")
    result = run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mico_cli",
            "--",
            "repair-json",
            "--apply",
            "--json",
            source_arg(ast_path),
            source_arg(patch_path),
        ],
        REPO_ROOT / "rust_project",
        stdout_path=stdout_path,
        stderr_path=stderr_path,
    )
    response = parse_json_file(stdout_path)
    if response.get("phase") != "check":
        return None
    if result.returncode not in {0, 1}:
        return None
    parsed = parse_json_file(ast_path)
    return parsed or None


def summarize(results: list[dict[str, Any]]) -> dict[str, Any]:
    def count(predicate) -> int:
        return sum(1 for item in results if predicate(item))

    positive = [item for item in results if item["task_type"] == "positive"]
    negative = [item for item in results if item["task_type"] == "negative"]
    mico_results = [item for item in results if item["baseline"] in MICO_BASELINES]
    sv_results = [item for item in results if item["baseline"] in SV_BASELINES]
    return {
        "attempts": len(results),
        "positive_attempts": len(positive),
        "negative_attempts": len(negative),
        "provider_requests": count(lambda item: item["response"]["requested"] is True),
        "cache_hits": count(lambda item: item["cache_hit"] is True),
        "response_json_valid": count(lambda item: item["response"]["json_valid"] is True),
        "mico_compiler_pass": {
            "passed": count(
                lambda item: item["baseline"] in MICO_BASELINES
                and item["task_type"] == "positive"
                and item["compiler_result"]["check_pass"] is True
            ),
            "total": len([item for item in mico_results if item["task_type"] == "positive"]),
        },
        "sv_lint_pass": {
            "passed": count(
                lambda item: item["task_type"] == "positive"
                and item["eda_result"]["lint_pass"] is True
            ),
            "total": len(positive),
        },
        "unsafe_rejection": {
            "passed": count(
                lambda item: item["task_type"] == "negative"
                and item["baseline"] in MICO_BASELINES
                and item["compiler_result"].get("unsafe_rejection") is True
            ),
            "total": len([item for item in negative if item["baseline"] in MICO_BASELINES]),
        },
        "baselines": sorted({str(item["baseline"]) for item in results}),
        "profiles": sorted({str(item["profile"]["name"]) for item in results}),
        "sv_baseline_attempts": len(sv_results),
        "mico_baseline_attempts": len(mico_results),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", default="config/llm-provider.local.yaml")
    parser.add_argument("--manifest", default="benchmarks/module_compose_bench_manifest.yaml")
    parser.add_argument("--profiles", default="smoke,low_cost_crosscheck")
    parser.add_argument("--baselines", default=",".join(DEFAULT_BASELINES))
    parser.add_argument("--task-id", action="append", default=None)
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--output", default="build/llm/bench_results.json")
    parser.add_argument("--cache-dir", default="build/llm/cache")
    parser.add_argument(
        "--baseline-prompts",
        default=str(DEFAULT_BASELINE_PROMPTS.relative_to(REPO_ROOT)),
    )
    parser.add_argument("--no-cache", action="store_true")
    parser.add_argument("--execute", action="store_true", help="make provider requests")
    parser.add_argument("--offline-fixture", action="store_true", help="use task fixtures instead of provider requests")
    parser.add_argument("--max-repair-turns", type=int, default=3)
    args = parser.parse_args()

    if args.execute and args.offline_fixture:
        raise SystemExit("--execute and --offline-fixture are mutually exclusive")
    if args.limit is not None and args.limit <= 0:
        raise SystemExit("--limit must be positive")
    if args.max_repair_turns < 0:
        raise SystemExit("--max-repair-turns must be non-negative")

    config_path = repo_path(args.config)
    manifest_path = repo_path(args.manifest)
    config = load_yaml(config_path)
    provider = require_mapping(config, "provider")
    policy = config.get("policy") if isinstance(config.get("policy"), dict) else {}
    base_url = validate_provider(provider)
    api_key, key_source = resolve_api_key(provider)
    profile_names = parse_csv(args.profiles, ["smoke", "low_cost_crosscheck"])
    baseline_names = parse_csv(args.baselines, DEFAULT_BASELINES)
    baseline_instructions = load_baseline_instructions(repo_path(args.baseline_prompts))
    for baseline in baseline_names:
        if baseline not in baseline_instructions:
            raise SystemExit(f"unknown baseline: {baseline}")
    profiles = resolve_profiles(config, profile_names)
    task_ids = set(args.task_id) if args.task_id else None
    tasks = load_tasks(manifest_path, task_ids, args.limit)
    if args.execute and api_key is None:
        raise SystemExit(f"Missing API key. Set {key_source} or provider.api_key in an ignored local config.")

    mode = "execute" if args.execute else "offline_fixture" if args.offline_fixture else "validate_only"
    system_prompt = read_text(REPO_ROOT / "prompts" / "system_prompt_compose_agent.md")
    run_id = sha256_text(
        "|".join(
            [
                SCHEMA_VERSION,
                mode,
                utc_now(),
                str(manifest_path),
                ",".join(profile_names),
                ",".join(baseline_names),
            ]
        )
    )[:16]
    output = repo_path(args.output)
    build_root = output.parent / run_id
    cache_dir = repo_path(args.cache_dir)
    results: list[dict[str, Any]] = []

    for task in tasks:
        for profile in profiles:
            for baseline in baseline_names:
                prompt = build_prompt(task, baseline, system_prompt, baseline_instructions)
                prompt_sha = sha256_text(prompt)
                task_id = str(task["id"])
                artifact_dir = build_root / task_id / profile.name / baseline
                artifact_dir.mkdir(parents=True, exist_ok=True)
                prompt_path = artifact_dir / "prompt.txt"
                prompt_path.write_text(prompt, encoding="utf-8")
                key = cache_key(provider, base_url, profile, baseline, task_id, prompt)
                cache_path = cache_dir / f"{key}.json"

                def provider_call(repair_prompt: str, suffix: str):
                    repair_key = sha256_text(key + "|" + suffix + "|" + sha256_text(repair_prompt))[:24]
                    repair_cache = cache_dir / f"{repair_key}.json"
                    if not args.execute:
                        payload = {
                            "reject": True,
                            "reason": "repair not requested in non-execute mode",
                        }
                        return (
                            {
                                "requested": False,
                                "content_sha256": sha256_text(json.dumps(payload, sort_keys=True)),
                                "json_valid": True,
                                "json": payload,
                                "parse_error": None,
                            },
                            empty_usage(),
                            False,
                        )
                    assert api_key is not None
                    return request_model(
                        api_key=api_key,
                        base_url=base_url,
                        profile=profile,
                        prompt=repair_prompt,
                        cache_path=repair_cache,
                        use_cache=not args.no_cache,
                    )

                cache_hit = False
                if mode == "validate_only":
                    response = {
                        "requested": False,
                        "content_sha256": None,
                        "json_valid": None,
                        "json": None,
                        "parse_error": None,
                    }
                    usage = empty_usage()
                    compiler = not_run_compiler("validate_only")
                    eda = not_run_eda("validate_only")
                    repairs: list[dict[str, Any]] = []
                elif mode == "offline_fixture":
                    response, usage = offline_response(task, baseline, artifact_dir)
                    compiler, eda, repairs = evaluate_response(
                        task,
                        baseline,
                        response,
                        artifact_dir,
                        args.max_repair_turns,
                        provider_call,
                    )
                else:
                    assert api_key is not None
                    response, usage, cache_hit = request_model(
                        api_key=api_key,
                        base_url=base_url,
                        profile=profile,
                        prompt=prompt,
                        cache_path=cache_path,
                        use_cache=not args.no_cache,
                    )
                    compiler, eda, repairs = evaluate_response(
                        task,
                        baseline,
                        response,
                        artifact_dir,
                        args.max_repair_turns,
                        provider_call,
                    )

                results.append(
                    {
                        "task_id": task_id,
                        "task_type": task["type"],
                        "level": task["level"],
                        "baseline": baseline,
                        "profile": {
                            "name": profile.name,
                            "model": profile.data["model"],
                            "tier": profile.data.get("tier", "unknown"),
                        },
                        "prompt": {
                            "path": display_path(prompt_path),
                            "sha256": prompt_sha,
                            "bytes": len(prompt.encode("utf-8")),
                        },
                        "request": {
                            "temperature": float(profile.data.get("temperature", 0.1)),
                            "max_tokens": int(profile.data.get("max_tokens", 1024)),
                        },
                        "response": response,
                        "cache_key": key,
                        "cache_hit": cache_hit,
                        "usage": usage,
                        "cost": estimate_cost(profile.data, usage),
                        "compiler_result": compiler,
                        "eda_result": eda,
                        "repair": {
                            "max_turns": args.max_repair_turns,
                            "turns": len(repairs),
                            "records": repairs,
                        },
                        "artifacts": {
                            "dir": display_path(artifact_dir),
                            "mico_source": str(task["mico_source"]),
                            "rtl_collateral": str(task["rtl_collateral"]),
                        },
                    }
                )

    payload = {
        "schema_version": SCHEMA_VERSION,
        "run": {
            "id": run_id,
            "mode": mode,
            "timestamp_utc": utc_now(),
            "sdk": "openai-python",
            "manifest": display_path(manifest_path),
            "config": display_path(config_path),
            "profiles": profile_names,
            "baselines": baseline_names,
            "max_repair_turns": args.max_repair_turns,
            "baseline_prompts": display_path(repo_path(args.baseline_prompts)),
        },
        "provider": {
            "name": provider.get("name", "unknown"),
            "api": provider.get("api"),
            "base_url": base_url,
            "api_key_source": key_source,
            "api_key_present": api_key is not None,
            "api_key_redacted": True,
        },
        "policy": {
            "default_profile": policy.get("default_profile"),
            "escalation_order": [str(item) for item in policy.get("escalation_order", [])]
            if isinstance(policy.get("escalation_order", []), list)
            else [],
        },
        "summary": summarize(results),
        "results": results,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {display_path(output)}")
    print(f"mode={mode}")
    print(f"tasks={len(tasks)} profiles={len(profiles)} baselines={len(baseline_names)}")
    print(f"attempts={payload['summary']['attempts']}")
    print(f"provider_requests={payload['summary']['provider_requests']}")
    print(
        "mico_compiler_pass: "
        f"{payload['summary']['mico_compiler_pass']['passed']}/"
        f"{payload['summary']['mico_compiler_pass']['total']}"
    )
    print(
        "unsafe_rejection: "
        f"{payload['summary']['unsafe_rejection']['passed']}/"
        f"{payload['summary']['unsafe_rejection']['total']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
