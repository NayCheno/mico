# Subsystem Case Studies

MICO includes five dedicated subsystem case studies in addition to the original
57 hand-written seed tasks. They live in `rtl/case_studies/` and are scored by
the standard ModuleComposeBench runner, so they share the same compiler, lint,
simulation, formal, and QoR result schema as the seed tasks.

## Case Study Tasks

| Task | Focus | RTL | Simulation | Formal | QoR |
|---|---|---|---|---|---|
| `T058_streaming_accelerator_case` | DMA-like source -> skid buffer -> XOR filter -> result sink | `rtl/case_studies/mico_case_studies.sv` | yes | bounded ready-valid/data property | yes |
| `T059_width_protocol_bridge_case` | 32-bit sensor stream -> explicit width adapter -> 64-bit accelerator -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | not run | yes |
| `T060_register_status_case` | APB-like command source -> register/status transform -> status sink | `rtl/case_studies/mico_case_studies.sv` | yes | not run | yes |
| `T061_protocol_bridge_case` | request/response command source -> protocol bridge -> response sink | `rtl/case_studies/mico_case_studies.sv` | yes | generated smoke | yes |
| `T062_multi_ip_telemetry_case` | telemetry source -> filter -> explicit width adapter -> accumulator -> host sink | `rtl/case_studies/mico_case_studies.sv` | yes | generated smoke | yes |

## Reproduction

Run the deterministic benchmark in Docker:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

Expected current case-study-inclusive result:

- `expected_outcome_pass: 62/62`
- `compose_pass_1: 36/36`
- `lint_pass: 36/36`
- `sim_pass: 36/36`
- `formal_pass: 31/31`
- `qor_available: 9/9`
- `unsafe_rejection: 26/26`
- `json_ast_path: 62/62`

Generate aggregate CSV and TeX snippets from the same JSON:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

The generated files remain under ignored `build/` directories. The committed
source of truth is the case-study RTL, MICO task source, simulation/formal
harnesses, QoR references, and benchmark manifest metadata.
