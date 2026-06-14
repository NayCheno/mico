# Project Plan

## Research objective

Build a Rust-based compiler framework for a module-centered hardware composition language that helps LLMs connect RTL modules safely.

## Scope

In scope:

- Existing leaf RTL/IP module composition.
- Interface schemas and role/direction checks.
- Clock/reset domain metadata.
- Adapter declarations and legality checks.
- SystemVerilog wrapper emission.
- SVA/contract skeleton emission.
- LLM structured-output and repair loop.
- Benchmarking of module interconnection tasks.

Out of scope for v0:

- Replacing all Verilog/SystemVerilog logic design.
- Full behavioral HDL semantics.
- Analog/mixed-signal, DFT, timing constraints, physical constraints.
- Full AXI/CHI formalization beyond pilot adapters.

## Immediate deliverables

1. Minimal grammar and parser.
2. Typed IR and semantic checker.
3. Conservative SystemVerilog emitter.
4. Structured diagnostics.
5. 60 ModuleComposeBench tasks before LLM baseline runs, including three dedicated subsystem case studies.
6. Baseline prompt scripts.

## Research hypothesis

MICO reduces LLM interconnection errors because it replaces primitive wire-level generation with typed interface graph generation.

## Key metrics

- Compose-Pass@1
- Repair-Turns
- Lint-Pass
- Sim-Pass
- Formal-Pass
- Adapter-Correct
- QoR-Delta
- Connection-Entropy-Reduction
