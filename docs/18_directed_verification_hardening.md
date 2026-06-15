# Directed Verification Hardening

Snapshot date: 2026-06-15.

This records the M3 verification hardening step for the DAC 2027 plan. It does
not expand CDC correctness claims or timing/QoR claims.

## Scope

The public-development manifest now has:

- 36/36 positive simulation pass.
- 36 committed directed Icarus testbenches.
- 0 generated ready/valid simulation smoke harnesses in the current main
  public-development result.
- 31/31 single-clock bounded formal pass.
- 31 committed directed formal monitors.
- 0 generated ready/valid formal smoke monitors in the current main
  public-development result.
- CDC tasks remain lint/simulation smoke only; no CDC proof is claimed.

New directed collateral covers:

- direct-stream variants: T013, T014, T017, T020;
- FIFO-chain variants: T015, T016, T018, T019;
- width-adapter variants: T021, T022, T023.
- M3 added directed simulation for T024, T025, T026, T031, T032, T051, T052,
  and T056.
- M3 added directed single-clock formal monitors for T001, T024, T025, T026,
  T031, and T032.
- The 2026-06-15 M3 audit pass added directed simulation for T053, T054, T055,
  and T057.
- The 2026-06-15 M3.1 audit pass added directed simulation for T027, T028,
  T029, and T030, replacing the remaining public generated simulation fallback.
- The 2026-06-15 M3.1 audit pass bound existing directed simulation for
  held-out seed calibration positives T013, T019, and T021.
- The 2026-06-15 M3 audit pass added directed single-clock formal monitors for
  T051, T052, T053, and T054.
- The 2026-06-15 M3.2 audit pass added directed single-clock formal monitors
  for T055, T056, T057, and T059--T062, replacing the remaining public
  generated formal fallback.
- The 2026-06-15 M3.2 audit pass bound existing directed formal monitors for
  held-out seed calibration positives T013, T019, and T021.

## Evidence

Command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_manifest.yaml --output build/bench/m3_public_directed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/m3_heldout_directed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_public_directed_results.json --out-json build/bench/aggregate_m3_public_directed.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-result build/bench/m3_public_directed_results.json"
```

Result:

- `expected_outcome_pass: 62/62`
- `compose_pass_1: 36/36`
- `lint_pass: 36/36`
- `sim_pass: 36/36`
- `formal_pass: 31/31`
- `qor_available: 9/9`
- `unsafe_rejection: 26/26`
- `json_ast_path: 62/62`
- `sim_mode_counts: {declared: 36}`
- `formal_mode_counts: {declared: 31}`

Artifact hash:

| Artifact | SHA-256 |
|---|---|
| `build/bench/m3_public_directed_results.json` | `7eb098d4bac7c537c4051a743897f9913d5f95d935e639282aa4652d22a553bb` |
| `build/bench/m3_heldout_directed_results.json` | `436585587c2f9e4560f7c93e4f33fdaa30aaedc7d4c05f82b9d14c97532cef7f` |
| `build/bench/aggregate_m3_public_directed.json` | `06da5a85b1e2a2d25c491f988167dc070fbab5ee86a8ebec525652cd89252c1d` |

Held-out directed audit result:

- `sim_mode_counts: {declared: 10}`
- `formal_mode_counts: {declared: 9}`
- T063, T064, T069, T071, T073, and T075 now use committed single-clock
  formal monitors. T065 remains CDC smoke-only and intentionally has no formal
  proof claim.
