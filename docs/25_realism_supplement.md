# Supplemental Realism Benchmark

Snapshot date: 2026-06-15.

This note records the M3.4 deterministic realism supplement. It intentionally
does not change the locked public-development or held-out manifests used by the
authenticated v3 LLM matrix; LLM claims must rerun explicitly before including
this supplement. The current M5 failure-taxonomy and ablation evidence is
summarized in `docs/benchmark_realism_ablation_report_2026-06-16.md`.

## Scope

`benchmarks/module_compose_bench_realism.yaml` contains 14 tasks:

- 8 L1-L4 calibration tasks reused from existing manifests to keep the
  runner's per-level positive/negative invariant active.
- 3 new subsystem positives:
  - `T077_dma_register_map_case`
  - `T079_axis_packetizer_case`
  - `T081_mmio_control_data_path_case`
- 3 paired subsystem negatives:
  - `T078_dma_register_map_reversed_status`
  - `T080_axis_packetizer_missing_stage`
  - `T082_mmio_control_missing_widen`

The new positives cover DMA register-map/status composition, AXI-stream-like
packetization, and memory-mapped control/data width adaptation. Each new
positive has committed MICO source, JSON AST fixture, RTL collateral, directed
simulation, and directed single-clock formal monitors.

## Evidence

Commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/realism_results.json --manifest benchmarks/module_compose_bench_realism.yaml --out-json build/bench/aggregate_realism_results.json --out-dir build/bench/realism_tables --paper-table-dir build/paper_tables/realism"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-manifest benchmarks/module_compose_bench_realism.yaml --bench-result build/bench/realism_results.json --aggregate-result build/bench/aggregate_realism_results.json"
```

Result:

- `expected_outcome_pass: 14/14`
- `compose_pass_1: 7/7`
- `lint_pass: 7/7`
- `sim_pass: 7/7`
- `formal_pass: 6/6`
- `unsafe_rejection: 7/7`
- `json_ast_path: 14/14`
- `sim_mode_counts: {declared: 7}`
- `formal_mode_counts: {declared: 6}`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_realism.yaml` | `9b991781c7cc5f6029229e9c2caabbdf249ec0fdae5dba5e40d9e93bbb370b33` |
| `build/bench/realism_results.json` | `84149b6515df65a7927f02a16f73ce550bcd4e67955b77d8694434810d113bab` |
| `build/bench/aggregate_realism_results.json` | `cabb881aa2c5d869cbd81ae0e21a3709ef4d2256176ae21acb0faa3c16176e0f` |

## Claim Boundary

This supplement raises the committed case-study corpus to 15 positives and at
least 10 paired negative variants across the main, held-out, and supplemental
manifests. It is deterministic-only evidence until the LLM matrix is rerun with
the supplemental manifest included.
