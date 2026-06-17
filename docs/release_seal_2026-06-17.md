# P8 Artifact Release Seal

Snapshot date: 2026-06-17.

This note records the tracked source-side release seal for the P8 artifact
objective. Generated evidence remains under ignored `build/` paths, and the
paper PDF remains ignored under `paper/main.pdf`. Exact run hashes are emitted
by `full-check.ps1 -WithLatex` and `make-release-bundle.ps1`; they are not
copied into this tracked note so that the final clean-tree bundle can be created
after this commit without another source edit.

## Required Clean-Tree Commands

Run from `main` after the P8 source commit:

```powershell
git status --short
git diff --check
.\scripts\full-check.ps1 -WithLatex
.\scripts\make-release-bundle.ps1
```

The Docker portion of `full-check.ps1` covers Rust format/check/tests, the
open-source EDA smoke, public-development, held-out, and supplemental realism
deterministic benchmarks, validate-only LLM records, aggregate/table
generation, schema validation, release claim JSON, LLM evidence hashes, and
paper-summary table generation. The PowerShell wrapper then uses host LaTeX to
build `paper/main.pdf` and updates the release manifest with the paper hash.

## Bundle Policy

The release bundle must include:

- source archive for the exact committed tree;
- anonymous artifact quickstart;
- schemas, prompts, benchmark manifests, and example provider config;
- deterministic, validate-only LLM, authenticated v4 LLM, aggregate, paper
  table, and optional Vivado evidence that has been sanitized and hash-recorded;
- `artifact_manifest.json` plus a ZIP SHA-256 sidecar.

The bundle must exclude:

- `config/*.local.yaml`;
- API keys or secret-like strings;
- raw provider payloads, response caches, and local logs;
- `build/`, `target/`, Vivado project outputs, and host-local configuration
  paths inside the staged ZIP tree.

`scripts/make-release-bundle.ps1` enforces these policy checks before writing
the ZIP and sidecar under `build/release/`.
