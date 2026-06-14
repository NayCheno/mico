# Scorecard

| Dimension | Score | Comment |
|---|---:|---|
| Novelty | 8.4 | Strong if framed as contract-guided LLM module composition, not generic HDL. |
| Technical depth | 8.2 | Requires static semantics, diagnostics, adapter correctness, contracts. |
| Feasibility | 8.0 | Rust frontend + conservative SV backend is practical. |
| Evaluation clarity | 8.3 | ModuleComposeBench yields measurable pass/fail results. |
| Distinction from prior work | 8.0 | Clear if CPPL/Chisel/Amaranth are handled directly. |
| Engineering impact | 8.5 | Top-level glue and wrapper generation are real pain points. |
| CCF-A potential | 8.1 | Artifact and benchmark quality will decide. |

## Minimum publishable artifact

- 50+ tasks in ModuleComposeBench. The deterministic manifest currently has 60 tasks.
- 4 baselines minimum. The batch runner currently defines five; committed full
  result matrices are still pending.
- 3 real subsystem case studies with committed RTL, simulation, and QoR collateral.
- Rust compiler with checker and emitter.
- Structured diagnostics + LLM repair loop.
- Lint/sim/formal scripts.

## Strong paper threshold

MICO should show:

- 20–40 percentage point improvement in Compose-Pass@1 or final pass rate over direct Verilog prompting on composition tasks;
- 2x or more reduction in repair turns/human fix time;
- near-zero unsafe CDC/protocol acceptance on negative tests;
- low QoR delta for direct connections;
- transparent QoR costs for adapters.
