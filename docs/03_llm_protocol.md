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

Benchmark profiles default to `response_format: json_object`; profiles may set
`response_format: null` or `response_format: none` only for a provider that
cannot support JSON object mode. The batch runner passes JSON object mode as
the OpenAI-compatible Chat Completions response format, includes it in the cache
key, and records it in sanitized result metadata. If a provider rejects that
option, fix provider/model routing before recording a scored matrix.

The runner also applies a baseline-aware output budget so local configs do not
accidentally truncate structured outputs: MICO JSON AST baselines receive at
least 4096 output tokens, MICO source baselines receive at least 2048, repair
turns receive at least 2048, and direct RTL baselines use the configured profile
budget.

Provider/model quirks are normalized in the runner when they are required for a
valid request. The Kimi code profile currently uses an effective temperature of
`1.0` because that model rejects lower temperature values.

Batch benchmark runs use `scripts/run_llm_bench.py`. The runner reads the
selected ModuleComposeBench manifest, defaulting to the current expanded
83-task public-development manifest, generates deterministic prompts from
`prompts/system_prompt_compose_agent.md` and
`prompts/llm_bench_baselines.yaml`, supports five baselines, caches provider
responses by prompt/profile/model hash, evaluates MICO outputs through the
compiler, runs open-source lint/elaboration for accepted positive candidates,
and writes sanitized `mico.llm.bench.v0` output. Current authenticated
submission claims use the expanded v4 public-development, held-out, and realism
records in `docs/26_llm_matrix_v4.md`; historical v3 records are retained only
as pre-expansion audit evidence in `docs/24_llm_matrix_v3.md`.

JSON AST repair turns use the repository-owned compiler path:
`mico_cli repair-json --apply --json <ast.json> <patch.json>`. The runner
writes each model patch to an ignored artifact file, invokes that CLI command,
and then re-runs the normal compiler/EDA scoring path on the patched AST. This
keeps repair semantics aligned with `schemas/mico_repair_patch.schema.json`.

Use `benchmarks/aggregate_results.py` to merge one or more sanitized LLM batch
outputs with deterministic benchmark results. The aggregate record preserves
validate-only attempts as not-scored rows and emits repair-turn, token/cost,
paired-comparison, and failure-taxonomy CSV/TeX tables without exposing local
provider secrets. Failure taxonomy separates invalid response JSON, explicit
model rejection, missing baseline payloads, compiler diagnostics, unsafe
rejection, EDA lint outcomes, and repair-patch application outcomes.

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

Plan the full low-cost benchmark matrix without paid requests:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json"
```

Exercise the compiler/EDA scoring path without provider requests:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke --task-id T004_direct_stream --task-id T005_invalid_width_no_adapter --offline-fixture --output build/llm/bench_offline_fixture.json"
```

Run an authenticated LLM benchmark subset only when cost is intended:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke --baselines mico_source --task-id T004_direct_stream --execute --output build/llm/bench_execute_smoke.json"
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
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "replace_connection",
      "compose": "Top",
      "from": {
        "instance": "dma",
        "port": "tx"
      },
      "to": {
        "instance": "aes",
        "port": "rx"
      },
      "connection": {
        "from": {
          "instance": "dma",
          "port": "tx"
        },
        "to": {
          "instance": "fifo",
          "port": "sink"
        },
        "adapter": "AsyncFifo32"
      }
    }
  ]
}
```

The runner sends compact compiler diagnostics to repair prompts: stable code,
message, repair action, hints, graph nodes, and label messages. It does not ask
the model to interpret raw provider payloads, local paths, API keys, or full
tool logs.

## Evaluation-specific prompts

See `prompts/`.
