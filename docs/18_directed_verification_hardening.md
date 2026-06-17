# Directed Verification Hardening

Snapshot date: 2026-06-16.

This records the M3 verification hardening step for the DAC 2027 plan. It does
not expand CDC correctness claims or timing/QoR claims.

## Scope

The public-development manifest now has:

- 46/46 positive simulation pass.
- 46 committed directed Icarus testbenches.
- 0 generated ready/valid simulation smoke harnesses in the current main
  public-development result.
- 40/40 single-clock bounded formal pass.
- 40 committed directed formal monitors.
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

- `expected_outcome_pass: 83/83`
- `compose_pass_1: 46/46`
- `lint_pass: 46/46`
- `sim_pass: 46/46`
- `formal_pass: 40/40`
- `qor_available: 11/11`
- `unsafe_rejection: 37/37`
- `json_ast_path: 83/83`
- `sim_mode_counts: {declared: 46}`
- `formal_mode_counts: {declared: 40}`

Artifact hashes:

| Artifact | SHA-256 |
|---|---|
| `build/bench/seed_results.json` | `a99afa29d04b6e4a95522173ef998dd5ae20c1eca19ecc2452bb7585e6151c6c` |
| `build/bench/heldout_results.json` | `f9520d217f294aaea9b6928dc52b07cd1593ffd505854af1c1960984ce7534bd` |
| `build/bench/realism_results.json` | `6015baea58ef9dc2ce2cf9e184f46e9ac3bdae076c15d7338d37fd22ef9b6f96` |
| `build/bench/formal_coverage/formal_coverage_matrix.csv` | `d7169660339475b5e5393b5947681549047e0ce095aa5aa3989267e86456118b` |
| `build/bench/formal_coverage/formal_coverage_tasks.csv` | `6b557312d59f1564fd1f58de6c6d1c36eca396f6573a36e4212332b77c95f1cc` |
| `paper/tables/formal_coverage_matrix.tex` | `1039108c1e9903ebc2c3eb0875466df52ece639fa2a00e8c92b7829efc783b3f` |

Held-out directed audit result:

- `sim_mode_counts: {declared: 20}`
- `formal_mode_counts: {declared: 17}`
- T063, T064, T069, T071, T073, and T075 now use committed single-clock
  formal monitors. T065 remains CDC smoke-only and intentionally has no formal
  proof claim.

Supplemental realism directed audit result:

- `sim_mode_counts: {declared: 15}`
- `formal_mode_counts: {declared: 13}`
- T077, T079, T081, and the single-clock seed-style realism tasks are covered
  by committed directed monitors; the explicit CDC realism case remains outside
  the formal denominator.
