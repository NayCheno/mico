# MICO Final Claim Freeze

Snapshot date: 2026-06-15.

This document freezes the claim boundary for the DAC 2027 research-manuscript
candidate. It is intentionally stricter than the current implementation status:
paper, README, release, and artifact text may only make claims that map to
`docs/release_claim_table.md` and to generated evidence hashes from the release
gate.

## Frozen Main Line

MICO is a compiler-gated structured representation for LLM-assisted RTL module
composition. Models propose typed interface graphs or JSON repair patches; the
Rust compiler remains the authority for parsing, name resolution, direction,
width, protocol, clock/reset domain, adapter, and v0 ready/valid contract
checks.

The current paper branch is bounded Branch A:

- scope: the v3 OpenCode Go profiles `smoke`, `low_cost_crosscheck`, and
  `quality_code`;
- prompts: the committed baseline prompts and schemas under `prompts/`;
- splits: the 62-task public-development manifest and the 20-task held-out
  manifest;
- accepted repair wins: the recorded
  `deterministic_adapter_instance_collapse` compiler-feedback fallback plus the
  compiler-gated JSON AST repair path;
- evidence binding: sanitized run records and aggregate tables are hashed by
  `build/release/full_check_manifest.json` when present.

The supplemental 14-task realism manifest is deterministic-only evidence until
it is rerun in an authenticated LLM matrix.

## Required Claim Mapping

Every numeric claim in the paper abstract, introduction, evaluation,
limitations, conclusion, README, or release notes must map to
`docs/release_claim_table.md`. The mapping must identify:

- committed source path or generated ignored artifact;
- schema name or source document;
- release hash source;
- paper section or reviewer-facing document using the number.

Generated artifacts remain ignored and must not be committed. Final submission
evidence is produced by `scripts/full-check.ps1 -WithLatex` and packaged by
`scripts/make-release-bundle.ps1`.

## Frozen Non-Claims

The repository and paper must not claim:

- arbitrary-model LLM improvement or generalization beyond the tested v3
  profiles, prompts, and splits;
- broad free-form or autonomous model repair reliability;
- exhaustive formal verification, full formal proof, or arbitrary LTL support;
- CDC correctness proof;
- routed timing closure, board-level implementation, bitstream generation, or
  complete-benchmark technology-mapped delay;
- Vivado QoR beyond the dedicated 12-task out-of-context subset unless a new
  hashed Vivado artifact is generated and the claim table is updated.

These phrases may appear only in explicit limitation, non-claim, unsupported,
or claim-boundary contexts.

## Automated Guard

Run the claim guard in Docker before each release-oriented commit:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/check-doc-claims.py
```

The guard checks manifest denominators, required claim-table references, stale
numeric claims, and affirmative uses of unsupported claim phrases outside
limitation or boundary contexts.

## Go / No-Go Rule

Submission readiness remains false until all of the following pass from the
final source commit:

- `git status --short` is clean before packaging;
- `git diff --check` passes;
- `scripts/full-check.ps1 -WithLatex` passes;
- generated release sidecars include deterministic, LLM, claim-table, paper,
  and optional Vivado hashes;
- `scripts/make-release-bundle.ps1` creates a ZIP and SHA-256 sidecar without
  secrets, local YAML, raw provider payloads, logs, Vivado project outputs,
  PDFs outside the intended paper artifact, or build/target directories;
- official DAC 2027 CFP page limit, anonymity, artifact policy, deadline, and
  topic taxonomy have been copied into `docs/dac2027_submission_plan.md`.
