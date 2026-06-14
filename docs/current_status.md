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
  legality, ready/valid width rules, and v0 contract guarantee coverage.
- Typed IR JSON with `schema_version = mico.ir.v0`.
- CLI commands for parse/check/build/dump-ir/emit-sv/emit-sva/emit-trace/verify/report.
- Diagnostics JSON envelope with `schema_version = mico.diagnostics.v0`,
  semantic labels, affected graph nodes, repair hints, and `repair_action`.
- Source-level JSON AST input/output with `schema_version = mico.ast.v0`,
  plus JSON AST CLI commands for check/build/emit.
- Parsed ready/valid contract subset for `stable`, `fire`, boolean operators,
  implication, and `until`; known adapter guarantees are checked against
  sink-side contract requirements and adapter kind boundaries.
- Conservative SystemVerilog wrapper emission, SVA skeleton emission, and
  traceability JSON emission with source references, generated signal mappings,
  leaf module port names, adapter boundaries, and contract IDs.
- Golden SV/SVA/traceability fixture tests for the four positive seed tasks.
- Docker EDA smoke flow using Verilator, Icarus, Yosys, and a minimal
  SymbiYosys proof smoke.
- Per-task Icarus simulation harnesses for the four positive seed tasks.
- Selected bounded SymbiYosys formal harnesses for the direct stream and width
  adapter seed tasks.
- Twelve ModuleComposeBench seed tasks with four positives and eight negatives.
- Repository-owned LLM provider validate/smoke script that writes sanitized
  `mico.llm.run.v0` records.
- IEEE-style paper draft with conservative twelve-seed-task claims.

## Not Yet Implemented

- Checker diagnostics with concrete source spans for semantic errors; current
  semantic diagnostics carry graph references and `span: null`.
- Full repair patch ingestion and application; the patch schema exists, but
  the compiler does not yet apply patches.
- Arbitrary LTL or temporal contract proving beyond the v0 ready/valid subset.
- Formal harnesses beyond the selected direct stream and width adapter seeds.
- CDC correctness proof for the smoke FIFO collateral.
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
  `T006_direct_cdc_without_adapter`, `T007_reversed_direction`,
  `T008_width_missing_contract`, `T009_width_unknown_contract`,
  `T010_width_wrong_contract_kind`, `T011_cdc_missing_contract`,
  `T012_cdc_wrong_contract_kind`.
- Expected current result: expected outcome 12/12, positive compose 4/4,
  positive lint/elaboration smoke 4/4, positive simulation 4/4, unsafe
  rejection 8/8, JSON AST path 12/12, selected bounded formal 2/2.
- `formal_pass` is claimed only for formal-enabled direct stream and width
  adapter seeds; CDC proof and `qor.available` remain intentionally unclaimed.

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
