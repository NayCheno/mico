# Vivado QoR Subset

Snapshot date: 2026-06-14.

This records the M5 Vivado/QoR hardening step for the DAC 2027 plan. It adds a
small, representative host-Vivado synthesis and timing subset without expanding
the deterministic benchmark runner's normal Yosys QoR scope.

## Scope

The Vivado subset covers four representative positive tasks:

- `T001_stream_fifo`
- `T003_width_adapter`
- `T058_streaming_accelerator_case`
- `T063_axi_apb_wrapper_case`

The flow uses `D:\Application\vivado\2025.2\Vivado` through
`scripts/run-vivado-host.ps1`, targets `xc7a35tcpg236-1`, and writes all
reports under ignored `build/reports/vivado-host/`.

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
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/m4_results.json --out-json build/bench/aggregate_m5.json"
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
| `T063_axi_apb_wrapper_case` | 16 | 16 | 5.496 | 5.496 |

All rows passed out-of-context synthesis and timing-path extraction in the
measurement-copy flow. FF, BRAM, and DSP counts are zero for this subset because
the current representative leaf RTL is combinational smoke collateral.

Artifact hashes from the ignored report directory:

| Artifact | SHA-256 |
|---|---|
| `build/reports/vivado-host/vivado_qor_subset_summary.json` | `57d4a113b7a6defdb4f1fd640fc0c97fdc7f6f25599fe10f1c6b8965377b8dd0` |
| `build/reports/vivado-host/vivado_qor_subset_summary.csv` | `f72d27ca85fcb50f54fd5fa7636099bd6fb2fb1daf083d98e11673785d1947b8` |
| `build/reports/vivado-host/vivado_qor_subset_delta.csv` | `eaa4f6dc696d421d19a64751bcae121120178881dc0ab6da46fbf0c7464f544a` |

## Claim Boundary

This evidence supports only a representative Vivado out-of-context synthesis and
measurement-copy timing subset for four tasks. It does not support board-level
implementation, route timing closure, bitstream generation, CDC timing signoff,
technology-mapped conclusions for all 62 public-development tasks, or QoR
claims beyond the reported generated/reference rows.
