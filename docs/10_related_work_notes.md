# Related Work Notes

## Closest work

### CPPL

CPPL proposes a Python frontend DSL, JSON-based circuit IR, static checks, and lowering to CIRCT. It is the closest threat to novelty. MICO should not compete on "LLM-friendly frontend + CIRCT" alone. MICO should compete on module interconnection, protocol contracts, clock-domain safety, adapter planning, and a dedicated benchmark.

### Chisel

Chisel interfaces and bulk connections reduce wiring and support ready-valid interfaces such as Decoupled. MICO should acknowledge this and make the difference precise: MICO is not a Scala-embedded RTL generator; it is a Rust-compiled integration layer for existing RTL modules and LLM-generated composition graphs.

### Amaranth wiring

Amaranth provides signatures, In/Out directions, flipping, connect(), and component metadata. MICO should adopt similar interface discipline but add contracts, adapter legality, LLM diagnostics, and benchmark design.

### CIRCT HW/ESI/Verif/LTL

CIRCT provides the natural backend. MICO should reuse HW module semantics and ESI channels/buffers/FIFOs where possible.

### Anvil

Anvil supports timing safety through types and timing contracts. MICO can cite Anvil to justify moving timing/temporal constraints into language semantics.

### AutoSVA

AutoSVA demonstrates automatic formal verification of module interactions. MICO can adapt this idea by generating SVA from interface contracts.

## LLM-for-RTL context

- VerilogEval: simulation-based benchmark for Verilog generation.
- RTLLM/OpenLLM-RTL: benchmarks, datasets, syntax/functionality/design-quality goals.
- RTLCoder: open-source RTL generation model/data, privacy-oriented local deployment.
- ChipNeMo: domain-adapted LLMs for industrial chip design.
- VRank: self-consistency / candidate ranking for Verilog generation.

## Gap

The missing piece is a benchmarked, compiler-checked, LLM-oriented module composition layer that integrates interface signatures, clock domains, contracts, and adapters.
