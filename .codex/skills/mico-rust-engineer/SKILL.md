---
name: mico-rust-engineer
description: Apply mature open-source Rust project practices to evolve the MICO Rust compiler workspace. Use when modifying rust_project crates, designing IR/parser/checker/codegen/CLI behavior, adding diagnostics or tests, choosing Rust dependencies, implementing MICO roadmap milestones, or reviewing Rust compiler architecture in this repository.
---

# Mico Rust Engineer

## Overview

Use this skill to turn MICO's research scaffold into a disciplined Rust compiler project. Treat `docs/01_language_spec_v0.md`, `docs/02_architecture.md`, and `docs/08_roadmap.md` as the product spec, and apply practices distilled from mature Rust projects such as rust-analyzer, Cargo, ripgrep, clippy, rustfmt, serde, and Tokio only where they fit this repository's scale.

## First Pass

1. Read the task, then inspect only the relevant local context before editing:
   - language or semantics: `docs/01_language_spec_v0.md`, `rust_project/crates/mico_ir`
   - pipeline or crate ownership: `docs/02_architecture.md`, `rust_project/README.md`
   - milestone scope: `docs/08_roadmap.md`
   - CLI/user behavior: `rust_project/crates/mico_cli`, `rust_project/examples`
2. Map the change to the narrowest crate boundary:
   - `mico_ir`: core data model, typed IR, diagnostics, semantic checks
   - `mico_frontend`: parsing, recovery, source spans, concrete syntax
   - `mico_codegen`: deterministic JSON/SystemVerilog/SVA emission
   - `mico_cli`: argument parsing, exit codes, file IO, human output
3. Prefer an IR-first change. Update data structures and checker behavior before parser sugar or code generation.
4. Add tests close to the behavior. Use unit tests for pure logic, fixture/golden tests for parsing and emitters, and CLI smoke tests for end-to-end flows.
5. Run validation in the Ubuntu 24.04 Docker EDA image unless the user asks for analysis only.

## Environment Rule

- Use `docker/eda/Dockerfile` for Rust compilation, tests, and open-source RTL/EDA checks.
- Use `scripts/eda-docker.ps1` on Windows or `scripts/eda-docker.sh` on Linux/WSL to run commands in the container.
- The Docker image `mico-eda:ubuntu24.04` is persistent. The wrapper scripts reuse it and only rebuild when it is missing or `MICO_EDA_REBUILD=1` is set.
- The Docker image defaults to Tsinghua TUNA for apt/PyPI and RsProxy for rustup/Cargo crates.io; keep mirror changes in `docker/eda/Dockerfile` and `docs/12_docker_eda_environment.md`.
- Cargo registry/git caches are persistent Docker named volumes: `mico-cargo-registry` and `mico-cargo-git`.
- Do not rely on Windows-host Rust, Yosys, Verilator, Icarus, GHDL, Tcl, or Python EDA packages for validation.
- The only host-tool exception is Vivado, and only for Vivado-specific FPGA/Xilinx flows.
- Paper LaTeX work is another host exception and belongs to `$paper-latex-writer`, not this Rust skill.
- Read `docs/12_docker_eda_environment.md` when tool availability or command routing is relevant.

## Engineering Rules

- Keep user input fallible. Parser, checker, and CLI paths must return diagnostics or controlled exits, not panic.
- Make diagnostics stable: code, severity, human message, and actionable hint. Add source spans when touching parsing or checker internals.
- Preserve deterministic output ordering. Use source order or an explicit deterministic map when generating JSON/SV/SVA.
- Keep dependency additions conservative. Add a crate only when it removes real complexity; prefer mature, common crates with narrow feature sets.
- Use typed domain concepts instead of strings: identifiers, domains, roles, directions, endpoints, adapter kinds, and diagnostic codes should remain explicit Rust types.
- Avoid introducing async, global state, macros, or generic abstraction unless the current workflow needs them.
- Maintain `unsafe_code = "forbid"` and keep `unsafe` out of this research compiler unless the user explicitly changes the safety policy.

## Reference Routing

- Read `references/mico-workflow.md` when implementing or reviewing a concrete MICO compiler milestone.
- Read `references/open-source-rust-patterns.md` when making an architecture, dependency, testing, diagnostics, or API design decision.
- Use `$opencode-go-provider` when running or modifying LLM provider config, prompt smoke tests, model selection, or MICO benchmark evaluation.
- Read `scripts/mico_rust_audit.rs` only when modifying the audit script or interpreting its output.

## Validation

Use the lightest validation that covers the change, then expand when touching shared behavior.

```bash
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo fmt --check"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo check --workspace"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo test --workspace"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- check examples/stream_fifo.mico"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico"
```

From the repository root, optionally run the bundled Rust audit:

```bash
./scripts/eda-docker.sh bash -lc "rustc .codex/skills/mico-rust-engineer/scripts/mico_rust_audit.rs -o rust_project/target/mico_rust_audit && ./rust_project/target/mico_rust_audit ."
```

On Windows PowerShell, use:

```powershell
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo test --workspace"
```

## Output Discipline

- Report which crate changed and why.
- Report validation commands actually run.
- If a dependency is added, explain what complexity it removed and which alternatives were rejected.
- If the implementation intentionally leaves a research placeholder, name the placeholder and the next concrete check that should replace it.
