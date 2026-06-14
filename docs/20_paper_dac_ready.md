# DAC-Ready Paper Pass

Snapshot date: 2026-06-14.

This records the M6 paper hardening step for the DAC 2027 plan. DAC 2027 has
not published its research manuscript CFP in this repository snapshot, so the
paper is held to the recent DAC research-track convention of six manuscript
pages plus one references-only page, as documented by the official
[DAC 2026 research FAQ](https://dac.com/2026/research-frequently-asked-questions).
The current source compiles to five pages.

## Scope

The paper now follows the conservative route supported by the evidence:

- deterministic benchmark, compiler gate, repairable representation, and
  failure-taxonomy artifact;
- explicit negative low-cost LLM matrix result;
- no positive LLM pass-rate improvement claim;
- no full formal proof, CDC correctness, broad timing closure, or all-task
  Vivado QoR claim.

The main deterministic result table is included from generated aggregate output:

```tex
\input{../build/paper_tables/deterministic_summary.tex}
```

This keeps the paper table tied to `benchmarks/aggregate_results.py` rather than
hand-edited counts.

## Evidence

Aggregate command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m4_results.json --out-json build/bench/aggregate_m5.json"
```

Paper command:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

Result:

- `paper/main.pdf`: 5 pages.
- Fatal LaTeX errors: none.
- Overfull boxes: none in the final compile.
- Remaining warnings: underfull boxes and IEEEtran final-page column balancing
  reminder.

## Remaining Paper Work

This is a length- and claim-disciplined draft, not the final submission package.
Before submission, the official DAC 2027 CFP must be checked, author anonymity
and metadata must be reviewed, references must be finalized, and any new LLM or
artifact results must regenerate the aggregate table snippets before LaTeX is
run.
