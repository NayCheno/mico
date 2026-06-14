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
