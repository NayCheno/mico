# RTL Methodology

This reference distills mature open RTL practices from lowRISC/OpenTitan-style methodology, Yosys/OpenROAD flow habits, and production ASIC/FPGA project structure. Use it as a checklist, not as a heavy process mandate.

Source anchors:

- lowRISC style guides: https://github.com/lowRISC/style-guides
- OpenTitan hardware methodology: https://opentitan.org/book/doc/contributing/hw/methodology.html
- OpenTitan hardware development stages: https://opentitan.org/book/doc/project_governance/development_stages.html
- OpenTitan RTL linting: https://opentitan.org/book/hw/lint/index.html
- OpenROAD Flow Scripts: https://github.com/The-OpenROAD-Project/OpenROAD-flow-scripts
- Yosys documentation: https://yosyshq.readthedocs.io/

## Project Layout

Prefer an explicit layout. Adapt names to the existing repository rather than forcing a new tree.

```text
hw/
  ip/<block>/
    rtl/             synthesizable SystemVerilog/Verilog
    dv/              UVM/lightweight testbench, agents, scoreboards
    formal/          SVA harnesses, cover/prove scripts
    data/            generated register packages, memory maps
    doc/             block spec and integration notes
    lint/            lint waivers and reports
    syn/             synthesis scripts and constraints
    fpga/            board-specific wrappers and constraints
    filelists/       ordered .f/.tcl filelists
```

For small repositories, collapse this to:

```text
rtl/
tb/
formal/
constraints/
scripts/
reports/
```

Keep generated outputs out of source directories.

## Design Lifecycle

Use lightweight stage gates:

1. Specification: interface, registers, clocks/resets, CDC assumptions, protocol timing, error behavior.
2. RTL draft: compiles, lint is mostly clean, reset behavior is intentional.
3. Verified RTL: directed and randomized tests, assertions, code/functional coverage where available.
4. Integration-ready: synthesis elaborates, timing constraints exist, CDC/RDC reviewed, known waivers documented.
5. Signed-off for the repo scope: regressions pass, artifacts are reproducible, limitations are documented.

## RTL Structure

- Use one primary module per file when practical.
- Put shared types and parameters in packages.
- Keep module ports typed and grouped by function: clocks/resets, bus/interfaces, control/status, data.
- Prefer explicit ready/valid, request/acknowledge, or credit protocols over ad hoc enable wires.
- Use `logic` for SystemVerilog signals; keep `wire/reg` only for legacy compatibility.
- Use `always_ff` for flops and `always_comb` for combinational logic when supported.
- Assign defaults at the top of combinational blocks to avoid latches.
- Encode FSMs with enums and separate next-state logic when it improves reviewability.
- Avoid implicit nets; require `default_nettype none` when the tool flow supports it.

## Clock And Reset

- Name clocks and resets consistently. Common convention: `clk_i` and active-low `rst_ni`, or project-local equivalents.
- Document reset polarity, synchronization, and release behavior.
- Reset control state; avoid resetting large datapath flops unless required by architecture or safety.
- Keep derived clocks rare; prefer clock enables.
- Treat multiple clocks as an explicit architecture feature requiring CDC review.

## CDC And RDC

Crossings must be represented as named structures:

- single-bit control: two-flop synchronizer or qualified pulse synchronizer
- multi-bit data: async FIFO, handshake, gray-coded pointer, or stable bus with valid/ack protocol
- reset domain crossing: reset synchronizer or documented sequence
- ready/valid stream crossing: async FIFO or a proven CDC adapter

Do not connect different domains directly and rely on synthesis or timing exceptions to make it safe.

## Quality Gates

Minimum gates for serious RTL:

- format/style check
- lint
- compile/elaboration
- unit simulation
- assertion/formal checks for protocols and FIFOs
- synthesis elaboration
- clock/reset/CDC review
- constraints sanity
- timing report generation

Waivers must include owner, reason, scope, and expiration or review trigger.

## MICO Integration

For this repository, map MICO concepts to RTL practice:

- `clockdom`: clock/reset contract and constraints seed
- `interface`: protocol bundle with producer/consumer roles
- `extern module`: leaf RTL/IP integration point
- `adapter`: explicit CDC/width/protocol bridge
- `compose`: top-level wrapper and filelist generation target
- `contract`: SVA property skeleton or formal assumption/assertion

When MICO emits SystemVerilog, expect downstream Tcl to consume:

- ordered RTL filelist
- top module name
- generated wrapper path
- optional SVA bind files
- clock/reset constraints
- report directory

## Review Checklist

- Is every clock/reset named and explained?
- Are all crossings explicit?
- Are widths parameterized or checked?
- Are protocol events named, such as `fire = valid && ready`?
- Can the block compile from a clean checkout with one command?
- Are constraints and generated reports reproducible?
- Are waivers narrower than the code they waive?
