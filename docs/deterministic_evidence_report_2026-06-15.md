# MICO Deterministic Evidence Report 2026-06-15

This report records the M2 deterministic evidence gate for the public-development,
held-out, and supplemental realism splits. Numeric claims are mapped in
`docs/release_claim_table.md`; generated JSON, CSV, TeX, and hash sidecars stay
under ignored `build/` paths.

## Commands

Run from the repository root through the Docker EDA wrapper:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_realism.yaml --output build/bench/realism_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_heldout_results.json --out-dir build/bench/heldout_tables --paper-table-dir build/paper_tables/heldout"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/realism_results.json --manifest benchmarks/module_compose_bench_realism.yaml --out-json build/bench/aggregate_realism_results.json --out-dir build/bench/realism_tables --paper-table-dir build/paper_tables/realism"
.\scripts\eda-docker.ps1 python3 scripts/write-paper-summary-tables.py --out-dir paper/tables
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json --aggregate-result build/bench/aggregate_results.json --aggregate-result build/bench/aggregate_heldout_results.json --aggregate-result build/bench/aggregate_realism_results.json"
```

## Expected Results

| Split | Expected | Compose | Lint | Simulation | Formal | QoR | Unsafe | JSON AST |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Public-development | 62/62 | 36/36 | 36/36 | 36/36 | 31/31 | 9/9 | 26/26 | 62/62 |
| Held-out | 20/20 | 10/10 | 10/10 | 10/10 | 9/9 | 3/3 | 10/10 | 20/20 |
| Supplemental realism | 14/14 | 7/7 | 7/7 | 7/7 | 6/6 | 0/0 | 7/7 | 14/14 |

## Reviewer-Facing Table

`scripts/write-paper-summary-tables.py` now generates
`paper/tables/benchmark_split_summary.tex` with:

- per-split task and positive/negative counts;
- L1--L6 level distribution;
- non-calibration case-study positive and paired-negative counts;
- expected outcome, simulation, formal, and QoR denominators.

The case-study/paired-negative column is derived from manifest
`expected_features`: positive `case_study` tasks and negative
`case_study_negative` tasks are counted, while calibration entries are excluded.

## Claim Boundary

The supplemental realism split remains deterministic-only evidence until an
authenticated LLM matrix explicitly reruns it. CDC proof, exhaustive formal
coverage, routed timing closure, and arbitrary-model LLM claims remain outside
the M2 evidence boundary.
