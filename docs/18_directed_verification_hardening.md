# Directed Verification Hardening

Snapshot date: 2026-06-14.

This records the M3 verification hardening step for the DAC 2027 plan. It does
not expand CDC correctness claims or timing/QoR claims.

## Scope

The public-development manifest now has:

- 36/36 positive simulation pass.
- 28 committed directed Icarus testbenches.
- 8 generated ready/valid simulation smoke harnesses.
- 31/31 single-clock bounded formal pass.
- 20 committed directed formal monitors.
- 11 generated ready/valid formal smoke monitors.
- CDC tasks remain lint/simulation smoke only; no CDC proof is claimed.

New directed collateral covers:

- direct-stream variants: T013, T014, T017, T020;
- FIFO-chain variants: T015, T016, T018, T019;
- width-adapter variants: T021, T022, T023.
- M3 added directed simulation for T024, T025, T026, T031, T032, T051, T052,
  and T056.
- M3 added directed single-clock formal monitors for T001, T024, T025, T026,
  T031, and T032.

## Evidence

Command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_manifest.yaml --output build/bench/m3_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/m3_heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_results.json --out-json build/bench/aggregate_m3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-result build/bench/m3_results.json"
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
- `sim_mode_counts: {declared: 28, autogen: 8}`
- `formal_mode_counts: {declared: 20, autogen: 11}`

Artifact hash:

| Artifact | SHA-256 |
|---|---|
| `build/bench/m3_results.json` | `c3c3911f2521d0f1d9e5c3932a9b705c91b717894dca1b1286494f24cc20b8ca` |
| `build/bench/m3_heldout_results.json` | `10f5e13982ff4dcc585b7831e7f2f27d87e66656f0bddea56aa37701dd1a8db4` |
| `build/bench/aggregate_m3.json` | `e90333191861bf5980f7b51179c4a27b1a4796f76d963e257c4ccf557bc2cabf` |
