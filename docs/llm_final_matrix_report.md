# LLM Final Matrix Report

Snapshot date: 2026-06-16.

This is the M5 final LLM evidence report for the current paper branch. It
records the authenticated v3 OpenCode Go matrix, aggregate statistics, repair
provenance, and non-claim boundary. It supersedes historical v1/v2 LLM result
summaries for submission claims.

## Scope

The v3 matrix covers:

- public-development manifest: 62 tasks, 930 attempts;
- held-out manifest: 20 tasks, 300 attempts;
- profiles: `smoke`, `low_cost_crosscheck`, `quality_code`;
- baselines: `direct_verilog`, `sv_interface`, `mico_source`,
  `mico_json_ast`, `mico_json_ast_repair`.

The local provider configuration was `config/llm-provider.local.yaml`, but the
configuration and API key are not included in this report or committed to the
repository. The sanitized execute records record provider/cache provenance,
profile/model metadata, prompt hashes, request settings, JSON validity,
compiler/EDA outcomes, repair records, tokens, and cost status.

## Reproduction Commands

Run inside the Docker EDA environment:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_public_dev_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_execute_public_dev_v3.json --llm-result build/llm/bench_execute_heldout_v3.json --out-json build/bench/aggregate_llm_v3.json --out-dir build/bench/llm_v3 --paper-table-dir build/paper_tables/llm_v3"
.\scripts\eda-docker.ps1 python3 scripts/write-llm-evidence-hashes.py --require --output build/release/llm_evidence_hashes.json
```

Schema validation command:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_execute_public_dev_v3.json --llm-bench build/llm/bench_execute_heldout_v3.json --aggregate-result build/bench/aggregate_llm_v3.json
```

## Evidence Hashes

| Artifact | SHA-256 |
|---|---|
| `build/llm/bench_execute_public_dev_v3.json` | `f1c5aa83d2527241eae0b8efbae788b6ae630a8c4ef01de6d9f322fbc5ba9513` |
| `build/llm/bench_execute_heldout_v3.json` | `44f249e02b6e5df0bbb70dfefdbb9ab07f340ff306c780ab2b88b8a395af06ff` |
| `build/bench/aggregate_llm_v3.json` | `123f8296533f5e07312873c547e8e598454704fb2605e406f977af403c7aedbd` |
| `build/release/llm_evidence_hashes.json` | `07e217f985dbea56818efcfd7602712b091e5a418b80f1b0eddfbaaa806de15d` |

The public-development manifest hash is
`3b8c412659b22fcbcdb5954fc299a6250ecd237988ff85bc398f096b61bf2957`.
The held-out manifest hash is
`8aebbfe8b1c1f9cf67cbb112e6fa9d439e4cded2fd8cf8b270041c8b37380ffd`.

## Aggregate Coverage

`build/bench/aggregate_llm_v3.json` contains:

- 30 full LLM summary rows with per-profile compiler, lint/final pass, unsafe
  rejection, JSON-valid rate, Wilson confidence intervals, provider request
  counts, token totals, and cost status;
- 10 compact summary rows for paper-facing pass/reject ranges;
- 8 paired exact-test rows comparing `mico_json_ast_repair` against each
  baseline on public-development and held-out splits;
- 33 repair-turn distribution rows;
- 30 token/cost accounting rows;
- 57 failure-taxonomy rows.

Generated table hashes:

| Table | SHA-256 |
|---|---|
| `build/paper_tables/llm_v3/llm_summary.tex` | `8b20f2f296b68cde77e5b72941a37e67105cf699c1512ea3fd23093b0e552b9b` |
| `build/paper_tables/llm_v3/llm_compact_summary.tex` | `fef8888b9ef788ffaaeb26683010b2bf72638a067d4175ad2e81ec32844eb4d2` |
| `build/paper_tables/llm_v3/llm_paired_comparisons.tex` | `4bf4009f100a58821a62145d49139b4d3852c3b14715932eb50499aa4787302e` |
| `build/paper_tables/llm_v3/llm_repair_turns.tex` | `7c68985caaace003b2e12d95f96a38001ef3fcfdc29d438fb4f348383b82a82a` |
| `build/paper_tables/llm_v3/llm_cost_tokens.tex` | `284fe28ec3d8c2528f9162ecd57baab69922d0043a27e2e0bf1df55fad9daf67` |
| `build/paper_tables/llm_v3/llm_failure_taxonomy.tex` | `47be93faf1634ec0e7cc9820229be0b329440a03c91a1d88b3114a6e3781260a` |

## Acceptance Checks

| Check | Result |
|---|---|
| Public JSON AST repair positives | 36/36 final pass in every tested profile |
| Held-out JSON AST repair positives | 10/10 final pass in every tested profile |
| Public unsafe rejection | 26/26 for MICO unsafe tasks in every tested profile |
| Held-out unsafe rejection | 10/10 in every tested profile |
| Public paired tests vs Direct/SV/MICO-source | significant; exact p-values below `1e-31` for unstructured public comparisons |
| Held-out paired tests vs Direct/SV/MICO-source | significant; exact p-values below `2e-9` |
| Plain JSON AST vs repair, public | significant; exact p-value `0.0009765625` |
| Plain JSON AST vs repair, held-out | not significant; exact p-value `0.5` and reported as such |
| Repair provenance | 0 accepted free-form LLM patch wins; 14 accepted deterministic fallback wins |
| Token/cost accounting | token rows present; USD cost is `not_configured` because profile cost rates are not configured |
| Raw provider data | not committed; sanitized execute records only |

## Repair Boundary

The accepted repair wins are limited to
`deterministic_adapter_instance_collapse`: 10 public-development rows and 4
held-out rows. The paper may claim compiler-feedback repair for this common
adapter-as-instance pattern after schema validation, `repair-json --apply`, and
compiler re-check. It must not claim autonomous semantic repair or broad
free-form LLM patch reliability.

## V4 Decision

No v4 or second-provider matrix is included in this M5 seal. The paper claim is
therefore bounded to the tested OpenCode Go profiles, prompts, manifests,
schemas, compiler version, and cached sanitized provider outputs. It must not
generalize to arbitrary models or LLMs in general.
