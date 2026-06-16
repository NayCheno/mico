# QoR Vivado Subset Report

Snapshot date: 2026-06-16.

This report is the M4 reviewer-facing Vivado/QoR evidence record. It supports
only the claim that the selected generated wrappers are not obviously bloated
relative to same-condition hand-written references in a small out-of-context
Vivado subset.

## Scope

The host Vivado subset covers 12 generated/reference task pairs:

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

The flow uses `D:\Application\vivado\2025.2\Vivado`, Vivado `2025.2`, target
part `xc7a35tcpg236-1`, and out-of-context synthesis. Constraint assumptions
are 10 ns clocks on declared clock ports plus zero-delay reset inputs and
`mico_observe` output constraints. There is no board, placement, routing, or
bitstream constraint.

## Reproduction Commands

Run host Vivado first:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

Then run the threshold check inside the Docker EDA environment:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/check-vivado-qor-summary.py --paper-tex paper/tables/vivado_qor_thresholds.tex
```

## Threshold Result

The final 2026-06-16 run produced:

| Metric | Value |
|---|---:|
| Task pairs checked | 12/12 |
| Median generated/reference LUT delta | 0.000% |
| Maximum absolute LUT delta | 0.000% |
| Minimum generated WNS | 4.854 ns |
| Minimum reference WNS | 4.584 ns |
| Total host Vivado elapsed time | 375.721 s |
| Threshold status | pass |

The threshold is median generated/reference LUT delta <= 5.0%, all generated
and reference statuses pass, and all generated/reference WNS values are
nonnegative. The Vivado log contains no `ERROR:`, `CRITICAL WARNING:`, or
`FATAL` diagnostics. Non-critical unconnected-port synthesis warnings are
expected for this measurement-copy flow.

## Artifact Hashes

All artifacts below live under ignored `build/reports/vivado-host/` and are
copied into the release bundle when present.

| Artifact | SHA-256 |
|---|---|
| `vivado_qor_subset_summary.json` | `3747045f2a2f3399555bdf40cedf81ba5a340708e0240f955f873c2437eed05c` |
| `vivado_qor_subset_summary.csv` | `9c5387d8330b85c11ce4b22e92533fa2d57bf6dc2611752abac466871eaa8d2b` |
| `vivado_qor_subset_delta.csv` | `5953f168b6a78e5f11c10e32e6af6642ff09792c9f41c03722d98c5f08d77060` |
| `vivado_qor_subset_summary.tex` | `62f8b650775252d77afe97b739b58853cd441270dd8635bea04b12c2fc555f13` |
| `vivado_qor_thresholds.json` | `d59bb7b14d1c8d204444e480c037fd5900e33b168cac7139161c16bac514aa22` |
| `vivado_qor_thresholds.tex` | `771f238db9f24f4fbefb5f8381185921849261f5047110a39761a7b2e487c3c9` |

## Non-Claims

This report does not support timing closure, routed implementation readiness,
bitstream readiness, CDC correctness, or all-task Vivado QoR. Tasks outside the
12-task subset remain covered only by the Docker open-source validation and
Yosys structural/generic-mapped QoR proxy where the benchmark manifest enables
that proxy.
