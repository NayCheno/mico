#!/usr/bin/env python3
"""Validate and smoke-test the configured OpenAI-compatible LLM provider."""

from __future__ import annotations

import argparse
from datetime import UTC, datetime
import hashlib
import json
import os
from pathlib import Path
from typing import Any

try:
    from openai import OpenAI
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("openai Python SDK is required. Run this through scripts/eda-docker.*.") from exc

try:
    import yaml
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("PyYAML is required. Run this through scripts/eda-docker.*.") from exc


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PROMPT = REPO_ROOT / "prompts" / "provider_smoke_prompt.md"
DEFAULT_OUTPUT = "build/llm/provider_smoke.json"
SCHEMA_VERSION = "mico.llm.run.v0"


def repo_path(value: str) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError("provider config must be a YAML mapping")
    return data


def display_path(path: Path) -> str:
    return str(path.relative_to(REPO_ROOT) if path.is_relative_to(REPO_ROOT) else path)


def sha256_text(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def require_mapping(data: dict[str, Any], key: str) -> dict[str, Any]:
    value = data.get(key)
    if not isinstance(value, dict):
        raise ValueError(f"missing or invalid mapping: {key}")
    return value


def resolve_profile(data: dict[str, Any], requested: str | None) -> tuple[str, dict[str, Any]]:
    profiles = require_mapping(data, "profiles")
    if requested is None:
        policy = data.get("policy") if isinstance(data.get("policy"), dict) else {}
        requested = str(policy.get("default_profile", "smoke"))
    profile = profiles.get(requested)
    if not isinstance(profile, dict):
        available = ", ".join(sorted(str(name) for name in profiles))
        raise ValueError(f"unknown profile '{requested}'. Available profiles: {available}")
    if not profile.get("model"):
        raise ValueError(f"profile '{requested}' must set model")
    return requested, profile


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


def validate_provider(provider: dict[str, Any]) -> str:
    if provider.get("api") != "openai-chat-completions":
        raise ValueError("provider.api must be openai-chat-completions")
    base_url = str(provider.get("base_url", "")).strip().rstrip("/")
    if not base_url:
        raise ValueError("provider.base_url is required")
    if base_url.endswith("/chat/completions") or base_url.endswith("/models"):
        raise ValueError("provider.base_url must be the API root, not an endpoint path")
    return base_url


def prompt_text(args: argparse.Namespace) -> str:
    if args.prompt is not None:
        return args.prompt
    path = repo_path(args.prompt_file)
    return path.read_text(encoding="utf-8")


def prompt_metadata(args: argparse.Namespace, prompt: str) -> dict[str, Any]:
    if args.prompt is not None:
        return {
            "source": "inline",
            "path": None,
            "sha256": sha256_text(prompt),
            "bytes": len(prompt.encode("utf-8")),
        }
    path = repo_path(args.prompt_file)
    return {
        "source": "file",
        "path": display_path(path),
        "sha256": sha256_text(prompt),
        "bytes": len(prompt.encode("utf-8")),
    }


def load_json_artifact(path_value: str | None) -> dict[str, Any]:
    if path_value is None:
        return {
            "status": "not_run",
            "source": None,
            "sha256": None,
            "data": None,
        }
    path = repo_path(path_value)
    raw = path.read_bytes()
    try:
        data = json.loads(raw.decode("utf-8"))
    except json.JSONDecodeError as exc:
        raise ValueError(f"{display_path(path)} is not valid JSON: {exc}") from exc
    return {
        "status": "recorded",
        "source": display_path(path),
        "sha256": sha256_bytes(raw),
        "data": data,
    }


def usage_dict(response: Any) -> dict[str, int | None]:
    usage = getattr(response, "usage", None)
    return {
        "prompt_tokens": getattr(usage, "prompt_tokens", None),
        "completion_tokens": getattr(usage, "completion_tokens", None),
        "total_tokens": getattr(usage, "total_tokens", None),
    }


def empty_usage() -> dict[str, int | None]:
    return {
        "prompt_tokens": None,
        "completion_tokens": None,
        "total_tokens": None,
    }


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


def parse_json_content(content: str, require_json: bool) -> tuple[bool, Any]:
    try:
        return True, json.loads(content)
    except json.JSONDecodeError as exc:
        if require_json:
            raise ValueError(f"provider response was not valid JSON: {exc}") from exc
        return False, None


def smoke_completion(
    client: OpenAI,
    provider: dict[str, Any],
    profile_name: str,
    profile: dict[str, Any],
    prompt: str,
    require_json: bool,
) -> tuple[dict[str, Any], dict[str, int | None]]:
    response = client.chat.completions.create(
        model=str(profile["model"]),
        messages=[
            {
                "role": "system",
                "content": "You are a MICO provider smoke-test assistant. Return only valid JSON.",
            },
            {"role": "user", "content": prompt},
        ],
        temperature=float(profile.get("temperature", 0.1)),
        max_tokens=int(profile.get("max_tokens", 1024)),
        stream=False,
    )
    content = response.choices[0].message.content if response.choices else ""
    json_valid, parsed_json = parse_json_content(content or "", require_json)
    return (
        {
            "requested": True,
            "content": content,
            "content_sha256": sha256_text(content or ""),
            "json_valid": json_valid,
            "json": parsed_json,
        },
        usage_dict(response),
    )


def response_not_requested() -> dict[str, Any]:
    return {
        "requested": False,
        "content": None,
        "content_sha256": None,
        "json_valid": None,
        "json": None,
    }


def run_record(
    *,
    mode: str,
    config_path: Path,
    provider: dict[str, Any],
    policy: dict[str, Any],
    profile_name: str,
    profile: dict[str, Any],
    base_url: str,
    api_key: str | None,
    key_source: str,
    prompt: str,
    args: argparse.Namespace,
    response: dict[str, Any],
    usage: dict[str, int | None],
    compiler_result: dict[str, Any],
    eda_result: dict[str, Any],
) -> dict[str, Any]:
    escalation = policy.get("escalation_order", [])
    if not isinstance(escalation, list):
        escalation = []
    timestamp = utc_now()
    run_id = sha256_text(
        "|".join(
            [
                SCHEMA_VERSION,
                mode,
                timestamp,
                str(provider.get("name", "unknown")),
                profile_name,
                str(profile["model"]),
                sha256_text(prompt),
            ]
        )
    )[:16]
    request = {
        "temperature": float(profile.get("temperature", 0.1)),
        "max_tokens": int(profile.get("max_tokens", 1024)),
        "require_json": not args.no_require_json,
    }
    return {
        "schema_version": SCHEMA_VERSION,
        "run": {
            "id": run_id,
            "mode": mode,
            "timestamp_utc": timestamp,
            "sdk": "openai-python",
        },
        "provider": {
            "name": provider.get("name", "unknown"),
            "api": provider.get("api"),
            "base_url": base_url,
            "config": display_path(config_path),
            "api_key_source": key_source,
            "api_key_present": api_key is not None,
            "api_key_redacted": True,
        },
        "policy": {
            "default_profile": policy.get("default_profile"),
            "escalation_order": [str(item) for item in escalation],
        },
        "profile": {
            "name": profile_name,
            "model": profile["model"],
            "tier": profile.get("tier", "unknown"),
            "purpose": profile.get("purpose"),
        },
        "prompt": prompt_metadata(args, prompt),
        "request": request,
        "response": response,
        "usage": usage,
        "cost": estimate_cost(profile, usage),
        "repair": {
            "turns": args.repair_turns,
        },
        "compiler_result": compiler_result,
        "eda_result": eda_result,
    }


def print_validation(
    config_path: Path,
    provider: dict[str, Any],
    policy: dict[str, Any],
    profile_name: str,
    profile: dict[str, Any],
    base_url: str,
    api_key: str | None,
    key_source: str,
    prompt: str,
    args: argparse.Namespace,
) -> None:
    escalation = policy.get("escalation_order", [])
    if not isinstance(escalation, list):
        escalation = []
    print(f"OK config={display_path(config_path)}")
    print(f"schema={SCHEMA_VERSION}")
    print(f"provider={provider.get('name', 'unknown')} api={provider.get('api')}")
    print(f"profile={profile_name} model={profile['model']} tier={profile.get('tier', 'unknown')}")
    print(f"base_url={base_url}")
    print(f"api_key_source={key_source} api_key_present={api_key is not None}")
    print(f"prompt_sha256={sha256_text(prompt)}")
    print(f"repair_turns={args.repair_turns}")
    print(f"cost_source={estimate_cost(profile, empty_usage())['source']}")
    print(f"escalation_order={','.join(str(item) for item in escalation)}")
    print("sdk=openai-python")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", default="config/llm-provider.local.yaml", help="YAML provider config")
    parser.add_argument("--profile", default=None, help="Profile name under profiles")
    parser.add_argument("--prompt", default=None, help="Inline smoke prompt")
    parser.add_argument("--prompt-file", default=str(DEFAULT_PROMPT.relative_to(REPO_ROOT)), help="Prompt file")
    parser.add_argument("--output", default=None, help="Sanitized JSON result path")
    parser.add_argument("--validate-only", action="store_true", help="Validate config without a model request")
    parser.add_argument("--no-require-json", action="store_true", help="Do not fail when the response is not JSON")
    parser.add_argument("--repair-turns", type=int, default=0, help="Repair turns represented by this run")
    parser.add_argument("--compiler-result-json", default=None, help="Optional compiler diagnostic JSON to attach")
    parser.add_argument("--eda-result-json", default=None, help="Optional EDA result JSON to attach")
    args = parser.parse_args()
    if args.repair_turns < 0:
        raise SystemExit("--repair-turns must be non-negative")

    config_path = repo_path(args.config)
    data = load_yaml(config_path)
    provider = require_mapping(data, "provider")
    policy = data.get("policy") if isinstance(data.get("policy"), dict) else {}
    base_url = validate_provider(provider)
    profile_name, profile = resolve_profile(data, args.profile)
    api_key, key_source = resolve_api_key(provider)
    prompt = prompt_text(args)
    compiler_result = load_json_artifact(args.compiler_result_json)
    eda_result = load_json_artifact(args.eda_result_json)

    if args.validate_only:
        print_validation(
            config_path,
            provider,
            policy,
            profile_name,
            profile,
            base_url,
            api_key,
            key_source,
            prompt,
            args,
        )
        if args.output:
            result = run_record(
                mode="validate_only",
                config_path=config_path,
                provider=provider,
                policy=policy,
                profile_name=profile_name,
                profile=profile,
                base_url=base_url,
                api_key=api_key,
                key_source=key_source,
                prompt=prompt,
                args=args,
                response=response_not_requested(),
                usage=empty_usage(),
                compiler_result=compiler_result,
                eda_result=eda_result,
            )
            output = repo_path(args.output)
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
            print(f"wrote {display_path(output)}")
        return 0

    if api_key is None:
        raise SystemExit(f"Missing API key. Set {key_source} or provider.api_key in an ignored local config.")

    client = OpenAI(
        api_key=api_key,
        base_url=base_url,
        default_headers={"User-Agent": "MICO LLM provider smoke test"},
    )
    response, usage = smoke_completion(client, provider, profile_name, profile, prompt, not args.no_require_json)
    result = run_record(
        mode="smoke_request",
        config_path=config_path,
        provider=provider,
        policy=policy,
        profile_name=profile_name,
        profile=profile,
        base_url=base_url,
        api_key=api_key,
        key_source=key_source,
        prompt=prompt,
        args=args,
        response=response,
        usage=usage,
        compiler_result=compiler_result,
        eda_result=eda_result,
    )
    output = repo_path(args.output or DEFAULT_OUTPUT)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {display_path(output)}")
    print(f"profile={profile_name} model={profile['model']} json_valid={result['response']['json_valid']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
