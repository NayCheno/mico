# MICO Benchmark Realism And Ablation Report 2026-06-16

This report records the M5 benchmark-realism, failure-taxonomy, and ablation
evidence for the current DAC 2027 paper branch. Generated JSON, CSV, and TeX
artifacts stay under ignored `build/` paths.

## Commands

Run in the Docker EDA environment:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_heldout_results.json --out-dir build/bench/heldout_tables --paper-table-dir build/paper_tables/heldout"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/realism_results.json --manifest benchmarks/module_compose_bench_realism.yaml --out-json build/bench/aggregate_realism_results.json --out-dir build/bench/realism_tables --paper-table-dir build/paper_tables/realism"
.\scripts\eda-docker.ps1 python3 scripts/write-paper-summary-tables.py --out-dir paper/tables
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json --aggregate-result build/bench/aggregate_results.json --aggregate-result build/bench/aggregate_heldout_results.json --aggregate-result build/bench/aggregate_realism_results.json"
```

## Split Results

| Split | Expected | Compose | Lint | Simulation | Formal | QoR | Unsafe | JSON AST |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Public-development | 83/83 | 46/46 | 46/46 | 46/46 | 40/40 | 11/11 | 37/37 | 83/83 |
| Held-out | 40/40 | 20/20 | 20/20 | 20/20 | 17/17 | 6/6 | 20/20 | 40/40 |
| Supplemental realism | 30/30 | 15/15 | 15/15 | 15/15 | 13/13 | 4/4 | 15/15 | 30/30 |

The committed case-study corpus now includes expanded public-development,
held-out, and supplemental realism subsystem positives plus paired negative
variants. The supplemental realism split contributes deterministic subsystem
cases and paired negative subsystem variants including:

- `T077_dma_register_map_case` / `T078_dma_register_map_reversed_status`
- `T079_axis_packetizer_case` / `T080_axis_packetizer_missing_stage`
- `T081_mmio_control_data_path_case` / `T082_mmio_control_missing_widen`

The supplemental realism split remains deterministic-only evidence until an
authenticated LLM matrix explicitly reruns that manifest.

## Failure Taxonomy

The deterministic aggregate records include compiler diagnostic taxonomy rows:

| Split | Taxonomy rows | Source CSV SHA-256 |
|---|---:|---|
| Public-development | 17 | `97b6cc2d07f66ad0f3b4959a3f9889f0ebd8aec0cb25bcc9be030a67e51359ac` |
| Held-out | 10 | `475101bf824b3dca2ccbfbda383083d5020ae8f0540488ca966ad57e64297070` |
| Supplemental realism | 7 | `094b16e729b96085f851e1a839aad8eb38c1f90fb5725af6538a03407627a6be` |

The public-development taxonomy covers adapter misuse, clock-domain mismatch,
contract violations, direction reversal, duplicate declarations, interface and
protocol mismatch, unknown module/interface/port/instance references, and width
mismatch. The LLM v3 aggregate additionally records 57 failure-taxonomy rows for
schema invalid JSON, direct RTL lint failures, hallucinated binding/reference
errors, compiler diagnostics, and repair outcomes.

## Ablation Evidence

The deterministic aggregate emits six conservative guard-surface rows matching
the CCF-A checklist:

- `no_json_schema`
- `no_compiler_feedback`
- `no_repair`
- `no_adapter_contract_check`
- `no_eda_lint_gate`
- `no_prompt_leakage_controls`

| Split | Ablation rows | Source CSV SHA-256 |
|---|---:|---|
| Public-development | 6 | `f246b3d06392f4be974b948a14ccf02a6374b0934c45297dda045e5fd8efa083` |
| Held-out | 6 | `02ca947f572ff1abfb007a41486ab26217d30284baa56b2b70840762b27d3c72` |
| Supplemental realism | 6 | `00f27c84b882839e98e80bd22a1082b73a156f7c37b008c15da868ea6af451fd` |

The LLM v3 ablation surface is represented by the five baseline families:
Direct Verilog, SystemVerilog interface, MICO source, MICO JSON AST, and MICO
JSON AST repair. The paired comparison table now includes exact sign-test
p-values and net matched-pair effect sizes. The compact summary tables are bound in
`docs/24_llm_matrix_v3.md` and `docs/llm_v3_artifact_readme.md`.

## Artifact Hashes

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_realism.yaml` | `5a79fbe5171506143c0382a5854e1adabac9596fe999576ae7ae01db307d3654` |
| `build/bench/aggregate_results.json` | `ed1a8ebfbf8ca71303271ab9023ee4eff8bcfc4af4048b3d819a5f3d2e957bb0` |
| `build/bench/aggregate_heldout_results.json` | `24207a889350fa6dba9682776fe9394f8ae98743d80a0dfe8ab1630e90854860` |
| `build/bench/aggregate_realism_results.json` | `2372710ba7d9a4e8ecd61cc3357c459cb0256bf79fe488859f0f5ba25ca3f690` |
| `paper/tables/benchmark_split_summary.tex` | `b995e738fb055487cbbd311b7c334d6b5feb1c70e1a6a5e4ab88167e4ddfd17d` |
| `paper/tables/ablation_counterfactual.tex` | `64a3bd8388534548530d926d743ac8ba7f33910db982ae3be55d7f0cddf52e28` |
| `paper/tables/llm_paired_comparisons.tex` | `81f1da056ddaa90c5da0e0c46e4ab03047eb630ca601ad294c7425bed69ad091` |

## Claim Boundary

M5 does not add LLM claims for the supplemental realism split and does not add
new Vivado or QoR claims beyond the scoped 12-task subset. Public-development,
held-out, and supplemental realism results must be reported separately.
