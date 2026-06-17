# LLM Full Matrix v3

Snapshot date: 2026-06-17.

This document supersedes the v2 evidence in `docs/22_llm_full_matrix_v2.md` for
bounded submission claims. The v3 execute records are locked to pre-expansion
LLM-scored public-development and held-out manifest hashes. They clear the
historical held-out manifest-hash mismatch for that LLM evidence line, but they
do not cover the expanded deterministic 83/40/30 manifests.

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
| `build/llm/bench_execute_public_dev_v3.json` | Locked pre-expansion public-development LLM manifest | `3b8c412659b22fcbcdb5954fc299a6250ecd237988ff85bc398f096b61bf2957` |
| `build/llm/bench_execute_heldout_v3.json` | Locked pre-expansion held-out LLM manifest | `8aebbfe8b1c1f9cf67cbb112e6fa9d439e4cded2fd8cf8b270041c8b37380ffd` |

## Execute Summary

| Split | Run id | Attempts | Execute requests in record | Cache hits | JSON-valid responses | MICO positive compiler pass | Unsafe rejection |
|---|---|---:|---:|---:|---:|---:|---:|
| Public-development | `a35fb473a8059e1a` | 930 | 930 | 879 | 782 | 204/324 | 228/234 |
| Held-out | `8890a6647ba48793` | 300 | 300 | 90 | 262 | 55/90 | 86/90 |

The runner's `provider_requests` field records execute-mode responses whose
payload came from provider requests; `cache_hits` records retry-local cache
reuse after Docker EOF interruptions during this from-empty-cache rerun. The v3
evidence is still an authenticated execute record because the final sanitized
records contain request-backed responses re-evaluated against the locked
pre-expansion LLM-scored manifests, compiler, and EDA gates.

## Compact Results

| Split | Baseline | Profiles | Positive final pass | Unsafe rejection |
|---|---|---:|---:|---:|
| Public-development | Direct Verilog | 3 | 0-5/36 | n/a |
| Public-development | SV interface | 3 | 0/36 | n/a |
| Public-development | MICO source | 3 | 0-1/36 | 24-25/26 |
| Public-development | JSON AST | 3 | 24-36/36 | 25-26/26 |
| Public-development | JSON AST repair | 3 | 35-36/36 | 25-26/26 |
| Held-out | Direct Verilog | 3 | 0-2/10 | n/a |
| Held-out | SV interface | 3 | 0/10 | n/a |
| Held-out | MICO source | 3 | 0/10 | 9-10/10 |
| Held-out | JSON AST | 3 | 7-10/10 | 8-10/10 |
| Held-out | JSON AST repair | 3 | 9-10/10 | 10/10 |

## Paired Tests

| Split | Comparison | Target wins | Baseline wins | Ties | Exact p-value |
|---|---|---:|---:|---:|---:|
| Public-development | repair vs direct Verilog | 103 | 1 | 82 | `1.035379938102578e-29` |
| Public-development | repair vs SV interface | 108 | 1 | 77 | `3.389636702121535e-31` |
| Public-development | repair vs MICO source | 109 | 1 | 76 | `1.7102257906158654e-31` |
| Public-development | repair vs plain JSON AST | 17 | 2 | 167 | `0.000728607177734375` |
| Held-out | repair vs direct Verilog | 27 | 0 | 33 | `1.4901161193847656e-08` |
| Held-out | repair vs SV interface | 30 | 0 | 30 | `1.862645149230957e-09` |
| Held-out | repair vs MICO source | 31 | 0 | 29 | `9.313225746154785e-10` |
| Held-out | repair vs plain JSON AST | 6 | 1 | 53 | `0.125` |

## Repair Boundary

The v3 run keeps the same repair boundary as v2:

- `mico_json_ast` and `mico_json_ast_repair` are reported separately.
- Public-development repair rows: 186.
- Public-development paired wins over plain JSON AST: 17.
- Held-out repair rows: 60.
- Held-out paired wins over plain JSON AST: 6.
- Accepted free-form LLM repair-patch wins: 0.
- Recorded deterministic fallback wins: 23 total, all marked
  `deterministic_adapter_instance_collapse` and accepted only after
  `mico_cli repair-json --apply --json` plus re-check passes.

The paper may claim compiler-feedback repair for the common
adapter-as-instance error pattern. It must not claim broad autonomous
free-form semantic repair.

## Artifact Hashes

| File | SHA-256 |
|---|---|
| `build/llm/bench_execute_public_dev_v3.json` | `47d2ef8eba9e36ed6cabb7cd77cd4f773c8b6b2a725bf070c616a7eb921406b2` |
| `build/llm/bench_execute_heldout_v3.json` | `04fd36350ddb17dfd220bcf2825df2c3cd4f9188d3f014dd01b70fc9e48d5f7e` |
| `build/bench/aggregate_llm_v3.json` | `60c9964c37bf1bc5d2a3aa782013995f68e2bf0c2d5d0f1074a490e576cd334a` |

Generated CSV and TeX snippets are under `build/paper_tables/llm_v3/` and stay
out of source control. `docs/llm_final_matrix_report.md` records the M5 final
hashes, table hashes, provenance, token/cost status, and v4 non-claim boundary.

## Branch Decision

Branch A is the current paper branch for the tested OpenCode Go profiles,
prompts, and locked pre-expansion public-development and held-out manifest
hashes:

- the held-out execute record matches the locked pre-expansion 20-task manifest
  hash recorded in the v3 evidence, not the expanded 40-task deterministic
  manifest;
- structured JSON AST baselines have nonzero positive pass rates on all three
  tested profiles;
- JSON AST repair remains the strongest tested structured path, reaching
  35--36/36 public-development positives and 9--10/10 held-out positives;
- unsafe rejection is bounded to 25--26/26 public-development MICO unsafe tasks
  and 10/10 held-out unsafe tasks for the JSON AST repair baseline;
- paired tests against unstructured baselines are significant on both splits;
- the repair claim is explicitly limited to the recorded deterministic
  fallback and compiler-gated repair path.

This does not make the repository submission-ready by itself. The remaining
paper and release gates must keep CDC, formal, timing, arbitrary-model, and
broad repair limitations visible.
