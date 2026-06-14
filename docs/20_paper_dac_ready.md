# DAC-Ready Paper Pass

Snapshot date: 2026-06-14.

This records the M6 paper hardening step for the DAC 2027 plan. DAC 2027 has
not published its research manuscript CFP in this repository snapshot, so the
paper is held to the recent DAC research-track convention of six manuscript
pages plus one references-only page, as documented by the official
[DAC 2026 research FAQ](https://dac.com/2026/research-frequently-asked-questions).
The current source compiles to six pages.

## Scope

The paper now follows Branch A, bounded to the current evidence:

- typed JSON AST composition plus compiler-feedback repair improves tested
  LLM-assisted RTL/IP integration correctness and unsafe rejection;
- public-development and held-out deterministic results are reported separately;
- authenticated v2 LLM results are reported for the tested OpenCode Go profiles;
- repair wins are bounded to the recorded deterministic adapter-instance
  fallback and are not generalized to arbitrary semantic repair;
- no full formal proof, CDC correctness, broad timing closure, or all-task
  Vivado QoR claim.

The main result tables are included from generated aggregate output:

```tex
\input{../build/paper_tables/deterministic_summary.tex}
\input{../build/paper_tables/heldout_m5/deterministic_summary.tex}
\input{../build/paper_tables/llm_summary.tex}
\input{../build/paper_tables/llm_paired_comparisons.tex}
```

This keeps the main paper counts tied to `benchmarks/aggregate_results.py`
rather than hand-edited tables.

## Evidence

Aggregate command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_results.json --llm-result build/llm/bench_execute_dac2027_public_dev_v2.json --llm-result build/llm/bench_execute_dac2027_heldout_20.json --out-json build/bench/aggregate_dac2027_paper_m6.json --paper-table-dir build/paper_tables"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m5_heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_m6_heldout.json --out-dir build/bench/heldout_m6_tables --paper-table-dir build/paper_tables/heldout_m5"
```

Paper command:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

Result:

- `paper/main.pdf`: 6 pages.
- PDF SHA-256: `d53089797775adab662b244d10fb3d461ab39761abed1d0405239115fa75a140`.
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
