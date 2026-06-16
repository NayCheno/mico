# Held-Out Benchmark Hardening

Snapshot date: 2026-06-16.

This records the M5 held-out benchmark expansion for the DAC 2027 plan. The
held-out manifest is committed for reproducible scoring, but generated result
JSON and aggregate tables remain under ignored `build/` paths.

## Scope

`benchmarks/module_compose_bench_heldout.yaml` now contains:

- 20 held-out tasks.
- 10 positive and 10 negative tasks.
- 7 subsystem positive case-study variants.
- 7 paired case-study negative variants.
- 10 committed directed held-out simulation testbenches; the three seed
  calibration positives now reuse committed directed public seed benches.
- 9 committed directed single-clock held-out formal monitors covering all
  non-CDC held-out positives; the CDC case remains formal not-run.
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
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_heldout_results.json --out-dir build/bench/heldout_tables --paper-table-dir build/paper_tables/heldout"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/heldout_results.json --aggregate-result build/bench/aggregate_heldout_results.json"
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
- `sim_mode_counts: {declared: 10}`
- `formal_mode_counts: {declared: 9}`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_heldout.yaml` | `8aebbfe8b1c1f9cf67cbb112e6fa9d439e4cded2fd8cf8b270041c8b37380ffd` |
| `build/bench/heldout_results.json` | `dfb08cff44789c07be42d66800eb21431128f22f2cddc6008ec2776f1a8bfc14` |
| `build/bench/aggregate_heldout_results.json` | `1809f695c65b57443f3cf3aba803445547e209aed5a2746f6c5c2b2676c92376` |

## LLM Split Refresh

The original M5 change revised the held-out task set. The refreshed v3 LLM
matrix now binds the authenticated held-out execute record to the current
20-task held-out manifest hash above and supersedes the v2 held-out binding for
submission claims. Detailed v3 pass-rate tables live in
`docs/24_llm_matrix_v3.md` and `docs/llm_final_matrix_report.md`.

Additional commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_execute_public_dev_v3.json --llm-result build/llm/bench_execute_heldout_v3.json --out-json build/bench/aggregate_llm_v3.json --out-dir build/bench/llm_v3 --paper-table-dir build/paper_tables/llm_v3"
```

LLM refresh hashes:

| Artifact | SHA-256 |
|---|---|
| `build/llm/bench_execute_heldout_v3.json` | `04fd36350ddb17dfd220bcf2825df2c3cd4f9188d3f014dd01b70fc9e48d5f7e` |
| `build/bench/aggregate_llm_v3.json` | `b300110f02742db13627fba8ed7d3382ac6ddbab7d101eefa4a1633ca9930309` |

## Claim Boundary

This strengthens the dev/held-out split, the subsystem case-study corpus, and
the directed verification denominator. It does not make the held-out split
private, and it does not add new Vivado or QoR claims for T069--T076. LLM and
paper claims must report public-development and held-out scores separately and
bind any archived results to the manifest SHA-256 above.
