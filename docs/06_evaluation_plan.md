# Evaluation Plan

## Research questions

1. Does MICO improve LLM first-pass correctness for RTL module composition?
2. Does structured compiler feedback reduce repair turns?
3. Does contract/domain checking catch errors missed by direct Verilog prompting?
4. What is the PPA/QoR impact of adapter insertion?
5. Is the generated wrapper maintainable and traceable?

## Baselines

- Direct Verilog prompting.
- SystemVerilog interface prompting.
- Chisel/Amaranth style prompting.
- CPPL-style JSON IR prompting.
- MICO source prompting.
- MICO JSON AST prompting.
- MICO JSON AST + repair loop.
- Human-written wrapper/top.

## Metrics

| Metric | Definition |
|---|---|
| Compose-Pass@1 | Compile-check success on first LLM output. |
| Repair-Turns | Average number of repair iterations to pass. |
| Lint-Pass | Generated SV passes linter/elaboration. |
| Sim-Pass | Generated design passes testbench. |
| Formal-Pass | Generated contracts/adapters pass formal checks. |
| Unsafe-Rejection | Compiler rejects intentionally unsafe CDC/protocol bugs. |
| QoR-Delta | Area/timing/latency delta vs human baseline. |
| Token-Cost | Prompt+completion tokens per successful task. |
| Human-Fix-Time | Minutes of human repair needed. |
| CER | Connection Entropy Reduction. |

## Dataset construction

The expanded deterministic public-development manifest has 83 tasks: 46
positives and 37 negatives, covering 11 L1, 13 L2, 10 L3, 12 L4, 18 L5, and 19
L6 tasks. The expanded deterministic held-out manifest has 40 tasks with 20
positives and 20 negatives. The supplemental realism manifest has 30 tasks with
15 positives and 15 paired negatives.
Public-development, held-out, and realism results must be aggregated and
reported separately, with numeric claim sources recorded in
`docs/release_claim_table.md` and denominator mixing rules recorded in
`docs/submission_claim_lock_2026Q3.md`.

The committed public-development manifest is allowed for prompt debugging and
deterministic regression. Final LLM advantage claims remain bound to separately
versioned scored manifests and their manifest SHA-256 values. Prompt
construction may use task requests, inventories, and interface/module
declarations, but it must not include committed expected compose bodies,
diagnostics, testbench checks, formal monitors, or QoR references.

## Ablations

- No JSON schema.
- No compiler feedback.
- No repair loop.
- No adapter contract check.
- No EDA lint gate.
- No prompt leakage controls.

## Result Aggregation

`benchmarks/aggregate_results.py` merges deterministic ModuleComposeBench JSON
with optional sanitized LLM batch records. It emits `mico.aggregate.results.v0`
JSON, CSV tables under `build/bench/`, and LaTeX snippets under
`build/paper_tables/` for main results, per-level denominators and confidence
intervals, unsafe diagnostics, structural/generic-mapped QoR, conservative
ablation guard-surface rows, repair-turn distributions, token/cost accounting,
paired comparisons with exact sign tests and net effect sizes, and failure
taxonomy. Unsupported metrics must remain marked as `not_run` or
`not_applicable` in their source records and must not be treated as zero-pass
failures.

## Expected result pattern

MICO should outperform direct Verilog on composition correctness and repair efficiency, while remaining comparable on final SV quality and PPA for direct connections. Adapter-heavy tasks may show QoR tradeoffs; those must be reported transparently.
