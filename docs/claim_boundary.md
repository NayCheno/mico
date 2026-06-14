# MICO Claim Boundary

Snapshot date: 2026-06-14.

This file is the authoritative boundary between evidence-backed MICO claims and
work that remains unsupported. It should be read together with
`docs/current_status.md`, `docs/13_architecture_audit.md`, and
`docs/14_reproduction_workflow.md`.

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
  code generation path for the 60-task ModuleComposeBench manifest.
- The deterministic benchmark has 60 tasks across L1-L6, with 34 positive and
  26 negative tasks.
- Positive benchmark tasks compose successfully and pass the currently enabled
  open-source lint/elaboration checks.
- Negative benchmark tasks reject unsafe compositions with stable diagnostic
  codes, graph-node references, and repair actions.
- The ready/valid v0 contract subset checks conservative adapter-guarantee
  coverage for supported patterns.
- Seven harness-enabled positive tasks pass committed Icarus/VVP simulations.
- Three selected direct, width-adapter, and streaming case-study tasks pass
  bounded SymbiYosys checks.
- Seven reference-enabled positive tasks have structural Yosys QoR summaries
  against committed hand-written wrappers.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and produce sanitized `mico.llm.run.v0` metadata.
- The LLM benchmark runner can plan the full baseline matrix, run offline
  fixture checks, execute authenticated provider subsets when local credentials
  and budget are configured, and emit sanitized `mico.llm.bench.v0` records.
- The compiler/CLI can dry-run, apply, and re-check schema-valid JSON AST
  repair patches through `repair-json`; the LLM benchmark runner uses this path
  for JSON AST repair turns.
- Aggregate scripts can merge deterministic and optional LLM artifacts into
  CSV, JSON, and TeX table snippets for the paper.

## Unsupported Claims

The current repository must not claim:

- Multi-model or multi-baseline LLM pass-rate improvement before sanitized paid
  matrix results are archived and aggregated.
- Full per-task simulation coverage beyond the current simulation-enabled
  denominator.
- Full formal proof coverage beyond the selected bounded formal denominator.
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
