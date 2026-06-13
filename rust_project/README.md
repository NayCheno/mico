# MICO Rust Project Skeleton

This is a minimal Rust workspace for MICO.

## Commands

```bash
cargo check
cargo test
cargo run -p mico_cli -- check examples/stream_fifo.mico
cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico
cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico
```

## Crates

- `mico_ir`: AST/IR, diagnostics, static checker.
- `mico_frontend`: minimal line-oriented parser for v0 examples.
- `mico_codegen`: SystemVerilog and JSON emitters.
- `mico_cli`: CLI wrapper.

## Notes

The parser is intentionally simple. Replace it with `chumsky`, `logos`, `lalrpop`, `pest`, or a hand-written recursive descent parser when the grammar stabilizes.
