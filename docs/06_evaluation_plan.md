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

The current deterministic seed manifest has 57 tasks:

- 10 L1 direct stream wiring tasks;
- 13 L2 parameter/width tasks;
- 10 L3 adapter/backpressure/latency seed tasks;
- 10 L4 CDC/RDC tasks;
- 8 L5 bus/register wrapper seed tasks;
- 6 L6 subsystem integration seed tasks.

The full LLM evaluation should preserve at least this 50+ task scale while
adding dedicated non-smoke L3/L5/L6 RTL case studies:

- 10 direct stream wiring tasks;
- 10 parameter/width tasks;
- 10 adapter tasks;
- 10 CDC/RDC tasks;
- 5 bus bridge tasks;
- 5 subsystem integration tasks.

## Ablations

- No contract checks.
- No clock-domain checks.
- No structured diagnostics.
- Text DSL vs JSON AST.
- No adapter library.
- No retrieval over module inventory.

## Expected result pattern

MICO should outperform direct Verilog on composition correctness and repair efficiency, while remaining comparable on final SV quality and PPA for direct connections. Adapter-heavy tasks may show QoR tradeoffs; those must be reported transparently.
