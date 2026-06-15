# Vivado QoR Subset

Snapshot date: 2026-06-15.

This records the M3.3 Vivado/QoR hardening step for the DAC 2027 plan. It adds
a host-Vivado synthesis and timing subset for every QoR-enabled public and
held-out task without expanding the deterministic benchmark runner's normal
Yosys QoR scope.

## Scope

The Vivado subset covers 12 QoR-enabled positive tasks:

- `T001_stream_fifo`
- `T002_cdc_fifo`
- `T003_width_adapter`
- `T004_direct_stream`
- `T058_streaming_accelerator_case`
- `T059_width_protocol_bridge_case`
- `T060_register_status_case`
- `T061_protocol_bridge_case`
- `T062_multi_ip_telemetry_case`
- `T063_axi_apb_wrapper_case`
- `T064_video_filter_pipeline_case`
- `T065_cdc_event_status_case`

The flow uses `D:\Application\vivado\2025.2\Vivado` through
`scripts/run-vivado-host.ps1`, targets `xc7a35tcpg236-1`, and writes all
reports under ignored `build/reports/vivado-host/`.
The Tcl flow emits JSON, CSV, per-task delta CSV, and a paper-ready TeX table
for release-manifest hashing and bundle inclusion.

The committed source RTL and benchmark wrappers are not modified. The Tcl script
creates build-only sanitized copies that:

- replace ``default_nettype none`` with ``default_nettype wire`` for Vivado 2025.2
  parsing compatibility;
- add a measurement-only `mico_observe` output to the copied top wrapper;
- mark internal nets and instances with `KEEP` / `DONT_TOUCH` attributes so the
  representative wrapper structure remains observable after out-of-context
  synthesis.

## Evidence

Open-source aggregate command:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --out-json build/bench/aggregate_results.json --paper-table-dir build/paper_tables/public_dev"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/heldout_results.json --out-json build/bench/aggregate_heldout_results.json --paper-table-dir build/paper_tables/heldout --manifest benchmarks/module_compose_bench_heldout.yaml"
```

Vivado command:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

Result summary:

| Task | Generated LUT | Reference LUT | Generated WNS | Reference WNS |
|---|---:|---:|---:|---:|
| `T001_stream_fifo` | 16 | 16 | 5.496 | 5.496 |
| `T002_cdc_fifo` | 14 | 14 | 6.087 | 6.087 |
| `T003_width_adapter` | 22 | 22 | 5.526 | 5.526 |
| `T004_direct_stream` | 9 | 9 | 5.623 | 5.623 |
| `T058_streaming_accelerator_case` | 31 | 31 | 5.184 | 4.584 |
| `T059_width_protocol_bridge_case` | 36 | 36 | 5.005 | 4.828 |
| `T060_register_status_case` | 16 | 16 | 5.496 | 5.496 |
| `T061_protocol_bridge_case` | 16 | 16 | 5.496 | 5.496 |
| `T062_multi_ip_telemetry_case` | 62 | 62 | 4.854 | 4.964 |
| `T063_axi_apb_wrapper_case` | 16 | 16 | 5.496 | 5.496 |
| `T064_video_filter_pipeline_case` | 24 | 24 | 5.184 | 5.401 |
| `T065_cdc_event_status_case` | 14 | 14 | 6.087 | 6.087 |

All generated/reference rows passed out-of-context synthesis and timing-path extraction in the
measurement-copy flow. FF, BRAM, and DSP counts are zero for this subset because
the current representative leaf RTL is combinational smoke collateral.

Artifact hashes from the ignored report directory:

| Artifact | SHA-256 |
|---|---|
| `build/reports/vivado-host/vivado_qor_subset_summary.json` | `1ab79afa2bc32881fa05af7e896a5e1019f015b73d1986edc3f5c59fc43e5d89` |
| `build/reports/vivado-host/vivado_qor_subset_summary.csv` | `a2be4556f6c127ddf5d65d2befc060c094ff0eaedd02b42fbf99a88c01f21ce3` |
| `build/reports/vivado-host/vivado_qor_subset_delta.csv` | `5953f168b6a78e5f11c10e32e6af6642ff09792c9f41c03722d98c5f08d77060` |
| `build/reports/vivado-host/vivado_qor_subset_summary.tex` | `62f8b650775252d77afe97b739b58853cd441270dd8635bea04b12c2fc555f13` |

## Claim Boundary

This evidence supports only a Vivado out-of-context synthesis and
measurement-copy timing subset for the 12 QoR-enabled public and held-out tasks.
It does not support board-level implementation, route timing closure, bitstream
generation, CDC timing signoff, technology-mapped conclusions for all 62
public-development tasks, or QoR claims beyond the reported generated/reference
rows.
