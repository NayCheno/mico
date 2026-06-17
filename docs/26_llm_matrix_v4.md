# LLM Expanded Matrix v4

Snapshot date: 2026-06-17.

This document records the authenticated expanded LLM rerun for P5. It supersedes
`docs/24_llm_matrix_v3.md` for current submission claims while leaving v3 as a
historical pre-expansion evidence record.

## Commands Run

All commands below were run inside the repository Docker EDA environment. The
local provider config was `config/llm-provider.local.yaml`; the run output is
sanitized and does not contain API keys.

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_public_expanded_v4.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_expanded_v4.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_realism_v4.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json --llm-result build/llm/bench_execute_public_expanded_v4.json --llm-result build/llm/bench_execute_heldout_expanded_v4.json --llm-result build/llm/bench_execute_realism_v4.json --out-json build/bench/aggregate_llm_v4.json --out-dir build/bench/llm_v4 --paper-table-dir build/paper_tables/llm_v4"
```

## Manifest Binding

| Artifact | Manifest | Manifest SHA-256 |
|---|---|---|
| `build/llm/bench_execute_public_expanded_v4.json` | Expanded public-development manifest | `584ac98045ee6c2a02e1afb3e3f0f4ad5cf6057954cb4637f1165fbdaacbfdc4` |
| `build/llm/bench_execute_heldout_expanded_v4.json` | Expanded held-out manifest | `cb04838fbe4332b4bb94d9bfeddb7f2cdd62bd59993c6475696161418d7c69c5` |
| `build/llm/bench_execute_realism_v4.json` | Supplemental realism manifest | `5a79fbe5171506143c0382a5854e1adabac9596fe999576ae7ae01db307d3654` |

## Execute Summary

| Split | Run id | Attempts | Execute responses in record | Cache hits | JSON-valid responses | MICO positive compiler pass | Unsafe rejection |
|---|---|---:|---:|---:|---:|---:|---:|
| Public-development | `d3439394e0386a7f` | 1,245 | 1,245 | 1,140 | 1,053 | 260/414 | 323/333 |
| Held-out | `55bfb35d5260cf9e` | 600 | 600 | 585 | 519 | 112/180 | 174/180 |
| Realism | `2753a6a2d77bdc9f` | 450 | 450 | 435 | 393 | 85/135 | 130/135 |

The runner's `provider_requests` field records execute-mode responses whose
payload came from provider requests; `cache_hits` records local cache reuse.
The final sanitized records contain request-backed responses re-evaluated
against the expanded manifests, compiler, and EDA gates.

## Compact Results

| Split | Baseline | Profiles | Positive final pass | Unsafe rejection |
|---|---|---:|---:|---:|
| Public-development | Direct Verilog | 3 | 0--6/46 | n/a |
| Public-development | SV interface | 3 | 0/46 | n/a |
| Public-development | MICO source | 3 | 0--1/46 | 34--36/37 |
| Public-development | JSON AST | 3 | 32--46/46 | 34--37/37 |
| Public-development | JSON AST repair | 3 | 45--46/46 | 36--37/37 |
| Held-out | Direct Verilog | 3 | 0--5/20 | n/a |
| Held-out | SV interface | 3 | 0/20 | n/a |
| Held-out | MICO source | 3 | 0/20 | 18--19/20 |
| Held-out | JSON AST | 3 | 14--20/20 | 18--20/20 |
| Held-out | JSON AST repair | 3 | 19--20/20 | 20/20 |
| Realism | Direct Verilog | 3 | 0--4/15 | n/a |
| Realism | SV interface | 3 | 0/15 | n/a |
| Realism | MICO source | 3 | 0/15 | 12--15/15 |
| Realism | JSON AST | 3 | 11--15/15 | 14--15/15 |
| Realism | JSON AST repair | 3 | 15/15 | 15/15 |

## Artifact Hashes

| File | SHA-256 |
|---|---|
| `build/llm/bench_execute_public_expanded_v4.json` | `61f8c59d85959c9ec40a54c6ddf27b6a1cc9265f414d83c44bc2447b17f32827` |
| `build/llm/bench_execute_heldout_expanded_v4.json` | `e1f34005cf0f2c8961f52af74ed9af6c93238ca8193f34bd429d07ebe5a1180a` |
| `build/llm/bench_execute_realism_v4.json` | `fea1e7cdae883d4f5995f00430d44756960f6d5f70ce615dc9b1264dc8a0a6a3` |
| `build/bench/aggregate_llm_v4.json` | `de6f090be33ec5ce7f7eceb36a89135ecc5dd6268e6125c16900a8e070d3ddd3` |

Generated CSV and TeX snippets are under `build/bench/llm_v4/` and
`build/paper_tables/llm_v4/` and stay out of source control unless a specific
paper table is intentionally refreshed under `paper/tables/`.

## Branch Decision

Branch A is the current paper branch for the tested OpenCode Go profiles,
prompts, and expanded manifests. It supports a bounded claim that schema-guided
MICO JSON AST repair improves positive-task compiler/lint success and unsafe
rejection over direct RTL, SystemVerilog-interface, and MICO-source prompting on
the tested profiles. It does not support broad autonomous repair,
arbitrary-model generalization, CDC correctness proof, full formal proof, or
routed timing closure.
