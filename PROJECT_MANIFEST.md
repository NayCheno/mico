# Project Manifest

| Path | Purpose |
|---|---|
| `paper/main.tex` | Authoritative IEEE-style LaTeX entrypoint for the MICO paper; compile with Windows host LaTeX. |
| `paper/sections/` | Split TeX section files included by `paper/main.tex`. |
| `paper/related_work.bib` | BibTeX references for HDL, CIRCT, LLM-for-RTL, verification. |
| `paper/*.md` | Historical drafts and notes; convert substantive content into `paper/main.tex`. |
| `docs/01_language_spec_v0.md` | Minimal MICO syntax and static semantics. |
| `docs/02_architecture.md` | Compiler architecture and lowering plan. |
| `docs/03_llm_protocol.md` | How LLMs interact with MICO without being trusted for correctness. |
| `config/llm-provider.example.yaml` | OpenCode Go provider template with OpenAI-compatible base URL, model profiles, and key handling policy. |
| `scripts/llm-provider-smoke.py` | SDK-backed provider validation and smoke request script; writes sanitized outputs under ignored `build/llm/`. |
| `schemas/mico_ast.schema.json` | Source-level MICO JSON AST schema used by `dump-ast-json`, `check-json`, and JSON AST baselines. |
| `schemas/mico_repair_patch.schema.json` | Minimal compiler-feedback repair patch schema for instance, connection, adapter, endpoint, and contract-attribute edits. |
| `schemas/traceability.schema.json` | Generated traceability report schema for source references, SV signals, leaf ports, adapter boundaries, and SVA contract IDs. |
| `schemas/llm_run.schema.json` | Sanitized LLM validate/smoke run record schema with prompt hash, profile, repair, compiler, EDA, usage, and cost metadata. |
| `docs/06_evaluation_plan.md` | Experiments, baselines, metrics, ablations. |
| `docs/08_roadmap.md` | Month-by-month roadmap. |
| `docs/12_docker_eda_environment.md` | Persistent Ubuntu 24.04 Docker environment for Rust and open-source RTL/EDA validation, with Windows-host Vivado and LaTeX exceptions. |
| `docs/13_architecture_audit.md` | Current architecture audit, implemented state, claim boundary, gaps, and next priority order. |
| `docs/14_reproduction_workflow.md` | Clean-checkout reproduction workflow covering Docker Rust/EDA, benchmarks, LLM provider smoke, host Vivado, and host LaTeX. |
| `docs/current_status.md` | Short status page summarizing implemented features, non-claims, seed benchmark boundary, and validation commands. |
| `docs/diagnostics.md` | Stable diagnostics envelope, diagnostic code list, JSON AST diagnostic behavior, and CLI JSON behavior. |
| `rust_project/` | Rust workspace for MICO parser, checker, typed IR, codegen, and CLI. |
| `docker/eda/` | Dockerfile, Compose file, and tool verification script for Rust/RTL/EDA development. |
| `benchmarks/` | Twelve-task ModuleComposeBench seed suite, manifest, runner, and scoring schema. |
| `benchmarks/formal/` | SymbiYosys harness monitors for selected seed formal checks. |
| `benchmarks/qor/` | Hand-written reference wrappers for structural Yosys QoR comparison. |
| `benchmarks/sim/` | Icarus/VVP SystemVerilog testbenches for positive seed simulation. |
| `prompts/` | Prompt templates and JSON schema for LLM output. |
| `source/` | Uploaded and edited source reports. |
