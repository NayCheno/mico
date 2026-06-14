# Repair Prompt Template

You generated a MICO composition graph that failed compiler checks.

## Original task

{{task_description}}

## Module inventory

{{module_inventory}}

## Interface library

{{interface_library}}

## Current JSON AST

{{current_ast}}

## Compiler diagnostics

{{diagnostics}}

## Required repair patch skeleton

{{repair_patch_skeleton}}

## Operation selection rules

- Use only declarations already present in the current JSON AST or visible in
  the module/interface inventory.
- Do not invent module names. If an instance has an unknown module, use
  `replace_instance` with a module from the inventory.
- Adapters are not compose instances. To use an adapter, set
  `connection.adapter` to the adapter name string on a connection between the
  original source and sink endpoints.
- Use `replace_connection` only when the exact `from` and `to` endpoint pair
  already exists in the current JSON AST.
- Use `add_connection` or `remove_connection` only when a connection must be
  inserted or deleted rather than edited in place.
- For width or CDC adapter diagnostics, prefer `replace_connection` that keeps
  the original source and sink endpoints and changes only `connection.adapter`.
- For wrong or missing contract attributes on an existing adapter, use
  `update_contract_attribute`.
- Keep every endpoint as `{"instance": "...", "port": "..."}`. Never return
  dotted endpoint strings.

## Valid operation shapes

Replace an instance module:

```json
{
  "op": "replace_instance",
  "compose": "Top",
  "name": "u_bad",
  "instance": {"name": "u_bad", "module": "KnownModule"}
}
```

Replace an existing connection and add an adapter name:

```json
{
  "op": "replace_connection",
  "compose": "Top",
  "from": {"instance": "src", "port": "out"},
  "to": {"instance": "dst", "port": "in"},
  "connection": {
    "from": {"instance": "src", "port": "out"},
    "to": {"instance": "dst", "port": "in"},
    "adapter": "adapter_name"
  }
}
```

Update an existing adapter contract:

```json
{
  "op": "update_contract_attribute",
  "adapter": "adapter_name",
  "value": "stable(out.valid) until out.ready"
}
```

Add an adapter declaration only if the inventory shows the exact interface,
domain, kind, and contract needed:

```json
{
  "op": "add_adapter",
  "adapter": {
    "name": "adapter_name",
    "from_interface": "rv32",
    "from_domain": "clk",
    "to_interface": "rv64",
    "to_domain": "clk",
    "kind": "width",
    "attributes": [{"name": "contract", "value": "stable(out.valid) until out.ready"}]
  }
}
```

## Required response

Return exactly one valid `mico.repair_patch.v0` JSON object. Do not use
markdown, comments, code fences, prose, `patch_id`, `reason`, or `edits`. Use
only `schema_version`, `kind`, and `operations`. Keep the patch minimal and do
not rewrite unrelated nodes. Do not return a complete MICO AST. Use endpoint
objects with `instance` and `port`; do not use dotted endpoint strings.

```json
{
  "schema_version": "mico.repair_patch.v0",
  "kind": "repair_patch",
  "operations": [
    {
      "op": "replace_connection",
      "compose": "Top",
      "from": {"instance": "<old_source>", "port": "<old_output>"},
      "to": {"instance": "<old_sink>", "port": "<old_input>"},
      "connection": {
        "from": {"instance": "<new_source>", "port": "<new_output>"},
        "to": {"instance": "<new_sink>", "port": "<new_input>"},
        "adapter": null
      }
    }
  ]
}
```
