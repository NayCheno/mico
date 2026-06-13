# MICO-HDL / MICO-Connect Research Package

**MICO** = **M**odule–**I**nterface–**C**ontract–**O**riented HDL.

This package is a research starter kit for a Rust-based language and compiler framework that targets LLM-assisted RTL module composition. The intended scope is deliberately narrow: use LLMs to propose module interface schemas, composition graphs, adapter plans, and contract skeletons; use a deterministic compiler to check direction, width, protocol, clock/reset domain, and contract obligations; then lower the checked design to SystemVerilog and, later, CIRCT HW/ESI/Verif/LTL.

## What is included

```text
paper/              IEEE-style LaTeX paper source, split sections, references, figures, and historical notes
docs/               Language spec, architecture, LLM workflow, evaluation plan, roadmap, risks
rust_project/       Rust workspace skeleton for parser/IR/checker/codegen/CLI
benchmarks/         ModuleComposeBench seed manifest and scoring schema
prompts/            Prompt templates and structured-output schemas
source/             Original/edited input reports used as research context
```

## Core research claim

Direct Verilog generation asks an LLM to solve too many coupled problems in one textual target: local logic, module binding, port naming, protocol semantics, timing domains, resets, wrappers, and validation. MICO instead treats module composition as a typed graph synthesis problem. The LLM is a proposal engine; the compiler is the authority.

## Recommended first milestone

1. Implement the Rust frontend for the minimal grammar in `docs/01_language_spec_v0.md`.
2. Add semantic checks for role compatibility, interface identity, width equality, and clock-domain equality.
3. Emit conservative SystemVerilog wrappers for existing leaf RTL modules.
4. Build `ModuleComposeBench` tasks from existing open-source RTL modules.
5. Compare direct Verilog prompting vs. MICO AST prompting vs. MICO AST + compiler-feedback repair.

## Status

This is a scaffold, not a finished compiler. The Rust code is intentionally small and dependency-light so the project can be extended in a research repository. The generation environment used to produce this package did not include a local Rust toolchain, so run the following commands on a machine with Rust installed:

```bash
cd rust_project
cargo fmt
cargo check
cargo test
cargo run -p mico_cli -- check examples/stream_fifo.mico
cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico
```

For repeatable Rust and open-source RTL/EDA validation, use the persistent Ubuntu 24.04 Docker environment in `docker/eda/`. See `docs/12_docker_eda_environment.md`. Vivado-specific flows use the Windows host Vivado installation; paper writing and PDF compilation use `paper/main.tex` with the Windows host LaTeX installation; other compilation and testing should run in Docker.

## License

MIT for this scaffold. Papers and cited works retain their respective licenses.
