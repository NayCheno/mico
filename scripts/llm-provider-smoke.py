#!/usr/bin/env python3
"""Validate and smoke-test the configured OpenAI-compatible LLM provider."""

from __future__ import annotations

import argparse
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


def repo_path(value: str) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict):
        raise ValueError("provider config must be a YAML mapping")
    return data


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


def usage_dict(response: Any) -> dict[str, int | None]:
    usage = getattr(response, "usage", None)
    return {
        "prompt_tokens": getattr(usage, "prompt_tokens", None),
        "completion_tokens": getattr(usage, "completion_tokens", None),
        "total_tokens": getattr(usage, "total_tokens", None),
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
) -> dict[str, Any]:
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
    return {
        "provider": provider.get("name", "unknown"),
        "api": provider.get("api"),
        "base_url": str(provider.get("base_url", "")).strip().rstrip("/"),
        "profile": profile_name,
        "model": profile["model"],
        "tier": profile.get("tier", "unknown"),
        "prompt_sha256": hashlib.sha256(prompt.encode("utf-8")).hexdigest(),
        "request": {
            "temperature": float(profile.get("temperature", 0.1)),
            "max_tokens": int(profile.get("max_tokens", 1024)),
        },
        "response": {
            "content": content,
            "json_valid": json_valid,
            "json": parsed_json,
        },
        "usage": usage_dict(response),
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
) -> None:
    escalation = policy.get("escalation_order", [])
    if not isinstance(escalation, list):
        escalation = []
    print(f"OK config={config_path.relative_to(REPO_ROOT) if config_path.is_relative_to(REPO_ROOT) else config_path}")
    print(f"provider={provider.get('name', 'unknown')} api={provider.get('api')}")
    print(f"profile={profile_name} model={profile['model']} tier={profile.get('tier', 'unknown')}")
    print(f"base_url={base_url}")
    print(f"api_key_source={key_source} api_key_present={api_key is not None}")
    print(f"escalation_order={','.join(str(item) for item in escalation)}")
    print("sdk=openai-python")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", default="config/llm-provider.local.yaml", help="YAML provider config")
    parser.add_argument("--profile", default=None, help="Profile name under profiles")
    parser.add_argument("--prompt", default=None, help="Inline smoke prompt")
    parser.add_argument("--prompt-file", default=str(DEFAULT_PROMPT.relative_to(REPO_ROOT)), help="Prompt file")
    parser.add_argument("--output", default="build/llm/provider_smoke.json", help="Sanitized JSON result path")
    parser.add_argument("--validate-only", action="store_true", help="Validate config without a model request")
    parser.add_argument("--no-require-json", action="store_true", help="Do not fail when the response is not JSON")
    args = parser.parse_args()

    config_path = repo_path(args.config)
    data = load_yaml(config_path)
    provider = require_mapping(data, "provider")
    policy = data.get("policy") if isinstance(data.get("policy"), dict) else {}
    base_url = validate_provider(provider)
    profile_name, profile = resolve_profile(data, args.profile)
    api_key, key_source = resolve_api_key(provider)

    if args.validate_only:
        print_validation(config_path, provider, policy, profile_name, profile, base_url, api_key, key_source)
        return 0

    if api_key is None:
        raise SystemExit(f"Missing API key. Set {key_source} or provider.api_key in an ignored local config.")

    prompt = prompt_text(args)
    client = OpenAI(
        api_key=api_key,
        base_url=base_url,
        default_headers={"User-Agent": "MICO LLM provider smoke test"},
    )
    result = smoke_completion(client, provider, profile_name, profile, prompt, not args.no_require_json)
    output = repo_path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {output.relative_to(REPO_ROOT) if output.is_relative_to(REPO_ROOT) else output}")
    print(f"profile={profile_name} model={profile['model']} json_valid={result['response']['json_valid']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
