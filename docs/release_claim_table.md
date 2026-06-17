# MICO Release Claim Table

Snapshot date: 2026-06-17.

This table is the single source for numeric claims used by README, workflow
docs, paper text, and release packaging. `docs/claim_boundary.md`,
`docs/submission_claim_freeze.md`, and
`docs/submission_claim_lock_2026Q3.md` remain the normative claim boundary;
this file maps each current number to the artifact path, schema, hash source,
and paper location that must support it in a release candidate.

Generated evidence stays under ignored `build/` paths. Hashes are recorded by
`build/release/full_check_manifest.json`,
`build/release/deterministic_evidence_hashes.json`, and release bundle sidecars
after the Docker and host-tool gates run. Do not commit generated JSON, CSV,
TeX snippets, Vivado reports, PDFs, local provider config, raw provider
payloads, or API keys.

## Deterministic Public-Development Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---:|---|---|---|---|
| Public-development manifest tasks | 83 total, 46 positive, 37 negative | `benchmarks/module_compose_bench_manifest.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Level distribution | L1 11, L2 13, L3 10, L4 12, L5 18, L6 19 | `benchmarks/module_compose_bench_manifest.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Public-development case-study positives and paired negatives | 14 positive, 9 paired negative variants | `benchmarks/module_compose_bench_manifest.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/tables/benchmark_split_summary.tex` |
| Expected outcomes | 83/83 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive compose | 46/46 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive lint/elaboration | 46/46 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive simulation | 46/46 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Simulation mode split | 46 declared, 0 generated | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Unsafe rejection | 37/37 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| JSON AST path | 83/83 | `build/bench/seed_results.json` | `mico.bench.results.v0`, `mico.ast.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Single-clock bounded formal smoke | 40/40 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Formal mode split | 40 declared, 0 generated | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Formal property coverage matrix | 9 property obligations; public 40/40, held-out 17/17, realism 13/13 base obligations; adapter-specific rows include no-drop/no-duplicate, width extension, register/status, protocol, and telemetry predicates | `build/bench/formal_coverage/formal_coverage_matrix.csv`, `build/bench/formal_coverage/formal_coverage_tasks.csv` | `scripts/write-formal-coverage-matrix.py` | `build/release/deterministic_evidence_hashes.json: generated_tables` | `paper/tables/formal_coverage_matrix.tex` |
| CDC structural boundary matrix | Explicit CDC adapter rows and direct-CDC rejection rows recorded separately; CDC correctness proof remains unclaimed | `build/bench/formal_coverage/cdc_structural_boundaries.csv` | `scripts/write-formal-coverage-matrix.py` | `build/release/deterministic_evidence_hashes.json: generated_tables` | `paper/tables/cdc_structural_boundaries.tex`, `paper/sections/09_limitations.tex` |
| Structural and generic-mapped QoR | 11/11 reference-enabled public tasks | `build/bench/seed_results.json`, `build/bench/aggregate_results.json` | `mico.bench.results.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |

## Held-Out Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---:|---|---|---|---|
| Held-out manifest tasks | 40 total, 20 positive, 20 negative | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out subsystem positives | 7 | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out paired negative variants | 7 | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out expected outcomes | 40/40 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive compose | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive lint/elaboration | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive simulation | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out simulation mode split | 20 declared, 0 generated | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Held-out unsafe rejection | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out JSON AST path | 40/40 | `build/bench/heldout_results.json` | `mico.bench.results.v0`, `mico.ast.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out single-clock bounded formal smoke | 17/17 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out formal mode split | 17 declared, 0 generated; explicit CDC cases formal not-run | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Held-out QoR | 6/6 reference-enabled held-out tasks | `build/bench/heldout_results.json`, `build/bench/aggregate_heldout_results.json` | `mico.bench.results.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |

## Supplemental Realism Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---:|---|---|---|---|
| Supplemental realism manifest tasks | 30 total, 15 positive, 15 negative | `benchmarks/module_compose_bench_realism.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `docs/25_realism_supplement.md`, `paper/sections/08_evaluation.tex` |
| New supplemental subsystem pairs | 5 positive and 3 paired negative plus calibration rows | `benchmarks/module_compose_bench_realism.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `docs/25_realism_supplement.md` |
| Supplemental expected outcomes | 30/30 | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `docs/25_realism_supplement.md` |
| Supplemental positive simulation | 15/15 declared | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `docs/25_realism_supplement.md` |
| Supplemental simulation mode split | 15 declared, 0 generated | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Supplemental bounded formal smoke | 13/13 declared | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `docs/25_realism_supplement.md` |
| Supplemental formal mode split | 13 declared, 0 generated | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Supplemental unsafe rejection | 15/15 | `build/bench/realism_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `docs/25_realism_supplement.md` |
| Supplemental QoR | 4/4 reference-enabled realism tasks | `build/bench/realism_results.json`, `build/bench/aggregate_realism_results.json` | `mico.bench.results.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |

## LLM And Repair Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---|---|---|---|---|
| Historical low-cost matrix | Negative result for original prompts | `docs/16_llm_matrix_results.md` plus ignored raw records | `mico.llm.bench.v0` when raw records are supplied | External archive or `build/release/full_check_manifest.json` when present | Background only |
| Structured v3 matrix | Bounded Branch A result for tested profiles, prompts, and locked pre-expansion 62/20 manifest hashes | `docs/24_llm_matrix_v3.md`, `docs/llm_final_matrix_report.md`, sanitized v3 execute records under `build/llm/` when present | `mico.llm.bench.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: authenticated_llm_evidence` when present | `paper/sections/08_evaluation.tex` |
| Tested profiles | `smoke`, `low_cost_crosscheck`, `quality_code` | `config/llm-provider.example.yaml`, sanitized execute records | `mico.llm.run.v0`, `mico.llm.bench.v0` | `build/release/full_check_manifest.json: llm` | `paper/sections/08_evaluation.tex` |
| Baselines | `direct_verilog`, `sv_interface`, `mico_source`, `mico_json_ast`, `mico_json_ast_repair` | `prompts/llm_bench_baselines.yaml`, sanitized execute records | `mico.llm.bench.v0` | `build/release/full_check_manifest.json: prompts`, `authenticated_llm_evidence` | `paper/sections/06_llm_workflow.tex`, `paper/sections/08_evaluation.tex` |
| Repair boundary | 0 accepted free-form repair-patch wins; 23 recorded deterministic fallback wins limited to the adapter-as-instance pattern plus compiler-gated JSON AST repair path | `docs/llm_final_matrix_report.md`, sanitized execute records | `mico.llm.bench.v0` | `build/release/full_check_manifest.json: authenticated_llm_evidence` when present | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |

## Vivado And Release Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---|---|---|---|---|
| Vivado subset | 12 QoR-enabled tasks: `T001`--`T004`, `T058`--`T065` | `build/reports/vivado-host/vivado_qor_subset_summary.json` | `scripts/vivado-qor-subset.tcl` output | `build/release/full_check_manifest.json: vivado_subset` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Vivado QoR threshold | 12/12 task pairs; median LUT delta 0.000%; maximum absolute LUT delta 0.000%; minimum generated/reference WNS 4.854/4.584 ns | `build/reports/vivado-host/vivado_qor_thresholds.json` | `scripts/check-vivado-qor-summary.py` over `mico.vivado_qor_subset.v0` | `build/release/full_check_manifest.json: vivado_subset` | `paper/tables/vivado_qor_thresholds.tex`, `paper/sections/08_evaluation.tex` |
| Vivado tool root | `D:\Application\vivado\2025.2\Vivado` | `scripts/run-vivado-host.ps1` | Host launcher policy | Source commit hash | `paper/sections/09_artifact_reproducibility.tex` |
| Full release gate | Docker gate plus optional host LaTeX | `build/release/full_check_manifest.json` | `mico.release.full_check.v0` | release bundle sidecar | `paper/sections/09_artifact_reproducibility.tex` |
| Paper PDF hash | Final host-built PDF only | `paper/main.pdf` generated locally | Host LaTeX output | `build/release/full_check_manifest.json: paper_pdf` | Artifact appendix |

## Unsupported Numeric Claims

Do not write these as supported claims in README, workflow docs, paper abstract,
introduction, evaluation, conclusion, or release notes:

- 60-task or 62-task public benchmark as the current deterministic manifest.
- 12-task, 20-task, or 12/12 held-out split as the current deterministic manifest.
- 20 or 36 directed public simulations, or 14 or 31 directed public formal monitors, as the current deterministic manifest.
- Four-task, four-wrapper, or stale nine-task Vivado subset.
- Exhaustive or randomized simulation coverage beyond the committed directed
  smoke scenarios.
- Exhaustive task-specific formal proof beyond bounded single-clock obligation
  denominators.
- CDC correctness proof.
- Full timing closure, routed implementation, or technology-mapped delay for
  the complete benchmark.
- LLM improvement beyond the exact v3 tested profiles, prompts,
  public-development split, and held-out split.
- Broad free-form repair reliability beyond the recorded deterministic
  adapter-instance fallback and compiler-gated repair path.
