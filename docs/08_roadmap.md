# Roadmap

## Phase 0: Research hardening

- Finalize novelty against CPPL, Chisel, Amaranth, CIRCT/ESI, Anvil, AutoSVA.
- Freeze MICO v0 grammar.
- Define ModuleComposeBench schema.
- Seed ModuleComposeBench with deterministic tasks.

## Phase 1: Rust compiler MVP

- Implement parser and AST.
- Implement name resolution.
- Implement interface and direction checks.
- Implement same-domain direct connect checks.
- Emit basic SystemVerilog wrapper.
- Emit JSON design graph.

## Phase 2: Contracts and diagnostics

- Add contract syntax.
- Emit basic SVA templates.
- Add machine-readable diagnostics.
- Add repair prompt generation.
- Add traceability reports.

## Phase 3: Adapter library

- Add structural adapters.
- Add width adapters.
- Add skid/pipeline adapters.
- Add CDC FIFO declarations.
- Implement explicit `adapt` syntax.

## Phase 4: LLM closed loop

- Implement JSON AST ingestion.
- Maintain batch-runner compiler-feedback repair-loop plumbing.
- Add prompt templates and result logging.
- Run and archive full paid multi-model benchmark matrices.

## Phase 5: Benchmark and paper

- Maintain the 60-task ModuleComposeBench manifest and expand dedicated L3/L5/L6 case-study RTL.
- Run baselines.
- Report pass rates, repair turns, PPA, formal results.
- Prepare open-source artifact.
- Write paper.

## Phase 6: CIRCT integration

- Lower MICO IR to CIRCT HW.
- Represent streams as ESI channels.
- Lower contracts to Verif/LTL or SVA.
- Compare native SV emitter vs CIRCT emitter.
