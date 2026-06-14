# MICO Current Status

Snapshot date: 2026-06-14.

This file is the short, traceable status page for the current repository. Use
`docs/claim_boundary.md` for the authoritative claim boundary,
`docs/13_architecture_audit.md` for the detailed audit, and
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
  `verify` defaults to compiler and typed-IR checks; `verify --eda` emits a
  wrapper plus SVA skeleton into an ignored artifact directory and runs
  Verilator wrapper lint, Verilator SVA lint, Icarus elaboration, and Yosys
  hierarchy/proc/opt/stat checks in the Docker EDA environment.
- Diagnostics JSON envelope with `schema_version = mico.diagnostics.v0`,
  semantic labels, affected graph nodes, repair hints, and `repair_action`.
- Semantic diagnostics from `.mico` input attach parser source-map spans for
  declarations, endpoints, fields, ports, adapters, and compose members where
  available; JSON AST input falls back to graph-node references.
- Source-level JSON AST input/output with `schema_version = mico.ast.v0`,
  plus JSON AST CLI commands for check/build/emit.
- Parsed ready/valid contract subset for `stable`, `fire`, boolean operators,
  implication, and `until`; known adapter guarantees are checked against
  sink-side contract requirements and adapter kind boundaries.
- Conservative SystemVerilog wrapper emission, SVA skeleton emission, and
  traceability JSON emission with source references, generated signal mappings,
  leaf module port names, adapter boundaries, and contract IDs.
- Golden SV/SVA/traceability fixture tests for the seven sim/QoR-enabled
  positive seed and case-study tasks.
- Docker EDA smoke flow using Verilator, Icarus, Yosys, and a minimal
  SymbiYosys proof smoke.
- Icarus simulation coverage for all 34 positive tasks: seven tasks use
  committed directed testbenches, and the remaining accepted positives use
  auto-generated ready/valid smoke harnesses derived from traceability JSON.
- Selected bounded SymbiYosys formal harnesses for the direct stream, width
  adapter, and streaming accelerator case-study tasks.
- Structural Yosys QoR extraction for supported positive benchmark wrappers,
  compared against committed hand-written reference wrappers with generated
  CSV/TeX summaries.
- Aggregate benchmark result generator that merges deterministic results and
  optional LLM batch records into CSV plus LaTeX table snippets for main
  results, per-level metrics, unsafe diagnostics, QoR, ablations, repair turns,
  token/cost, paired comparisons, and failure taxonomy.
- JSON Schema validation gate for diagnostics, source AST, typed IR,
  traceability, repair patches, deterministic benchmark results, LLM run
  records, LLM benchmark records, and aggregate results.
- 60 ModuleComposeBench tasks with required natural-language requests,
  module/interface/adapter inventories, expected diagnostics, explicit RTL
  collateral, and three dedicated subsystem case studies.
- Repository-owned LLM provider validate/smoke script that writes sanitized
  `mico.llm.run.v0` records.
- Batch LLM benchmark runner for the 60-task manifest with Direct Verilog,
  SystemVerilog-interface, MICO source, MICO JSON AST, and MICO JSON AST +
  compiler-feedback repair baselines. It supports validate-only planning,
  offline fixture checks, authenticated OpenAI-compatible execution, response
  caching, compiler/EDA scoring, and sanitized `mico.llm.bench.v0` records.
- Repository-owned JSON AST repair patch applicator in the compiler/CLI. The
  `repair-json` command supports dry-run, apply, and immediate re-check using
  `schemas/mico_repair_patch.schema.json`; the LLM batch runner delegates patch
  application to this CLI path.
- Full release-candidate validation wrappers in `scripts/full-check.sh` and
  `scripts/full-check.ps1`, plus a top-level release checklist and generated
  `build/release/full_check_manifest.json` metadata record.
- IEEE-style paper draft with conservative deterministic benchmark claims.

## Not Yet Implemented

- Broader semantic repair policies beyond schema-valid JSON AST operations.
- Arbitrary LTL or temporal contract proving beyond the v0 ready/valid subset.
- Formal harnesses beyond the selected direct stream, width adapter, and
  streaming case-study tasks.
- CDC correctness proof for the smoke FIFO collateral.
- Timing/Vivado QoR and technology-mapped delay reporting.
- Committed full paid multi-profile LLM baseline result artifacts and pass-rate
  claims.
- Direct inclusion of generated paper-table snippets in the final submission
  text.
- Broader subsystem case studies beyond the three committed deterministic cases.
- Immutable release tag; the current policy uses a reviewable release branch
  first, then tags only after final artifact approval.

## Current ModuleComposeBench Boundary

Current deterministic benchmark scope:

- Total tasks: 60, with 34 positives and 26 negatives.
- Level coverage: L1 10, L2 13, L3 10, L4 10, L5 9, and L6 8.
- The deterministic compiler baseline includes same-domain stream wiring,
  width adaptation, latency/backpressure seed tasks, CDC/RDC adapter tasks,
  bus/register wrapper seeds, subsystem seeds, and 26 unsafe-rejection cases.
- Expected current result: expected outcome 60/60, positive compose 34/34,
  positive lint/elaboration smoke 34/34, positive simulation 34/34, unsafe
  rejection 26/26, JSON AST path 60/60, selected bounded formal 3/3, structural
  QoR available 7/7.
- `formal_pass` is claimed only for formal-enabled direct stream, width adapter,
  and streaming case-study tasks; CDC proof and timing QoR remain intentionally
  unclaimed.
- L3/L5/L6 include seed approximations plus the dedicated T058--T060 streaming,
  width-bridge, and register/status subsystem case studies.

## Validation Commands

Run Rust, Python, benchmark, and open-source EDA validation inside Docker:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

Component commands:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- verify --eda --json --artifact-dir ../build/mico-verify/stream_fifo_cli --schema-path ../schemas examples/stream_fifo.mico | python3 -m json.tool >/dev/null"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-result build/bench/seed_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_validate.json --aggregate-result build/bench/aggregate_results.json"
```

Host exceptions:

- Vivado-only flows must use `D:\Application\vivado\2025.2\Vivado` through
  `scripts/run-vivado-host.ps1`.
- Final paper PDF builds use Windows-host LaTeX with `paper/main.tex`.
  Paper-related Python/statistical validation and generated table production
  still run in Docker.

Never commit `build/`, `rust_project/target/`, logs, PDFs, Vivado project
outputs, local YAML configs, or API keys.
