# Held-Out Benchmark Hardening

Snapshot date: 2026-06-14.

This records the M5 held-out benchmark expansion for the DAC 2027 plan. The
held-out manifest is committed for reproducible scoring, but generated result
JSON and aggregate tables remain under ignored `build/` paths.

## Scope

`benchmarks/module_compose_bench_heldout.yaml` now contains:

- 20 held-out tasks.
- 10 positive and 10 negative tasks.
- 7 subsystem positive case-study variants.
- 7 paired case-study negative variants.
- Per-task request text, module/interface/adapter inventories, expected
  diagnostics, RTL collateral, expected features, and prompt-leakage policy.

New M5 tasks:

| Task | Type | Focus |
|---|---|---|
| `T069_telemetry_filter_holdout_case` | positive | telemetry filter -> explicit width adapter -> accumulator -> host |
| `T070_telemetry_missing_widen_holdout` | negative | 32-bit telemetry output connected directly to 64-bit accumulator |
| `T071_protocol_bridge_holdout_case` | positive | request/response protocol bridge with renamed instances |
| `T072_protocol_reversed_response_holdout` | negative | reversed response direction |
| `T073_register_status_holdout_case` | positive | APB-like command -> register file -> status sink |
| `T074_register_status_reversed_holdout` | negative | reversed status direction |
| `T075_video_pipeline_holdout_case` | positive | pixel stream -> line buffer -> threshold stage -> frame sink |
| `T076_video_missing_stage_holdout` | negative | undeclared threshold stage instance |

## Evidence

Commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/m5_heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m5_heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_m5_heldout.json --out-dir build/bench/heldout_m5_tables --paper-table-dir build/paper_tables/heldout_m5"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/m5_heldout_results.json --aggregate-result build/bench/aggregate_m5_heldout.json"
```

Result:

- `expected_outcome_pass: 20/20`
- `compose_pass_1: 10/10`
- `lint_pass: 10/10`
- `sim_pass: 10/10`
- `formal_pass: 9/9`
- `qor_available: 3/3`
- `unsafe_rejection: 10/10`
- `json_ast_path: 20/20`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_heldout.yaml` | `64953d83d910e840b400d0c58825442a1a3e6318aeb23b3b0b3b6cfde2e9e251` |
| `build/bench/m5_heldout_results.json` | `de4724da19caabf367759021db4e1fbd4e864abb3ddd9033450ede301c06ca7e` |
| `build/bench/aggregate_m5_heldout.json` | `1535c21ada6eae75c71c66890a7d5ee3a2524be17388d0bdeaba556c079b835e` |

## LLM Split Refresh

Because this change revises the held-out manifest, the authenticated held-out
LLM matrix was rerun for the 20-task split and the public-dev v2 matrix was
re-aggregated with the new held-out result. Detailed pass-rate tables live in
`docs/22_llm_full_matrix_v2.md`.

Additional commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --manifest benchmarks/module_compose_bench_heldout.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_dac2027_heldout_20.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_results.json --llm-result build/llm/bench_execute_dac2027_public_dev_v2.json --llm-result build/llm/bench_execute_dac2027_heldout_20.json --out-json build/bench/aggregate_dac2027_llm_heldout20.json"
```

LLM refresh hashes:

| Artifact | SHA-256 |
|---|---|
| `build/llm/bench_execute_dac2027_heldout_20.json` | `866902f272cf072b17c5161a3d32e91f592e2b9be2ff67b32924dfc8954b9072` |
| `build/bench/aggregate_dac2027_llm_heldout20.json` | `65165e8c55ff2d8c4abf1d15a8b793c2ba9caa153b42aabba666e1b7ba832e2e` |

## Claim Boundary

This strengthens the dev/held-out split and the subsystem case-study corpus.
It does not make the held-out split private, and it does not add new Vivado or
QoR claims for T069--T076. LLM and paper claims must report public-development
and held-out scores separately and bind any archived results to the manifest
SHA-256 above.
