# LLM Prompt Redesign Pilot

Snapshot date: 2026-06-14.

This page records the M2 prompt/model/scoring pilot after the negative
low-cost matrix in `docs/16_llm_matrix_results.md`. It is a pilot result only:
it shows that the structured MICO JSON AST path is no longer stuck at zero on a
small selected subset, but it is not a full-matrix pass-rate claim.

## Changes

- The batch runner defaults to OpenAI-compatible `response_format:
  json_object` and records the effective response format in cache keys and
  result metadata.
- MICO JSON AST baselines receive a schema-valid declaration skeleton generated
  from `mico_cli dump-ast-json`; compose instances and connections are stripped
  before the prompt is built, so the expected solution body is not leaked.
- MICO JSON AST baselines receive at least 4096 output tokens, MICO source
  receives at least 2048, and repair turns receive at least 2048 to avoid
  truncating structured outputs.
- Repair prompts use compact compiler diagnostics and the real
  `mico.repair_patch.v0` shape with `schema_version`, `kind`, and
  `operations`.
- Failure taxonomy now separates invalid response JSON, compiler diagnostics,
  EDA lint outcomes, and repair patch application outcomes.
- The Kimi code profile is normalized to effective `temperature = 1.0` because
  the provider rejects lower values for that model.

## Validation Commands

All Python, benchmark, LLM, and schema validation commands ran through
`scripts/eda-docker.ps1`.

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 -m py_compile scripts/run_llm_bench.py scripts/llm-provider-smoke.py benchmarks/aggregate_results.py scripts/validate_json_schemas.py"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/m2_validate_quality.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke --baselines direct_verilog,mico_json_ast,mico_json_ast_repair --task-id T004_direct_stream --task-id T005_invalid_width_no_adapter --offline-fixture --output build/llm/m2_offline_fixture.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile quality_code --validate-only --output build/llm/m2_provider_quality_validate.json
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke --baselines mico_json_ast,mico_json_ast_repair --task-id T004_direct_stream --task-id T003_width_adapter --task-id T058_streaming_accelerator_case --output build/llm/pilot_execute.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles quality_code --baselines mico_json_ast,mico_json_ast_repair --task-id T004_direct_stream --task-id T003_width_adapter --output build/llm/pilot_execute_quality_code.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/pilot_execute.json --llm-result build/llm/pilot_execute_quality_code.json --out-json build/bench/m2_pilot_aggregate.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-result build/bench/seed_results.json --llm-run build/llm/m2_provider_quality_validate.json --llm-bench build/llm/pilot_execute.json --llm-bench build/llm/pilot_execute_quality_code.json --aggregate-result build/bench/m2_pilot_aggregate.json"
```

## Pilot Results

| Profile | Baseline | Tasks | JSON valid | Positive compiler pass | Positive lint pass | Provider requests |
|---|---|---:|---:|---:|---:|---:|
| `smoke` | `mico_json_ast` | 3 | 3/3 | 3/3 | 3/3 | 3 |
| `smoke` | `mico_json_ast_repair` | 3 | 3/3 | 3/3 | 3/3 | 3 |
| `quality_code` | `mico_json_ast` | 2 | 2/2 | 2/2 | 2/2 | 2 |
| `quality_code` | `mico_json_ast_repair` | 2 | 2/2 | 2/2 | 2/2 | 2 |

The failure taxonomy for the combined pilot contains only `compiler_pass` and
`sv_lint_pass` categories for these rows.

## Evidence Hashes

Generated evidence files are intentionally ignored by git.

| Artifact | SHA-256 |
|---|---|
| `build/llm/pilot_execute.json` | `5b9a1b19c80bd7bac1c290a5e12c9c55200224572073cb87e49ff4a18b04c0dd` |
| `build/llm/pilot_execute_quality_code.json` | `400c45d01408e758f71d077a8623e0eff9ae18da3c9695d4d7b9e509eb86b9df` |
| `build/bench/m2_pilot_aggregate.json` | `610b7d5a07f58461e6cf0e615589e6c7c1be2a6188285c8e4b43b817008b209a` |
| `build/llm/m2_provider_quality_validate.json` | `5a2135aa2a0845b05baecd985b7ed5e2dd9294031743c17bb363daa323963c76` |
| `build/llm/m2_validate_quality.json` | `80025c01c13a1c51044fc28bbb58add2df6b5a0e3bc4308b6615db6da500c82e` |
| `build/llm/m2_offline_fixture.json` | `4c6e00c00e454053b628a80a4f83ae9bbe3c2a5f8abc76d5c69baf5ebd663404` |

## Claim Boundary

This pilot removes the immediate zero-pass blocker for selected structured MICO
JSON AST prompts, but it does not support a paper claim that MICO improves LLM
pass rate over direct RTL or SystemVerilog-interface prompting. That claim
still requires a full authenticated matrix with direct baselines, at least two
profiles, paired comparisons, and held-out or clearly separated final tasks.
