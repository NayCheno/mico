# MICO Rust Workflow

Use this reference when implementing or reviewing concrete Rust changes in this repository.

## Repository Contract

MICO is a module-composition language for existing RTL/IP. The LLM proposes structures; the Rust compiler is the authority for parsing, name resolution, interface matching, direction checking, clock/reset domain checking, adapter legality, contracts, and lowering.

Primary specs:

- `docs/01_language_spec_v0.md`: syntax, static errors, lowering contract.
- `docs/02_architecture.md`: pipeline and crate ownership.
- `docs/08_roadmap.md`: phase ordering.
- `rust_project/examples/*.mico`: executable examples.

## Crate Ownership

`mico_ir`

- Owns AST/IR structs, typed concepts, diagnostics, and semantic checking.
- Should not read files, parse concrete syntax, or print CLI output.
- Good changes: new diagnostic fields, duplicate-name checks, interface field compatibility, adapter validation, contract placeholders.

`mico_frontend`

- Owns `.mico` parsing and recovery.
- May produce a loose parsed design first, but should preserve line/column spans as soon as diagnostics need them.
- Good changes: lexer/parser upgrade, better parse errors, explicit grammar tests.

`mico_codegen`

- Owns JSON IR, SystemVerilog, SVA, and future CIRCT-facing emission.
- Should operate on checked IR.
- Good changes: deterministic JSON via `serde_json`, field-to-wire lowering, golden tests.

`mico_cli`

- Owns command dispatch, file IO, stderr/stdout behavior, and exit codes.
- Good changes: `clap` migration, machine-readable diagnostic mode, subcommands for benchmarks or repair prompts.

## Common Implementation Recipes

### Add A Semantic Check

1. Add or refine IR fields in `mico_ir` only if the check needs new structured data.
2. Implement the check in `check_design` or split `check_design` into focused passes when it becomes hard to read.
3. Emit a stable diagnostic code matching `docs/01_language_spec_v0.md` or update the docs when adding a new code.
4. Add a valid fixture and an invalid fixture when the behavior crosses parser/checker boundaries.
5. Add a unit test that asserts the diagnostic code, not just message text.

### Add Syntax

1. Update `docs/01_language_spec_v0.md` first if the syntax is part of the language, unless the user asked for a prototype.
2. Extend parser tests with minimal positive and negative cases.
3. Keep parse errors recoverable enough to report multiple independent mistakes in one file.
4. Do not encode semantic validation in `mico_frontend` unless it is required to parse.

### Add Code Generation

1. Decide whether the output is debug-only or durable user surface.
2. Preserve stable ordering from the source or explicitly sort by identifier.
3. Add golden tests for nontrivial emitted text.
4. Keep comments in generated SystemVerilog useful for traceability: source endpoint, adapter name, contract name.

### Add A Dependency

1. Check whether the dependency belongs at workspace level or crate level.
2. Disable default features unless they are needed.
3. Explain why local code would be worse.
4. Run `cargo check --workspace` and relevant tests.

## Environment Boundary

Run Rust and open-source EDA validation in the Ubuntu 24.04 Docker image by default:

```bash
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo test --workspace"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo test --workspace"
```

Use Windows-host Vivado only for Vivado-specific FPGA/Xilinx flows. Do not use host Rust or host open-source EDA tools as the default validation surface.

The image and Cargo caches are persistent:

- image tag: `mico-eda:ubuntu24.04`
- Cargo registry volume: `mico-cargo-registry`
- Cargo git volume: `mico-cargo-git`
- force rebuild: set `MICO_EDA_REBUILD=1`

The Rust workspace targets current stable Rust 1.96.0 and Rust 2024 edition. Keep `rust_project/rust-toolchain.toml`, `rust_project/Cargo.toml`, and Docker validation aligned.

## Current Scaffold Gaps To Notice

- Parser is line-oriented and lacks spans/recovery beyond line numbers.
- `emit_json_ir` is JSON-like debug output and should move to `serde_json` before becoming a machine contract.
- Semantic checking does not yet cover duplicate declarations, compose domain validity, field width compatibility, contract compatibility, or adapter kind semantics.
- SystemVerilog emission records checked connections as comments but does not lower interface fields to wires.
- CLI uses manual argument parsing; this is acceptable for the scaffold but should move to `clap` once options grow.

## Validation Ladder

Start small, then expand:

```bash
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo fmt --check"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo check --workspace"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo test --workspace"
```

For user-visible behavior:

```bash
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- check examples/stream_fifo.mico"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico"
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico"
```

For invalid-program behavior:

```bash
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo run -p mico_cli -- check examples/invalid_width.mico"
```
