# DAC 2027 Full-Check Baseline

Snapshot date: 2026-06-14.

This file records the public, source-controlled summary of the M1 baseline
evidence run for `docs/dac2027_submission_plan.md`. The generated JSON, logs,
paper PDF, and release manifest remain under ignored build or paper output
paths and are not committed.

## Source

- Source commit checked: `23ac45b772bf3fe508172742c27b44567de82d32`
- Source branch: `main`
- Gate command: `.\scripts\full-check.ps1 -WithLatex`
- Host Docker: `Docker version 29.4.3, build 055a478`
- Container OS: Ubuntu 24.04.4 LTS

## Result Summary

The full release-candidate gate passed.

| Gate | Result |
|---|---|
| Docker tool verification | pass |
| Rust fmt/check/test | pass |
| EDA smoke | pass |
| Deterministic expected outcome | 62/62 |
| Positive compose | 36/36 |
| Positive lint/elaboration | 36/36 |
| Positive simulation smoke | 36/36 |
| Single-clock bounded formal smoke | 31/31 |
| Reference-enabled QoR summaries | 9/9 |
| Unsafe rejection | 26/26 |
| JSON AST path | 62/62 |
| LLM provider validate-only | pass, sanitized metadata only |
| LLM batch validate-only | pass, 620 planned attempts, 0 provider requests |
| Aggregate result generation | pass |
| JSON schema validation | pass |
| Host LaTeX paper build | pass, 7-page PDF |

The LaTeX build completed with underfull/overfull box warnings already visible
in the paper workflow; no citation, reference, missing-figure, or fatal build
error was reported.

## Evidence Hashes

Generated evidence files are intentionally ignored by git. These hashes identify
the local artifacts produced by this baseline run:

| Artifact | SHA-256 |
|---|---|
| `build/bench/seed_results.json` | `8b10e160e0e1d69ea029931bd65135c8f443c5ff7f5343c4d5f3432b33d7d143` |
| `build/bench/aggregate_results.json` | `1f08cf20f313994b6fa57bffee36887fb7f7ebda287bfdb9407cdb33004efebe` |
| `build/llm/provider_validate.json` | `2279b78cd2f5319cd30e005951671ca96adef67438f1a8038221d27d07728ac9` |
| `build/llm/bench_validate.json` | `5578aaf3092c6f348a52c00b1d29e7205937a0e7097cbe1014d8c62de90ddc47` |
| `build/release/full_check_manifest.json` | `554e4a66f64d3af1b034123831935a69f7394404ab2851a97edb26116a1cc3e1` |
| `paper/main.pdf` | `f468b6db9afb789a96d90addab6342dbe01c1af0836ae3112132f561d19cab98` |

## Claim Boundary

This baseline preserves the current claim boundary:

- deterministic artifact evidence is reproducible through the release gate;
- the LLM validate-only matrix is not scored pass-rate evidence;
- no positive LLM-improvement claim is supported by this M1 run;
- full directed simulation coverage, full task-specific formal coverage, CDC
  correctness proof, timing closure, and Vivado QoR remain unsupported.
