# CCF-A Positioning Notes

## Core novelty boundary

Do **not** claim: "a new HDL that lets LLMs write hardware".

Claim instead:

> A compiler-checked module-composition language that turns LLM-produced RTL integration from free-form text generation into typed graph generation with protocol, clock-domain, and contract checks.

## Reviewer objections and answers

### Objection 1: SystemVerilog interface already exists.

Answer: SV interface reduces port boilerplate but does not provide an LLM-oriented AST repair loop, contract-guided adapter planning, explicit CDC rejection, or a module-composition benchmark.

### Objection 2: Chisel/Amaranth already have bundles/signatures/connect.

Answer: They are general-purpose HDL DSLs. MICO is a narrower module-integration layer designed around existing RTL/IP, compiler diagnostics, LLM structured output, adapter synthesis, and verification collateral.

### Objection 3: CPPL already proposes LLM-friendly frontend + CIRCT.

Answer: CPPL focuses on general circuit generation via Python DSL/JSON IR. MICO focuses on module composition, interface protocol contracts, clock/reset domain compatibility, CDC/adapters, and benchmarking of top-level glue tasks.

### Objection 4: LLMs are unreliable.

Answer: MICO explicitly does not trust LLMs. LLMs propose; compiler, simulation, formal tools, and synthesis dispose.

## Artifact requirements

To be competitive, the artifact should include:

- Open-source Rust compiler.
- Parser, checker, codegen, diagnostics.
- At least 50 benchmark tasks.
- Generated SV/SVA outputs.
- Reproducible scripts for lint/sim/formal/synthesis.
- Baseline prompts and outputs.
- Error taxonomy.
