# MICO-HDL / MICO-Connect Research Package

**MICO** = **M**odule–**I**nterface–**C**ontract–**O**riented HDL.

This package is a Rust-based research prototype for LLM-assisted RTL module composition. The intended scope is deliberately narrow: use LLMs to propose module interface schemas, composition graphs, adapter plans, and contract skeletons; use a deterministic compiler to check direction, width, protocol, clock/reset domain, and contract obligations; then lower the checked design to SystemVerilog and, later, CIRCT HW/ESI/Verif/LTL.

## What is included

```text
paper/              IEEE-style LaTeX paper source, split sections, references, figures, and historical notes
docs/               Language spec, architecture, LLM workflow, evaluation plan, roadmap, risks
rust_project/       Rust workspace for parser/IR/checker/codegen/CLI
benchmarks/         Expanded ModuleComposeBench manifests, task set, runner, and result aggregator
prompts/            Prompt templates and structured-output schemas
source/             Original/edited input reports used as research context
```

## Core research claim

Direct Verilog generation asks an LLM to solve too many coupled problems in one textual target: local logic, module binding, port naming, protocol semantics, timing domains, resets, wrappers, and validation. MICO instead treats module composition as a typed graph synthesis problem. The LLM is a proposal engine; the compiler is the authority.

## Current release posture

1. Treat DAC 2027 Research Track as the primary target and use
   `docs/dac2027_submission_plan.md` as the venue-specific control document.
2. Keep the current claim boundary frozen and evidence-backed; see
   `docs/claim_boundary.md` and `docs/submission_claim_freeze.md`.
3. Treat the authenticated v3 OpenCode Go matrix as a bounded tested-profile
   Branch A result for the public-development and held-out splits only.
4. Keep the supplemental realism split as deterministic-only evidence until it
   is rerun in an authenticated LLM matrix.
5. Use generated release manifests, paper tables, and hashes as the source of
   truth for submission and artifact claims.

## Status And Reproduction

MICO now has a working Rust parser/checker/typed-IR/codegen/CLI path, source-level JSON AST input/output, a parsed ready/valid v0 contract subset, seed RTL smoke collateral, golden SV/SVA/trace fixtures for selected sim/QoR-enabled positive seeds and case studies, Icarus simulation coverage for all 46 public-development positive tasks through committed directed harnesses, bounded SymbiYosys smoke coverage for all 40 public-development single-clock positive tasks through committed directed monitors, structural and generic-mapped Yosys QoR summaries for supported positive tasks, a 12-task QoR-enabled public/held-out Vivado out-of-context measurement subset, an 83-task public-development ModuleComposeBench runner with required task metadata plus 40-task held-out and 30-task realism manifests, an aggregate-results generator for CSV/TeX paper tables, schema-versioned diagnostic/AST/IR/trace/LLM records, an SDK-backed LLM provider smoke test, and a batch LLM benchmark runner with five baselines plus JSON-AST repair-loop plumbing. The authenticated v3 structured LLM matrix supports only the bounded tested-profile JSON-AST claim described in `docs/24_llm_matrix_v3.md` for its locked pre-expansion manifest hashes; it does not support arbitrary-model or broad free-form repair claims. The paper is still a submission candidate in progress and does not yet claim exhaustive formal coverage, broad timing closure, arbitrary LTL, or unbounded LLM improvements.

For the current claim boundary and numeric claim source of truth, read
`docs/claim_boundary.md`, `docs/release_claim_table.md`,
`docs/current_status.md`, `docs/13_architecture_audit.md`, and the DAC-specific
plan in `docs/dac2027_submission_plan.md`.

For repeatable Rust, Python, benchmark, LLM, paper-table, and open-source
RTL/EDA validation, use the persistent Ubuntu 24.04 Docker environment in
`docker/eda/`. The only host-tool exceptions are Vivado-specific flows through
the pinned Windows Vivado installation and final paper PDF compilation from
`paper/main.tex` with the Windows host LaTeX installation.

Start with the end-to-end workflow in `docs/14_reproduction_workflow.md`:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

Or run the component checks manually:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

## License

MIT for this scaffold. Papers and cited works retain their respective licenses.
