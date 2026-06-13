# Paper Outline: MICO

## Title
MICO: Contract-Guided Module Composition for LLM-Assisted RTL Integration

## Abstract claim
MICO reframes LLM-assisted RTL generation as compiler-checked module composition. LLMs emit structured interface graphs and adapter plans; MICO checks direction, width, protocol, clock/reset domain, and contracts; then emits SystemVerilog/SVA and CIRCT-ready IR.

## Contributions
1. A problem formulation for LLM-assisted RTL module composition.
2. A small language core: `interface`, `clockdom`, `contract`, `adapter`, `compose`.
3. A Rust compiler architecture with structured diagnostics for LLM repair.
4. ModuleComposeBench, a benchmark for existing RTL/IP module integration.
5. Evaluation against direct Verilog prompting, SV interface prompting, Chisel/Amaranth-style prompting, CPPL-style JSON IR, and MICO variants.

## Target venues
- DAC / ICCAD if framed as EDA + artifact + benchmark.
- ASPLOS if framed as hardware/software co-design infrastructure.
- PLDI/OOPSLA if formal language semantics and type system are strong.

## Differentiation from CPPL
CPPL is a compiler-mediated frontend for LLM circuit generation. MICO is a compiler-mediated frontend for LLM module integration, protocol contracts, clock-domain safety, and adapter synthesis.
