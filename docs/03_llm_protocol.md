# LLM Protocol

## Principle

The LLM is never the source of correctness. It proposes structured design edits. MICO validates them.

## Recommended loop

1. **Inventory**: provide modules, ports, protocols, domains.
2. **Schema proposal**: LLM groups primitive ports into MICO interfaces.
3. **Composition proposal**: LLM emits JSON AST for instances, connections, adapters.
4. **Check**: compiler returns structured diagnostics.
5. **Patch**: LLM emits a minimal patch against the AST.
6. **Verify**: lint/sim/formal/synthesis run.
7. **Explain**: compiler generates a traceability report.

## Provider policy

Use `$opencode-go-provider` for OpenCode Go model access. Provider settings live in `config/llm-provider.example.yaml`; copy it to ignored local config before adding credentials. Use an SDK client, preferably the OpenAI SDK, with the OpenAI-compatible base URL `https://opencode.ai/zen/go/v1`; keep the API key in `OPENCODE_GO_API_KEY` or an ignored local YAML file.

Repository-owned provider checks run through `scripts/llm-provider-smoke.py`. The script reads `base_url`, profile model settings, optional profile cost rates, and the API key source from YAML. It calls Chat Completions through the OpenAI Python SDK only when not in validate-only mode, and writes sanitized JSON output under ignored `build/llm/`.

LLM run records use `schemas/llm_run.schema.json` with schema version `mico.llm.run.v0`. They include:

- provider name, API root, config path, API key source, and whether a key was present;
- profile name, model, tier, prompt SHA-256, request settings, and response JSON validity;
- usage and estimated USD cost when local profile rates are configured;
- repair turn count;
- optional compiler diagnostic JSON and optional EDA result JSON, stored with source path and SHA-256.

The record never stores or prints the API key. Model price fields in the repository template are `null`; fill them only in ignored local config if a run needs cost estimates.

Validate configuration without a paid request:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
```

Write a sanitized validate-only metadata record without calling a model:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only --output build/llm/provider_validate.json
```

Run the cheap smoke profile:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --output build/llm/provider_smoke.json
```

Attach compiler and EDA results to an authenticated smoke run:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --compiler-result-json build/llm/compiler_result.json --eda-result-json build/bench/seed_results.json --repair-turns 1 --output build/llm/provider_smoke.json
```

Run early prompt, schema, and benchmark harness tests with low-cost profiles first:

1. `deepseek-v4-flash`
2. `mimo-v2.5`

Escalate to higher-cost profiles only after the low-cost runs pass compiler checks and relevant RTL validation:

1. `kimi-k2.7-code`
2. `glm-5.1`
3. `deepseek-v4-pro`

## Avoid

- Direct final Verilog generation as the only artifact.
- Unstructured free-text repair.
- Silent auto-CDC insertion.
- Silent adapter insertion that changes latency/backpressure semantics.

## JSON patch shape

```json
{
  "patch_id": "repair-001",
  "reason": "ClockDomainMismatch",
  "edits": [
    {
      "op": "replace_connection_with_adapter",
      "from": "dma.tx",
      "adapter": "AsyncFifo32",
      "to": "aes.rx"
    }
  ]
}
```

## Evaluation-specific prompts

See `prompts/`.
