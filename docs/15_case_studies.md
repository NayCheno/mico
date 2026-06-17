# Subsystem Case Studies

MICO includes five public-development subsystem case studies in the main
83-task manifest, seven additional held-out subsystem case studies in
`benchmarks/module_compose_bench_heldout.yaml`, and three deterministic
supplemental subsystem positives in
`benchmarks/module_compose_bench_realism.yaml`. They live in
`rtl/case_studies/` and are scored by the standard ModuleComposeBench runner, so
they share the same compiler, lint, simulation, formal, and QoR result schema as
the seed tasks.

## Case Study Tasks

| Task | Focus | RTL | Simulation | Formal | QoR |
|---|---|---|---|---|---|
| `T058_streaming_accelerator_case` | DMA-like source -> skid buffer -> XOR filter -> result sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded ready-valid/data property | yes |
| `T059_width_protocol_bridge_case` | 32-bit sensor stream -> explicit width adapter -> 64-bit accelerator -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data/valid property | yes |
| `T060_register_status_case` | APB-like command source -> register/status transform -> status sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded status property | yes |
| `T061_protocol_bridge_case` | request/response command source -> protocol bridge -> response sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded protocol property | yes |
| `T062_multi_ip_telemetry_case` | telemetry source -> filter -> explicit width adapter -> accumulator -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data/valid property | yes |
| `T063_axi_apb_wrapper_case` | AXI-lite-like command source -> AXI/APB bridge -> APB-like peripheral sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded protocol property | yes |
| `T064_video_filter_pipeline_case` | pixel source -> line buffer -> threshold filter -> frame sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data/valid property | yes |
| `T065_cdc_event_status_case` | event source -> explicit CDC FIFO -> status sink | `rtl/case_studies/mico_case_studies.sv` | yes | not run; CDC smoke only | yes |
| `T069_telemetry_filter_holdout_case` | telemetry source -> filter -> explicit width adapter -> accumulator -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data/valid property | no |
| `T071_protocol_bridge_holdout_case` | request source -> protocol bridge -> response sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded protocol property | no |
| `T073_register_status_holdout_case` | command source -> register file -> status sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded status property | no |
| `T075_video_pipeline_holdout_case` | pixel source -> line buffer -> threshold stage -> frame sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data/valid property | no |
| `T077_dma_register_map_case` | DMA register command source -> register file -> interrupt/status sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded status property | no |
| `T079_axis_packetizer_case` | AXI-stream-like word source -> packetizer -> packet sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded packet property | no |
| `T081_mmio_control_data_path_case` | MMIO control source -> explicit width adapter -> 64-bit data path -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded data property | no |

The held-out case studies have paired negative variants:

- `T066_apb_direct_protocol_mismatch_case` omits the AXI/APB bridge.
- `T067_video_reversed_direction_case` reverses the video stream direction.
- `T068_cdc_without_adapter_case` omits the explicit CDC adapter.
- `T070_telemetry_missing_widen_holdout` omits the widening adapter.
- `T072_protocol_reversed_response_holdout` reverses the response direction.
- `T074_register_status_reversed_holdout` reverses the status direction.
- `T076_video_missing_stage_holdout` references an undeclared threshold stage.
- `T078_dma_register_map_reversed_status` reverses a register-map status path.
- `T080_axis_packetizer_missing_stage` omits the packetizer stage.
- `T082_mmio_control_missing_widen` omits the control/data width adapter.

The public, core held-out, and supplemental positive case-study tasks declare
committed source-level JSON AST fixtures (`expected.ast.json`) in their task
directories. The later held-out extensions rely on regenerated JSON AST checks
under ignored `build/` paths and do not add new QoR or Vivado claims.

## Reproduction

Run the deterministic benchmark in Docker:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

Expected current case-study-inclusive result:

- `expected_outcome_pass: 83/83`
- `compose_pass_1: 46/46`
- `lint_pass: 46/46`
- `sim_pass: 46/46`
- `formal_pass: 40/40`
- `qor_available: 11/11`
- `unsafe_rejection: 37/37`
- `json_ast_path: 83/83`

Run the held-out split separately:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
```

Expected held-out result:

- `expected_outcome_pass: 40/40`
- `compose_pass_1: 20/20`
- `lint_pass: 20/20`
- `sim_pass: 20/20`
- `formal_pass: 17/17`
- `qor_available: 6/6`
- `unsafe_rejection: 20/20`
- `json_ast_path: 40/40`

Run the deterministic realism supplement separately:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
```

Expected supplemental result:

- `expected_outcome_pass: 30/30`
- `compose_pass_1: 15/15`
- `lint_pass: 15/15`
- `sim_pass: 15/15`
- `formal_pass: 13/13`
- `qor_available: 4/4`
- `unsafe_rejection: 15/15`
- `json_ast_path: 30/30`

Generate aggregate CSV and TeX snippets from the same JSON:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

The generated files remain under ignored `build/` directories. The committed
source of truth is the case-study RTL, MICO task source, simulation/formal
harnesses, QoR references, and benchmark manifest metadata.
