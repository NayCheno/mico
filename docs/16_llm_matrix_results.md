# LLM Matrix Result Snapshot

Snapshot date: 2026-06-14.

This page records the sanitized summary of the authenticated low-cost LLM matrix
run. Raw provider responses, response caches, local config, and generated JSON
artifacts remain under ignored `build/` paths and must not be committed.

## Command

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_low_cost.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_execute_low_cost.json --out-json build/bench/aggregate_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-result build/bench/seed_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_execute_low_cost.json --aggregate-result build/bench/aggregate_results.json"
```

The first execute attempt timed out after two hours after populating the
response cache. Re-running the same command reused cached provider responses
and completed the matrix.

## Matrix

- Run ID: `574eed054b294bf3`
- Mode: `execute`
- Tasks: 62
- Profiles: `smoke`, `low_cost_crosscheck`
- Baselines: `direct_verilog`, `sv_interface`, `mico_source`,
  `mico_json_ast`, `mico_json_ast_repair`
- Attempts: 620
- Provider requests recorded by result rows: 620
- Cost status: token counts recorded, USD rates not configured

## Summary

| Profile | Baseline | JSON Valid | Positive Compiler Pass | Positive Lint Pass | Unsafe Rejection |
|---|---|---:|---:|---:|---:|
| `smoke` | `direct_verilog` | 18/62 | n/a | 0/36 | n/a |
| `smoke` | `sv_interface` | 18/62 | n/a | 0/36 | n/a |
| `smoke` | `mico_source` | 16/62 | 0/36 | 0/36 | 15/26 |
| `smoke` | `mico_json_ast` | 15/62 | 0/36 | 0/36 | 15/26 |
| `smoke` | `mico_json_ast_repair` | 17/62 | 0/36 | 0/36 | 17/26 |
| `low_cost_crosscheck` | `direct_verilog` | 23/62 | n/a | 0/36 | n/a |
| `low_cost_crosscheck` | `sv_interface` | 15/62 | n/a | 0/36 | n/a |
| `low_cost_crosscheck` | `mico_source` | 25/62 | 0/36 | 0/36 | 16/26 |
| `low_cost_crosscheck` | `mico_json_ast` | 17/62 | 0/36 | 0/36 | 17/26 |
| `low_cost_crosscheck` | `mico_json_ast_repair` | 19/62 | 0/36 | 0/36 | 18/26 |

## Interpretation

This matrix is a negative result for the current prompt/model settings. It does
not support any claim that MICO prompting improves positive-task pass rate over
direct Verilog or SystemVerilog-interface prompting. The main observed failures
are invalid JSON responses, explicit model rejections, and compiler rejection of
malformed MICO source or JSON AST.

Any paper or README claim must therefore stay conservative until prompts,
structured-response enforcement, or model/profile selection are improved and a
new schema-valid authenticated matrix is archived.
