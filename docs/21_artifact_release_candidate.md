# DAC Artifact Release Candidate

Snapshot date: 2026-06-14.

This note records the current DAC artifact release-candidate policy. Generated
evidence remains under ignored `build/` paths and is intended for an external
archive, not for source control.

## Gate

Run the release gate from a clean tree after the final source commit:

```powershell
git status --short
git diff --check
.\scripts\full-check.ps1 -WithLatex
.\scripts\make-release-bundle.ps1
```

The Docker portion of `full-check` now covers Rust format/check/tests, open
source EDA smoke, the 62-task public-development benchmark, the 12-task
held-out benchmark, validate-only LLM provider and batch records, aggregate
table generation, schema validation, and generated-output policy checks. The
PowerShell wrapper then builds the paper with host LaTeX and updates the release
manifest with the final `paper/main.pdf` SHA-256 hash.

## Manifest Contents

`build/release/full_check_manifest.json` records:

- source commit and branch;
- Docker/container and tool versions for Rust, Python, Verilator, Icarus,
  Yosys, SymbiYosys, and Z3;
- selected redacted LLM provider/profile metadata;
- prompt SHA-256 hashes;
- public-development and held-out benchmark manifest SHA-256 hashes;
- deterministic, held-out, validate-only LLM, and aggregate result JSON hashes;
- optional Vivado subset summary hashes when host Vivado evidence exists;
- final paper PDF SHA-256 hash when `-WithLatex` is used.

The manifest is generated output. Do not commit it.

## Bundle Contents

`scripts/make-release-bundle.ps1` writes a ZIP plus `.sha256` sidecar under
`build/release/`. The bundle includes:

- a `git archive` source ZIP for the exact source commit;
- schemas, prompts, benchmark manifests, and the LLM provider example config;
- reproduction, claim-boundary, LLM, verification, Vivado, paper, and release
  notes;
- deterministic public-development and held-out results;
- validate-only sanitized LLM records;
- aggregate tables and held-out aggregate tables;
- optional sanitized Vivado subset summaries;
- the final paper PDF and an `artifact_manifest.json` with per-file hashes.

The bundle gate rejects local provider configs, raw execute-result JSON,
`build/` or `target/` paths inside the ZIP staging tree, logs, Vivado project
outputs, and secret-like strings in text artifacts.

## Publication Boundary

The current release candidate is reviewable but not immutable. Create a release
branch first. Create a tag, GitHub Release, Zenodo record, or other permanent
archive only after final human approval of the generated bundle and sidecar
hash.
