# MICO Claim Boundary

Snapshot date: 2026-06-14.

This file is the authoritative boundary between evidence-backed MICO claims and
work that remains unsupported. It should be read together with
`docs/current_status.md`, `docs/13_architecture_audit.md`, and
`docs/14_reproduction_workflow.md`. DAC 2027 submission planning lives in
`docs/dac2027_submission_plan.md`; this file remains the source of truth for
what the repository may currently claim.

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

Until that evidence exists, README, documentation, and paper text must not
claim that MICO improves LLM pass rate over direct RTL or
SystemVerilog-interface prompting.

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
  code generation path for the 62-task ModuleComposeBench manifest.
- The deterministic benchmark has 62 tasks across L1-L6, with 36 positive and
  26 negative tasks.
- Positive benchmark tasks compose successfully and pass the currently enabled
  open-source lint/elaboration checks.
- Negative benchmark tasks reject unsafe compositions with stable diagnostic
  codes, graph-node references, and repair actions.
- The committed benchmark manifest is schema-valid and documented as the public
  development split; final LLM advantage claims still require separately
  archived held-out results.
- The ready/valid v0 contract subset checks conservative adapter-guarantee
  coverage for supported patterns.
- All 36 positive tasks pass Icarus/VVP simulation smoke checks. Nine use
  committed directed testbenches; the remaining positives use generated
  ready/valid smoke harnesses derived from traceability JSON.
- Thirty-one single-clock positive tasks pass bounded SymbiYosys formal smoke
  checks. Three use committed directed monitors; the remaining single-clock
  positives use generated ready/valid formal harnesses derived from traceability
  JSON.
- Nine reference-enabled positive tasks have structural Yosys area/wire and
  generic mapped-cell QoR summaries against committed hand-written wrappers.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and produce sanitized `mico.llm.run.v0` metadata.
- The LLM benchmark runner can plan the full baseline matrix, run offline
  fixture checks, execute authenticated provider subsets when local credentials
  and budget are configured, and emit sanitized `mico.llm.bench.v0` records.
- The current authenticated low-cost matrix summary is a negative result and
  does not support a MICO-vs-baseline LLM pass-rate improvement claim.
- The compiler/CLI can dry-run, apply, and re-check schema-valid JSON AST
  repair patches through `repair-json`; the LLM benchmark runner uses this path
  for JSON AST repair turns.
- Aggregate scripts can merge deterministic and optional LLM artifacts into
  CSV, JSON, and TeX table snippets for the paper.

## Unsupported Claims

The current repository must not claim:

- Multi-model or multi-baseline LLM pass-rate improvement from the current
  low-cost matrix.
- Full paid LLM benchmark matrix raw results committed as artifact data.
- Full directed functional simulation coverage beyond the nine committed
  directed harnesses.
- Full task-specific formal proof coverage beyond the bounded formal smoke
  denominator.
- CDC correctness proof for the smoke FIFO collateral.
- Timing closure, technology-mapped delay, or Vivado QoR unless a dedicated
  Vivado or mapped-timing artifact is produced.
- Arbitrary LTL proving or complete temporal contract verification.
- Semantic correctness of arbitrary model-proposed repairs beyond the compiler,
  schema, and EDA gates that accept or reject the patched result.
- Final submission-readiness until the release manifest, result hashes, paper
  build, and clean-tree checks all pass.

## Required Evidence Files

The release gate produces evidence under ignored `build/` paths. These files
must be reviewed or archived externally before stronger claims are made:

- `build/bench/seed_results.json`
- `build/bench/aggregate_results.json`
- `build/llm/provider_validate.json`
- `build/llm/bench_validate.json`
- optional sanitized authenticated LLM result files under `build/llm/`
- `build/release/full_check_manifest.json`
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
  technology-mapped timing, or Vivado QoR;
- CDC, formal, and timing limitations must remain visible in the abstract,
  evaluation, threats, and conclusion until stronger evidence exists.
