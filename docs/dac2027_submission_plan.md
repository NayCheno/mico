# DAC 2027 Submission Plan

Snapshot date: 2026-06-15.

This file is the submission-oriented control document for moving MICO from a
research prototype toward a DAC 2027 Research Manuscript candidate. It does not
replace `docs/claim_boundary.md`; instead, it records the venue target, the
allowed claim branches, and the evidence gates that must be satisfied before
the paper can make stronger claims.
Current numeric result values and release evidence mappings are centralized in
`docs/release_claim_table.md`.

## Venue Target

Primary target:

- DAC 2027 Research Track / Research Manuscript.
- CCF classification: CCF A.
- Conference dates: 2027-07-10 through 2027-07-16.
- Location: San Jose Convention Center, San Jose, California.
- Technical fit: AI-assisted EDA, RTL/IP composition, hardware design
  automation, compiler-checked design representations, SystemVerilog/SVA
  generation, and reproducible artifacts.

Backup target:

- ICCAD 2027, if the final emphasis becomes a narrower EDA method paper.
- CCF classification: CCF B backup, not a CCF A target.
- DAC/ICCAD tool, demo, workshop, or artifact track if the LLM result remains
  negative and the strongest contribution is a benchmark plus compiler gate.

Venue classification is part of the frozen submission boundary: repository and
paper planning may call DAC the CCF A full-paper target, while ICCAD must be
described only as a CCF B EDA-method backup.

ICCAD 2026 is not a full-paper target for work starting in July 2026 because
its regular paper deadline was 2026-04-14.

The DAC 2027 research CFP and submission deadline are not committed in this
repository. When the official CFP appears, update this document, the paper
schedule, and the release gate dates before claiming submission readiness.

## CFP Monitor

Latest manual check: 2026-06-16.

| Item | Current record |
|---|---|
| Official DAC 2027 event page | Rechecked at `https://dac.com/2026/events/dac-2027`; lists July 10--16, 2027 at the San Jose Convention Center. |
| DAC future-dates archive | Rechecked at `https://archive.dac.com/about/future-dac-dates.html`; also lists 64th DAC 2027 on July 10--16, 2027 at the San Jose Convention Center. |
| DAC 2027 Research CFP | Not published in the reviewed DAC event/future-date pages or DAC web search results at this snapshot. |
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

Check log:

- 2026-06-15: Official DAC 2027 event and future-date pages confirm only date
  and venue. No DAC 2027 Research Track CFP, abstract deadline, manuscript
  deadline, page limit, anonymity policy, rebuttal policy, artifact policy, or
  topic taxonomy was found. Keep the provisional freeze and do not cite an
  unconfirmed DAC 2027 submission deadline.
- 2026-06-16: Rechecked the official DAC 2027 event page and DAC call-for-
  contributions search results. The official DAC 2027 page still lists only
  July 10--16, 2027 at the San Jose Convention Center; no DAC 2027 Research
  CFP, deadline, page limit, anonymity policy, artifact policy, or topic
  taxonomy was found. Continue using DAC 2026 rules only as provisional
  planning constraints.

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

The current repository supports a deterministic artifact claim plus a bounded
tested-profile Branch A LLM-improvement claim. It is still not final submission
ready until the M3-M5 verification, paper, and release gates pass.

Supported now:

- 62 ModuleComposeBench tasks with 36 positive and 26 negative tasks.
- 62/62 expected deterministic outcomes.
- 36/36 positive lint/elaboration smoke.
- 36/36 positive Icarus smoke simulations, all with committed directed
  harnesses.
- 31/31 single-clock bounded formal smoke checks, all with committed directed
  monitors.
- 9/9 structural and generic-mapped Yosys QoR summaries for reference-enabled
  tasks.
- 12-task host-Vivado out-of-context QoR/timing subset covering all
  QoR-enabled public and held-out tasks: `T001`--`T004` and `T058`--`T065`.
- Held-out manifest with 20 scoring tasks, including seven subsystem positives
  and seven paired negative variants; deterministic held-out scoring currently
  passes 20/20 expected outcomes. The non-CDC held-out subsystem positives now
  have committed directed formal monitors, while the explicit CDC held-out case
  remains smoke-only.
- Historical sanitized low-cost LLM matrix summary showing a negative result
  for the original prompts.
- Structured v3 authenticated LLM full matrix across public-development and
  held-out splits showing a tested-profile Branch A result for MICO JSON AST
  and compiler-feedback repair, bound to the current manifest hashes in
  `docs/24_llm_matrix_v3.md`.
- Five-page DAC-style paper draft with deterministic summary table generated
  from aggregate JSON.

M2 branch decision:

- Branch A is the current paper branch for the tested OpenCode Go profiles,
  prompts, public-development manifest, and held-out manifest.
- The v3 held-out execute record is bound to the current held-out manifest
  SHA-256.
- JSON AST repair reaches 36/36 public-development positives and 10/10
  held-out positives across the tested profiles, while direct Verilog and
  SV-interface baselines remain weaker.
- Accepted repair-turn wins remain limited to the recorded
  `deterministic_adapter_instance_collapse` fallback; broad free-form repair is
  still unsupported.

Unsupported now:

- LLM pass-rate improvement beyond the exact v3 tested profiles, prompts, and
  benchmark splits;
- broad free-form model-generated repair reliability beyond the recorded
  adapter-instance compiler-feedback fallback;
- exhaustive or randomized simulation coverage beyond the committed directed
  smoke harnesses;
- exhaustive task-specific formal proof coverage beyond the bounded
  single-clock smoke denominator;
- CDC correctness proof;
- full timing closure, technology-mapped delay, or broad Vivado QoR beyond the
  current 12-task out-of-context subset;
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
