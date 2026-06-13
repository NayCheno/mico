# OpenCode Go Provider Workflow

## Source Facts

OpenCode Go exposes an OpenAI-compatible Chat Completions endpoint for GLM, Kimi, DeepSeek, and MiMo models at:

```text
https://opencode.ai/zen/go/v1/chat/completions
```

Use the base URL form in client configuration:

```text
https://opencode.ai/zen/go/v1
```

The public model list is:

```text
https://opencode.ai/zen/go/v1/models
```

OpenCode's Go documentation also lists Anthropic-style `/messages` endpoints for MiniMax and Qwen models. Do not put those models into this OpenAI-compatible provider unless an Anthropic-compatible adapter is added.

## Cost Policy

Use low-cost profiles first:

- `deepseek-v4-flash`: default smoke model.
- `mimo-v2.5`: low-cost crosscheck model.

Escalate only after compiler and RTL gates pass:

- `kimi-k2.7-code`: higher-cost code profile.
- `glm-5.1`: higher-cost reasoning profile.
- `deepseek-v4-pro`: higher-cost comparison profile.

As of the OpenCode Go documentation checked on 2026-06-14, the low-cost DeepSeek V4 Flash and MiMo V2.5 prices are much lower than Kimi K2.7 Code, GLM-5.1, and DeepSeek V4 Pro. Treat the documentation and `/models` endpoint as source of truth before changing model policy.

## SDK Request Shape

Use the OpenAI SDK rather than constructing HTTP requests manually:

```python
from openai import OpenAI

client = OpenAI(
    api_key=api_key,
    base_url="https://opencode.ai/zen/go/v1",
)

response = client.chat.completions.create(
    model="deepseek-v4-flash",
    messages=[
        {"role": "system", "content": "Return only valid JSON."},
        {"role": "user", "content": "..."},
    ],
    temperature=0.1,
    max_tokens=1024,
)
```

The equivalent Chat Completions payload is:

```json
{
  "model": "deepseek-v4-flash",
  "messages": [
    {
      "role": "system",
      "content": "Return only valid JSON."
    },
    {
      "role": "user",
      "content": "..."
    }
  ],
  "temperature": 0.1,
  "max_tokens": 1024,
  "stream": false
}
```

The SDK is responsible for transport, headers, retries, and API path handling. Do not hand-build requests unless debugging the SDK itself.

## MICO Evaluation Loop

1. Run prompt and parser smoke tests with `smoke`.
2. Run a small seed set with `smoke` and `low_cost_crosscheck`.
3. Compile every generated MICO artifact with `mico_cli check`.
4. Emit IR/SystemVerilog only after the MICO check passes.
5. Run RTL checks through the Docker EDA environment or host Vivado when required.
6. Record each result using `benchmarks/scoring_schema.json`.
7. Promote to high-cost profiles only when the low-cost pass rate and safety gates are met.

## Failure Handling

- On 401/403: check `OPENCODE_GO_API_KEY` or local config; do not print the key.
- On 404 model errors: refresh `https://opencode.ai/zen/go/v1/models` and update the YAML profile.
- On invalid JSON output: keep the same model profile and fix prompt/schema first.
- On compiler failure: feed structured diagnostics into `prompts/repair_prompt.md`; do not escalate cost until cheap repair loops are understood.
