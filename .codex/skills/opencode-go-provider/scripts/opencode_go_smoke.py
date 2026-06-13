#!/usr/bin/env python3
"""Validate and smoke-test OpenCode Go through the OpenAI Python SDK."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
from typing import Any

try:
    from openai import OpenAI
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("openai Python SDK is required. Run this through scripts/eda-docker.* after rebuilding the image.") from exc

try:
    import yaml
except ImportError as exc:  # pragma: no cover - environment guidance
    raise SystemExit("PyYAML is required. Run this through scripts/eda-docker.*.") from exc


def load_config(path: Path) -> dict[str, Any]:
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


def resolve_profile(data: dict[str, Any], profile_name: str | None) -> tuple[str, dict[str, Any]]:
    profiles = require_mapping(data, "profiles")
    if profile_name is None:
        policy = data.get("policy") if isinstance(data.get("policy"), dict) else {}
        profile_name = str(policy.get("default_profile", "smoke"))
    profile = profiles.get(profile_name)
    if not isinstance(profile, dict):
        available = ", ".join(sorted(profiles))
        raise ValueError(f"unknown profile '{profile_name}'. Available profiles: {available}")
    if not profile.get("model"):
        raise ValueError(f"profile '{profile_name}' must set model")
    return profile_name, profile


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


def build_client(provider: dict[str, Any], api_key: str | None, allow_missing_key: bool) -> OpenAI:
    base_url = str(provider.get("base_url", "")).strip().rstrip("/")
    if not base_url:
        raise ValueError("provider.base_url is required")
    if base_url.endswith("/chat/completions"):
        raise ValueError("provider.base_url must be the API root, for example https://opencode.ai/zen/go/v1")
    if api_key is None and not allow_missing_key:
        raise ValueError("api key is required for chat completion smoke requests")
    return OpenAI(
        api_key=api_key or "not-needed-for-public-model-list",
        base_url=base_url,
        default_headers={"User-Agent": "MICO OpenCode Go provider smoke test"},
    )


def model_ids(client: OpenAI) -> list[str]:
    models = client.models.list()
    return sorted(str(model.id) for model in models.data)


def smoke_completion(client: OpenAI, profile_name: str, profile: dict[str, Any], prompt: str) -> dict[str, Any]:
    response = client.chat.completions.create(
        model=str(profile["model"]),
        messages=[
            {"role": "system", "content": "You are a strict JSON-producing smoke-test assistant."},
            {"role": "user", "content": prompt},
        ],
        temperature=float(profile.get("temperature", 0.1)),
        max_tokens=int(profile.get("max_tokens", 1024)),
        stream=False,
    )
    content = response.choices[0].message.content if response.choices else ""
    return {"profile": profile_name, "model": profile["model"], "response": content}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--config", default="config/llm-provider.local.yaml", help="YAML provider config")
    parser.add_argument("--profile", default=None, help="Profile name under profiles")
    parser.add_argument("--prompt", default='Return JSON only: {"ok": true}', help="Smoke prompt")
    parser.add_argument("--validate-only", action="store_true", help="Validate config without a paid request")
    parser.add_argument("--list-models", action="store_true", help="Fetch and print available model ids through the SDK")
    args = parser.parse_args()

    config_path = Path(args.config)
    data = load_config(config_path)
    provider = require_mapping(data, "provider")
    if provider.get("api") != "openai-chat-completions":
        raise ValueError("provider.api must be openai-chat-completions")
    profile_name, profile = resolve_profile(data, args.profile)
    api_key, key_source = resolve_api_key(provider)
    base_url = str(provider.get("base_url", "")).strip().rstrip("/")

    if args.validate_only:
        print(f"OK config={config_path} profile={profile_name} model={profile['model']} base_url={base_url}")
        print("sdk=openai-python")
        print(f"api_key_source={key_source}")
        return 0

    client = build_client(provider, api_key, allow_missing_key=args.list_models)

    if args.list_models:
        for model_id in model_ids(client):
            print(model_id)
        return 0

    if api_key is None:
        raise SystemExit(f"Missing API key. Set {key_source} or provider.api_key in an ignored local config.")

    print(json.dumps(smoke_completion(client, profile_name, profile, args.prompt), ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
