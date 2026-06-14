# T065 CDC Event/Status Case

Held-out subsystem case for an event stream crossing from a source clock domain
to a status sink through an explicit CDC FIFO adapter. The FIFO collateral is a
smoke adapter and is not a CDC correctness proof.

- Source: `benchmarks/tasks/T065_cdc_event_status_case/expected.mico`
- RTL collateral: `rtl/case_studies/mico_case_studies.sv`
- Simulation: `benchmarks/sim/tb_cdc_event_status_case.sv`
- QoR reference: `benchmarks/qor/reference/T065_cdc_event_status_case_ref.sv`
- Negative variant: `T068_cdc_without_adapter_case`
