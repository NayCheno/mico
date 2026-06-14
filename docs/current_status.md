# MICO Current Status

Snapshot date: 2026-06-14.

This file is the short, traceable status page for the current repository. Use
`docs/13_architecture_audit.md` for the detailed audit and
`docs/14_reproduction_workflow.md` for reproduction commands.

## Implemented

- Rust workspace with `mico_ir`, `mico_frontend`, `mico_codegen`, and
  `mico_cli`.
- Hand-written lexer/parser with source spans, parser diagnostics, comments,
  multiline contracts, and basic recovery.
- Semantic checker for duplicates, missing references, direction mismatch,
  direct interface/domain mismatch, adapter endpoint mismatch, adapter kind
  legality, ready/valid width rules, and contract-preservation attributes.
- Typed IR JSON with `schema_version = mico.ir.v0`.
- CLI commands for parse/check/build/dump-ir/emit-sv/emit-sva/emit-trace/verify/report.
- Diagnostics JSON envelope with `schema_version = mico.diagnostics.v0`,
  semantic labels, affected graph nodes, repair hints, and `repair_action`.
- Conservative SystemVerilog wrapper emission, SVA skeleton emission, and
  traceability JSON emission.
- Docker EDA smoke flow using Verilator, Icarus, Yosys, and a minimal
  SymbiYosys proof smoke.
- Seven ModuleComposeBench seed tasks with four positives and three negatives.
- Repository-owned LLM provider validate/smoke script that writes sanitized
  `mico.llm.run.v0` records.
- IEEE-style paper draft with conservative seven-seed-task claims.

## Not Yet Implemented

- Checker diagnostics with concrete source spans for semantic errors; current
  semantic diagnostics carry graph references and `span: null`.
- Schema-validated MICO JSON AST input and repair patch ingestion.
- Parsed contract AST and formalized ready/valid contract compatibility.
- Golden SV/SVA/traceability fixture tests.
- Per-task simulation harnesses.
- Per-task formal harnesses beyond the minimal smoke proof.
- QoR parser and report aggregation.
- 50+ task ModuleComposeBench suite.
- LLM batch runner, baselines, compiler-feedback repair loop, caching, and
  failure taxonomy.
- Paper tables generated from committed benchmark artifacts.
- Reproducible subsystem case studies.
- Full release-candidate validation script.

## Current Seed Benchmark Boundary

Current deterministic seed scope:

- Positive tasks: `T001_stream_fifo`, `T002_cdc_fifo`,
  `T003_width_adapter`, `T004_direct_stream`.
- Negative tasks: `T005_invalid_width_no_adapter`,
  `T006_direct_cdc_without_adapter`, `T007_reversed_direction`.
- Expected current result: expected outcome 7/7, positive compose 4/4,
  positive lint/elaboration smoke 4/4, unsafe rejection 3/3.
- `sim_pass`, `formal_pass`, and `qor.available` are intentionally not claimed
  until the corresponding harnesses and parsers are implemented.

## Validation Commands

Run Rust, Python, benchmark, and open-source EDA validation inside Docker:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
```

Host exceptions:

- Vivado-only flows must use `D:\Application\vivado\2025.2\Vivado` through
  `scripts/run-vivado-host.ps1`.
- Paper PDF builds use Windows-host LaTeX with `paper/main.tex`.

Never commit `build/`, `rust_project/target/`, logs, PDFs, Vivado project
outputs, local YAML configs, or API keys.
