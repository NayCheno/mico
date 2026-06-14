# MICO Current Architecture Audit

Audit date: 2026-06-14.

This audit supersedes the initial scaffold audit. The repository is now a working
research prototype with a Rust parser/checker/codegen path, open-source EDA
smoke flow, seed benchmark runner, LLM provider validation script, and a
cautiously worded paper draft. It is still not a complete "engineering +
experiments + paper" artifact: simulation, formal, QoR, large-scale LLM
baselines, case studies, and paper tables remain open milestones.

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
  `benchmarks/run_bench.py`, `schemas/*.schema.json`, `prompts/`, and
  `scripts/llm-provider-smoke.py`.
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
endpoint mismatch, known adapter kind validation, ready/valid width checks, and
basic contract-preservation attributes for adapters.

Current limitations:

- Semantic diagnostics still lack source spans, graph nodes, primary/secondary
  labels, and structured repair actions.
- Contracts are still mostly string/attribute based; the compiler does not yet
  parse a contract AST or prove source/adapter/sink obligation coverage.
- JSON AST input and repair patch ingestion are not implemented.

### Codegen And CLI

`mico_codegen` emits deterministic `serde_json` typed IR with
`schema_version = mico.ir.v0`, conservative SystemVerilog wrappers,
ready/valid SVA skeleton modules, and traceability JSON with
`schema_version = mico.traceability.v0`. Generated wrappers use
``default_nettype none``, flatten interface fields into primitive wires,
instantiate leaf modules, instantiate explicit adapters, and pass clock/reset
signals to CDC adapters.

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

The CLI supports `--format text|json` for diagnostic-bearing commands and emits
the diagnostics envelope documented in `docs/diagnostics.md` and
`schemas/diagnostics.schema.json`. The `verify` command currently reports
compiler and typed-IR status only; it does not invoke Verilator, Yosys, Icarus,
or SymbiYosys directly.

Current limitations:

- CLI argument parsing is still hand-written.
- `verify` is not yet an end-to-end EDA runner.
- JSON diagnostics reserve semantic span/node fields but currently emit `null`
  spans and empty node/label arrays for checker diagnostics.

### RTL And EDA Flow

The repository has a Docker-first open-source EDA flow. `scripts/eda-smoke.sh`
generates wrappers and SVA skeletons for `stream_fifo`, `cdc_fifo`, and
`width_adapter`, then runs Verilator lint, SVA lint, Icarus elaboration, Yosys
hierarchy/proc/opt/stat, and a minimal SymbiYosys smoke proof.

The committed RTL collateral in `rtl/examples/mico_example_leafs.sv` is
smoke-only. The CDC FIFO collateral is not a CDC correctness proof. Vivado is
not required for seed results; when Vivado is needed, the only allowed host
root is `D:\Application\vivado\2025.2\Vivado`.

Current limitations:

- No per-task simulation testbenches are committed.
- No per-task formal harnesses are committed.
- No QoR parser or report aggregation exists.
- Adapter correctness boundaries are documented but not yet backed by full
  properties.

### ModuleComposeBench

`benchmarks/module_compose_bench_manifest.yaml` currently contains seven seed
tasks:

| Task | Type | Level | Purpose |
|---|---|---|---|
| `T001_stream_fifo` | positive | L1 | FIFO stream chain |
| `T002_cdc_fifo` | positive | L4 | explicit CDC adapter |
| `T003_width_adapter` | positive | L2 | explicit width adapter |
| `T004_direct_stream` | positive | L1 | direct ready/valid wiring |
| `T005_invalid_width_no_adapter` | negative | L2 | reject width mismatch without adapter |
| `T006_direct_cdc_without_adapter` | negative | L4 | reject direct CDC |
| `T007_reversed_direction` | negative | L1 | reject reversed connection direction |

`benchmarks/run_bench.py` executes the deterministic compiler baseline,
records expected diagnostic codes for negative tasks, emits SV/SVA/trace
artifacts for positive tasks, runs open-source EDA smoke checks where
supported, and writes `schema_version = mico.bench.results.v0`.

Current limitations:

- The benchmark is seven seed tasks, not the target 50+ task suite.
- L3 latency/backpressure, L5 bus/register wrappers, and L6 subsystem tasks
  are not represented at publishable scale.
- Natural-language prompts, model baselines, repair loops, statistical
  aggregation, simulation, formal, and QoR are still pending.

### LLM Provider Workflow

`scripts/llm-provider-smoke.py` is now a repository-owned SDK-backed OpenAI
Chat Completions validation and smoke script. It reads provider configuration
from `config/llm-provider.example.yaml` or an ignored local config, validates
profile/model/base URL shape, records prompt SHA-256, model/profile metadata,
repair turns, optional compiler and EDA JSON artifact attachments, token usage,
and cost fields in a sanitized `mico.llm.run.v0` record.

Current limitations:

- There is no prompt-to-MICO batch benchmark runner.
- Compiler diagnostics are not automatically fed into a repair prompt.
- Direct Verilog, SV interface, MICO source, MICO JSON, and MICO JSON + repair
  baselines are not yet implemented.
- No retry/resume/cache/rate-limit or failure taxonomy exists for paid runs.

### Paper

The paper source is split under `paper/main.tex` and `paper/sections/*.tex`.
The current abstract and evaluation section deliberately describe the artifact
as a seven-task seed result and do not claim per-task simulation, formal proof,
QoR, or multi-model pass-rate improvements. Host LaTeX is the repository policy
for paper builds.

Current limitations:

- The paper is still an evidence-limited submission candidate, not a complete
  experimental paper.
- Tables are manually maintained rather than generated from committed result
  aggregation scripts.
- Case studies, ablations, confidence intervals, token/cost tables, and full
  reproducibility hashes are pending.

## Validation Gates For This Snapshot

The M0 baseline is validated with these commands from the repository root:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
```

All Rust, Python, benchmark, and open-source EDA validation must run in the
repository Docker environment. Host Vivado is allowed only for Vivado-specific
flows through `scripts/run-vivado-host.ps1`. Host LaTeX is allowed only for the
paper workflow.

## Priority Gap List

The next work should proceed in this order:

1. Add source spans, graph nodes, labels, and suggested repair actions to
   semantic diagnostics.
2. Add schema-validated JSON AST input and repair patch schemas.
3. Parse and check a v0 ready/valid contract subset.
4. Add codegen golden tests for SV, SVA, and traceability outputs.
5. Add per-task simulation and selected formal harnesses.
6. Add QoR parsing and aggregation.
7. Expand ModuleComposeBench to 50+ tasks across L1-L6.
8. Add LLM batch baselines and compiler-feedback repair loops.
9. Generate paper tables from benchmark artifacts.
10. Add subsystem case studies and release-candidate validation scripts.

## Claim Boundary

Current claims supported by the repository:

- MICO can parse, check, build typed IR, and emit traceable SV/SVA/JSON for a
  small v0 language.
- The compiler rejects key unsafe seed cases: missing width adaptation, direct
  CDC, and reversed direction.
- Positive seed wrappers pass open-source lint/elaboration smoke checks.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and write sanitized run metadata.

Claims not yet supported:

- 50-task benchmark maturity.
- Multi-model or multi-baseline LLM pass-rate improvements.
- Per-task simulation pass rates.
- Per-task formal proofs.
- QoR overhead or timing conclusions.
- CDC correctness proof for the smoke FIFO collateral.
