# MICO Claim Boundary

Snapshot date: 2026-06-15.

This file is the authoritative boundary between evidence-backed MICO claims and
work that remains unsupported. It should be read together with
`docs/final_claim_freeze.md`, `docs/submission_claim_freeze.md`,
`docs/submission_claim_lock_2026Q3.md`, `docs/current_status.md`,
`docs/13_architecture_audit.md`, and `docs/14_reproduction_workflow.md`. DAC
2027 submission planning lives in `docs/dac2027_submission_plan.md`; this file
remains the source of truth for what the repository may currently claim.

## Submission Target Boundary

The primary full-paper target is DAC 2027 Research Track / Research Manuscript.
The current repository is not yet DAC-ready as a full research submission. It is
a strong deterministic artifact candidate with an explicitly negative
low-cost LLM matrix.

The final paper must choose one branch before submission:

- positive LLM-improvement paper, only after a new authenticated matrix shows
  nonzero positive compiler/lint pass rates and a statistically supported MICO
  structured-output advantage;
- negative LLM study plus compiler-gated benchmark/tool paper, if improved
  prompts and model profiles still fail but the deterministic benchmark,
  failure taxonomy, and reproducible artifact become the main contribution.

The v3 authenticated matrix in `docs/24_llm_matrix_v3.md` now supports a
bounded Branch A claim for the tested profiles and prompts: MICO JSON
AST prompting, with the recorded compiler-feedback repair fallback, improves
positive-task pass rate and unsafe rejection over direct RTL,
SystemVerilog-interface, and MICO-source prompting on the locked
pre-expansion LLM-scored public-development and held-out manifest hashes.
README, documentation, and paper text must keep this claim tied to that matrix
and must not generalize it to untested models, prompts, expanded deterministic
manifests, or benchmark splits.

Numeric claim values, evidence artifact paths, schema names, release hash
locations, and paper locations are tracked in `docs/release_claim_table.md`.

## Environment Policy

Rust, Python, benchmark scripts, LLM validation, open-source RTL/EDA checks,
schema validation, aggregate-result generation, and paper-table/data generation
must run in the repository Docker environment through `scripts/eda-docker.ps1`
or `scripts/eda-docker.sh`.

There are exactly two host-tool exceptions:

- Vivado-specific Xilinx flows run on Windows through
  `scripts/run-vivado-host.ps1`. The launcher is pinned to
  `D:\Application\vivado\2025.2\Vivado` and rejects Vivado executables outside
  that root.
- Final paper PDF compilation uses the Windows host LaTeX toolchain for
  `paper/main.tex`, following the repository `paper-latex-writer` workflow.
  Paper-related Python/statistical validation and table generation still run
  in Docker.

Do not commit `build/`, `rust_project/target/`, logs, PDFs, Vivado project
outputs, `config/*.local.yaml`, API keys, or other local credentials.

## Supported Claims

The current repository supports these claims when the release gate passes:

- MICO parses, checks, builds typed IR, and emits deterministic SV, SVA, JSON
  IR, and traceability records for the v0 language subset.
- Source `.mico` and source-level JSON AST inputs share the same checker and
  code generation path for the 83-task ModuleComposeBench manifest.
- The deterministic benchmark has 83 tasks across L1-L6, with 46 positive and
  37 negative tasks.
- Positive benchmark tasks compose successfully and pass the currently enabled
  open-source lint/elaboration checks.
- Negative benchmark tasks reject unsafe compositions with stable diagnostic
  codes, graph-node references, and repair actions.
- The committed benchmark manifest is schema-valid and documented as the public
  development split; final LLM advantage claims still require separately
  archived held-out results.
- The committed held-out manifest contains 40 scoring tasks with twenty positives,
  twenty negatives, seven subsystem positives, seven paired negative variants,
  and balanced per-level calibration rows; deterministic held-out scoring is
  expected to pass 40/40 outcomes. Its directed verification denominator includes
  twenty declared simulations and seventeen declared single-clock formal monitors;
  explicit CDC cases remain smoke-only for formal.
- The supplemental realism manifest contains 30 deterministic-only tasks,
  including subsystem realism positives, paired negatives, and balanced L1-L6
  calibration rows. It raises the committed deterministic case-study corpus
  without changing the locked v3 LLM-scored manifest hashes.
- The ready/valid v0 contract subset checks conservative adapter-guarantee
  coverage for supported patterns.
- All 46 public-development positive tasks pass Icarus/VVP simulation smoke checks through
  committed directed testbenches.
- Forty public-development single-clock positive tasks pass bounded SymbiYosys formal smoke
  checks through committed directed monitors.
- Eleven public-development reference-enabled positive tasks have structural Yosys area/wire and
  generic mapped-cell QoR summaries against committed hand-written wrappers.
- A dedicated host-Vivado subset script synthesizes measurement-only copies for
  12 QoR-enabled public and held-out tasks (`T001`--`T004` and `T058`--`T065`) on
  `xc7a35tcpg236-1` and reports LUT/FF/BRAM/DSP plus WNS summaries under
  ignored `build/reports/vivado-host/`.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and produce sanitized `mico.llm.run.v0` metadata.
- The LLM benchmark runner can plan the full baseline matrix, run offline
  fixture checks, execute authenticated provider subsets when local credentials
  and budget are configured, and emit sanitized `mico.llm.bench.v0` records.
- The historical authenticated low-cost matrix summary is a negative result for
  the original prompts. The v3 structured matrix supersedes it for current
  LLM claims and supports a bounded tested-profile MICO-vs-baseline pass-rate
  improvement claim.
- The compiler/CLI can dry-run, apply, and re-check schema-valid JSON AST
  repair patches through `repair-json`; the LLM benchmark runner uses this path
  for JSON AST repair turns. The v3 matrix additionally records a narrow
  deterministic adapter-instance repair fallback that is accepted only through
  the same compiler path.
- Aggregate scripts can merge deterministic and optional LLM artifacts into
  CSV, JSON, and TeX table snippets for the paper.

## Unsupported Claims

The current repository must not claim:

- LLM pass-rate improvement beyond the tested v3 OpenCode Go profiles,
  prompts, and locked pre-expansion public-development and held-out split
  manifest hashes.
- Full paid LLM benchmark matrix raw results committed as artifact data.
- Broad free-form LLM repair reliability. Current recorded repair wins are
  limited to the explicitly recorded deterministic adapter-instance fallback.
- Exhaustive or randomized simulation coverage beyond the committed directed
  smoke scenarios.
- Exhaustive task-specific formal proof coverage beyond the bounded
  single-clock obligation denominator.
- CDC correctness proof for the smoke FIFO collateral.
- Full timing closure, routed implementation, bitstream generation,
  technology-mapped delay claims for the complete benchmark, or Vivado QoR
  beyond the dedicated 12-task out-of-context subset.
- Arbitrary LTL proving or complete temporal contract verification.
- Semantic correctness of arbitrary model-proposed repairs beyond the compiler,
  schema, and EDA gates that accept or reject the patched result.
- Final submission-readiness until the release manifest, result hashes, paper
  build, and clean-tree checks all pass.

## Required Evidence Files

The release gate produces evidence under ignored `build/` paths. These files
must be reviewed or archived externally before stronger claims are made:

- `build/bench/seed_results.json`
- `build/bench/heldout_results.json`
- `build/bench/aggregate_results.json`
- `build/bench/aggregate_heldout_results.json`
- `build/release/deterministic_evidence_hashes.json`
- `build/llm/provider_validate.json`
- `build/llm/bench_validate.json`
- optional sanitized authenticated LLM result files under `build/llm/`
- `build/release/full_check_manifest.json`
- optional `build/reports/vivado-host/vivado_qor_subset_summary.json` and CSV
  summaries from the host-Vivado subset
- generated paper table snippets under `build/paper_tables/`
- the host LaTeX build log and final paper PDF hash for release artifacts

Generated evidence should remain out of git unless a future release policy
explicitly creates a redacted `artifacts/results/` area.

## Paper Claim Mapping

Paper text must map every result claim to committed source plus one of the
evidence files above:

- deterministic compiler and benchmark claims map to
  `build/bench/seed_results.json`;
- aggregate tables map to `build/bench/aggregate_results.json` and generated
  TeX snippets;
- LLM claims map only to sanitized `mico.llm.bench.v0` files from authenticated
  runs;
- QoR claims must identify whether they are structural Yosys summaries,
  generic-mapped Yosys summaries, the 12-task Vivado measurement-copy subset,
  or a future technology-mapped timing artifact;
- CDC, formal, and timing limitations must remain visible in the abstract,
  evaluation, threats, and conclusion until stronger evidence exists.
