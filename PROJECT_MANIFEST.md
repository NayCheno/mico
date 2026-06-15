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
| `scripts/run_llm_bench.py` | Batch ModuleComposeBench LLM runner with five baselines, caching, compiler/EDA scoring, and JSON-AST repair-loop plumbing. |
| `benchmarks/aggregate_results.py` | Aggregates deterministic and optional LLM benchmark JSON into CSV and TeX table snippets under ignored `build/`. |
| `schemas/mico_ast.schema.json` | Source-level MICO JSON AST schema used by `dump-ast-json`, `check-json`, and JSON AST baselines. |
| `schemas/mico_repair_patch.schema.json` | Minimal compiler-feedback repair patch schema for instance, connection, adapter, endpoint, and contract-attribute edits. |
| `schemas/traceability.schema.json` | Generated traceability report schema for source references, SV signals, leaf ports, adapter boundaries, and SVA contract IDs. |
| `schemas/llm_run.schema.json` | Sanitized LLM validate/smoke run record schema with prompt hash, profile, repair, compiler, EDA, usage, and cost metadata. |
| `schemas/llm_bench.schema.json` | Sanitized LLM benchmark batch result schema for validate-only, offline-fixture, and execute modes. |
| `schemas/aggregate_results.schema.json` | Aggregate benchmark result schema covering deterministic metrics, QoR rows, ablations, LLM summaries, repair turns, cost/tokens, paired comparisons, and failure taxonomy. |
| `prompts/llm_bench_baselines.yaml` | Deterministic baseline-specific response instructions for Direct Verilog, SV interface, MICO source, MICO JSON AST, and JSON AST + repair. |
| `docs/06_evaluation_plan.md` | Experiments, baselines, metrics, ablations. |
| `docs/08_roadmap.md` | Month-by-month roadmap. |
| `docs/12_docker_eda_environment.md` | Persistent Ubuntu 24.04 Docker environment for Rust and open-source RTL/EDA validation, with Windows-host Vivado and LaTeX exceptions. |
| `docs/13_architecture_audit.md` | Current architecture audit, implemented state, claim boundary, gaps, and next priority order. |
| `docs/14_reproduction_workflow.md` | Clean-checkout reproduction workflow covering Docker Rust/EDA, benchmarks, LLM provider smoke, host Vivado, and host LaTeX. |
| `docs/15_case_studies.md` | Dedicated subsystem case-study inventory, collateral paths, and reproduction commands. |
| `docs/24_llm_matrix_v3.md` | Current manifest-bound authenticated LLM matrix summary and Branch A decision. |
| `docs/25_realism_supplement.md` | Deterministic supplemental subsystem realism manifest, validation results, hashes, and LLM-claim boundary. |
| `docs/current_status.md` | Short status page summarizing implemented features, non-claims, deterministic benchmark boundary, and validation commands. |
| `docs/release_claim_table.md` | Single source for current numeric claim values, evidence artifact paths, schema versions, hash sources, and paper locations. |
| `docs/diagnostics.md` | Stable diagnostics envelope, diagnostic code list, JSON AST diagnostic behavior, and CLI JSON behavior. |
| `rust_project/` | Rust workspace for MICO parser, checker, typed IR, codegen, and CLI. |
| `docker/eda/` | Dockerfile, Compose file, and tool verification script for Rust/RTL/EDA development. |
| `benchmarks/` | 62-task public-development ModuleComposeBench suite plus the 20-task held-out split, supplemental realism manifest, runner, result aggregator, and scoring schema. |
| `rtl/case_studies/` | Dedicated subsystem RTL collateral for streaming, width-bridge, and register/status case-study tasks. |
| `benchmarks/formal/` | SymbiYosys harness monitors for selected seed formal checks. |
| `benchmarks/qor/` | Hand-written reference wrappers for structural Yosys QoR comparison. |
| `benchmarks/sim/` | Icarus/VVP SystemVerilog testbenches for supported positive benchmark tasks. |
| `prompts/` | Prompt templates and JSON schema for LLM output. |
| `source/` | Uploaded and edited source reports. |
