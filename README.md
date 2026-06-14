# MICO-HDL / MICO-Connect Research Package

**MICO** = **M**odule–**I**nterface–**C**ontract–**O**riented HDL.

This package is a Rust-based research prototype for LLM-assisted RTL module composition. The intended scope is deliberately narrow: use LLMs to propose module interface schemas, composition graphs, adapter plans, and contract skeletons; use a deterministic compiler to check direction, width, protocol, clock/reset domain, and contract obligations; then lower the checked design to SystemVerilog and, later, CIRCT HW/ESI/Verif/LTL.

## What is included

```text
paper/              IEEE-style LaTeX paper source, split sections, references, figures, and historical notes
docs/               Language spec, architecture, LLM workflow, evaluation plan, roadmap, risks
rust_project/       Rust workspace for parser/IR/checker/codegen/CLI
benchmarks/         ModuleComposeBench seed manifest, task set, and runner
prompts/            Prompt templates and structured-output schemas
source/             Original/edited input reports used as research context
```

## Core research claim

Direct Verilog generation asks an LLM to solve too many coupled problems in one textual target: local logic, module binding, port naming, protocol semantics, timing domains, resets, wrappers, and validation. MICO instead treats module composition as a typed graph synthesis problem. The LLM is a proposal engine; the compiler is the authority.

## Current next milestones

1. Add QoR parsing and report aggregation.
2. Expand ModuleComposeBench from the 12 seed tasks to a publishable 50+ task suite.
3. Add LLM baseline and compiler-feedback repair-loop runners.
4. Generate paper tables from benchmark artifacts and keep claims aligned with results.

## Status And Reproduction

MICO now has a working Rust parser/checker/typed-IR/codegen/CLI path, source-level JSON AST input/output, a parsed ready/valid v0 contract subset, seed RTL smoke collateral, golden SV/SVA/trace fixtures for positive seeds, per-task Icarus simulation harnesses for positive seeds, selected bounded SymbiYosys harnesses for direct and width seeds, a 12-task ModuleComposeBench runner, schema-versioned diagnostic/AST/IR/trace/LLM records, and an SDK-backed LLM provider smoke test. The paper is still a submission candidate in progress and does not yet claim full per-task formal coverage, QoR, arbitrary LTL, or multi-model pass-rate improvements.

For the current claim boundary, read `docs/current_status.md` and `docs/13_architecture_audit.md`.

For repeatable Rust and open-source RTL/EDA validation, use the persistent Ubuntu 24.04 Docker environment in `docker/eda/`. Vivado-specific flows use the Windows host Vivado installation; paper writing and PDF compilation use `paper/main.tex` with the Windows host LaTeX installation; other compilation and testing should run in Docker.

Start with the end-to-end workflow in `docs/14_reproduction_workflow.md`:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

## License

MIT for this scaffold. Papers and cited works retain their respective licenses.
