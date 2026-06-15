# Vivado QoR Subset

Snapshot date: 2026-06-15.

This records the M4 Vivado/QoR hardening step for the DAC 2027 plan. It adds a
representative host-Vivado synthesis and timing subset without expanding
the deterministic benchmark runner's normal Yosys QoR scope.

## Scope

The Vivado subset covers nine representative positive tasks:

- `T001_stream_fifo`
- `T003_width_adapter`
- `T058_streaming_accelerator_case`
- `T059_width_protocol_bridge_case`
- `T060_register_status_case`
- `T061_protocol_bridge_case`
- `T062_multi_ip_telemetry_case`
- `T063_axi_apb_wrapper_case`
- `T064_video_filter_pipeline_case`

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
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m3_results.json --out-json build/bench/aggregate_m4.json"
```

Vivado command:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

Result summary:

| Task | Generated LUT | Reference LUT | Generated WNS | Reference WNS |
|---|---:|---:|---:|---:|
| `T001_stream_fifo` | 16 | 16 | 5.496 | 5.496 |
| `T003_width_adapter` | 22 | 22 | 5.526 | 5.526 |
| `T058_streaming_accelerator_case` | 31 | 31 | 5.184 | 4.584 |
| `T059_width_protocol_bridge_case` | 36 | 36 | 5.005 | 4.828 |
| `T060_register_status_case` | 16 | 16 | 5.496 | 5.496 |
| `T061_protocol_bridge_case` | 16 | 16 | 5.496 | 5.496 |
| `T062_multi_ip_telemetry_case` | 62 | 62 | 4.854 | 4.964 |
| `T063_axi_apb_wrapper_case` | 16 | 16 | 5.496 | 5.496 |
| `T064_video_filter_pipeline_case` | 24 | 24 | 5.184 | 5.401 |

All generated/reference rows passed out-of-context synthesis and timing-path extraction in the
measurement-copy flow. FF, BRAM, and DSP counts are zero for this subset because
the current representative leaf RTL is combinational smoke collateral.

Artifact hashes from the ignored report directory:

| Artifact | SHA-256 |
|---|---|
| `build/reports/vivado-host/vivado_qor_subset_summary.json` | `486bb24f6117b873c7b69feefaad6956de5b5625ac92cc3122e88d05480bfd5c` |
| `build/reports/vivado-host/vivado_qor_subset_summary.csv` | `accff80b22a93a72a710abd0cdf9cf12c1da4d5ead847f906194d174f6ac056e` |
| `build/reports/vivado-host/vivado_qor_subset_delta.csv` | `f87fe0646ac7588db98b7c4005619fc62accefdbc61ad1d4cefa08e5d7135db9` |
| `build/reports/vivado-host/vivado_qor_subset_summary.tex` | `eb011bb2eab92b247a6357c078d5fe980b2693e77e65a5dfc4cc8d3988144d49` |
| `build/bench/aggregate_m4.json` | `e90333191861bf5980f7b51179c4a27b1a4796f76d963e257c4ccf557bc2cabf` |

## Claim Boundary

This evidence supports only a representative Vivado out-of-context synthesis and
measurement-copy timing subset for nine tasks. It does not support board-level
implementation, route timing closure, bitstream generation, CDC timing signoff,
technology-mapped conclusions for all 62 public-development tasks, or QoR
claims beyond the reported generated/reference rows.
