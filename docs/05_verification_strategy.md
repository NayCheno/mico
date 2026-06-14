# Verification Strategy

## Verification artifacts

MICO should generate:

- SystemVerilog wrapper/top;
- SVA assertion modules for each interface contract;
- SystemVerilog or cocotb smoke tests for each compose graph;
- formal harnesses for selected adapters;
- traceability report mapping MICO endpoints to RTL wires.

The current compiler emits the first traceability contract through:

```bash
cargo run -p mico_cli -- emit trace examples/stream_fifo.mico
```

The report schema is `schemas/traceability.schema.json`. It maps each checked
MICO connection to a stable compose-connection source reference, generated SV
field wires, leaf module port names, adapter boundary signals, and the
ready/valid stable-payload assertion skeletons emitted by `emit-sva`. Assertion
records include a `contract_id` such as `StreamU32.stable_payload` so generated
properties can be traced back to the interface contract subset.

## Interface contract examples

### Ready/valid safety

```systemverilog
property stable_payload_when_waiting;
  @(posedge clk) disable iff (rst)
    valid && !ready |=> $stable(payload);
endproperty
```

### Ready/valid liveness template

```systemverilog
property eventual_fire;
  @(posedge clk) disable iff (rst)
    valid |-> ##[0:$] ready;
endproperty
```

Liveness may require fairness assumptions and should be configurable.

## Adapter proof targets

| Adapter | Core property |
|---|---|
| SkidBuffer | no drop, no duplicate, ordered delivery |
| Pipeline | latency increases by N, payload preserved |
| AsyncFifo | CDC-safe blackbox assumptions + ordering |
| WidthPack | output sequence is equivalent to packed input sequence |
| Bridge | protocol-level refinement |

## Tool flow

```text
MICO-generated SV/SVA
  -> Verilator lint/sim
  -> cocotb tests
  -> Yosys/SymbiYosys formal for bounded properties
  -> optional commercial CDC/formal tools for industrial case studies
```

## Current Docker smoke flow

The repository includes a minimal open-source EDA smoke flow:

```bash
./scripts/eda-docker.sh bash -lc "bash scripts/eda-smoke.sh"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
```

The script generates wrappers and SVA skeletons for `stream_fifo`, `cdc_fifo`, and `width_adapter` into ignored `build/eda-smoke/`, then runs Verilator lint, Icarus elaboration, and Yosys hierarchy/proc/opt/stat against `rtl/examples/mico_example_leafs.sv`. It also runs a minimal SymbiYosys smoke proof to verify that the Docker formal entry point works. The CDC FIFO in that file is a smoke-only stub, not a CDC correctness proof. Real CDC signoff still requires a proven FIFO implementation, assertions, and CDC/formal collateral.

ModuleComposeBench runs Icarus/VVP simulation for every accepted positive task.
Nine tasks use committed directed testbenches; the remaining positives use a
generated ready/valid smoke harness derived from traceability JSON. The
generated harness is a dynamic wiring/protocol sanity check, not a substitute
for task-specific functional scoreboards.

ModuleComposeBench additionally runs bounded SymbiYosys checks for the
single-clock formal smoke denominator. `T004_direct_stream`,
`T003_width_adapter`, and `T058_streaming_accelerator_case` use committed
directed monitors for direct stream, width-adapter, and streaming case-study
properties. The remaining single-clock positives use generated ready/valid
formal smoke harnesses derived from traceability JSON. These generated harnesses
check no-unknowns, asserted ready/valid after reset, and bounded stable-payload
properties when the compiler emitted matching ready/valid SVA metadata. CDC
tasks are still simulation/lint smoke only and must not be treated as CDC proof.
