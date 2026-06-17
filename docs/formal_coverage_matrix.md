# MICO Formal Coverage Matrix

Snapshot date: 2026-06-17.

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
payload, ready/valid, bridge, register/status, width, or filter relation
expected for that task. `scripts/write-formal-coverage-matrix.py` expands each
task into one or more proof obligations so the reviewer-facing table is
property-obligation by split, not just a pass-count rollup.

## Property Obligations

`paper/tables/formal_coverage_matrix.tex` is generated from the deterministic
benchmark result JSON and summarizes:

| Property obligation | Public-dev | Held-out | Realism | Boundary |
|---|---:|---:|---:|---|
| ready_valid_stability | 40/40 | 17/17 | 13/13 | payload stability while valid is held before ready |
| fire_implies_transfer | 40/40 | 17/17 | 13/13 | valid-and-ready fire event advances the modeled transfer |
| no_combinational_self_loop | 40/40 | 17/17 | 13/13 | bounded smoke excludes combinational ready/valid self-loop |
| no_drop_bounded | 14/14 | 6/6 | 4/4 | FIFO, skid, or pipeline monitor observes no dropped transfer in bound |
| no_duplicate_bounded | 14/14 | 6/6 | 4/4 | FIFO, skid, or pipeline monitor observes no duplicate transfer in bound |
| width_extension_correctness | 10/10 | 3/3 | 3/3 | width adapter output preserves and zero-extends source payload |
| register_status_visibility | 6/6 | 2/2 | 2/2 | command payload becomes visible on status path in bound |
| protocol_request_response | 4/4 | 2/2 | 2/2 | request/response or bridge payload refinement is preserved |
| telemetry_filter_predicate | 2/2 | 1/1 | 0/0 | filter or accumulator predicate relation is preserved |

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

`scripts/write-formal-coverage-matrix.py` also writes
`build/bench/formal_coverage/cdc_structural_boundaries.csv` and
`paper/tables/cdc_structural_boundaries.tex`. The table records explicit CDC
adapter rows and direct CDC rejection rows for public-development, held-out,
and realism tasks. These rows are structural evidence only: they document the
compiler gate and smoke boundary, not a CDC correctness proof.

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

## Evidence Hashes

Current hashes for generated CSV/TeX evidence are recorded by
`build/release/deterministic_evidence_hashes.json` after
`.\scripts\full-check.ps1 -WithLatex` runs. Do not hard-code generated hash
values in committed documentation.

## Reproduction

Run after deterministic benchmark results exist:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/write-formal-coverage-matrix.py
```

The command writes:

- `build/bench/formal_coverage/formal_coverage_tasks.csv`
- `build/bench/formal_coverage/formal_coverage_matrix.csv`
- `build/bench/formal_coverage/cdc_structural_boundaries.csv`
- `build/paper_tables/formal/formal_coverage_matrix.tex`
- `build/paper_tables/formal/cdc_structural_boundaries.tex`

To refresh the committed paper table snapshot after a validated evidence run:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/write-formal-coverage-matrix.py --paper-table-dir paper/tables
```

Do not commit generated CSV files under `build/`.
