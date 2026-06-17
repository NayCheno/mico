# MICO Current Status

Snapshot date: 2026-06-17.

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
- Icarus simulation coverage for all 46 public-development positive tasks through committed
  directed testbenches.
- Bounded SymbiYosys formal smoke coverage for all 40 public-development single-clock positive
  tasks through committed directed monitors.
- Structural and generic-mapped Yosys QoR extraction for supported positive
  benchmark wrappers, compared against committed hand-written reference wrappers
  with generated CSV/TeX summaries.
- Host-Vivado out-of-context QoR/timing subset for all 21 reference-enabled
  public-development, held-out, and realism split rows, mapped to 12 unique task
  pairs (`T001`--`T004` and `T058`--`T065`) through
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
- 83 public-development ModuleComposeBench tasks with required natural-language requests,
  module/interface/adapter inventories, expected diagnostics, explicit RTL
  collateral, 14 public-development subsystem positives, and a hardened
  40-task held-out case-study/calibration split.
- Documented public-development and held-out split policy. The committed main
  manifest is the public development split; the separate held-out manifest has
  40 scoring tasks, including seven subsystem positives, seven paired
  negative variants, and balanced per-level calibration rows. Prompt
  construction strips expected compose bodies from MICO sources to avoid
  solution leakage.
- Deterministic and LLM benchmark result records include the evaluated manifest
  path and SHA-256 hash.
- Repository-owned LLM provider validate/smoke script that writes sanitized
  `mico.llm.run.v0` records.
- Batch LLM benchmark runner for the expanded 83-task manifest with Direct Verilog,
  SystemVerilog-interface, MICO source, MICO JSON AST, and MICO JSON AST +
  compiler-feedback repair baselines. It supports validate-only planning,
  offline fixture checks, authenticated OpenAI-compatible execution, response
  caching, compiler/EDA scoring, and sanitized `mico.llm.bench.v0` records.
- Historical authenticated low-cost LLM matrix execution has been run for 62 tasks, two
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
  `docs/24_llm_matrix_v3.md` covers the locked pre-expansion
  public-development and held-out splits
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
  `build/release/deterministic_evidence_hashes.json`,
  `build/release/release_claim_table.json`, and
  `build/release/llm_evidence_hashes.json` release sidecars, and
  `scripts/make-release-bundle.ps1` review ZIP/sidecar packager with a bundled
  artifact quickstart. The release gate records public-development, held-out,
  and supplemental realism benchmark hashes, sanitized LLM validate-only and
  authenticated v3 hashes, optional Vivado subset hashes, and the final paper
  PDF hash when `-WithLatex` is used.
- IEEE-style paper draft compressed to a five-page DAC-style manuscript, with
  generated deterministic tables, conservative claim boundaries, and a bounded
  tested-profile v3 structured LLM matrix summary.

## Not Yet Implemented

- Broader semantic repair policies beyond schema-valid JSON AST operations.
- Arbitrary LTL or temporal contract proving beyond the v0 ready/valid subset.
- Exhaustive task-specific formal proof beyond the bounded single-clock smoke
  denominator.
- CDC correctness proof for the smoke FIFO collateral.
- Full timing closure, broad Vivado QoR, and technology-mapped delay reporting
  beyond the dedicated 21-row, 12-task-pair Vivado subset.
- Release-archived full paid multi-profile LLM baseline result artifacts.
- Positive pass-rate improvement claims beyond the exact v3 tested profiles,
  prompts, public-development split, and held-out split.
- Broad free-form LLM repair reliability beyond the adapter-as-instance
  deterministic fallback recorded in `docs/24_llm_matrix_v3.md`.
- Full generated statistical appendix and any final submission-only table
  integration beyond the deterministic summary table.
- Authenticated LLM reruns for the expanded 83-task public-development,
  40-task held-out, and 30-task realism manifests.
- Immutable release tag, GitHub Release, or Zenodo archive; the current policy
  uses a reviewable release branch and generated bundle first, then publishes
  permanent archives only after final artifact approval.

## Current ModuleComposeBench Boundary

Current deterministic benchmark scope:

- Total tasks: 83, with 46 positives and 37 negatives.
- Level coverage: L1 11, L2 13, L3 10, L4 12, L5 18, and L6 19.
- The deterministic compiler baseline includes same-domain stream wiring,
  width adaptation, latency/backpressure seed tasks, CDC/RDC adapter tasks,
  bus/register wrapper seeds, subsystem seeds, and 37 unsafe-rejection cases.
- Expected current result: expected outcome 83/83, positive compose 46/46,
  positive lint/elaboration smoke 46/46, positive simulation 46/46, unsafe
  rejection 37/37, JSON AST path 83/83, single-clock bounded formal smoke
  40/40, structural plus generic-mapped QoR available 11/11. The mode split is
  46 declared and 0 generated simulations, plus 40 declared and 0 generated
  single-clock formal checks.
- `formal_pass` is claimed only for the single-clock formal smoke denominator;
  CDC proof, full task-specific formal coverage, broad timing QoR,
  technology-mapped delay, and broad Vivado QoR remain intentionally unclaimed.
  The separate Vivado subset covers 21 reference-enabled split rows through 12
  unique task pairs and uses measurement-only build copies.
- L3/L5/L6 include seed approximations plus dedicated T058--T082 streaming,
  width-bridge, register/status, protocol-bridge, telemetry, AXI/APB, video,
  DMA register-map, packetizer, and MMIO subsystem case studies.
- Held-out split: `benchmarks/module_compose_bench_heldout.yaml` has 40 tasks
  with twenty positives and twenty negatives, including seven held-out subsystem
  positives for AXI/APB wrapper, video pipeline, explicit CDC event/status,
  telemetry, protocol bridge, and register/status composition. Its expected
  current result is 40/40 expected outcome, 20/20 positive lint/sim, 17/17
  single-clock formal smoke, 6/6 QoR, 20/20 unsafe rejection, and 40/40 JSON
  AST path. The held-out mode split is 20 declared and 0 generated simulations,
  plus 17 declared and 0 generated single-clock formal checks; explicit CDC
  case remains formal not-run.
- Supplemental realism split:
  `benchmarks/module_compose_bench_realism.yaml` has 30 deterministic-only
  tasks with 15 positives and 15 negatives. It adds subsystem realism
  positives, paired negatives, and balanced L1-L6 calibration rows, and is
  expected to pass 30/30 expected outcomes, 15/15 positive lint/sim, 13/13
  bounded single-clock formal, 15/15 unsafe rejection, 4/4 QoR, and 30/30 JSON
  AST path. It is not included in v3 LLM claims until a separate
  authenticated matrix reruns it.

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
