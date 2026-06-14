# MICO-HDL / MICO-Connect Research Package

**MICO** = **M**odule–**I**nterface–**C**ontract–**O**riented HDL.

This package is a Rust-based research prototype for LLM-assisted RTL module composition. The intended scope is deliberately narrow: use LLMs to propose module interface schemas, composition graphs, adapter plans, and contract skeletons; use a deterministic compiler to check direction, width, protocol, clock/reset domain, and contract obligations; then lower the checked design to SystemVerilog and, later, CIRCT HW/ESI/Verif/LTL.

## What is included

```text
paper/              IEEE-style LaTeX paper source, split sections, references, figures, and historical notes
docs/               Language spec, architecture, LLM workflow, evaluation plan, roadmap, risks
rust_project/       Rust workspace for parser/IR/checker/codegen/CLI
benchmarks/         60-task ModuleComposeBench manifest, task set, runner, and result aggregator
prompts/            Prompt templates and structured-output schemas
source/             Original/edited input reports used as research context
```

## Core research claim

Direct Verilog generation asks an LLM to solve too many coupled problems in one textual target: local logic, module binding, port naming, protocol semantics, timing domains, resets, wrappers, and validation. MICO instead treats module composition as a typed graph synthesis problem. The LLM is a proposal engine; the compiler is the authority.

## Current next milestones

1. Run and archive the full paid low-cost LLM baseline matrix when cost settings are configured.
2. Broaden simulation, formal, and QoR coverage beyond the current enabled subsets.
3. Promote generated paper-table snippets into the final evidence-driven evaluation.

## Status And Reproduction

MICO now has a working Rust parser/checker/typed-IR/codegen/CLI path, source-level JSON AST input/output, a parsed ready/valid v0 contract subset, seed RTL smoke collateral, golden SV/SVA/trace fixtures for positive seeds, per-task Icarus simulation harnesses for supported positive and case-study tasks, selected bounded SymbiYosys harnesses for direct, width, and streaming case-study properties, structural Yosys QoR summaries for supported positive tasks, a 60-task ModuleComposeBench runner with required task metadata and three dedicated subsystem case studies, an aggregate-results generator for CSV/TeX paper tables, schema-versioned diagnostic/AST/IR/trace/LLM records, an SDK-backed LLM provider smoke test, and a batch LLM benchmark runner with five baselines plus JSON-AST repair-loop plumbing. The paper is still a submission candidate in progress and does not yet claim full per-task formal coverage, timing QoR, arbitrary LTL, or multi-model pass-rate improvements.

For the current claim boundary, read `docs/current_status.md` and `docs/13_architecture_audit.md`.

For repeatable Rust and open-source RTL/EDA validation, use the persistent Ubuntu 24.04 Docker environment in `docker/eda/`. Vivado-specific flows use the Windows host Vivado installation; paper writing and PDF compilation use `paper/main.tex` with the Windows host LaTeX installation; other compilation and testing should run in Docker.

Start with the end-to-end workflow in `docs/14_reproduction_workflow.md`:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

## License

MIT for this scaffold. Papers and cited works retain their respective licenses.
