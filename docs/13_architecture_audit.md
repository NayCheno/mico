# MICO Current Architecture Audit

Audit date: 2026-06-14.

This audit supersedes the initial scaffold audit. The repository is now a working
research prototype with a Rust parser/checker/codegen path, open-source EDA
smoke flow, source-level JSON AST path, 57-task seed benchmark runner, LLM provider
validation script, and a cautiously worded paper draft. It is still not a complete "engineering +
experiments + paper" artifact: large-scale LLM baselines, case studies, full
formal coverage, timing QoR, and paper tables remain open milestones.

## Sources Reviewed

- Top-level package: `README.md`, `PROJECT_MANIFEST.md`, `.gitignore`, and
  `CHANGELOG.md`.
- Product specs and workflow docs: `docs/01_language_spec_v0.md`,
  `docs/02_architecture.md`, `docs/03_llm_protocol.md`,
  `docs/06_evaluation_plan.md`, `docs/12_docker_eda_environment.md`,
  `docs/14_reproduction_workflow.md`, and `docs/diagnostics.md`.
- Rust workspace: `rust_project/Cargo.toml`, `rust_project/examples/*.mico`,
  and the `mico_ir`, `mico_frontend`, `mico_codegen`, and `mico_cli` crates.
- RTL and EDA flow: `rtl/examples/mico_example_leafs.sv`,
  `scripts/eda-smoke.sh`, `scripts/eda-docker.ps1`,
  `scripts/eda-docker.sh`, and `scripts/run-vivado-host.ps1`.
- Benchmark and LLM assets: `benchmarks/module_compose_bench_manifest.yaml`,
  `benchmarks/run_bench.py`, `schemas/*.schema.json`, `prompts/`,
  `scripts/llm-provider-smoke.py`, and `scripts/run_llm_bench.py`.
- Paper source: `paper/main.tex`, `paper/sections/*.tex`, and
  `paper/related_work.bib`.

Local provider configuration under `config/*.local.yaml` is intentionally
ignored and must not be committed or printed. The repository-owned provider
scripts report only redacted key presence and source metadata.

## Current Implementation

### Rust Frontend And IR

The frontend is no longer line-oriented. `mico_frontend` now contains a
hand-written lexer, token stream, byte/line/column `SourceSpan`, parser
diagnostics, recursive parser support for `clockdom`, `interface`, `module`,
`adapter`, and `compose`, line comments, multiline contract text, and basic
syntax recovery.

`mico_ir` owns the parsed design model, typed IR model, semantic diagnostics,
adapter kind model, protocol inference, reset polarity inference, and typed
connection metadata. Implemented semantic checks include duplicate top-level
declarations, duplicate fields, duplicate ports, duplicate compose instances,
unknown domains/interfaces/modules/instances/ports/adapters, direction
mismatch, direct interface mismatch, direct clock-domain mismatch, adapter
endpoint mismatch, known adapter kind validation, ready/valid width checks, a
parsed ready/valid v0 contract subset, and adapter guarantee coverage checks.

Current limitations:

- Semantic diagnostics carry graph nodes, labels, repair actions, and optional
  spans; many checker diagnostics still use `span: null` where only graph
  references are available.
- Contracts are parsed only for a small v0 ready/valid subset. The compiler
  checks conservative source/adapter/sink requirement coverage, but it does not
  prove arbitrary temporal logic.
- Repair patch ingestion is not implemented; the schema exists for future
  compiler-feedback loops.

### Codegen And CLI

`mico_codegen` emits deterministic `serde_json` typed IR with
`schema_version = mico.ir.v0`, conservative SystemVerilog wrappers,
ready/valid SVA skeleton modules, and traceability JSON with
`schema_version = mico.traceability.v0`. Traceability reports include stable
compose-connection source references, generated signal names, leaf module port
names, adapter boundary records, and SVA contract IDs. Generated wrappers use
``default_nettype none``, flatten interface fields into primitive wires,
instantiate leaf modules, instantiate explicit adapters, and pass clock/reset
signals to CDC adapters. The positive seed tasks have committed golden
SV/SVA/traceability fixtures checked by `mico_codegen` tests.

`mico_cli` supports:

- `parse`
- `check`
- `build`
- `dump-ir`
- `emit-sv`
- `emit-sva`
- `emit-trace`
- `verify`
- `report`
- `dump-ast-json`
- `check-json`
- `build-json`
- `dump-json-ir`
- `emit-json-sv`
- `emit-json-sva`
- `emit-json-trace`

The CLI supports `--format text|json` for diagnostic-bearing commands and emits
the diagnostics envelope documented in `docs/diagnostics.md` and
`schemas/diagnostics.schema.json`. The `verify` command currently reports
compiler and typed-IR status only; it does not invoke Verilator, Yosys, Icarus,
or SymbiYosys directly. Source-level JSON AST documents use
`schema_version = mico.ast.v0` and are validated by the CLI before checking or
emission.

Current limitations:

- CLI argument parsing is still hand-written.
- `verify` is not yet an end-to-end EDA runner.
- JSON diagnostics still use `null` spans for checker errors that are tracked
  by graph node rather than source-byte location.

### RTL And EDA Flow

The repository has a Docker-first open-source EDA flow. `scripts/eda-smoke.sh`
generates wrappers and SVA skeletons for `stream_fifo`, `cdc_fifo`, and
`width_adapter`, then runs Verilator lint, SVA lint, Icarus elaboration, Yosys
hierarchy/proc/opt/stat, and a minimal SymbiYosys smoke proof.
ModuleComposeBench also has Icarus/VVP simulation harnesses for the four
positive seed tasks and reports `sim_pass: 4/4` in the deterministic runner.
It has selected bounded SymbiYosys harnesses for the width adapter and direct
stream seeds and reports `formal_pass: 2/2` over that enabled subset.
It also parses Yosys structural `stat -json` output for positive seed wrappers,
compares against committed hand-written references, and reports
`qor_available: 4/4`.

The committed RTL collateral in `rtl/examples/mico_example_leafs.sv` is
smoke-only. The CDC FIFO collateral is not a CDC correctness proof. Vivado is
not required for seed results; when Vivado is needed, the only allowed host
root is `D:\Application\vivado\2025.2\Vivado`.

Current limitations:

- Formal coverage is limited to the direct stream and width adapter seed
  tasks.
- QoR is structural area/wire accounting only; it is not timing closure,
  technology-mapped delay, or Vivado QoR.
- Adapter correctness boundaries are documented but not yet backed by full
  properties.

### ModuleComposeBench

`benchmarks/module_compose_bench_manifest.yaml` currently contains 57 seed
tasks with required natural-language requests, module/interface/adapter
inventories, expected diagnostics, and RTL collateral. The current deterministic
scope is:

| Level | Total | Positive | Negative | Focus |
|---|---:|---:|---:|---|
| L1 | 10 | 9 | 1 | direct same-domain stream wiring |
| L2 | 13 | 7 | 6 | width and parameter adaptation |
| L3 | 10 | 6 | 4 | latency/backpressure and protocol seeds |
| L4 | 10 | 5 | 5 | CDC/RDC adapter and rejection seeds |
| L5 | 8 | 3 | 5 | bus/register wrapper seeds |
| L6 | 6 | 1 | 5 | multi-IP subsystem seeds |

The current expected result is 57/57 expected outcomes, 31/31 positive
compose-pass, 31/31 positive lint/elaboration pass, 26/26 unsafe rejection, and
57/57 JSON AST path equivalence. Supported subsets remain 4/4 positive
simulations, 2/2 selected bounded formal proofs, and 4/4 structural QoR
comparisons.

`benchmarks/run_bench.py` executes the deterministic compiler baseline,
records expected diagnostic codes for negative tasks, emits SV/SVA/trace
artifacts for positive tasks, runs open-source EDA smoke checks where
supported, and writes `schema_version = mico.bench.results.v0`.

Current limitations:

- L3 latency/backpressure, L5 bus/register wrappers, and L6 subsystem entries
  are seed approximations over the existing smoke RTL collateral, not dedicated
  subsystem case studies.
- Natural-language prompts, model baselines, repair loops, statistical
  aggregation, full formal coverage, and broader QoR are still pending.

### LLM Provider Workflow

`scripts/llm-provider-smoke.py` is now a repository-owned SDK-backed OpenAI
Chat Completions validation and smoke script. It reads provider configuration
from `config/llm-provider.example.yaml` or an ignored local config, validates
profile/model/base URL shape, records prompt SHA-256, model/profile metadata,
repair turns, optional compiler and EDA JSON artifact attachments, token usage,
and cost fields in a sanitized `mico.llm.run.v0` record.

`scripts/run_llm_bench.py` adds the batch benchmark harness for the 57-task
manifest. It supports Direct Verilog, SystemVerilog-interface, MICO source,
MICO JSON AST, and MICO JSON AST + compiler-feedback repair baselines. It has
validate-only mode for prompt/profile matrix checks, offline-fixture mode for
compiler/EDA path validation without provider requests, authenticated execute
mode through the OpenAI SDK, response caching, JSON extraction, compiler checks,
open-source lint/elaboration for accepted positive candidates, and a Python
repair-patch applicator for the JSON AST repair baseline. Batch records use
`schema_version = mico.llm.bench.v0` and never store API keys.

Current limitations:

- Full paid low-cost and cross-model result matrices are not committed.
- The current runner records per-attempt failure status, but paper-ready failure
  taxonomy aggregation is still pending.
- Cost estimates still require local ignored profile rates.

### Paper

The paper source is split under `paper/main.tex` and `paper/sections/*.tex`.
The current abstract and evaluation section deliberately describe the artifact
as a 57-task deterministic seed result with four positive seed simulations and two
selected bounded formal proofs plus structural Yosys QoR summaries. They do
not claim full per-task formal proof, timing QoR, arbitrary LTL, or multi-model
pass-rate improvements. Host LaTeX is the repository policy for paper builds.

Current limitations:

- The paper is still an evidence-limited submission candidate, not a complete
  experimental paper.
- Tables are manually maintained rather than generated from committed result
  aggregation scripts.
- Case studies, ablations, confidence intervals, token/cost tables, and full
  reproducibility hashes are pending.

## Validation Gates For This Snapshot

The current snapshot is validated with these commands from the repository root:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json"
```

All Rust, Python, benchmark, and open-source EDA validation must run in the
repository Docker environment. Host Vivado is allowed only for Vivado-specific
flows through `scripts/run-vivado-host.ps1`. Host LaTeX is allowed only for the
paper workflow.

## Priority Gap List

The next work should proceed in this order:

1. Generate paper tables from deterministic and LLM benchmark artifacts.
2. Run and archive full low-cost LLM baseline matrices when cost settings are configured.
3. Add broader formal/QoR coverage, subsystem case studies, and release-candidate
   validation scripts.

## Claim Boundary

Current claims supported by the repository:

- MICO can parse, check, build typed IR, and emit traceable SV/SVA/JSON for a
  small v0 language.
- The 57-task manifest meets the publishable-scale deterministic benchmark
  threshold and spans L1-L6 with 31 positive and 26 negative tasks.
- The compiler rejects key unsafe seed cases: missing width adaptation, direct
  CDC, reversed direction, missing adapter guarantees, unknown adapter
  guarantees, and adapter guarantees invalid for their kind.
- The compiler parses and checks a conservative ready/valid v0 contract subset
  for adapter requirement coverage.
- Positive seed wrappers pass open-source lint/elaboration smoke checks.
- Positive seed SV/SVA/traceability output is covered by committed golden
  fixtures.
- Positive seed simulations pass with committed Icarus/VVP testbenches.
- Selected direct-stream and width-adapter seeds pass bounded SymbiYosys
  checks.
- Positive seed wrappers have structural Yosys area/wire QoR metrics against
  committed hand-written references.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and write sanitized run metadata.
- The LLM benchmark runner can plan the full 57-task low-cost baseline matrix,
  validate offline compiler/EDA scoring paths, and execute authenticated
  provider subsets without storing secrets.

Claims not yet supported:

- Multi-model or multi-baseline LLM pass-rate improvements.
- Full paid LLM benchmark matrix results committed as artifact data.
- Simulation coverage beyond the four positive seed tasks.
- Formal proof coverage beyond the selected direct and width seeds.
- Timing QoR, Vivado QoR, or technology-mapped delay conclusions.
- CDC correctness proof for the smoke FIFO collateral.
- Arbitrary LTL or complete temporal contract proving.
