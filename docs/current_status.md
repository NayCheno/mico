# MICO Current Status

Snapshot date: 2026-06-15.

This file is the short, traceable status page for the current repository. Use
`docs/claim_boundary.md` for the authoritative claim boundary,
`docs/release_claim_table.md` for numeric claim values and evidence mapping,
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
- Golden SV/SVA/traceability fixture tests for selected sim/QoR-enabled
  positive seed and case-study tasks.
- Docker EDA smoke flow using Verilator, Icarus, Yosys, and a minimal
  SymbiYosys proof smoke.
- Icarus simulation coverage for all 36 positive tasks: 32 tasks use committed
  directed testbenches, and the remaining accepted positives use auto-generated
  ready/valid smoke harnesses derived from traceability JSON.
- Bounded SymbiYosys formal smoke coverage for 31 single-clock positive tasks:
  24 tasks use committed directed monitors, and the remaining single-clock
  positives use generated ready/valid formal harnesses derived from
  traceability JSON.
- Structural and generic-mapped Yosys QoR extraction for supported positive
  benchmark wrappers, compared against committed hand-written reference wrappers
  with generated CSV/TeX summaries.
- Representative host-Vivado out-of-context QoR/timing subset for nine tasks
  (`T001`, `T003`, and `T058`--`T064`) through
  `scripts/vivado-qor-subset.tcl`. The flow uses build-only measurement copies,
  targets `xc7a35tcpg236-1`, and writes JSON/CSV summaries under ignored
  `build/reports/vivado-host/`.
- Aggregate benchmark result generator that merges deterministic results and
  optional LLM batch records into CSV plus LaTeX table snippets for main
  results, per-level metrics, unsafe diagnostics, QoR, ablations, repair turns,
  token/cost, paired comparisons, and failure taxonomy.
- JSON Schema validation gate for the ModuleComposeBench manifest,
  diagnostics, source AST, typed IR, traceability, repair patches,
  deterministic benchmark results, LLM run
  records, LLM benchmark records, and aggregate results.
- 62 public-development ModuleComposeBench tasks with required natural-language requests,
  module/interface/adapter inventories, expected diagnostics, explicit RTL
  collateral, five public-development subsystem case studies, and a hardened
  20-task held-out case-study split.
- Documented public-development and held-out split policy. The committed main
  manifest is the public development split; the separate held-out manifest has
  20 scoring tasks, including seven subsystem positives and seven paired
  negative variants. Prompt construction strips expected compose bodies from
  MICO sources to avoid solution leakage.
- Deterministic and LLM benchmark result records include the evaluated manifest
  path and SHA-256 hash.
- Repository-owned LLM provider validate/smoke script that writes sanitized
  `mico.llm.run.v0` records.
- Batch LLM benchmark runner for the 62-task manifest with Direct Verilog,
  SystemVerilog-interface, MICO source, MICO JSON AST, and MICO JSON AST +
  compiler-feedback repair baselines. It supports validate-only planning,
  offline fixture checks, authenticated OpenAI-compatible execution, response
  caching, compiler/EDA scoring, and sanitized `mico.llm.bench.v0` records.
- Authenticated low-cost LLM matrix execution has been run for 62 tasks, two
  low-cost profiles, and five baselines. The sanitized summary in
  `docs/16_llm_matrix_results.md` is retained as a historical negative result
  for the original prompts.
- A follow-up prompt/model pilot in
  `docs/17_llm_prompt_redesign_pilot.md` adds JSON response mode, schema-valid
  JSON AST declaration skeletons with stripped compose bodies, larger structured
  output budgets, compact repair diagnostics, and stronger profile validation.
  The selected pilot subset is no longer zero-pass, but it is not a full
  pass-rate improvement claim.
- A full authenticated structured matrix rerun in
  `docs/24_llm_matrix_v3.md` covers public-development and held-out splits
  across `smoke`, `low_cost_crosscheck`, and `quality_code` profiles. It
  supports a Branch A claim for the tested profiles: MICO JSON AST
  and MICO JSON AST plus compiler-feedback repair produce nonzero to full
  positive compiler/lint passes and strong unsafe rejection, while direct
  Verilog, SystemVerilog-interface, and MICO-source baselines remain weak.
- Repository-owned JSON AST repair patch applicator in the compiler/CLI. The
  `repair-json` command supports dry-run, apply, and immediate re-check using
  `schemas/mico_repair_patch.schema.json`; the LLM batch runner delegates patch
  application to this CLI path. The batch runner also includes a narrow,
  explicitly recorded deterministic fallback for the common adapter-as-instance
  model error; broader free-form repair remains unsupported.
- Full release-candidate validation wrappers in `scripts/full-check.sh` and
  `scripts/full-check.ps1`, plus a top-level release checklist, generated
  `build/release/full_check_manifest.json` metadata record, and
  `build/release/deterministic_evidence_hashes.json` deterministic evidence
  hash sidecar, and `scripts/make-release-bundle.ps1` review ZIP/sidecar
  packager. The release
  gate records public-development and held-out benchmark hashes, sanitized LLM
  validate-only hashes, optional Vivado subset hashes, and the final paper PDF
  hash when `-WithLatex` is used.
- IEEE-style paper draft compressed to a five-page DAC-style manuscript, with
  generated deterministic tables, conservative claim boundaries, and a bounded
  tested-profile v3 structured LLM matrix summary.

## Not Yet Implemented

- Broader semantic repair policies beyond schema-valid JSON AST operations.
- Arbitrary LTL or temporal contract proving beyond the v0 ready/valid subset.
- Directed task-specific formal harnesses beyond the 24 committed single-clock
  monitors.
- CDC correctness proof for the smoke FIFO collateral.
- Full timing closure, broad Vivado QoR, and technology-mapped delay reporting
  beyond the representative nine-task Vivado subset.
- Release-archived full paid multi-profile LLM baseline result artifacts.
- Positive pass-rate improvement claims beyond the exact v3 tested profiles,
  prompts, public-development split, and held-out split.
- Broad free-form LLM repair reliability beyond the adapter-as-instance
  deterministic fallback recorded in `docs/24_llm_matrix_v3.md`.
- Full generated statistical appendix and any final submission-only table
  integration beyond the deterministic summary table.
- Broader subsystem case studies beyond the current five public-dev plus seven
  held-out deterministic cases.
- Immutable release tag, GitHub Release, or Zenodo archive; the current policy
  uses a reviewable release branch and generated bundle first, then publishes
  permanent archives only after final artifact approval.

## Current ModuleComposeBench Boundary

Current deterministic benchmark scope:

- Total tasks: 62, with 36 positives and 26 negatives.
- Level coverage: L1 10, L2 13, L3 10, L4 10, L5 10, and L6 9.
- The deterministic compiler baseline includes same-domain stream wiring,
  width adaptation, latency/backpressure seed tasks, CDC/RDC adapter tasks,
  bus/register wrapper seeds, subsystem seeds, and 26 unsafe-rejection cases.
- Expected current result: expected outcome 62/62, positive compose 36/36,
  positive lint/elaboration smoke 36/36, positive simulation 36/36, unsafe
  rejection 26/26, JSON AST path 62/62, single-clock bounded formal smoke
  31/31, structural plus generic-mapped QoR available 9/9. The mode split is
  32 declared and 4 generated simulations, plus 24 declared and 7 generated
  single-clock formal checks.
- `formal_pass` is claimed only for the single-clock formal smoke denominator;
  CDC proof, full task-specific formal coverage, broad timing QoR,
  technology-mapped delay, and broad Vivado QoR remain intentionally unclaimed.
  The separate Vivado subset covers only nine representative tasks and uses
  measurement-only build copies.
- L3/L5/L6 include seed approximations plus the dedicated T058--T062 streaming,
  width-bridge, register/status, protocol-bridge, and telemetry subsystem case
  studies.
- Held-out split: `benchmarks/module_compose_bench_heldout.yaml` has 20 tasks
  with ten positives and ten negatives, including seven held-out subsystem
  positives for AXI/APB wrapper, video pipeline, explicit CDC event/status,
  telemetry, protocol bridge, and register/status composition. Its expected
  current result is 20/20 expected outcome, 10/10 positive lint/sim, 9/9
  single-clock formal smoke, 3/3 QoR, 10/10 unsafe rejection, and 20/20 JSON
  AST path. The held-out mode split is 7 declared and 3 generated simulations,
  plus 6 declared and 3 generated single-clock formal checks; the explicit CDC
  case remains formal not-run.

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
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
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
