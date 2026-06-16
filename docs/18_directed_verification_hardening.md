# Directed Verification Hardening

Snapshot date: 2026-06-16.

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
- CDC tasks remain lint/simulation smoke only; proof of CDC correctness is not
  claimed.

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
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
.\scripts\eda-docker.ps1 python3 scripts/write-formal-coverage-matrix.py --paper-table-dir paper/tables
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json"
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

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `build/bench/seed_results.json` | `d8b365f4841869c1d5403512f727d62b72ee7e028a237cfc930586bb5d67414f` |
| `build/bench/heldout_results.json` | `dfb08cff44789c07be42d66800eb21431128f22f2cddc6008ec2776f1a8bfc14` |
| `build/bench/realism_results.json` | `84149b6515df65a7927f02a16f73ce550bcd4e67955b77d8694434810d113bab` |
| `build/bench/formal_coverage/formal_coverage_matrix.csv` | `c63dddf03bc8bab62fbd9d255243260d5f357ce0b118d93f71daab68b8c73148` |
| `build/bench/formal_coverage/formal_coverage_tasks.csv` | `1b32f8ed42ebcbecf5eb7625c67d59b9ee8b7abfedf702aa2ce0dd483294dc59` |
| `paper/tables/formal_coverage_matrix.tex` | `ac5e544addc20a801f354a5e74daca9f9576dbfe7188918ef95cf16f5704382a` |

Held-out directed audit result:

- `sim_mode_counts: {declared: 10}`
- `formal_mode_counts: {declared: 9}`
- T063, T064, T069, T071, T073, and T075 now use committed single-clock
  formal monitors. T065 remains CDC smoke-only and intentionally has no formal
  proof claim.

Supplemental realism directed audit result:

- `sim_mode_counts: {declared: 7}`
- `formal_mode_counts: {declared: 6}`
- T077, T079, T081, and the single-clock seed-style realism tasks are covered
  by committed directed monitors; the explicit CDC realism case remains outside
  the formal denominator.
