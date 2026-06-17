# Supplemental Realism Benchmark

Snapshot date: 2026-06-15.

This note records the M3.4 deterministic realism supplement. The supplement is
now also included in the authenticated v4 LLM matrix recorded in
`docs/26_llm_matrix_v4.md`. The older v3 LLM matrix did not include this split.
The current M5 failure-taxonomy and ablation evidence is summarized in
`docs/benchmark_realism_ablation_report_2026-06-16.md`.

## Scope

`benchmarks/module_compose_bench_realism.yaml` contains 30 scoring tasks:

- 18 L1-L6 calibration tasks reused from existing committed collateral to keep
  every level at or above two positives and two negatives.
- 5 subsystem positives:
  - `T063_axi_apb_wrapper_case`
  - `T064_video_filter_pipeline_case`
  - `T077_dma_register_map_case`
  - `T079_axis_packetizer_case`
  - `T081_mmio_control_data_path_case`
- 3 paired subsystem negatives:
  - `T078_dma_register_map_reversed_status`
  - `T080_axis_packetizer_missing_stage`
  - `T082_mmio_control_missing_widen`

The new positives cover DMA register-map/status composition, AXI-stream-like
packetization, and memory-mapped control/data width adaptation. Each new
positive has committed MICO source, RTL collateral, directed simulation, and
directed single-clock formal monitors where the task is in the single-clock
formal denominator.

## Evidence

Commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/realism_results.json --manifest benchmarks/module_compose_bench_realism.yaml --out-json build/bench/aggregate_realism_results.json --out-dir build/bench/realism_tables --paper-table-dir build/paper_tables/realism"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-manifest benchmarks/module_compose_bench_realism.yaml --bench-result build/bench/realism_results.json --aggregate-result build/bench/aggregate_realism_results.json"
```

Result:

- `expected_outcome_pass: 30/30`
- `compose_pass_1: 15/15`
- `lint_pass: 15/15`
- `sim_pass: 15/15`
- `formal_pass: 13/13`
- `qor_available: 4/4`
- `unsafe_rejection: 15/15`
- `json_ast_path: 30/30`
- `sim_mode_counts: {declared: 15}`
- `formal_mode_counts: {declared: 13}`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_realism.yaml` | `5a79fbe5171506143c0382a5854e1adabac9596fe999576ae7ae01db307d3654` |
| `build/bench/realism_results.json` | `6015baea58ef9dc2ce2cf9e184f46e9ac3bdae076c15d7338d37fd22ef9b6f96` |
| `build/bench/aggregate_realism_results.json` | `2372710ba7d9a4e8ecd61cc3357c459cb0256bf79fe488859f0f5ba25ca3f690` |

## Claim Boundary

This supplement raises deterministic subsystem and calibration coverage across
the main, held-out, and supplemental manifests. Its LLM-scored claims are
bounded to the authenticated v4 matrix and tested OpenCode Go profiles.
