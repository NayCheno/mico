---
name: rtl-tcl-engineer
description: Apply mature RTL project methodology and generate maintainable EDA Tcl flows. Use when structuring Verilog/SystemVerilog RTL repositories, writing or reviewing RTL blocks, building filelists, creating lint/simulation/synthesis/formal/CDC/STA flows, or producing Tcl scripts for Yosys, Vivado, OpenROAD, or vendor EDA tools.
---

# RTL Tcl Engineer

## Overview

Use this skill to turn RTL work into a repeatable engineering flow: clear source layout, explicit clock/reset/CDC contracts, deterministic filelists, staged quality gates, and Tcl scripts that can run in batch mode. Draw from mature open RTL programs such as lowRISC/OpenTitan and from open EDA flows such as Yosys and OpenROAD, but keep the output sized to the target repository.

## First Pass

1. Identify the target: new RTL block, existing RTL cleanup, FPGA build, ASIC synthesis, lint/formal flow, or Tcl automation.
2. Inspect local structure before proposing layout:
   - RTL: `rtl/`, `src/`, `hw/`, `ip/`, `constraints/`
   - verification: `dv/`, `tb/`, `formal/`, `sim/`
   - flows: `scripts/`, `syn/`, `fpga/`, `openroad/`, `vivado/`, `Makefile`, `.f` files, `.tcl` files
3. Separate concerns:
   - RTL source and packages
   - constraints and clocks
   - filelists and dependency order
   - tool scripts
   - reports and generated outputs
4. Prefer batch-reproducible flows over GUI state.
5. Keep generated Tcl templates parameterized by environment variables or a small project config.

## Environment Rule

- Use the Ubuntu 24.04 Docker EDA image for all non-Vivado compilation and testing: Rust, Yosys, Verilator, Icarus Verilog, GHDL, Tcl, Python EDA helpers, and open-source synthesis/formal checks.
- Reuse the persistent `mico-eda:ubuntu24.04` Docker image and named Cargo cache volumes; do not reinstall the environment each run.
- Preserve the Docker image's Tsinghua TUNA apt/PyPI policy and RsProxy rustup/Cargo policy unless the user explicitly asks to change mirrors.
- Use Windows-host Vivado only when the task specifically requires Vivado, Xilinx IP, XDC-based FPGA project synthesis/implementation, or bitstream generation.
- Paper LaTeX is also a Windows-host workflow and should use `$paper-latex-writer`.
- Use `scripts/eda-docker.ps1` on Windows or `scripts/eda-docker.sh` on Linux/WSL for Docker commands.
- Use `scripts/run-vivado-host.ps1` for Vivado batch Tcl on Windows.
- Read `docs/12_docker_eda_environment.md` when deciding where a command should run.

## Reference Routing

- Read `references/rtl-methodology.md` when designing RTL structure, reviewing RTL style, planning quality gates, or aligning MICO-generated SystemVerilog with hardware practice.
- Read `references/tcl-flow-patterns.md` when creating, reviewing, or adapting Tcl scripts for EDA tools.
- Use `$opencode-go-provider` when an RTL or Tcl task includes LLM-generated candidates, model evaluation, or OpenCode Go provider configuration.
- Use files in `assets/tcl/` as templates when the user wants scripts:
  - `common_flow.tcl`: reusable Tcl utilities for paths, environment variables, filelists, and report directories
  - `yosys_synth.tcl`: Yosys batch synthesis/check template
  - `vivado_project_flow.tcl`: Vivado project-mode synthesis template
  - `openroad_entry.tcl`: OpenROAD database/netlist/constraint entry template

## RTL Engineering Rules

- Treat reset and clocking as part of the interface contract, not incidental signals.
- Make CDC/RDC crossings explicit. Require synchronizers, async FIFOs, handshakes, or adapter modules; do not hide them in glue logic.
- Keep combinational and sequential logic separated in reviewable blocks.
- Prefer `always_ff`, `always_comb`, packages, interfaces, enums, and structs for SystemVerilog when the tool flow supports them.
- Use parameters for legitimate configurability, but avoid parameter combinations that are unverified.
- Keep lint, simulation, formal, synthesis, and timing reports as named artifacts under a build/report directory.
- Add assertions for protocol assumptions and invariants close to the RTL or bound in from verification files.

## Tcl Engineering Rules

- Make scripts batch-safe: no GUI-only state, no relative-path ambiguity, no hidden current directory assumptions.
- Fail fast on missing environment variables, missing filelists, unknown top modules, and missing constraints.
- Emit reports with stable names.
- Keep one common Tcl utility file and one tool/stage script per flow stage.
- Do not hard-code machine-local install paths, license server values, user directories, or absolute build paths.
- Keep generated outputs under `build/`, `out/`, or `reports/`, never mixed with source RTL.

## Validation

For Tcl templates:

```bash
./scripts/eda-docker.sh bash -lc "tclsh .codex/skills/rtl-tcl-engineer/assets/tcl/common_flow.tcl"
```

When a target EDA tool is installed, validate in that tool's batch mode instead of plain `tclsh`, because commands such as `read_verilog`, `create_project`, and `read_lef` are tool-provided.

For repository flows, prefer staged checks:

```bash
# examples; adapt to the local repo
./scripts/eda-docker.sh bash -lc "verilator --lint-only -Wall -f rtl/filelist.f"
./scripts/eda-docker.sh bash -lc "yosys -c scripts/yosys_synth.tcl"
./scripts/eda-docker.sh bash -lc "openroad -exit scripts/openroad_entry.tcl"
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado_project_flow.tcl
```

If Docker is unavailable, state that non-Vivado validation was not run. If Vivado is unavailable on Windows, state that Vivado validation was not run.
