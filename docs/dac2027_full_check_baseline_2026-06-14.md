# DAC 2027 Full-Check Baseline

Snapshot date: 2026-06-14.

This file records the public, source-controlled summary of the latest M1
baseline evidence run for `docs/dac2027_submission_plan.md`. The generated
JSON, logs, paper PDF, and release manifest remain under ignored build or paper
output paths and are not committed.

## Source

- Source commit checked: `0e863728cfc0b1b4bc218401f3661ff058b02c6a`
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
| Held-out expected outcome | 12/12 |
| Held-out positive lint/sim | 6/6 |
| Held-out single-clock formal smoke | 5/5 |
| Held-out reference-enabled QoR summaries | 3/3 |
| Held-out unsafe rejection | 6/6 |
| Held-out JSON AST path | 12/12 |
| LLM provider validate-only | pass, sanitized metadata only |
| LLM batch validate-only | pass, 620 planned attempts, 0 provider requests |
| Aggregate result generation | pass |
| JSON schema validation | pass |
| Host LaTeX paper build | pass, 5-page PDF |

The LaTeX build completed with underfull/overfull box warnings already visible
in the paper workflow; no citation, reference, missing-figure, or fatal build
error was reported.

## Evidence Hashes

Generated evidence files are intentionally ignored by git. These hashes identify
the local artifacts produced by this baseline run:

| Artifact | SHA-256 |
|---|---|
| `build/bench/seed_results.json` | `c1596b102263be3e4ec470e45f6f90a7fce44cd29ae305fdffe669ac3dfe3d86` |
| `build/bench/heldout_results.json` | `10f5e13982ff4dcc585b7831e7f2f27d87e66656f0bddea56aa37701dd1a8db4` |
| `build/bench/aggregate_results.json` | `e964cc30567ab181402c266cf5b7841757b727c572fb78aef45526897db0271e` |
| `build/bench/aggregate_heldout_results.json` | `f2c2a64385321cafacd98cec259f1e0a4f7a36acb8f9db422612b5b226206b3b` |
| `build/llm/provider_validate.json` | `a53885cddf2c35bc5e8d252dfcce26152a4590421cf5566db56ebb98f3eedba2` |
| `build/llm/bench_validate.json` | `7ae4683c628f60e34ab04bad9c56e96f6c55097cc01ede3353dfed80d75b9987` |
| `build/release/full_check_manifest.json` | `be5bdbb7ad705589f409384ed10b5e84f1e57495787dd597d67351ed385a1c3e` |
| `paper/main.pdf` | `26bdcd481d8c88f2bfaef218773888666b3e70f15e7c376c2c8783c8a419be42` |

## Claim Boundary

This baseline preserves the current claim boundary:

- deterministic artifact evidence is reproducible through the release gate;
- the LLM validate-only matrix is not scored pass-rate evidence;
- no positive LLM-improvement claim is supported by this M1 run;
- full directed simulation coverage, full task-specific formal coverage, CDC
  correctness proof, timing closure, and Vivado QoR remain unsupported.
