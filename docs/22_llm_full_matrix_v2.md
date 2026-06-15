# LLM Full Matrix v2

Snapshot date: 2026-06-15.

This page records the M2 structured LLM full-matrix rerun for the DAC 2027
plan. The result files, response caches, prompts emitted under `build/`, and
local provider configuration remain ignored and are not committed.

Manifest note: the held-out task IDs and prompts reported here are unchanged,
but a 2026-06-15 directed-verification update added committed simulation and
formal collateral to `benchmarks/module_compose_bench_heldout.yaml`, changing
that manifest's SHA-256 to `022839f2ad342d9050f392e43f001291c2560301742a00994ac20b1454548704`.
The authenticated held-out execute hashes below remain evidence for the prior
manifest hash. Before immutable release, rerun or explicitly rebind the
held-out LLM matrix to the current manifest hash.

## Runner Changes

- `prompts/repair_prompt_template.md` now includes the supported repair
  operation shapes and diagnostic-driven operation selection rules.
- `scripts/run_llm_bench.py` adds a narrow deterministic compiler-feedback
  fallback for a common model error: treating an adapter declaration as a
  compose instance. The fallback emits a normal `mico.repair_patch.v0` patch
  that removes the adapter instance and through-connections, then adds a single
  source-to-sink connection with `connection.adapter` set to the adapter name.
- The fallback is explicitly recorded as
  `deterministic_adapter_instance_collapse` inside the repair record. It is
  still accepted only if `mico_cli repair-json --apply --json` and the
  subsequent compiler check pass.
- No LLM output is trusted without compiler and EDA gates.

## Commands

Pilot expansion:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,quality_code --baselines mico_json_ast,mico_json_ast_repair --task-id T004_direct_stream --task-id T003_width_adapter --task-id T058_streaming_accelerator_case --output build/llm/pilot_dac_v2_public.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,quality_code --baselines mico_json_ast,mico_json_ast_repair --task-id T063_axi_apb_wrapper_case --output build/llm/pilot_dac_v2_heldout.json"
```

Full matrix:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_dac2027_public_dev_v2.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --manifest benchmarks/module_compose_bench_heldout.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_dac2027_heldout_20.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_public_directed_results.json --llm-result build/llm/bench_execute_dac2027_public_dev_v2.json --llm-result build/llm/bench_execute_dac2027_heldout_20.json --out-json build/bench/aggregate_dac2027_llm_stats.json --out-dir build/bench/dac2027_llm_stats --paper-table-dir build/paper_tables/dac2027_llm_stats"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/m3_heldout_directed_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_execute_dac2027_public_dev_v2.json --llm-bench build/llm/bench_execute_dac2027_heldout_20.json --aggregate-result build/bench/aggregate_dac2027_llm_stats.json"
```

The aggregate emits paper tables for the main LLM summary, paired comparisons,
Wilson confidence intervals, repair-turn distributions, failure taxonomy,
token/cost accounting, and JSON validity. The paired table reports a two-sided
exact sign-test p-value over discordant paired attempts; this is equivalent to
the exact McNemar/binomial test for these 2x2 paired outcomes.

## Public-Development Results

Run ID: `086e4e2035a675a2`.

| Profile | Baseline | JSON valid | Positive compiler pass | Positive lint pass | Unsafe rejection |
|---|---|---:|---:|---:|---:|
| `smoke` | `direct_verilog` | 22/62 | n/a | 0/36 | n/a |
| `smoke` | `sv_interface` | 25/62 | n/a | 0/36 | n/a |
| `smoke` | `mico_source` | 51/62 | 0/36 | 0/36 | 25/26 |
| `smoke` | `mico_json_ast` | 62/62 | 36/36 | 36/36 | 26/26 |
| `smoke` | `mico_json_ast_repair` | 62/62 | 36/36 | 36/36 | 26/26 |
| `low_cost_crosscheck` | `direct_verilog` | 61/62 | n/a | 0/36 | n/a |
| `low_cost_crosscheck` | `sv_interface` | 62/62 | n/a | 0/36 | n/a |
| `low_cost_crosscheck` | `mico_source` | 62/62 | 0/36 | 0/36 | 23/26 |
| `low_cost_crosscheck` | `mico_json_ast` | 62/62 | 27/36 | 27/36 | 26/26 |
| `low_cost_crosscheck` | `mico_json_ast_repair` | 62/62 | 36/36 | 36/36 | 26/26 |
| `quality_code` | `direct_verilog` | 41/62 | n/a | 8/36 | n/a |
| `quality_code` | `sv_interface` | 37/62 | n/a | 0/36 | n/a |
| `quality_code` | `mico_source` | 44/62 | 0/36 | 0/36 | 26/26 |
| `quality_code` | `mico_json_ast` | 62/62 | 36/36 | 35/36 | 25/26 |
| `quality_code` | `mico_json_ast_repair` | 62/62 | 36/36 | 36/36 | 26/26 |

Paired final-pass comparisons for `mico_json_ast_repair` on the public-dev
matrix:

| Comparison | Comparable attempts | Repair wins | Baseline wins | Exact p | Ties |
|---|---:|---:|---:|---:|---:|
| vs. `direct_verilog` | 186 | 105 | 0 | `4.93e-32` | 81 |
| vs. `sv_interface` | 186 | 110 | 0 | `1.54e-33` | 76 |
| vs. `mico_source` | 186 | 112 | 0 | `3.85e-34` | 74 |
| vs. `mico_json_ast` | 186 | 11 | 0 | `9.77e-4` | 175 |

## Held-Out Results

Run ID: `38a6b47f9d51a71b`.

| Profile | Baseline | JSON valid | Positive compiler pass | Positive lint pass | Unsafe rejection |
|---|---|---:|---:|---:|---:|
| `smoke` | `direct_verilog` | 9/20 | n/a | 0/10 | n/a |
| `smoke` | `sv_interface` | 9/20 | n/a | 0/10 | n/a |
| `smoke` | `mico_source` | 15/20 | 0/10 | 0/10 | 10/10 |
| `smoke` | `mico_json_ast` | 20/20 | 10/10 | 10/10 | 10/10 |
| `smoke` | `mico_json_ast_repair` | 20/20 | 10/10 | 10/10 | 10/10 |
| `low_cost_crosscheck` | `direct_verilog` | 19/20 | n/a | 0/10 | n/a |
| `low_cost_crosscheck` | `sv_interface` | 19/20 | n/a | 0/10 | n/a |
| `low_cost_crosscheck` | `mico_source` | 20/20 | 0/10 | 0/10 | 10/10 |
| `low_cost_crosscheck` | `mico_json_ast` | 20/20 | 9/10 | 9/10 | 9/10 |
| `low_cost_crosscheck` | `mico_json_ast_repair` | 20/20 | 10/10 | 10/10 | 10/10 |
| `quality_code` | `direct_verilog` | 12/20 | n/a | 2/10 | n/a |
| `quality_code` | `sv_interface` | 13/20 | n/a | 0/10 | n/a |
| `quality_code` | `mico_source` | 18/20 | 0/10 | 0/10 | 10/10 |
| `quality_code` | `mico_json_ast` | 20/20 | 10/10 | 10/10 | 10/10 |
| `quality_code` | `mico_json_ast_repair` | 20/20 | 10/10 | 10/10 | 10/10 |

Paired final-pass comparisons for `mico_json_ast_repair` on the held-out
matrix:

| Comparison | Comparable attempts | Repair wins | Baseline wins | Exact p | Ties |
|---|---:|---:|---:|---:|---:|
| vs. `direct_verilog` | 60 | 30 | 0 | `1.86e-9` | 30 |
| vs. `sv_interface` | 60 | 31 | 0 | `9.31e-10` | 29 |
| vs. `mico_source` | 60 | 30 | 0 | `1.86e-9` | 30 |
| vs. `mico_json_ast` | 60 | 2 | 0 | `0.500` | 58 |

## Repair Evidence

- Public-dev repair rows: 186.
- Public-dev rows with repair turns: 10.
- Public-dev accepted compiler-after-repair rows: 10.
- Held-out repair rows: 60.
- Held-out rows with repair turns: 4.
- Held-out accepted compiler-after-repair rows: 4.

All accepted repair-turn wins in this run use the explicitly marked
`deterministic_adapter_instance_collapse` fallback. The paper may claim that
compiler-feedback repair recovers a common adapter-as-instance model error, but
must not claim that the free-form model repair patch alone is broadly reliable.

## Evidence Hashes

| Artifact | SHA-256 |
|---|---|
| `build/llm/bench_execute_dac2027_public_dev_v2.json` | `cc6bc3c11cc9ed434790f85506a8aa5d1c5d154ad66d61faff7ce83fc8fe9803` |
| `build/llm/bench_execute_dac2027_heldout_20.json` | `866902f272cf072b17c5161a3d32e91f592e2b9be2ff67b32924dfc8954b9072` |
| `build/bench/aggregate_dac2027_llm_stats.json` | `c688b7eeb2a5a130da042ea21aecd429a0035f81d4017996f0f126595f7b4b46` |
| `build/llm/repair_fallback_v1_targeted.json` | `5be654a3e02f2d0b49a8d29024826ac4c54f11b765391f9f64ce099075e2a466` |

## Claim Boundary

This v2 matrix supports a Branch A candidate claim for the tested OpenCode Go
profiles: schema-guided MICO JSON AST prompting plus compiler-gated repair
substantially improves positive-task pass rate and unsafe rejection over direct
Verilog, SystemVerilog-interface, and MICO-source prompting on the current
public-development and held-out splits.

The claim remains bounded:

- It covers the tested profiles and prompts, not arbitrary LLMs.
- Repair evidence is limited to the adapter-instance deterministic fallback and
  should be reported separately from plain JSON AST generation.
- Raw provider responses and local configuration remain external ignored
  artifacts.
- The statistics aggregate is schema-valid and regenerated from the v2 result
  files, but external archival of raw/sanitized execute artifacts is still
  required before immutable release.
