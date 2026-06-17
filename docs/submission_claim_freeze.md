# MICO Submission Claim Freeze

Snapshot date: 2026-06-16.

This file is the reviewer-facing freeze for the DAC 2027 submission narrative.
It mirrors `docs/claim_boundary.md`, `docs/final_claim_freeze.md`, and the
numeric source of truth in `docs/release_claim_table.md`.
`docs/submission_claim_lock_2026Q3.md` gives the quarter-specific denominator
lock that prevents mixing expanded deterministic counts with locked
pre-expansion LLM counts.

## Main Submission Claim

MICO turns LLM-assisted RTL module composition into a compiler-gated typed-graph
problem. On the tested v3 OpenCode Go profiles and the locked pre-expansion
LLM-scored public-development and held-out manifest hashes, schema-guided JSON
AST prompting plus compiler-gated repair outperforms direct RTL,
SystemVerilog-interface, and MICO-source prompting while preserving unsafe
rejection.

The compiler remains the authority for parsing, name resolution, direction,
width, protocol, clock/reset domain, adapter, and v0 ready/valid contract
checks. Models propose structured composition artifacts; deterministic gates
decide whether those artifacts are accepted.

## Venue Classification

| Venue | CCF classification | Repository role |
|---|---:|---|
| DAC 2027 Research Manuscript | A | Primary full-paper target |
| ICCAD 2027 | B | EDA-method backup only |

## Allowed Claims

- The deterministic public-development, held-out, and supplemental realism
  results listed in `docs/release_claim_table.md`.
- A bounded v3 LLM result tied to the tested OpenCode Go profiles, committed
  prompts, and locked pre-expansion LLM-scored public-development and held-out
  manifest hashes.
- Compiler-gated JSON AST repair plus the recorded deterministic
  adapter-instance fallback, with repair provenance preserved in the LLM
  evidence.
- Directed simulation, bounded single-clock formal smoke, structural/generic
  Yosys QoR, and the 12-task Vivado out-of-context subset exactly as scoped in
  `docs/release_claim_table.md`.

## Forbidden Claims

- LLM improvement for profiles, prompts, provider versions, or benchmark splits
  outside the v3 evidence.
- Broad autonomous free-form repair reliability.
- Complete formal signoff, complete temporal contract verification, or
  unrestricted LTL support.
- Proof of CDC correctness for the smoke collateral.
- Routed implementation signoff, board-level implementation readiness,
  bitstream evidence, or complete-benchmark vendor timing evidence.

## Consistency Rule

README, paper abstract, evaluation tables, limitations, release notes, and
artifact quickstarts must keep these boundaries aligned. Every numeric result
must map back to `docs/release_claim_table.md`; generated release artifacts stay
under ignored `build/` paths and must not be committed.
