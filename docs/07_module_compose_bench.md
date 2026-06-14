# ModuleComposeBench

## Purpose

A benchmark for LLM-assisted composition of existing RTL/IP modules. It evaluates whether a model can produce a valid, verifiable interconnection graph rather than a standalone Verilog module.

## Task schema

```yaml
id: T001_stream_fifo
level: L1
description: Connect producer -> fifo -> consumer using StreamU32.
inputs:
  modules:
    - Producer
    - Fifo
    - Consumer
  interface_library:
    - StreamU32
requirements:
  - p.tx connects to f.input
  - f.output connects to c.rx
  - all endpoints are in Sys clock domain
expected:
  compose_pass: true
  lint_pass: true
  sim_pass: true
  formal_pass: false
  qor_available: true
```

## Levels

| Level | Description |
|---|---|
| L1 | same-domain direct interface connections |
| L2 | parameter and width adaptation |
| L3 | protocol/backpressure adaptation |
| L4 | CDC/RDC with explicit adapter |
| L5 | bus bridge / register block |
| L6 | multi-IP subsystem |

## Scoring

A task is solved only if:

1. MICO checker accepts the design;
2. generated wrapper/top passes lint;
3. provided tests pass;
4. required properties pass or are correctly emitted;
5. no forbidden implicit CDC/protocol conversion occurs.

## Negative tasks

Include intentionally invalid tasks to measure rejection ability:

- cross-domain direct connect;
- reversed ready/valid direction;
- width mismatch without adapter;
- missing reset synchronizer;
- ambiguous shorthand connection;
- unsafe truncation.

## Current seed runner

The repository includes a seed runner for the current positive and negative smoke tasks:

```bash
./scripts/eda-docker.sh bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

The runner reads `benchmarks/module_compose_bench_manifest.yaml`, runs
`mico_cli check --format json`, emits SystemVerilog/SVA/traceability artifacts
for accepted positive tasks, and executes Verilator, Icarus, and Yosys smoke
checks against `rtl/examples/mico_example_leafs.sv`. Positive seed tasks with
`sim_testbench` and `sim_top` also compile with Icarus and execute with `vvp`;
simulation stdout/stderr artifacts are written under ignored `build/bench/`.
Positive seed tasks with `formal_harness` and `formal_top` also generate a
SymbiYosys job under ignored `build/bench/` and run bounded proofs against the
generated wrapper plus committed harness monitor. The current enabled formal
subset is `T003_width_adapter` and `T004_direct_stream`; CDC remains smoke-only
and is not reported as a proof.
Positive seed tasks with `qor_reference` also run Yosys structural `stat -json`
for the generated wrapper and the committed hand-written reference wrapper. The
current QoR scope is area-cell and wire-count delta; no timing or Vivado result
is claimed. The runner writes `qor_summary.csv` and `qor_summary.tex` under
ignored `build/bench/`.
Negative tasks are scored by expected compiler rejection and expected diagnostic
codes. It writes a `mico.bench.results.v0` JSON object under ignored
`build/bench/` with `summary` aggregation plus per-task results. The current
runner aggregates `formal_pass` over formal-enabled tasks and `qor` over
QoR-enabled positive tasks.
