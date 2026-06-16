# LLM Full Matrix v3

Snapshot date: 2026-06-16.

This document supersedes the v2 evidence in `docs/22_llm_full_matrix_v2.md` for
current submission claims. The v3 execute records were refreshed against the
current public-development and held-out manifest SHA-256 hashes and therefore
clear the held-out manifest-hash mismatch called out in the CCF-A submission
plan.

## Commands Run

All commands below were run inside the repository Docker EDA environment. The
local provider config was `config/llm-provider.local.yaml`; the run output is
sanitized and does not contain API keys.

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_public_dev_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_execute_public_dev_v3.json --llm-result build/llm/bench_execute_heldout_v3.json --out-json build/bench/aggregate_llm_v3.json --out-dir build/bench/llm_v3 --paper-table-dir build/paper_tables/llm_v3"
.\scripts\eda-docker.ps1 python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_execute_public_dev_v3.json --llm-bench build/llm/bench_execute_heldout_v3.json --aggregate-result build/bench/aggregate_llm_v3.json
```

## Manifest Binding

| Artifact | Manifest | Manifest SHA-256 |
|---|---|---|
| `build/llm/bench_execute_public_dev_v3.json` | `benchmarks/module_compose_bench_manifest.yaml` | `3b8c412659b22fcbcdb5954fc299a6250ecd237988ff85bc398f096b61bf2957` |
| `build/llm/bench_execute_heldout_v3.json` | `benchmarks/module_compose_bench_heldout.yaml` | `8aebbfe8b1c1f9cf67cbb112e6fa9d439e4cded2fd8cf8b270041c8b37380ffd` |

## Execute Summary

| Split | Run id | Attempts | Execute requests in record | Cache hits | JSON-valid responses | MICO positive compiler pass | Unsafe rejection |
|---|---|---:|---:|---:|---:|---:|---:|
| Public-development | `b86f9028890e6bd9` | 930 | 930 | 930 | 777 | 207/324 | 229/234 |
| Held-out | `339f9a8c1e1a24e0` | 300 | 300 | 300 | 254 | 59/90 | 89/90 |

The runner's `provider_requests` field records execute-mode responses whose
original cache payload came from provider requests; `cache_hits` records cache
reuse in this v3 rerun. The v3 evidence is still an authenticated execute
record because the cached responses are sanitized provider outputs and the new
run re-evaluates them against the current manifests, compiler, and EDA gates.

## Compact Results

| Split | Baseline | Profiles | Positive final pass | Unsafe rejection |
|---|---|---:|---:|---:|
| Public-development | Direct Verilog | 3 | 0-8/36 | n/a |
| Public-development | SV interface | 3 | 0/36 | n/a |
| Public-development | MICO source | 3 | 0/36 | 23-26/26 |
| Public-development | JSON AST | 3 | 27-36/36 | 25-26/26 |
| Public-development | JSON AST repair | 3 | 36/36 | 26/26 |
| Held-out | Direct Verilog | 3 | 0-2/10 | n/a |
| Held-out | SV interface | 3 | 0/10 | n/a |
| Held-out | MICO source | 3 | 0/10 | 10/10 |
| Held-out | JSON AST | 3 | 9-10/10 | 9-10/10 |
| Held-out | JSON AST repair | 3 | 10/10 | 10/10 |

## Paired Tests

| Split | Comparison | Target wins | Baseline wins | Ties | Exact p-value |
|---|---|---:|---:|---:|---:|
| Public-development | repair vs direct Verilog | 105 | 0 | 81 | `4.930380657631324e-32` |
| Public-development | repair vs SV interface | 110 | 0 | 76 | `1.5407439555097887e-33` |
| Public-development | repair vs MICO source | 112 | 0 | 74 | `3.851859888774472e-34` |
| Public-development | repair vs plain JSON AST | 11 | 0 | 175 | `0.0009765625` |
| Held-out | repair vs direct Verilog | 30 | 0 | 30 | `1.862645149230957e-09` |
| Held-out | repair vs SV interface | 31 | 0 | 29 | `9.313225746154785e-10` |
| Held-out | repair vs MICO source | 30 | 0 | 30 | `1.862645149230957e-09` |
| Held-out | repair vs plain JSON AST | 2 | 0 | 58 | `0.5` |

## Repair Boundary

The v3 run keeps the same repair boundary as v2:

- `mico_json_ast` and `mico_json_ast_repair` are reported separately.
- Public-development repair rows: 186.
- Public-development rows with accepted repair records: 10.
- Held-out repair rows: 60.
- Held-out rows with accepted repair records: 4.
- Accepted free-form LLM repair-patch wins: 0.
- Accepted deterministic fallback wins: 14 total, all marked
  `deterministic_adapter_instance_collapse` and accepted only after
  `mico_cli repair-json --apply --json` plus re-check passes.

The paper may claim compiler-feedback repair for the common
adapter-as-instance error pattern. It must not claim broad autonomous
free-form semantic repair.

## Artifact Hashes

| File | SHA-256 |
|---|---|
| `build/llm/bench_execute_public_dev_v3.json` | `f1c5aa83d2527241eae0b8efbae788b6ae630a8c4ef01de6d9f322fbc5ba9513` |
| `build/llm/bench_execute_heldout_v3.json` | `44f249e02b6e5df0bbb70dfefdbb9ab07f340ff306c780ab2b88b8a395af06ff` |
| `build/bench/aggregate_llm_v3.json` | `123f8296533f5e07312873c547e8e598454704fb2605e406f977af403c7aedbd` |

Generated CSV and TeX snippets are under `build/paper_tables/llm_v3/` and stay
out of source control. `docs/llm_final_matrix_report.md` records the M5 final
hashes, table hashes, provenance, token/cost status, and v4 non-claim boundary.

## Branch Decision

Branch A is the current paper branch for the tested OpenCode Go profiles,
prompts, public-development manifest, and held-out manifest:

- the held-out execute record matches the current manifest hash;
- structured MICO baselines have nonzero positive pass rates on all three
  tested profiles;
- JSON AST repair beats direct Verilog and SV-interface final pass by more than
  15 percentage points on both public-development and held-out splits;
- unsafe rejection is not weaker than the comparable MICO-source baseline;
- paired tests against unstructured baselines are significant on both splits;
- the repair claim is explicitly limited to the recorded deterministic
  fallback and compiler-gated repair path.

This does not make the repository submission-ready by itself. The remaining
paper and release gates must keep CDC, formal, timing, arbitrary-model, and
broad repair limitations visible.
