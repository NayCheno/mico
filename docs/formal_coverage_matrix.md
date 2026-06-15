# MICO Formal Coverage Matrix

Snapshot date: 2026-06-15.

This matrix documents the bounded formal evidence used by the DAC 2027
candidate. Numeric claims are mapped in `docs/release_claim_table.md`; generated
CSV/TeX artifacts are written under ignored `build/` paths by
`scripts/write-formal-coverage-matrix.py`.

## Scope

The current formal denominator is intentionally bounded and single-clock:

- public-development: 31/31 enabled formal monitors pass;
- held-out: 9/9 enabled formal monitors pass;
- supplemental realism: 6/6 enabled formal monitors pass.

The enabled monitors are committed directed SystemVerilog bind modules under
`benchmarks/formal/`. They are task-specific in the repository sense: each
monitor binds named generated wires for one benchmark task and checks the
payload, ready/valid, bridge, register/status, or filter relation expected for
that task.

## Property Classes

`paper/tables/formal_coverage_matrix.tex` is generated from the deterministic
benchmark result JSON and summarizes:

| Property class | Public-dev | Held-out | Realism | Boundary |
|---|---:|---:|---:|---|
| direct_ready_valid_payload | 8/8 | 3/3 | 1/1 | payload stability and ready/valid after reset |
| fifo_pipeline_ordering | 11/11 | 1/1 | 1/1 | bounded no-drop/no-duplicate proxy and payload ordering |
| width_payload_relation | 9/9 | 1/1 | 2/2 | widening/packing relation and handshake coupling |
| register_status_visibility | 1/1 | 1/1 | 1/1 | command-to-status visibility and payload relation |
| protocol_request_response | 1/1 | 2/2 | 1/1 | request/response or bridge payload refinement |
| telemetry_filter_predicate | 1/1 | 1/1 | 0/0 | filter/accumulator payload predicate preservation |

This exceeds the minimum M3 gate of at least 8 public task-specific monitors,
at least 3 held-out task-specific monitors, and at least 4 property classes.

## CDC Boundary

CDC remains outside the formal proof claim. The current CDC treatment is:

- compiler requires explicit CDC adapters for cross-domain connections;
- simulation/lint smoke exercises explicit adapter integration;
- CDC formal monitors are not counted in the single-clock formal denominator;
- physical metastability, synchronizer MTBF, gray-pointer FIFO correctness, and
  full multi-clock CDC proof remain unclaimed.

A future CDC proof can add an assume/guarantee wrapper around an asynchronous
FIFO model, but that must be reported separately from the current single-clock
bounded matrix.

## Reproduction

Run after deterministic benchmark results exist:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/write-formal-coverage-matrix.py
```

The command writes:

- `build/bench/formal_coverage/formal_coverage_tasks.csv`
- `build/bench/formal_coverage/formal_coverage_matrix.csv`
- `build/paper_tables/formal/formal_coverage_matrix.tex`

To refresh the committed paper table snapshot after a validated evidence run:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/write-formal-coverage-matrix.py --paper-table-dir paper/tables
```

Do not commit generated CSV files under `build/`.
