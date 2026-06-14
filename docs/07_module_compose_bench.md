# ModuleComposeBench

## Purpose

A benchmark for LLM-assisted composition of existing RTL/IP modules. It evaluates whether a model can produce a valid, verifiable interconnection graph rather than a standalone Verilog module.

## Task schema

```yaml
id: T001_stream_fifo
level: L1
description: Connect producer -> fifo -> consumer using StreamU32.
request: Connect Producer.tx through Fifo to Consumer.rx on Sys using StreamU32 ready-valid flow control.
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
| L3 | protocol/backpressure and latency adapter seeds |
| L4 | CDC/RDC with explicit adapter |
| L5 | bus bridge / register block seeds |
| L6 | multi-IP subsystem seeds |

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

## Current runner

The repository includes a deterministic runner for the current positive and
negative benchmark tasks:

```bash
./scripts/eda-docker.sh bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

The runner reads `benchmarks/module_compose_bench_manifest.yaml`, validates
required task metadata, runs `mico_cli check --format json`, emits
SystemVerilog/SVA/traceability artifacts for accepted positive tasks, and
executes Verilator, Icarus, and Yosys smoke checks against
`rtl/examples/mico_example_leafs.sv` and dedicated case-study collateral under
`rtl/case_studies/`. The current manifest has 60 tasks: 34 positive composition
tasks and 26 negative unsafe-rejection tasks across L1-L6.
Every task declares a natural-language request, module inventory, interface
inventory, adapter inventory, expected diagnostics, and RTL collateral.
Positive tasks with `sim_testbench` and `sim_top` use committed directed Icarus
testbenches. Accepted positive tasks without a declared testbench get an
auto-generated ready/valid smoke harness from the emitted traceability JSON;
the generated harness instantiates `Top`, toggles clock/reset ports, and checks
generated payload/valid/ready wires for unknown values with valid/ready asserted
after reset. Simulation stdout/stderr artifacts are written under ignored
`build/bench/`.
Positive tasks with `formal_harness` and `formal_top` generate a SymbiYosys job
under ignored `build/bench/` and run bounded proofs against the generated
wrapper plus committed harness monitor. Accepted single-clock positives without
a declared harness get an auto-generated ready/valid formal smoke harness from
traceability JSON. The current enabled formal denominator is 29/29: three
committed directed harnesses plus 26 generated single-clock smoke harnesses.
CDC remains smoke-only and is not reported as a proof.
Positive tasks with `qor_reference` also run Yosys structural `stat -json`
for the generated wrapper and the committed hand-written reference wrapper. The
current QoR scope is area-cell and wire-count delta; no timing or Vivado result
is claimed. The runner writes `qor_summary.csv` and `qor_summary.tex` under
ignored `build/bench/`.
Negative tasks are scored by expected compiler rejection and expected diagnostic
codes. It writes a `mico.bench.results.v0` JSON object under ignored
`build/bench/` with `summary` aggregation plus per-task results. The current
runner aggregates `formal_pass` over formal-enabled tasks and `qor` over
QoR-enabled positive tasks.
L3/L5/L6 still contain seed approximations, but T058--T060 add dedicated
streaming, width-bridge, and register/status subsystem RTL case studies.

## Paper Table Aggregation

Generate paper-ready aggregate artifacts from the deterministic benchmark JSON:

```bash
./scripts/eda-docker.sh bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

The aggregator writes `schema_version = mico.aggregate.results.v0` to
`build/bench/aggregate_results.json`, emits CSV tables under `build/bench/`,
and emits LaTeX snippets under `build/paper_tables/`. Deterministic outputs
cover the main result table, per-level breakdown, unsafe diagnostic taxonomy,
structural QoR rows, and conservative ablation/counterfactual rows. When
`--llm-result build/llm/<run>.json` is supplied, it also emits LLM baseline
summary, repair-turn distribution, token/cost, paired comparison, and failure
taxonomy tables. These generated artifacts remain ignored build outputs; paper
claims should cite the generating command and input JSON rather than hand-copying
numbers without provenance.
