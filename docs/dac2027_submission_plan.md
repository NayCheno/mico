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

## CFP Monitor

Latest manual check: 2026-06-14.

| Item | Current record |
|---|---|
| Official DAC 2027 event page | Published at `https://dac.com/2026/events/dac-2027`; lists July 10--16, 2027 at the San Jose Convention Center. |
| DAC 2027 Research CFP | Not published in the reviewed DAC 2026 web tree or IEEE CEDA announcements at this snapshot. |
| Research manuscript deadline | Not published for DAC 2027. Do not infer a submission-ready deadline from prior years. |
| Provisional page limit | Use the DAC 2026 research FAQ convention as a planning constraint only: six manuscript pages plus one references-only page. Replace when the DAC 2027 CFP appears. |
| Provisional review policy | Treat the submission as double-blind, with no author names or affiliations in the manuscript, following the DAC 2026 FAQ until DAC 2027 publishes its own rules. |
| Provisional topic fit | AI, Electronic Design Automation, Design, Systems, and Verification/Validation are the nearest DAC 2026-style topic areas; re-map to DAC 2027 topics when available. |

Monthly update rule:

- Re-check the official DAC 2027 event page, DAC call-for-contributions pages,
  and IEEE CEDA announcements.
- If the official Research CFP appears, update the deadline, abstract deadline,
  page limit, anonymity policy, conflict-of-interest policy, rebuttal/review
  policy, artifact policy, and topic taxonomy in this file before changing
  paper claims.
- Set the internal artifact and paper freeze no later than six weeks before the
  official manuscript deadline. Until the DAC 2027 deadline is known, use
  2026-10-07 as the conservative provisional freeze date, based on the DAC 2026
  mid-November research-manuscript cadence.

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
- 36/36 positive Icarus smoke simulations, with 28 directed harnesses and
  generated ready/valid smoke harnesses for the remaining 8 positives.
- 31/31 single-clock bounded formal smoke checks, with 20 directed monitors
  and generated smoke harnesses for the remaining 11 single-clock positives.
- 9/9 structural and generic-mapped Yosys QoR summaries for reference-enabled
  tasks.
- Nine-task representative host-Vivado out-of-context QoR/timing subset with
  generated/reference summaries for `T001`, `T003`, and `T058`--`T064`.
- Held-out manifest with 12 scoring tasks, including three additional
  subsystem case studies and three paired negative variants; deterministic
  held-out scoring currently passes 12/12 expected outcomes.
- Historical sanitized low-cost LLM matrix summary showing a negative result
  for the original prompts.
- Structured v2 authenticated LLM full matrix across public-development and
  held-out splits showing a tested-profile Branch A candidate result for MICO
  JSON AST and compiler-feedback repair.
- Five-page DAC-style paper draft with deterministic summary table generated
  from aggregate JSON.

Unsupported now:

- LLM pass-rate improvement beyond the exact v2 tested profiles, prompts, and
  benchmark splits;
- broad free-form model-generated repair reliability beyond the recorded
  adapter-instance compiler-feedback fallback;
- full directed simulation coverage beyond the 28 committed directed
  harnesses;
- full task-specific formal proof coverage beyond the 20 committed
  single-clock monitors;
- CDC correctness proof;
- full timing closure, technology-mapped delay, or broad Vivado QoR beyond the
  current nine-task out-of-context subset;
- final submission-readiness.

## Evidence Schedule

Internal hard stops are used until the official DAC 2027 CFP is available.

| Month | Gate | Required result |
|---|---|---|
| 2026-07 | Claim freeze and full-check baseline | current deterministic evidence reproduced |
| 2026-08 | LLM prompt/model pilot | nonzero structured positive pass or clear negative taxonomy |
| 2026-09 | Full LLM matrix and split policy | dev/test or held-out result basis fixed |
| 2026-10 | Provisional paper/artifact freeze | directed verification, QoR, paper, and release manifest ready no later than the six-week safety margin |
| 2026-11 | Submission sprint if DAC follows the 2026 cadence | only CFP-driven formatting, metadata, COI, and anonymity fixes remain |
| 2026-12 | Post-submission archive hardening | immutable external archive and release tag only after human approval |

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
held-out benchmark results, sanitized LLM results, prompt templates, benchmark
manifests, aggregate tables, optional Vivado subset summaries, and the final
paper PDF. The review bundle is generated with:

```powershell
.\scripts\make-release-bundle.ps1
```

Baseline evidence records:

- `docs/dac2027_full_check_baseline_2026-06-14.md`
- `docs/17_llm_prompt_redesign_pilot.md`
- `docs/21_artifact_release_candidate.md`
