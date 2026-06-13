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

## Required response

Return a minimal JSON patch. Do not rewrite unrelated nodes.

```json
{
  "patch_id": "repair-N",
  "edits": []
}
```
