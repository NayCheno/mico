# T058 streaming accelerator case

Dedicated case-study task for a streaming accelerator chain:
DMA-like source -> skid buffer -> XOR filter -> result sink.

Validation collateral:

- RTL: `rtl/case_studies/mico_case_studies.sv`
- Simulation: `benchmarks/sim/tb_streaming_accelerator_case.sv`
- Formal monitor: `benchmarks/formal/tb_streaming_accelerator_case_formal.sv`
- QoR reference: `benchmarks/qor/reference/T058_streaming_accelerator_case_ref.sv`
