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

The repository includes a seed runner for the first three tasks:

```bash
./scripts/eda-docker.sh bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

The runner reads `benchmarks/module_compose_bench_manifest.yaml`, runs `mico_cli check`, emits SystemVerilog wrappers, and executes Verilator and Yosys smoke checks against `rtl/examples/mico_example_leafs.sv`. It writes JSON results under ignored `build/bench/`. The current runner records `sim_pass=false` and `formal_pass=false` because simulation and formal harnesses are not implemented yet.
