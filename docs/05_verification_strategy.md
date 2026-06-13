# Verification Strategy

## Verification artifacts

MICO should generate:

- SystemVerilog wrapper/top;
- SVA assertion modules for each interface contract;
- cocotb smoke tests for each compose graph;
- formal harnesses for selected adapters;
- traceability report mapping MICO endpoints to RTL wires.

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

The script generates wrappers for `stream_fifo`, `cdc_fifo`, and `width_adapter` into ignored `build/eda-smoke/`, then runs Verilator lint and Yosys hierarchy/proc/opt/stat against `rtl/examples/mico_example_leafs.sv`. The CDC FIFO in that file is a smoke-only stub, not a CDC correctness proof. Real CDC signoff still requires a proven FIFO implementation, assertions, and CDC/formal collateral.
