# Architecture

## Compiler pipeline

```text
.mico source / JSON AST
  -> lexer/parser
  -> AST
  -> name resolution
  -> typed MICO IR
  -> semantic checks
  -> adapter planning
  -> lowering
       -> SystemVerilog wrapper/top
       -> SVA contract file
       -> JSON design graph
       -> future CIRCT HW/ESI/Verif/LTL
  -> reports
       -> human-readable diagnostics
       -> machine-readable diagnostics for LLM repair
```

## Rust crate layout

```text
mico_ir        Core data structures, typed IR, diagnostics, checker.
mico_frontend  Source parser and recovery.
mico_codegen   SystemVerilog and JSON emitters.
mico_cli       Command line tool.
```

## Parsed design vs typed IR

The parser produces a loose `Design` that mirrors source declarations and keeps user input fallible. The checker owns semantic validity. After `check_design` succeeds, `build_typed_ir` lowers `Design` into `TypedDesign`, which resolves endpoint metadata and records:

- clock/reset metadata, including inferred reset polarity;
- interface fields with role, scalar type, and known bit width;
- inferred protocol metadata such as ready/valid payload, valid, and ready fields;
- adapter kind and adapter contract attributes;
- compose instances and resolved connection endpoints with module, port direction, interface, and domain metadata;
- source interface, sink interface, and adapter contracts associated with each connection.

Backends and benchmark/report flows should move toward `TypedDesign` instead of consuming parser-shaped `Design` directly.

## CIRCT lowering plan

| MICO concept | CIRCT target |
|---|---|
| `extern module` | `hw.module.extern` |
| `compose` | `hw.module` with `hw.instance` connections |
| Stream interface | `esi.channel<T>` or SV interface wrapper |
| FIFO adapter | `esi.fifo` or generated RTL blackbox |
| Pipeline adapter | `esi.buffer` or generated registers |
| Contract | `verif.assert`, `verif.assume`, LTL dialect, or SVA emission |

## Code generation policy

- v0 emits conservative SystemVerilog without relying on advanced SV interface features by default.
- v1 adds optional SystemVerilog interface emission.
- v2 adds CIRCT emission.

### SystemVerilog v0 naming convention

The v0 SystemVerilog emitter flattens each interface field into primitive module ports named `<port>_<field>`, such as `tx_payload`, `tx_valid`, and `tx_ready`. Direct connection wires use `<src_inst>_<src_port>__<dst_inst>_<dst_port>_<field>`. Explicit adapters are emitted as named instances with `in_<field>` and `out_<field>` ports. Same-domain adapters use `clk`/`rst`; CDC adapters use `src_clk`/`src_rst` and `dst_clk`/`dst_rst`.

## Error handling

All diagnostics should support:

- source span;
- error code;
- human message;
- machine-readable repair hints;
- severity: error/warning/note;
- affected graph nodes.
