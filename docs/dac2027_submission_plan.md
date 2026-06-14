# DAC 2027 Submission Plan

Snapshot date: 2026-06-14.

This file is the submission-oriented control document for moving MICO from a
research prototype toward a DAC 2027 Research Manuscript candidate. It does not
replace `docs/claim_boundary.md`; instead, it records the venue target, the
allowed claim branches, and the evidence gates that must be satisfied before
the paper can make stronger claims.

## Venue Target

Primary target:

- DAC 2027 Research Track / Research Manuscript.
- Conference dates: 2027-07-10 through 2027-07-16.
- Location: San Jose Convention Center, San Jose, California.
- Technical fit: AI-assisted EDA, RTL/IP composition, hardware design
  automation, compiler-checked design representations, SystemVerilog/SVA
  generation, and reproducible artifacts.

Backup target:

- ICCAD 2027, if the final emphasis becomes a narrower EDA method paper.
- DAC/ICCAD tool, demo, workshop, or artifact track if the LLM result remains
  negative and the strongest contribution is a benchmark plus compiler gate.

ICCAD 2026 is not a full-paper target for work starting in July 2026 because
its regular paper deadline was 2026-04-14.

The DAC 2027 research CFP and submission deadline are not committed in this
repository. When the official CFP appears, update this document, the paper
schedule, and the release gate dates before claiming submission readiness.

## Claim Branches

The paper must choose exactly one main contribution branch before final
submission.

### Branch A: Positive LLM Improvement

Use this branch only if a new authenticated, schema-valid matrix shows that
MICO JSON AST plus compiler-feedback repair produces a meaningful positive-task
advantage over direct RTL and SystemVerilog-interface prompting.

Minimum evidence before making this claim:

- positive-task compiler pass is nonzero for a MICO structured baseline;
- positive-task lint pass is nonzero;
- JSON AST repair has at least one accepted repair win;
- unsafe rejection remains at least competitive with direct baselines;
- paired comparisons and confidence intervals support the stated effect;
- the trend holds for at least two model profiles.

Preferred DAC-strength evidence:

- `mico_json_ast_repair` improves positive final pass rate by at least 15--20
  percentage points over direct RTL or SystemVerilog-interface baselines, or it
  shows a similarly clear advantage in unsafe rejection with no loss in
  accepted positive results.

### Branch B: Negative LLM Study Plus Compiler-Gated Benchmark

Use this branch if improved prompts and stronger profiles still fail to produce
positive-task passes, but the deterministic benchmark, failure taxonomy, repair
plumbing, and artifact are strong enough to stand as the contribution.

Allowed main claim:

> Current low-cost LLMs fail realistic RTL/IP module composition under the
> tested prompts and models; MICO contributes a compiler-checked benchmark,
> repairable representation, and reproducible artifact framework that exposes
> and contains these failures.

This branch must not imply that MICO improves LLM pass rate. It is more likely
to fit an artifact/tool/demo/workshop venue unless the failure taxonomy and
benchmark evidence become unusually strong.

## Current Boundary

The current repository supports a deterministic artifact claim, not a positive
LLM-improvement claim.

Supported now:

- 62 ModuleComposeBench tasks with 36 positive and 26 negative tasks.
- 62/62 expected deterministic outcomes.
- 36/36 positive lint/elaboration smoke.
- 36/36 positive Icarus smoke simulations, with nine directed harnesses and
  generated ready/valid smoke harnesses for the rest.
- 31/31 single-clock bounded formal smoke checks, with three directed monitors
  and generated smoke harnesses for the rest.
- 9/9 structural and generic-mapped Yosys QoR summaries for reference-enabled
  tasks.
- Held-out manifest with 12 scoring tasks, including three additional
  subsystem case studies and three paired negative variants; deterministic
  held-out scoring currently passes 12/12 expected outcomes.
- Sanitized low-cost LLM matrix summary showing a negative result.

Unsupported now:

- positive LLM pass-rate improvement;
- full directed simulation coverage;
- full task-specific formal proof coverage;
- CDC correctness proof;
- timing closure, technology-mapped delay, or Vivado QoR;
- final submission-readiness.

## Evidence Schedule

Internal hard stops are used until the official DAC 2027 CFP is available.

| Month | Gate | Required result |
|---|---|---|
| 2026-07 | Claim freeze and full-check baseline | current deterministic evidence reproduced |
| 2026-08 | LLM prompt/model pilot | nonzero structured positive pass or clear negative taxonomy |
| 2026-09 | Full LLM matrix and split policy | dev/test or held-out result basis fixed |
| 2026-10 | Verification, QoR, and case-study hardening | directed simulation/formal and QoR denominators increased |
| 2026-11 | DAC paper full draft v1 | tables, statistics, threats, and related work complete |
| 2026-12 | Artifact release candidate | manifest, hashes, sanitized results, and release guide complete |

## Environment Policy

All Rust, Python, benchmark, LLM, open-source EDA, schema-validation, aggregate
table, and paper-table generation commands run in the repository Docker
environment through `scripts/eda-docker.ps1` or `scripts/eda-docker.sh`.

There are exactly two host-tool exceptions:

- Vivado-specific flows run only through `scripts/run-vivado-host.ps1`, which
  is pinned to `D:\Application\vivado\2025.2\Vivado`.
- Final PDF compilation for `paper/main.tex` uses the Windows host LaTeX
  toolchain because this is the repository paper workflow. Any paper-related
  Python or statistical validation still runs in Docker.

Do not commit generated outputs, `build/`, `rust_project/target/`, logs, paper
PDFs, Vivado project outputs, `config/*.local.yaml`, raw provider responses,
API keys, or secrets.

## Release Evidence Gate

Before claiming DAC submission readiness, the following gate must pass from a
clean tree:

```powershell
git status --short
git diff --check
.\scripts\full-check.ps1 -WithLatex
```

The generated `build/release/full_check_manifest.json` must be reviewed or
archived externally together with hashes for deterministic benchmark results,
sanitized LLM results, prompt templates, benchmark manifests, aggregate tables,
and the final paper PDF.

Baseline evidence records:

- `docs/dac2027_full_check_baseline_2026-06-14.md`
- `docs/17_llm_prompt_redesign_pilot.md`
