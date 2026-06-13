# Open-Source Rust Patterns

Distill mature Rust projects into practices that fit MICO. Do not copy their scale blindly.

## Source Projects And Transferable Lessons

| Project | Use the lesson | Avoid overfitting |
|---|---|---|
| rust-analyzer | Separate syntax, semantic model, diagnostics, and assists. Use fixture-heavy tests for language behavior. Keep source spans and stable diagnostic codes. | Do not introduce query engines, salsa-style incremental state, or large IDE abstractions before MICO has enough repeated queries. |
| Cargo | Keep workspace ownership clear. Validate manifests/configuration early. Treat CLI behavior and exit codes as compatibility surface. | Do not grow plugin/config systems until users need them. |
| ripgrep | Favor small focused crates, fast deterministic paths, and direct error messages. Keep the CLI boring and scriptable. | Do not add parallelism or streaming complexity to tiny inputs. |
| clippy | Model checks as named lints/diagnostics with codes, levels, explanations, and suggestions. Add regression tests for every diagnostic. | Do not make heuristics noisy; MICO checker errors should remain semantic and authoritative. |
| rustfmt | Use golden outputs for deterministic formatting/emission. Keep formatting stable across unrelated changes. | Do not add formatting knobs before the emitted target stabilizes. |
| serde | Make data models explicit and serializable. Keep schema evolution in mind for JSON IR and machine diagnostics. | Do not expose internal debug shapes as durable external schema. |
| Tokio | Feature gates and layered crates can control compile time and dependency cost. | Avoid async in MICO unless IO-bound workflows appear; compiler passes should stay synchronous. |

## API Shape

- Use newtypes for domain-specific identifiers (`Ident`, `Endpoint`, future `NodeId`) instead of plain `String` at boundaries.
- Prefer enums for closed concepts (`Role`, `PortDir`, `Severity`) and structured structs for graph edges.
- Keep constructors and parsers small; return `Result<T, Diagnostic>` or `Result<T, Vec<Diagnostic>>` when user input is involved.
- Expose read-only slices or iterators before exposing mutable internals.
- Make invalid states hard to construct in later milestones: parsed AST may be loose, typed IR should be stricter.

## Diagnostics

- Every user-facing error should have a stable code, severity, message, and at least one repair hint when a plausible repair exists.
- Parser diagnostics should eventually include source span, expected token/kind, and recovery point.
- Checker diagnostics should include affected graph nodes: instance, module, port, interface, adapter, or connection.
- Machine-readable diagnostics should not depend on debug formatting.

## Dependency Policy

Good candidates when the project outgrows the scaffold:

- `serde`, `serde_json`: JSON IR and machine-readable diagnostics.
- `clap`: CLI once commands/options expand beyond the current manual parser.
- `thiserror`: internal error enums when ordinary `Result` plumbing becomes repetitive.
- `miette` or `ariadne`: rich diagnostics after source spans exist.
- `logos` plus `chumsky`, `pest`, or hand-written recursive descent: parser upgrade after the v0 grammar freezes.
- `insta`: snapshot/golden tests for IR and generated output, if the team accepts snapshot review workflow.
- `indexmap`: deterministic insertion-order maps when name lookup needs map semantics plus stable output.

Avoid new dependencies for one-off string splitting, tiny argument parsing, or speculative future integrations.

## Testing Pattern

- Unit tests: scalar parsing, endpoint parsing, name resolution, individual checks.
- Fixture tests: `.mico` source files for valid and invalid programs.
- Golden tests: JSON IR, SystemVerilog, SVA, and diagnostics.
- CLI smoke tests: end-to-end command behavior and exit codes.
- Regression tests: one test per reported bug or paper benchmark failure.

Name tests by behavior, not implementation detail: `rejects_cross_domain_direct_connect`, `emits_adapter_comment_for_cdc_fifo`.

## Codegen Pattern

- Emit deterministic text with a small writer abstraction once concatenation gets repetitive.
- Keep generated output conservative and readable before optimizing.
- Avoid mixing validation with emission. Codegen should assume checked IR and fail only on internal invariant violations.
- Treat generated output as user-facing API; update golden tests when changing it intentionally.
