---
name: opencode-go-provider
description: Configure and use OpenCode Go as an OpenAI-compatible LLM provider through an SDK, preferably the OpenAI SDK for Chat Completions. Use when setting base_url/model/api_key YAML config, selecting cheap versus expensive Go models, smoke-testing the provider, running LLM-assisted MICO/RTL evaluations, or updating prompt/model routing policy.
---

# OpenCode Go Provider

## Overview

Use this skill to route MICO LLM calls through OpenCode Go with an SDK-backed OpenAI-compatible Chat Completions client. Keep the provider external to correctness: models propose MICO JSON or patches, then the compiler and RTL/EDA checks decide whether outputs are accepted.

## SDK Rule

- Use an SDK client for provider calls. Prefer the OpenAI SDK with `base_url=https://opencode.ai/zen/go/v1`.
- Do not hand-build raw HTTP requests for normal provider use.
- Keep YAML responsible only for provider selection, model profile selection, and secret location.
- Add a separate adapter before using non-OpenAI-compatible APIs such as Anthropic-style `/messages`.

## Config Files

- Use `config/llm-provider.example.yaml` as the repository template.
- Put real credentials in `config/llm-provider.local.yaml` or in the `OPENCODE_GO_API_KEY` environment variable. Local config files are ignored by git.
- Keep the OpenAI-compatible base URL as `https://opencode.ai/zen/go/v1`; let the SDK select API paths.
- Use only OpenCode Go models that support the OpenAI-compatible chat completions endpoint for this provider.

Minimal local config:

```yaml
provider:
  name: opencode-go
  api: openai-chat-completions
  base_url: https://opencode.ai/zen/go/v1
  api_key_env: OPENCODE_GO_API_KEY
  api_key: null
profiles:
  smoke:
    model: deepseek-v4-flash
    temperature: 0.1
    max_tokens: 1024
```

## Model Escalation

Start cheap, then escalate only after objective gates pass:

1. `smoke`: `deepseek-v4-flash` for prompt syntax, schema, and harness checks.
2. `low_cost_crosscheck`: `mimo-v2.5` for a second low-cost model and regression sensitivity.
3. `quality_code`: `kimi-k2.7-code` after low-cost runs meet the pass-rate gate.
4. `quality_reasoning`: `glm-5.1` for hard ambiguous integration tasks.
5. `quality_deepseek`: `deepseek-v4-pro` as an alternate expensive comparison model.

Do not switch to high-cost profiles because a single answer looks good. Require compiler acceptance, structured-output validity, and relevant RTL validation from Docker or host Vivado before promotion.

## Workflow

1. Copy `config/llm-provider.example.yaml` to `config/llm-provider.local.yaml`.
2. Set `OPENCODE_GO_API_KEY` in the shell or fill `provider.api_key` only in the ignored local file.
3. Run a dry validation before any paid request:

```powershell
.\scripts\eda-docker.ps1 python3 .codex/skills/opencode-go-provider/scripts/opencode_go_smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
```

4. Run an authenticated smoke request with the cheap profile:

```powershell
.\scripts\eda-docker.ps1 python3 .codex/skills/opencode-go-provider/scripts/opencode_go_smoke.py --config config/llm-provider.local.yaml --profile smoke --prompt "Return JSON: {\"ok\": true}"
```

5. Run MICO benchmark prompts with `policy.escalation_order`, recording profile, model, prompt hash, compiler result, and RTL validation result.

## Reference Routing

- Read `references/opencode-go-workflow.md` when changing model policy, benchmark routing, request schema, or promotion gates.
- Read `scripts/opencode_go_smoke.py` when modifying provider validation or troubleshooting SDK calls.
- Read `assets/opencode-go-provider.example.yaml` only when copying a skill-local provider template.

## Constraints

- Never commit a real `api_key`.
- Do not treat LLM output as trusted RTL or trusted MICO IR.
- Keep temperature low for benchmark comparability.
- Use deterministic prompt templates under `prompts/`.
- Store benchmark scores using `benchmarks/scoring_schema.json`.
- Prefer Docker for provider smoke scripts so Python SDK and YAML dependencies are stable.

## Validation

```bash
python3 .codex/skills/opencode-go-provider/scripts/opencode_go_smoke.py --config config/llm-provider.example.yaml --profile smoke --validate-only
python3 .codex/skills/opencode-go-provider/scripts/opencode_go_smoke.py --config config/llm-provider.example.yaml --list-models
```
