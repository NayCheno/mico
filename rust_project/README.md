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
cargo run -p mico_cli -- emit trace examples/stream_fifo.mico
cargo run -p mico_cli -- verify examples/stream_fifo.mico
cargo run -p mico_cli -- verify --eda --json --artifact-dir ../build/mico-verify/stream_fifo_cli --schema-path ../schemas examples/stream_fifo.mico
cargo run -p mico_cli -- report examples/stream_fifo.mico
```

## Crates

- `mico_ir`: AST/IR, diagnostics, static checker.
- `mico_frontend`: hand-written lexer and recursive-descent parser for v0 examples.
- `mico_codegen`: SystemVerilog and JSON emitters.
- `mico_cli`: CLI wrapper.

## Notes

The parser is a small hand-written lexer plus recursive-descent parser for the documented v0 grammar. Run Rust and open-source EDA validation through the repository Docker wrapper; `verify --eda` assumes Verilator, Icarus, and Yosys are available in that environment.
