# MICO Diagnostics v0

MICO diagnostics are stable user-facing output. Text output is intended for people; `--format json` output is intended for benchmark runners and LLM repair prompts.

## JSON envelope

Commands that report diagnostics use this envelope:

```json
{
  "schema_version": "mico.diagnostics.v0",
  "ok": false,
  "phase": "check",
  "diagnostics": []
}
```

The schema is `schemas/diagnostics.schema.json`. Parser diagnostics include byte spans plus line and column. Checker diagnostics include stable codes, severity, messages, labels, affected graph nodes, repair hints, and a `repair_action` enum. For `.mico` input, checker diagnostics attach source-map spans when the parser can map the related declaration, endpoint, field, port, adapter, or compose member. For JSON AST input, spans may be `null`; the `nodes` array is the stable fallback for LLM repair and benchmark classification.

## Diagnostic shape

Each diagnostic has:

- `span`: a byte/line/column span when available, otherwise `null`.
- `labels`: primary or secondary labels; semantic labels may carry `span: null`
  when the input has no source-byte map.
- `nodes`: affected graph nodes such as `interface`, `module`, `instance`, `endpoint`, `adapter`, `clock_domain`, or `port`.
- `hints`: human-readable repair hints.
- `repair_action`: a stable enum such as `use_adapter`, `reverse_connection`, `add_declaration`, `fix_endpoint`, `fix_width`, or `add_contract`.

## Commands

```bash
mico check --format json examples/stream_fifo.mico
mico build --format json examples/stream_fifo.mico
mico report --format json examples/invalid_width.mico
mico dump-ir examples/stream_fifo.mico
mico dump-ast-json examples/stream_fifo.mico
mico check-json --format json build/ast/stream_fifo.json
mico dump-json-ir build/ast/stream_fifo.json
mico repair-json --dry-run --format json build/ast/broken.json build/patches/repair.json
mico verify --eda --format json --artifact-dir build/mico-verify/stream_fifo examples/stream_fifo.mico
```

`dump-ir` always emits JSON. Its schema is `schemas/mico_ir.schema.json`.
`dump-ast-json` emits source-level JSON AST with
`schema_version = mico.ast.v0`; its schema is
`schemas/mico_ast.schema.json`. JSON AST schema or deserialization failures are
reported as `JsonSchemaError` diagnostics instead of panics.

Golden JSON fixtures for valid and invalid diagnostic outputs live under
`rust_project/crates/mico_cli/tests/fixtures/diagnostics/` and are checked by
the CLI unit tests.

## Diagnostic codes

| Code | Phase | Meaning | Repair hint |
|---|---|---|---|
| `IoError` | read | The input file could not be read. | Check the path and permissions. |
| `UnexpectedToken` | parse | A token cannot start the expected declaration or member. | Replace it with a valid v0 grammar construct. |
| `ExpectedKeyword` | parse | A required keyword was missing. | Insert the keyword shown in the message. |
| `ExpectedIdentifier` | parse | An identifier was expected. | Use `[A-Za-z_][A-Za-z0-9_]*`. |
| `ExpectedToken` | parse | A required punctuation token was missing. | Insert the punctuation shown in the message. |
| `UnexpectedEof` | parse | The file ended before the declaration was complete. | Close the current declaration or block. |
| `JsonSchemaError` | parse | A MICO JSON AST document failed schema/version/kind validation. | Fix the JSON AST to match `schemas/mico_ast.schema.json`. |
| `RepairPatchError` | parse | A JSON AST repair patch failed schema validation or could not be applied to the requested AST. | Fix the patch to match `schemas/mico_repair_patch.schema.json` and the target AST. |
| `VerifyEdaError` | verify | `verify --eda` could not locate RTL collateral, create artifacts, write emitted SV/SVA, or invoke an open-source EDA tool. | Run inside the repository Docker EDA image, check `--artifact-dir`, and inspect the emitted stdout/stderr artifacts. |
| `DuplicateDeclaration` | check | A top-level clock domain, interface, module, adapter, or compose name is duplicated. | Rename one declaration or merge the duplicate. |
| `DuplicateField` | check | An interface declares the same field more than once. | Rename or remove the duplicate field. |
| `DuplicatePort` | check | A module declares the same port more than once. | Rename or remove the duplicate port. |
| `DuplicateInstance` | check | A compose block declares the same instance more than once. | Rename or remove the duplicate instance. |
| `UnknownClockDomain` | check | A declaration references a missing clock domain. | Add the clock domain or fix the domain name. |
| `UnknownInterface` | check | A module or adapter references a missing interface. | Add the interface or fix the interface name. |
| `UnknownModule` | check/build | A compose instance references a missing module. | Add the module declaration or fix the instance module. |
| `UnknownInstance` | check/build | A connection endpoint references a missing instance. | Add the instance or fix the endpoint. |
| `UnknownPort` | check/build | A connection endpoint references a missing module port. | Add the port or fix the endpoint. |
| `DirectionMismatch` | check | A source endpoint is not `out` or a sink endpoint is not `in`. | Reverse the connection or fix port directions. |
| `InterfaceMismatch` | check | A direct connection uses different interface types. | Use matching interfaces or declare an explicit adapter. |
| `ClockDomainMismatch` | check | A direct connection crosses clock/reset domains. | Use an explicit CDC adapter such as an async FIFO. |
| `UnknownAdapter` | check | A connection references a missing adapter. | Declare the adapter or fix the adapter name. |
| `UnknownAdapterKind` | check | An adapter kind is not in the v0 library and is not `custom`. | Use `cdc_fifo`, `width_adapter`, `skid_buffer`, `pipeline`, or `custom`. |
| `AdapterMismatch` | check | An adapter declaration does not match the connected endpoint interfaces or domains. | Fix the adapter `from`/`to` declaration or the connection. |
| `ProtocolMismatch` | check | An adapter kind requires protocol fields that are missing. | Use a ready/valid interface or a custom adapter with explicit contracts. |
| `WidthMismatch` | check | An adapter kind requires compatible known widths and the fields do not satisfy that rule. | Fix payload widths or use an appropriate adapter. |
| `ContractViolation` | check | A contracted sink is connected through an adapter whose known guarantees do not cover the v0 sink requirements, or whose guarantee is unknown/invalid for the adapter kind. | Add a known guarantee such as `preserves_ready_valid` or use compatible interfaces. |
| `InternalIrError` | build | The checked design could not be lowered to typed IR. | Treat this as a compiler bug unless prior diagnostics explain it. |
