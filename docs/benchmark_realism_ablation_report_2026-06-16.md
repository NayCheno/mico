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
| Public-development | 62/62 | 36/36 | 36/36 | 36/36 | 31/31 | 9/9 | 26/26 | 62/62 |
| Held-out | 20/20 | 10/10 | 10/10 | 10/10 | 9/9 | 3/3 | 10/10 | 20/20 |
| Supplemental realism | 14/14 | 7/7 | 7/7 | 7/7 | 6/6 | 0/0 | 7/7 | 14/14 |

The committed case-study corpus now includes 15 positive subsystem tasks and
at least 10 paired negative variants. The supplemental realism split contributes
three new positive subsystem cases and three paired negative subsystem variants:

- `T077_dma_register_map_case` / `T078_dma_register_map_reversed_status`
- `T079_axis_packetizer_case` / `T080_axis_packetizer_missing_stage`
- `T081_mmio_control_data_path_case` / `T082_mmio_control_missing_widen`

The supplemental realism split remains deterministic-only evidence until an
authenticated LLM matrix explicitly reruns that manifest.

## Failure Taxonomy

The deterministic aggregate records include compiler diagnostic taxonomy rows:

| Split | Taxonomy rows | Source CSV SHA-256 |
|---|---:|---|
| Public-development | 17 | `461e777d2563a1e01080b3857fadcbd1088be4ce9961781092e9b8eccc2abe13` |
| Held-out | 5 | `c411ac34f0b57fea635501facf285b39e90bc2b76894fcbe38f4416bacd34c7d` |
| Supplemental realism | 4 | `a4daec8c88e5da8a9fb586b6eefa8cf65f3c72a5f74d2299a658bec87b9cabd9` |

The public-development taxonomy covers adapter misuse, clock-domain mismatch,
contract violations, direction reversal, duplicate declarations, interface and
protocol mismatch, unknown module/interface/port/instance references, and width
mismatch. The LLM v3 aggregate additionally records 57 failure-taxonomy rows for
schema invalid JSON, direct RTL lint failures, hallucinated binding/reference
errors, compiler diagnostics, and repair outcomes.

## Ablation Evidence

The deterministic aggregate emits five conservative counterfactual rows:

- `no_contract_checks`
- `no_clock_domain_checks`
- `no_adapter_library`
- `no_structured_diagnostics`
- `dsl_vs_json_ast`

| Split | Ablation rows | Source CSV SHA-256 |
|---|---:|---|
| Public-development | 5 | `18d0134f9607ec83455e140dec9fdbb534559d45c8e6cf4a421ef4b5dff4680d` |
| Held-out | 5 | `cf26b2ddb07f9d9d5baacee20783fd528efa2cd05e226138a9052cf78762ab69` |
| Supplemental realism | 5 | `64a6db47ef86917d4edeed6db8354d7d8ee0e7a6c2a28f9e6ec744cfa8e80d5a` |

The LLM v3 ablation surface is represented by the five baseline families:
Direct Verilog, SystemVerilog interface, MICO source, MICO JSON AST, and MICO
JSON AST repair. The paired comparison and compact summary tables are bound in
`docs/24_llm_matrix_v3.md` and `docs/llm_v3_artifact_readme.md`.

## Artifact Hashes

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_realism.yaml` | `9b991781c7cc5f6029229e9c2caabbdf249ec0fdae5dba5e40d9e93bbb370b33` |
| `build/bench/aggregate_results.json` | `1d8144d7ffa09460782f9a40496425538e4adc2f0c7ba621c6d336e41964f7da` |
| `build/bench/aggregate_heldout_results.json` | `1809f695c65b57443f3cf3aba803445547e209aed5a2746f6c5c2b2676c92376` |
| `build/bench/aggregate_realism_results.json` | `cabb881aa2c5d869cbd81ae0e21a3709ef4d2256176ae21acb0faa3c16176e0f` |
| `paper/tables/benchmark_split_summary.tex` | `8f7f9cdb63a2db32a2fd39fc6bd4539dd1d50c286631124beeeec2c9bc1d32ff` |

## Claim Boundary

M5 does not add LLM claims for the supplemental realism split and does not add
new Vivado or QoR claims beyond the scoped 12-task subset. Public-development,
held-out, and supplemental realism results must be reported separately.
