# MICO Release Claim Table

Snapshot date: 2026-06-15.

This table is the single source for numeric claims used by README, workflow
docs, paper text, and release packaging. `docs/claim_boundary.md` remains the
normative claim boundary; this file maps each current number to the artifact
path, schema, hash source, and paper location that must support it in a release
candidate.

Generated evidence stays under ignored `build/` paths. Hashes are recorded by
`build/release/full_check_manifest.json`,
`build/release/deterministic_evidence_hashes.json`, and release bundle sidecars
after the Docker and host-tool gates run. Do not commit generated JSON, CSV,
TeX snippets, Vivado reports, PDFs, local provider config, raw provider
payloads, or API keys.

## Deterministic Public-Development Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---:|---|---|---|---|
| Public-development manifest tasks | 62 total, 36 positive, 26 negative | `benchmarks/module_compose_bench_manifest.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Level distribution | L1 10, L2 13, L3 10, L4 10, L5 10, L6 9 | `benchmarks/module_compose_bench_manifest.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Expected outcomes | 62/62 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive compose | 36/36 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive lint/elaboration | 36/36 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Positive simulation | 36/36 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Simulation mode split | 36 declared, 0 generated | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Unsafe rejection | 26/26 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| JSON AST path | 62/62 | `build/bench/seed_results.json` | `mico.bench.results.v0`, `mico.ast.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Single-clock bounded formal smoke | 31/31 | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Formal mode split | 31 declared, 0 generated | `build/bench/seed_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Structural and generic-mapped QoR | 9/9 reference-enabled public tasks | `build/bench/seed_results.json`, `build/bench/aggregate_results.json` | `mico.bench.results.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |

## Held-Out Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---:|---|---|---|---|
| Held-out manifest tasks | 20 total, 10 positive, 10 negative | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out subsystem positives | 7 | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out paired negative variants | 7 | `benchmarks/module_compose_bench_heldout.yaml` | `benchmarks/manifest_schema.json` | `build/release/full_check_manifest.json: benchmark_manifests` | `paper/sections/08_evaluation.tex` |
| Held-out expected outcomes | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive compose | 10/10 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive lint/elaboration | 10/10 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out positive simulation | 10/10 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out simulation mode split | 10 declared, 0 generated | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Held-out unsafe rejection | 10/10 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out JSON AST path | 20/20 | `build/bench/heldout_results.json` | `mico.bench.results.v0`, `mico.ast.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out single-clock bounded formal smoke | 9/9 | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |
| Held-out formal mode split | 9 declared, 0 generated; explicit CDC case formal not-run | `build/bench/heldout_results.json` | `mico.bench.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Held-out QoR | 3/3 reference-enabled held-out tasks | `build/bench/heldout_results.json`, `build/bench/aggregate_heldout_results.json` | `mico.bench.results.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: result_json_hashes` | `paper/sections/08_evaluation.tex` |

## LLM And Repair Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---|---|---|---|---|
| Historical low-cost matrix | Negative result for original prompts | `docs/16_llm_matrix_results.md` plus ignored raw records | `mico.llm.bench.v0` when raw records are supplied | External archive or `build/release/full_check_manifest.json` when present | Background only |
| Structured v3 matrix | Bounded Branch A result for tested profiles and prompts | `docs/24_llm_matrix_v3.md`, sanitized v3 execute records under `build/llm/` when present | `mico.llm.bench.v0`, `mico.aggregate.results.v0` | `build/release/full_check_manifest.json: authenticated_llm_evidence` when present | `paper/sections/08_evaluation.tex` |
| Tested profiles | `smoke`, `low_cost_crosscheck`, `quality_code` | `config/llm-provider.example.yaml`, sanitized execute records | `mico.llm.run.v0`, `mico.llm.bench.v0` | `build/release/full_check_manifest.json: llm` | `paper/sections/08_evaluation.tex` |
| Baselines | `direct_verilog`, `sv_interface`, `mico_source`, `mico_json_ast`, `mico_json_ast_repair` | `prompts/llm_bench_baselines.yaml`, sanitized execute records | `mico.llm.bench.v0` | `build/release/full_check_manifest.json: prompts`, `authenticated_llm_evidence` | `paper/sections/06_llm_workflow.tex`, `paper/sections/08_evaluation.tex` |
| Repair boundary | Accepted repair-turn wins are limited to the recorded deterministic adapter-instance fallback plus compiler-gated JSON AST repair path | `docs/24_llm_matrix_v3.md`, sanitized execute records | `mico.llm.bench.v0` | `build/release/full_check_manifest.json: authenticated_llm_evidence` when present | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |

## Vivado And Release Claims

| Claim | Current value | Evidence artifact | Schema or source | Hash source | Paper location |
|---|---|---|---|---|---|
| Vivado subset | 12 QoR-enabled tasks: `T001`--`T004`, `T058`--`T065` | `build/reports/vivado-host/vivado_qor_subset_summary.json` | `scripts/vivado-qor-subset.tcl` output | `build/release/full_check_manifest.json: vivado_subset` | `paper/sections/08_evaluation.tex`, `paper/sections/09_limitations.tex` |
| Vivado tool root | `D:\Application\vivado\2025.2\Vivado` | `scripts/run-vivado-host.ps1` | Host launcher policy | Source commit hash | `paper/sections/09_artifact_reproducibility.tex` |
| Full release gate | Docker gate plus optional host LaTeX | `build/release/full_check_manifest.json` | `mico.release.full_check.v0` | release bundle sidecar | `paper/sections/09_artifact_reproducibility.tex` |
| Paper PDF hash | Final host-built PDF only | `paper/main.pdf` generated locally | Host LaTeX output | `build/release/full_check_manifest.json: paper_pdf` | Artifact appendix |

## Unsupported Numeric Claims

Do not write these as supported claims in README, workflow docs, paper abstract,
introduction, evaluation, conclusion, or release notes:

- 60-task public benchmark.
- 12-task or 12/12 held-out split.
- 20 directed public simulations or 14 directed public formal monitors.
- Four-task, four-wrapper, or stale nine-task Vivado subset.
- Exhaustive or randomized simulation coverage beyond the committed directed
  smoke scenarios.
- Exhaustive task-specific formal proof beyond bounded ready/valid smoke
  denominators.
- CDC correctness proof.
- Full timing closure, routed implementation, or technology-mapped delay for
  the complete benchmark.
- LLM improvement beyond the exact v3 tested profiles, prompts,
  public-development split, and held-out split.
- Broad free-form repair reliability beyond the recorded deterministic
  adapter-instance fallback and compiler-gated repair path.
