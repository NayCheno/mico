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

## Error handling

All diagnostics should support:

- source span;
- error code;
- human message;
- machine-readable repair hints;
- severity: error/warning/note;
- affected graph nodes.
