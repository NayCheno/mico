# ModuleComposeBench

## Purpose

A benchmark for LLM-assisted composition of existing RTL/IP modules. It evaluates whether a model can produce a valid, verifiable interconnection graph rather than a standalone Verilog module.

## Task schema

```yaml
id: T001_stream_fifo
level: L1
type: positive
path: tasks/T001_stream_fifo
request: Connect Producer.tx through Fifo to Consumer.rx on Sys using StreamU32 ready-valid flow control.
module_inventory: [Producer, Fifo, Consumer]
interface_inventory: [StreamU32]
adapter_inventory: []
mico_source: rust_project/examples/stream_fifo.mico
rtl_collateral: rtl/examples/mico_example_leafs.sv
sim_testbench: benchmarks/sim/tb_stream_fifo.sv
sim_top: tb_stream_fifo
qor_reference: benchmarks/qor/reference/T001_stream_fifo_ref.sv
qor_top: Top
qor_reference_top: Top
expected_features: [direct_connect, fifo_chain, ready_valid]
expected:
  compose_pass: true
  lint_pass: true
  sim_pass: true
  qor_available: true
  diagnostics: []
```

The machine-checked manifest schema is
`benchmarks/manifest_schema.json`. It requires task IDs, level/type, public
request text, inventories, MICO source, RTL collateral, expected features, and
expected diagnostics. Optional simulation, formal, and QoR fields are paired
with their required top modules. `benchmarks/run_bench.py` additionally checks
that committed paths exist, task IDs are unique, every level has at least one
positive and one negative task, and negative tasks declare expected diagnostic
codes.

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

## Split And Leakage Policy

The committed `T001`--`T062` tasks are the public development split. They are
used for deterministic regression, documentation, prompt debugging, and
artifact reproduction. Full LLM advantage claims require a separately versioned
held-out manifest that is not used during prompt iteration; sanitized held-out
prompts, results, aggregate hashes, and manifest metadata should be archived as
release assets after scoring.

Prompt construction intentionally separates public requests from committed
solutions. The LLM batch runner includes the natural-language request,
inventories, and interface/module declarations, but strips the `compose` body
from committed expected MICO sources. Expected solutions, diagnostics,
testbenches, formal harnesses, and QoR references remain committed for
deterministic reproduction and are not inserted into benchmark prompts.

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
required task metadata and committed collateral paths, runs
`mico_cli check --format json`, emits
SystemVerilog/SVA/traceability artifacts for accepted positive tasks, and
executes Verilator, Icarus, and Yosys smoke checks against
`rtl/examples/mico_example_leafs.sv` and dedicated case-study collateral under
`rtl/case_studies/`. The current manifest has 62 tasks: 36 positive composition
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
traceability JSON. The current enabled formal denominator is 31/31: three
committed directed harnesses plus 28 generated single-clock smoke harnesses.
CDC remains smoke-only and is not reported as a proof.
Positive tasks with `qor_reference` also run Yosys structural `stat -json` and
flattened generic-mapped `stat -json` for the generated wrapper and the
committed hand-written reference wrapper. The current QoR scope is area-cell,
wire-count, and generic mapped-cell delta; no timing, technology-mapped delay,
or Vivado result is claimed. The runner writes `qor_summary.csv` and
`qor_summary.tex` under ignored `build/bench/`.
Negative tasks are scored by expected compiler rejection and expected diagnostic
codes. It writes a `mico.bench.results.v0` JSON object under ignored
`build/bench/` with `summary` aggregation plus per-task results. The current
runner aggregates `formal_pass` over formal-enabled tasks and `qor` over
QoR-enabled positive tasks.
L3/L5/L6 still contain seed approximations, but T058--T062 add dedicated
streaming, width-bridge, register/status, protocol-bridge, and telemetry
subsystem RTL case studies.

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
structural and generic-mapped QoR rows, and conservative ablation/counterfactual rows. When
`--llm-result build/llm/<run>.json` is supplied, it also emits LLM baseline
summary, repair-turn distribution, token/cost, paired comparison, and failure
taxonomy tables. These generated artifacts remain ignored build outputs; paper
claims should cite the generating command and input JSON rather than hand-copying
numbers without provenance.
