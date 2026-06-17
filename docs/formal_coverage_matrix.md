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

- public-development: 40/40 enabled formal monitors pass;
- held-out: 17/17 enabled formal monitors pass;
- supplemental realism: 13/13 enabled formal monitors pass.

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
| direct_ready_valid_payload | 10/10 | 6/6 | 4/4 | payload stability and ready/valid after reset |
| fifo_pipeline_ordering | 11/11 | 4/4 | 3/3 | bounded no-drop/no-duplicate proxy and payload ordering |
| width_payload_relation | 10/10 | 3/3 | 3/3 | widening/packing relation and handshake coupling |
| register_status_visibility | 3/3 | 1/1 | 1/1 | command-to-status visibility and payload relation |
| protocol_request_response | 4/4 | 2/2 | 2/2 | request/response or bridge payload refinement |
| telemetry_filter_predicate | 2/2 | 1/1 | 0/0 | filter/accumulator payload predicate preservation |

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
| `build/bench/formal_coverage/formal_coverage_matrix.csv` | `d7169660339475b5e5393b5947681549047e0ce095aa5aa3989267e86456118b` |
| `build/bench/formal_coverage/formal_coverage_tasks.csv` | `6b557312d59f1564fd1f58de6c6d1c36eca396f6573a36e4212332b77c95f1cc` |
| `paper/tables/formal_coverage_matrix.tex` | `1039108c1e9903ebc2c3eb0875466df52ece639fa2a00e8c92b7829efc783b3f` |

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
