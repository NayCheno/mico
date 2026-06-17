# Held-Out Benchmark Hardening

Snapshot date: 2026-06-16.

This records the M5 held-out benchmark expansion for the DAC 2027 plan. The
held-out manifest is committed for reproducible scoring, but generated result
JSON and aggregate tables remain under ignored `build/` paths.

## Scope

`benchmarks/module_compose_bench_heldout.yaml` now contains:

- 40 held-out tasks.
- 20 positive and 20 negative tasks.
- 7 subsystem positive case-study variants.
- 7 paired case-study negative variants.
- 20 committed directed held-out simulation testbenches; the expanded seed
  calibration positives now reuse committed directed public seed benches.
- 17 committed directed single-clock held-out formal monitors covering all
  non-CDC held-out positives; CDC cases remain formal not-run.
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

- `expected_outcome_pass: 40/40`
- `compose_pass_1: 20/20`
- `lint_pass: 20/20`
- `sim_pass: 20/20`
- `formal_pass: 17/17`
- `qor_available: 6/6`
- `unsafe_rejection: 20/20`
- `json_ast_path: 40/40`
- `sim_mode_counts: {declared: 20}`
- `formal_mode_counts: {declared: 17}`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `benchmarks/module_compose_bench_heldout.yaml` | `cb04838fbe4332b4bb94d9bfeddb7f2cdd62bd59993c6475696161418d7c69c5` |
| `build/bench/heldout_results.json` | `f9520d217f294aaea9b6928dc52b07cd1593ffd505854af1c1960984ce7534bd` |
| `build/bench/aggregate_heldout_results.json` | `24207a889350fa6dba9682776fe9394f8ae98743d80a0dfe8ab1630e90854860` |

## LLM Split Refresh

The original M5 change revised the held-out task set. The later v4 LLM matrix
binds the authenticated held-out execute record to the expanded 40-task
held-out manifest hash and supersedes the historical v3 held-out binding for
current submission claims. Detailed v4 pass-rate tables live in
`docs/26_llm_matrix_v4.md` and `docs/llm_final_matrix_report.md`.

Additional commands:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_expanded_v4.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json --llm-result build/llm/bench_execute_public_expanded_v4.json --llm-result build/llm/bench_execute_heldout_expanded_v4.json --llm-result build/llm/bench_execute_realism_v4.json --out-json build/bench/aggregate_llm_v4.json --out-dir build/bench/llm_v4 --paper-table-dir build/paper_tables/llm_v4"
```

LLM refresh hashes:

| Artifact | SHA-256 |
|---|---|
| `build/llm/bench_execute_heldout_expanded_v4.json` | `e1f34005cf0f2c8961f52af74ed9af6c93238ca8193f34bd429d07ebe5a1180a` |
| `build/bench/aggregate_llm_v4.json` | `de6f090be33ec5ce7f7eceb36a89135ecc5dd6268e6125c16900a8e070d3ddd3` |

## Claim Boundary

This strengthens the dev/held-out split, the subsystem case-study corpus, and
the directed verification denominator. It does not make the held-out split
private, and it does not add new Vivado or QoR claims for T069--T076. LLM and
paper claims must report public-development and held-out scores separately and
bind any archived results to the manifest SHA-256 above.
