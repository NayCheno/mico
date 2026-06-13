# Tcl Flow Patterns

Use Tcl as a thin, deterministic orchestration layer around EDA tools. Keep the design intent in RTL and constraints; keep tool state in generated build directories.

## Flow Shape

Prefer this split:

```text
scripts/
  common_flow.tcl        shared path/env/filelist helpers
  lint.tcl               lint setup
  sim_compile.tcl        simulator compile/elaboration
  yosys_synth.tcl        open-source synthesis/check
  vivado_project.tcl     FPGA project-mode synthesis/implementation
  openroad_entry.tcl     physical design entry point
constraints/
  clocks.sdc
  board.xdc
filelists/
  rtl.f
```

Each stage script should be rerunnable from a clean checkout.

## Configuration

Use environment variables for CI and batch flow entry points:

- `TOP`: top module
- `RTL_FILELIST`: ordered source list
- `BUILD_DIR`: generated outputs
- `REPORT_DIR`: reports
- `PART`: FPGA part for Vivado
- `SDC` or `XDC`: timing constraints
- `JOBS`: parallel job count

Use a project config Tcl only when the variable set becomes large.

## Tool Routing

Run non-Vivado tools in the repository Docker image:

```bash
./scripts/eda-docker.sh bash -lc "yosys -V && verilator --version"
```

On Windows PowerShell:

```powershell
.\scripts\eda-docker.ps1 bash -lc "yosys -V && verilator --version"
```

Use Windows-host Vivado only for Vivado-specific Tcl:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado_project_flow.tcl
```

Do not require Windows users to install Yosys, Verilator, Icarus, GHDL, Tcl, or Python EDA packages directly on the host.

The Docker image and Cargo cache volumes persist locally:

- `mico-eda:ubuntu24.04`
- `mico-cargo-registry`
- `mico-cargo-git`

Set `MICO_EDA_REBUILD=1` only when the Dockerfile or toolchain intentionally needs a rebuild.

## Robust Tcl Idioms

- Normalize paths with `file normalize`.
- Build paths with `file join`; never concatenate with `/`.
- Use `file mkdir` before writing outputs.
- Use `catch` around optional tool commands only when the failure is truly optional.
- Fail with a clear message when required inputs are missing.
- Keep one report path per check; overwrite intentionally with `-force` when the tool supports it.
- Avoid hidden dependence on the current working directory.

## Filelists

Filelists should preserve compile order and keep metadata simple:

```text
+incdir+rtl/include
+define+SYNTHESIS
rtl/pkg.sv
rtl/block.sv
rtl/top.sv
```

When the tool supports both `.f` and Tcl filelists, keep one canonical source and generate the other if needed. Avoid duplicating file order in multiple places.

## Yosys Pattern

Use Yosys for fast open-source synthesis sanity when language support is sufficient:

1. read libraries if required
2. read RTL with `-sv` when using SystemVerilog subset
3. run `hierarchy -check -top`
4. run synthesis or process/opt passes
5. write a netlist or JSON
6. emit `stat` and check logs

Do not treat Yosys success as full vendor signoff when the target requires vendor-specific IP, RAMs, DSPs, or constraints.

## Vivado Pattern

Use project-mode Tcl when a project artifact helps FPGA iteration:

1. `create_project -force`
2. `add_files` from ordered filelist
3. set top and include directories
4. add XDC constraints
5. `update_compile_order`
6. `launch_runs synth_1`
7. open run and write timing/utilization reports

Keep board constraints separate from generic RTL constraints.

In this repository, Vivado is a host exception. Do not attempt to install Vivado inside Docker.

## OpenROAD Pattern

Use OpenROAD Tcl as a stage entry point after synthesis or with OpenROAD Flow Scripts:

1. read LEF/liberty as required by the PDK
2. read synthesized Verilog or database
3. link design
4. read SDC
5. write an initial database checkpoint
6. emit timing, design-area, and setup reports

For full RTL-to-GDS, prefer OpenROAD Flow Scripts' configuration model rather than inventing a complete flow from scratch.

## What Not To Do

- Do not store absolute local paths in committed Tcl.
- Do not mix source RTL and generated netlists in the same directory.
- Do not make GUI clicks part of the required flow.
- Do not silently continue after missing constraints.
- Do not bury waivers in broad command-line flags without a tracked waiver file.
