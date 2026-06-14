# Directed Verification Hardening

Snapshot date: 2026-06-14.

This records the M4 verification hardening step for the DAC 2027 plan. It does
not expand CDC correctness claims or timing/QoR claims.

## Scope

The public-development manifest now has:

- 36/36 positive simulation pass.
- 20 committed directed Icarus testbenches.
- 16 generated ready/valid simulation smoke harnesses.
- 31/31 single-clock bounded formal pass.
- 14 committed directed formal monitors.
- 17 generated ready/valid formal smoke monitors.
- CDC tasks remain lint/simulation smoke only; no CDC proof is claimed.

New directed collateral covers:

- direct-stream variants: T013, T014, T017, T020;
- FIFO-chain variants: T015, T016, T018, T019;
- width-adapter variants: T021, T022, T023.

## Evidence

Command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_manifest.yaml --output build/bench/m4_results.json"
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
- `sim_mode_counts: {declared: 20, autogen: 16}`
- `formal_mode_counts: {declared: 14, autogen: 17}`

Artifact hash:

| Artifact | SHA-256 |
|---|---|
| `build/bench/m4_results.json` | `c1596b102263be3e4ec470e45f6f90a7fce44cd29ae305fdffe669ac3dfe3d86` |
