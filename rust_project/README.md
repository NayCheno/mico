# MICO Rust Project Skeleton

This is a minimal Rust workspace for MICO.

## Commands

```bash
cargo check
cargo test
cargo run -p mico_cli -- parse examples/stream_fifo.mico
cargo run -p mico_cli -- check examples/stream_fifo.mico
cargo run -p mico_cli -- check --format json examples/stream_fifo.mico
cargo run -p mico_cli -- build examples/stream_fifo.mico
cargo run -p mico_cli -- dump-ir examples/stream_fifo.mico
cargo run -p mico_cli -- emit-sv examples/stream_fifo.mico
cargo run -p mico_cli -- emit sv examples/stream_fifo.mico
cargo run -p mico_cli -- verify examples/stream_fifo.mico
cargo run -p mico_cli -- report examples/stream_fifo.mico
```

## Crates

- `mico_ir`: AST/IR, diagnostics, static checker.
- `mico_frontend`: minimal line-oriented parser for v0 examples.
- `mico_codegen`: SystemVerilog and JSON emitters.
- `mico_cli`: CLI wrapper.

## Notes

The parser is a small hand-written lexer plus recursive-descent parser for the documented v0 grammar. Keep it dependency-free until the grammar or diagnostics require a parser library.
