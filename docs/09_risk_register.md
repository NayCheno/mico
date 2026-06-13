# Risk Register

| Risk | Severity | Mitigation |
|---|---:|---|
| Too similar to CPPL | High | Focus on module composition, contract/adapters, ModuleComposeBench. |
| Too similar to Chisel/Amaranth interfaces | High | Avoid general RTL DSL claims; emphasize LLM repair and contract/domain checks. |
| Toy-only examples | High | Use real open-source RTL/IP modules and nontrivial wrappers. |
| Rust compiler incomplete | Medium | Start with narrow grammar and external module declarations only. |
| Adapter insertion hurts PPA | Medium | Report QoR delta and require explicit confirmation for nontrivial adapters. |
| Formal properties too hard | Medium | Start with safety properties; make liveness assumptions explicit. |
| LLM results model-dependent | Medium | Evaluate multiple models and report prompt/template details. |
| Toolchain compatibility | Medium | Emit conservative SV; use optional CIRCT path later. |
| CDC correctness overclaimed | High | Treat CDC adapters as blackbox-proven or require external CDC collateral. |
| Benchmark contamination | Medium | Include generated/held-out tasks and negative tests. |
