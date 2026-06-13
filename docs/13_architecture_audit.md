# MICO Architecture Audit and Milestone Plan

Audit date: 2026-06-14.

Baseline commit reviewed: `15e8d3b chore: initialize MICO research scaffold`.

## Sources Reviewed

- Top-level project material: `README.md`, `PROJECT_MANIFEST.md`, `.gitignore`, `CHANGELOG.md`.
- Product and research specs: `docs/00_project_plan.md` through `docs/12_docker_eda_environment.md`.
- Rust compiler workspace: `rust_project/Cargo.toml`, crate manifests, `mico_ir`, `mico_frontend`, `mico_codegen`, `mico_cli`, examples, and smoke script.
- RTL/EDA environment: `docker/eda/Dockerfile`, `docker/eda/compose.yaml`, `docker/eda/verify-tools.sh`, `scripts/eda-docker.*`, and `scripts/run-vivado-host.ps1`.
- Benchmark and prompt assets: `benchmarks/`, `prompts/`, and `config/llm-provider.example.yaml`.
- Paper source: `paper/main.tex`, `paper/sections/*.tex`, `paper/related_work.bib`, and positioning notes.
- Local engineering policies: `.codex/skills/mico-rust-engineer`, `.codex/skills/rtl-tcl-engineer`, `.codex/skills/opencode-go-provider`, `.codex/skills/paper-latex-writer`, and `.codex/skills/git-commit`.

`config/llm-provider.local.yaml` exists and is ignored by git. The audit only validated its structure and did not print or commit any secret value.

## Baseline Validation

The following commands were run from the repository root unless noted otherwise.

| Area | Command | Result |
|---|---|---|
| Docker EDA tools | `.\scripts\eda-docker.ps1 mico-verify-tools` | Passed. Docker image `mico-eda:ubuntu24.04` is present with Rust 1.96.0, Yosys 0.33, Verilator 5.020, Icarus 12.0, GHDL 4.1.0, Tcl 8.6.14, Z3 4.8.12, OpenAI Python SDK 2.41.1, and PyYAML 6.0.1. |
| Rust formatting | `.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check"` | Passed. |
| Rust check | `.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo check --workspace"` | Passed. |
| Rust tests | `.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo test --workspace"` | Passed; current suite has only scaffold unit tests. |
| CLI positive smoke | `.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -p mico_cli -- check examples/stream_fifo.mico"` | Passed. |
| CLI IR/SV smoke | `.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico >/tmp/mico_ir.json && cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico >/tmp/mico_top.sv"` | Passed. |
| CLI negative smoke | `.\scripts\eda-docker.ps1 bash -lc 'cd rust_project && ! cargo run -p mico_cli -- check examples/invalid_width.mico'` | Passed; `InterfaceMismatch` is reported. |
| LLM provider config | `.\scripts\eda-docker.ps1 python3 .codex/skills/opencode-go-provider/scripts/opencode_go_smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only` | Passed; SDK path is OpenAI Python, profile is `smoke`, model is `deepseek-v4-flash`, base URL is `https://opencode.ai/zen/go/v1`. No paid request was made. |
| Paper build | `latexmk -g -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex` | Passed on Windows host LaTeX; output is 3 pages. Warnings are underfull boxes and IEEEtran camera-ready reminders, with no missing citations or missing figures. |

## Current State

### Product Definition

The repository has a coherent research scope: MICO is a module-interface-contract-oriented language for composing existing RTL/IP, not a general HDL. The docs correctly frame the LLM as a proposal engine and the compiler plus EDA tools as the authority. The novelty boundary against CPPL, Chisel, Amaranth, CIRCT/ESI, Anvil, and AutoSVA is documented.

Current risk: the docs are stronger than the implementation. The project should avoid claiming CCF-A artifact maturity until the Rust compiler, generated RTL, benchmark runner, and experiments catch up.

### Rust Compiler

The workspace is small and clean:

- `mico_ir` owns AST-like structs, diagnostics, and basic semantic checks.
- `mico_frontend` parses the v0 examples with a line-oriented parser.
- `mico_codegen` emits debug JSON-like text, SV wrapper comments, and SVA contract comments.
- `mico_cli` supports `check`, `dump-ir`, `emit-sv`, and `emit-sva`.

Implemented checks include unknown clock domains, unknown interfaces, unknown modules, unknown instances, unknown ports, direction mismatch, direct interface mismatch, direct clock-domain mismatch, unknown adapter, and adapter endpoint mismatch.

Major gaps:

- No lexer, token stream, column spans, source map, or robust recovery.
- No duplicate-name checks.
- `compose` domain validity is not checked.
- Interface field width and role compatibility are not deeply checked.
- Contract compatibility is represented but not checked.
- Adapter kind semantics are not validated beyond endpoint types/domains.
- JSON output is debug-oriented string construction, not a stable `serde_json` schema.
- SystemVerilog output records checked interface edges as comments but does not lower fields to primitive wires or instantiate adapters.
- CLI argument parsing and machine-readable diagnostics are not yet production surfaces.

### RTL/EDA Flow

The Docker EDA image and wrapper scripts are real and verified. This is a good reproducibility base.

Major gaps:

- No committed RTL leaf modules, testbenches, formal harnesses, filelists, or generated wrapper output fixtures.
- No project-level Yosys, Verilator, Icarus, cocotb, or SymbiYosys flow scripts.
- No batch Tcl flow in `scripts/` yet, aside from the host Vivado launcher and skill-local templates.
- No report directory schema or score aggregation from EDA outcomes.

### Benchmarks

`ModuleComposeBench` is specified and has three seed task directories: stream FIFO, CDC FIFO, and width adapter.

Major gaps:

- No executable benchmark runner.
- No task manifests with input modules, expected outputs, or negative cases.
- No direct RTL, SV-interface, MICO-source, MICO-JSON, or repair-loop baselines.
- Current scope is 3 seed tasks, far below the minimum 50 tasks stated in the scorecard.

### LLM Provider Workflow

The provider policy is aligned with the repository requirements: OpenAI-compatible Chat Completions through an SDK, cheap profiles first, high-cost escalation only after compiler and RTL gates pass, and local secrets ignored by git.

Major gaps:

- The smoke script currently lives under `.codex/skills`, not as a repo-owned workflow script.
- No Rust or Python benchmark harness consumes `config/llm-provider.local.yaml`.
- No prompt hash, model/profile, compiler result, RTL validation result, or cost/result logging exists.
- No structured repair loop is wired from compiler diagnostics to `prompts/repair_prompt_template.md`.

### Paper

The paper compiles and has a defensible framing: MICO targets compiler-checked module composition rather than direct free-form RTL generation.

Major gaps:

- It is currently a 3-page position manuscript, not a full CCF-A submission candidate.
- Evaluation section describes planned metrics but contains no experimental results.
- No artifact appendix/checklist, reproducibility package description, benchmark table, model table, or statistical analysis exists.
- Claims must remain conditional until benchmark and EDA evidence exists.

## Engineering Decisions For Next Iterations

- Parser path: start with a hand-written lexer and recursive-descent parser for v0. Add `logos`/`chumsky` only if the grammar or diagnostics complexity justifies the dependency.
- JSON path: make `serde`/`serde_json` the first nontrivial dependency when JSON IR and machine diagnostics become a durable interface.
- Adapter policy: v0 must reject implicit CDC, width, or protocol adaptation. Explicit adapters are allowed only when the adapter declaration matches endpoint interface/domain metadata and the adapter kind is known.
- CDC claim: treat CDC FIFO adapters as named black boxes until an RTL implementation, assertions, and flow collateral exist. Do not claim CDC correctness from type matching alone.
- Codegen path: generate conservative SV first: `default_nettype none`, explicit primitive wires, deterministic instance ordering, adapter instances, and traceability comments.
- LLM path: use cheap `smoke` profile for provider and prompt harness checks. Do not escalate to high-cost profiles until compiler acceptance and RTL validation gates pass on a seed set.
- Paper path: update claims only after implementation and experiments produce reproducible evidence.

## Milestone Plan

Each milestone should be an atomic commit or split into smaller atomic commits if the diff grows beyond a reviewable unit. Before each commit, run `git status --short`, `git diff`, relevant validation, and `git diff --check`. Do not stage ignored local config, generated PDFs, LaTeX temporaries, Rust `target`, build reports, logs, or secrets.

| Planned commit | Goal | Implementation scope | Acceptance gates |
|---|---|---|---|
| `docs: audit MICO architecture and milestones` | Establish baseline and execution plan. | This document and manifest linkage only. | Markdown review, `git diff --check`, no generated artifacts staged. |
| `feat(parser): complete MICO grammar and diagnostics` | Replace line parser with v0 grammar support and stable parser diagnostics. | Lexer/token model, recursive-descent parser, spans, recovery, grammar fixtures, parse tests. | Docker `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`, CLI parse/check fixtures. |
| `feat(ir): add contract-aware intermediate representation` | Separate parsed AST from typed IR and encode interface contracts/domains/widths. | Typed IDs or interned symbols, domain/port/interface metadata, contract placeholders, deterministic ordering. | Unit tests for IR construction and lowering from AST; JSON debug output remains deterministic. |
| `feat(checker): implement semantic and contract checks` | Make checker authoritative for v0 composition safety. | Duplicate declarations, compose domains, field compatibility, adapter kind legality, CDC rejection, width adapter policy, diagnostic hints. | Positive/negative fixtures assert diagnostic codes; invalid CDC/width/direction tasks rejected. |
| `feat(codegen): emit SystemVerilog adapters and wrappers` | Emit reviewable synthesizable SV for checked direct and explicit-adapter graphs. | Primitive wire lowering, instance port mapping, adapter instantiation, SVA skeleton expansion, golden tests. | `emit-sv` golden tests; Verilator/Yosys smoke on generated wrapper where leaf stubs exist. |
| `feat(cli): provide end-to-end MICO commands` | Make CLI scriptable for users and benchmark automation. | `clap` commands for `parse`, `check`, `build`, `emit`, `verify`, `report`; JSON diagnostics mode; stable exit codes. | CLI integration tests and smoke script update. |
| `test(eda): add reproducible RTL verification flows` | Turn generated SV into EDA-checked artifacts. | RTL stubs/adapters, filelists, Verilator lint, Yosys hierarchy/synth smoke, optional Vivado batch entry. | Docker Verilator/Yosys pass; no Vivado dependency for open-source smoke. |
| `test(bench): implement benchmark runner and scoring` | Make ModuleComposeBench executable. | Task manifests, runner, result schema writer, seed tasks for stream FIFO, CDC FIFO, width adapter, negative cases. | Runner executes seed tasks and emits scorecard JSON matching schema. |
| `feat(llm): integrate configured provider workflow` | Wire provider config to prompt and repair experiments without leaking secrets. | Repo-owned SDK script/harness, local YAML loading, profile selection, prompt hash/result logging, smoke request option. | `--validate-only` passes; authenticated smoke passes when local key is present; no key in output/diff. |
| `docs: document developer and reproduction workflow` | Make the artifact reproducible from a clean checkout. | README and docs for Docker, Rust, EDA, LLM, paper, Vivado exception, troubleshooting. | Fresh-command checklist validated in Docker and host LaTeX. |
| `paper: upgrade manuscript to CCF-A candidate` | Convert position paper into evidence-backed submission draft. | Method details, implementation, benchmark, experiments, result tables, related work, threats. | `latexmk -cd -pdf` passes; tables and citations complete; claims match results. |
| `paper: finalize reproducibility and submission checklist` | Prepare artifact and submission readiness package. | Artifact checklist, reproducibility appendix, final limitations, release checklist, camera-ready cleanup. | Paper compile passes; README/docs align with final artifact; working tree clean after commit. |

## Immediate Next Work

The next engineering commit should be `feat(parser): complete MICO grammar and diagnostics`, but it should be split if needed:

1. Add source spans and parser diagnostics without changing semantics.
2. Add a lexer/token model and recursive-descent parser for the documented v0 grammar.
3. Add parser fixtures and CLI behavior tests.

This sequence keeps user-facing diagnostics stable before deeper IR and checker changes.
