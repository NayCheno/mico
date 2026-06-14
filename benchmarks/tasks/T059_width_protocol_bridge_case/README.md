# T059 width protocol bridge case

Dedicated case-study task for a width/protocol bridge:
32-bit sensor stream -> explicit 32-to-64 adapter -> 64-bit processing block
-> host sink.

Validation collateral:

- RTL: `rtl/case_studies/mico_case_studies.sv`
- Simulation: `benchmarks/sim/tb_width_protocol_bridge_case.sv`
- QoR reference: `benchmarks/qor/reference/T059_width_protocol_bridge_case_ref.sv`
