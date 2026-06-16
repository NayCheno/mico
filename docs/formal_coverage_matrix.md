# MICO Formal Coverage Matrix

Snapshot date: 2026-06-16.

This matrix documents the bounded formal evidence used by the DAC 2027
candidate. Numeric claims are mapped in `docs/release_claim_table.md`; generated
CSV/TeX artifacts are written under ignored `build/` paths by
`scripts/write-formal-coverage-matrix.py`.

## Scope

The current formal denominator is intentionally bounded and single-clock. Every
task counted in the denominator uses a declared committed harness; generated
fallback harnesses are not counted in the paper claim:

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
  full multi-clock proof of CDC correctness remains unclaimed.

A future CDC proof can add an assume/guarantee wrapper around an asynchronous
FIFO model, but that must be reported separately from the current single-clock
bounded matrix.

## Contract Boundary

The theorem-like claim for the current v0 contract subset is:

> If a MICO composition passes the compiler, every connection that requires a
> declared ready/valid adapter guarantee has a matching adapter declaration and
> every counted single-clock formal task has a committed bounded monitor for its
> task-specific ready/valid, payload, bridge, register/status, or filter
> relation.

This is a compiler coverage and bounded-smoke claim. It is not a temporal proof
of arbitrary contracts, not a liveness theorem, and not a proof of multi-clock
behavior. The generated SVA skeletons and directed monitors provide traceable
evidence paths; they do not replace a future complete contract-verification
engine.

## Current Evidence Hashes

| Artifact | SHA-256 |
|---|---|
| `build/bench/formal_coverage/formal_coverage_matrix.csv` | `c63dddf03bc8bab62fbd9d255243260d5f357ce0b118d93f71daab68b8c73148` |
| `build/bench/formal_coverage/formal_coverage_tasks.csv` | `1b32f8ed42ebcbecf5eb7625c67d59b9ee8b7abfedf702aa2ce0dd483294dc59` |
| `paper/tables/formal_coverage_matrix.tex` | `ac5e544addc20a801f354a5e74daca9f9576dbfe7188918ef95cf16f5704382a` |

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
