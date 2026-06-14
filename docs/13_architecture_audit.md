# MICO Current Architecture Audit

Audit date: 2026-06-14.

The authoritative claim and environment boundary for this snapshot is
`docs/claim_boundary.md`; this audit explains the implementation evidence and
remaining gaps behind that boundary.

This audit supersedes the initial scaffold audit. The repository is now a working
research prototype with a Rust parser/checker/codegen path, open-source EDA
smoke flow, source-level JSON AST path, 62-task benchmark runner, benchmark
aggregation script, LLM provider
validation script, and a cautiously worded paper draft. It is still not a complete "engineering +
experiments + paper" artifact: full paid LLM pass-rate matrices, broader case
study diversity, full formal coverage, broad timing/QoR evidence, and final
paper-table integration remain open milestones.

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
  `scripts/eda-docker.sh`, `scripts/run-vivado-host.ps1`, and
  `scripts/vivado-qor-subset.tcl`.
- Benchmark and LLM assets: `benchmarks/module_compose_bench_manifest.yaml`,
  `benchmarks/run_bench.py`, `benchmarks/aggregate_results.py`,
  `schemas/*.schema.json`, `prompts/`,
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

- Semantic diagnostics from `.mico` input carry graph nodes, labels, repair
  actions, and source-map spans where the parser can map the related
  declaration, endpoint, field, port, adapter, or compose member. JSON AST
  inputs still use graph nodes as the fallback when source-byte locations are
  unavailable.
- Contracts are parsed only for a small v0 ready/valid subset. The compiler
  checks conservative source/adapter/sink requirement coverage, but it does not
  prove arbitrary temporal logic.
- Repair patch ingestion is implemented for source-level JSON AST documents
  through the repository-owned `repair-json` CLI command, with dry-run, apply,
  and immediate re-check behavior.

### Codegen And CLI

`mico_codegen` emits deterministic `serde_json` typed IR with
`schema_version = mico.ir.v0`, conservative SystemVerilog wrappers,
ready/valid SVA skeleton modules, and traceability JSON with
`schema_version = mico.traceability.v0`. Traceability reports include stable
compose-connection source references, generated signal names, leaf module port
names, adapter boundary records, and SVA contract IDs. Generated wrappers use
``default_nettype none``, flatten interface fields into primitive wires,
instantiate leaf modules, instantiate explicit adapters, and pass clock/reset
signals to CDC adapters. The seven sim/QoR-enabled positive seeds and case
studies have committed golden SV/SVA/traceability fixtures checked by
`mico_codegen` tests.

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
`schemas/diagnostics.schema.json`. `--json` is accepted as a shorthand for
JSON output. The `verify` command defaults to compiler and typed-IR checks.
`verify --eda` emits `Top.sv` and `Top.sva.sv` into an ignored artifact
directory, then runs Verilator wrapper lint, Verilator SVA lint, Icarus
elaboration, and Yosys hierarchy/proc/opt/stat checks against the repository
smoke RTL collateral. The JSON report records the artifact directory,
per-tool pass/fail status, stdout/stderr artifact paths, command lines, and
exit codes; `--schema-path` is accepted as explicit schema metadata for
callers. Source-level JSON AST documents use `schema_version = mico.ast.v0` and
are validated by the CLI before checking or emission.

Current limitations:

- CLI argument parsing is still hand-written.
- `verify --eda` is an open-source lint/elaboration/hierarchy smoke gate; it
  is not a replacement for the full benchmark runner, selected formal
  harnesses, CDC proof work, or broad timing/Vivado QoR.
- JSON diagnostics still use `null` spans for checker errors that are tracked
  by graph node rather than source-byte location.

### RTL And EDA Flow

The repository has a Docker-first open-source EDA flow. `scripts/eda-smoke.sh`
generates wrappers and SVA skeletons for `stream_fifo`, `cdc_fifo`, and
`width_adapter`, then runs Verilator lint, SVA lint, Icarus elaboration, Yosys
hierarchy/proc/opt/stat, and a minimal SymbiYosys smoke proof.
ModuleComposeBench reports `sim_pass: 36/36` for positive tasks. Twenty tasks,
including the five public-development subsystem case studies, use committed
directed Icarus/VVP testbenches; the remaining accepted positive tasks use
generated ready/valid smoke harnesses derived from traceability JSON. It
reports `formal_pass: 31/31` over the single-clock formal smoke denominator:
fourteen committed directed harnesses plus generated ready/valid formal
harnesses for the remaining single-clock positives.
It also parses Yosys structural and flattened generic-mapped `stat -json` output
for positive benchmark wrappers, compares against committed hand-written
references, and reports `qor_available: 9/9`. A separate host-Vivado subset
script performs out-of-context synthesis and measurement-copy timing extraction
for four representative tasks.

The committed RTL collateral in `rtl/examples/mico_example_leafs.sv` is
smoke-only. The CDC FIFO collateral is not a CDC correctness proof. Vivado is
not required for deterministic benchmark results; when Vivado is needed, the only allowed host
root is `D:\Application\vivado\2025.2\Vivado`.

Current limitations:

- Formal coverage is limited to single-clock smoke properties, with fourteen
  task-specific directed monitors.
- Broad QoR is structural area/wire accounting plus a generic mapped-cell proxy.
  The Vivado evidence is limited to a four-task out-of-context subset; it is not
  routed timing closure, technology-mapped delay for the full benchmark, or
  board-level implementation.
- Adapter correctness boundaries are documented but not yet backed by full
  properties.

### ModuleComposeBench

`benchmarks/module_compose_bench_manifest.yaml` currently contains 62 tasks:
57 hand-written seeds plus 5 dedicated subsystem case studies with required
natural-language requests, module/interface/adapter inventories, expected
diagnostics, expected feature tags, and RTL collateral. The manifest is checked
against `benchmarks/manifest_schema.json`, and the runner also verifies
committed task, source, RTL, simulation, formal, and QoR paths. The current
deterministic scope is:

| Level | Total | Positive | Negative | Focus |
|---|---:|---:|---:|---|
| L1 | 10 | 9 | 1 | direct same-domain stream wiring |
| L2 | 13 | 7 | 6 | width and parameter adaptation |
| L3 | 10 | 6 | 4 | latency/backpressure and protocol seeds |
| L4 | 10 | 5 | 5 | CDC/RDC adapter and rejection seeds |
| L5 | 10 | 5 | 5 | bus/register wrapper seeds, register/status case, and protocol bridge |
| L6 | 9 | 4 | 5 | multi-IP subsystem seeds and streaming/width/telemetry case studies |

The current expected result is 62/62 expected outcomes, 36/36 positive
compose-pass, 36/36 positive lint/elaboration pass, 36/36 positive simulation
smoke pass, 26/26 unsafe rejection, and 62/62 JSON AST path equivalence.
Supported subsets are 31/31 single-clock bounded formal smoke proofs and 9/9
structural plus generic-mapped QoR comparisons.

`benchmarks/run_bench.py` executes the deterministic compiler baseline,
records expected diagnostic codes for negative tasks, emits SV/SVA/trace
artifacts for positive tasks, runs open-source EDA smoke checks where
supported, and writes `schema_version = mico.bench.results.v0`.

The committed task set is documented as the public development split. Held-out
LLM claims require a separately versioned manifest and sanitized result archive
that is not used for prompt iteration. The batch prompt builder includes task
requests, inventories, and interface/module declarations, but strips committed
`compose` bodies from expected MICO sources before constructing prompts.

Current limitations:

- L3 latency/backpressure still relies on seed approximations over smoke RTL;
  L5/L6 now include dedicated register/status, protocol bridge, streaming
  accelerator, width-bridge, and telemetry-chain case-study RTL, but broader
  subsystem diversity is still needed.
- Full task-specific formal coverage and broad timing/Vivado QoR remain pending.

### LLM Provider Workflow

`scripts/llm-provider-smoke.py` is now a repository-owned SDK-backed OpenAI
Chat Completions validation and smoke script. It reads provider configuration
from `config/llm-provider.example.yaml` or an ignored local config, validates
profile/model/base URL shape, records prompt SHA-256, model/profile metadata,
repair turns, optional compiler and EDA JSON artifact attachments, token usage,
and cost fields in a sanitized `mico.llm.run.v0` record.

`scripts/run_llm_bench.py` adds the batch benchmark harness for the 62-task
manifest. It supports Direct Verilog, SystemVerilog-interface, MICO source,
MICO JSON AST, and MICO JSON AST + compiler-feedback repair baselines. It has
validate-only mode for prompt/profile matrix checks, offline-fixture mode for
compiler/EDA path validation without provider requests, authenticated execute
mode through the OpenAI SDK, response caching, JSON extraction, compiler checks,
open-source lint/elaboration for accepted positive candidates, and the
repository-owned CLI repair-patch path for the JSON AST repair baseline. Batch
records use
`schema_version = mico.llm.bench.v0` and never store API keys. Patch
application is delegated to `mico_cli repair-json` so CLI and benchmark repair
semantics stay aligned.

An authenticated low-cost matrix has been executed for 62 tasks, two low-cost
profiles, and five baselines. The sanitized summary is recorded in
`docs/16_llm_matrix_results.md`. The result is negative for the current
prompt/model settings: positive-task compiler and lint pass rates are 0/36 for
all reported baselines, while MICO baselines reject 15--18 of 26 unsafe tasks
depending on profile and representation. This does not support an LLM advantage
claim.

Current limitations:

- Full paid low-cost raw result matrices remain ignored build artifacts and
  should be archived externally for release review; no raw provider payloads or
  secrets are committed.
- Failure taxonomy, repair-turn, token/cost, and paired-comparison aggregation
  are available when sanitized LLM batch result JSON files are supplied.
- Cost estimates still require local ignored profile rates.

### Paper

The paper source is split under `paper/main.tex` and `paper/sections/*.tex`.
The current abstract and evaluation section describe the 62-task deterministic
result, 36/36 positive-task smoke simulation coverage, 31/31 single-clock
bounded formal smoke coverage, structural plus generic-mapped Yosys QoR
summaries, the four-task representative Vivado subset, and the negative
authenticated low-cost LLM matrix. They must not claim full per-task formal
proof, broad timing closure, arbitrary LTL, or positive LLM pass-rate
improvements. Host LaTeX is the repository policy for paper builds.

Current limitations:

- The paper is still an evidence-limited submission candidate, not a complete
  experimental paper.
- Generated table snippets are available under ignored `build/paper_tables/`;
  the paper source now carries the deterministic table values and low-cost LLM
  summary, while raw result archives and final statistical appendices remain
  release work.
- Broader case-study diversity and full reproducibility hashes are pending.

## Validation Gates For This Snapshot

The current snapshot is validated with these commands from the repository root:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- verify --eda --json --artifact-dir ../build/mico-verify/stream_fifo_cli --schema-path ../schemas examples/stream_fifo.mico | python3 -m json.tool >/dev/null"
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json"
```

All Rust, Python, benchmark, LLM, paper-table, and open-source EDA validation
must run in the repository Docker environment. Host Vivado is allowed only for
Vivado-specific flows through `scripts/run-vivado-host.ps1`, which is pinned to
`D:\Application\vivado\2025.2\Vivado`. Host LaTeX is allowed only for final
paper PDF compilation from `paper/main.tex`.

## Priority Gap List

The next work should proceed in this order:

1. Run and archive full low-cost LLM baseline matrices when cost settings are configured.
2. Add broader formal/QoR coverage, additional subsystem studies, and release-candidate
   validation scripts.
3. Promote generated aggregate table snippets into the final paper evaluation.

## Claim Boundary

Current claims supported by the repository:

- MICO can parse, check, build typed IR, and emit traceable SV/SVA/JSON for a
  small v0 language.
- The 62-task manifest meets the publishable-scale deterministic benchmark
  threshold and spans L1-L6 with 36 positive and 26 negative tasks.
- The compiler rejects key unsafe seed cases: missing width adaptation, direct
  CDC, reversed direction, missing adapter guarantees, unknown adapter
  guarantees, and adapter guarantees invalid for their kind.
- The compiler parses and checks a conservative ready/valid v0 contract subset
  for adapter requirement coverage.
- Positive benchmark wrappers pass open-source lint/elaboration smoke checks.
- Selected sim/QoR-enabled positive seed and case-study SV/SVA/traceability
  outputs are covered by committed golden fixtures.
- All 36 positive tasks pass Icarus/VVP simulation; 20 use committed directed
  testbenches and the rest use generated ready/valid smoke harnesses.
- Thirty-one single-clock positive tasks pass bounded SymbiYosys formal smoke
  checks; 14 use committed directed monitors and the rest use generated
  ready/valid formal harnesses.
- Five dedicated subsystem case studies have committed RTL, MICO source,
  simulation testbenches, and structural/generic-mapped QoR references.
- Positive seed and case-study wrappers have structural Yosys area/wire and
  generic mapped-cell QoR metrics against committed hand-written references.
- The LLM provider path can validate redacted OpenAI-compatible configuration
  and write sanitized run metadata.
- The LLM benchmark runner can plan the full 62-task low-cost baseline matrix,
  validate offline compiler/EDA scoring paths, and execute authenticated
  provider subsets without storing secrets.
- `benchmarks/aggregate_results.py` can merge deterministic and optional LLM
  batch artifacts into CSV and TeX tables for deterministic summaries,
  per-level rates, QoR, ablations, repair turns, token/cost, paired comparison,
  and failure taxonomy.

Claims not yet supported:

- Multi-model or multi-baseline LLM pass-rate improvements.
- Full paid LLM benchmark matrix results committed as artifact data.
- Full directed functional simulation coverage beyond the 20 committed
  directed harnesses.
- Full task-specific formal proof coverage beyond the bounded formal smoke
  denominator.
- Full timing closure, broad Vivado QoR, or technology-mapped delay conclusions.
- CDC correctness proof for the smoke FIFO collateral.
- Arbitrary LTL or complete temporal contract proving.
