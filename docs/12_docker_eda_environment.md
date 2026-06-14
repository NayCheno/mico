# Docker EDA Environment

Use Docker for Rust, compiler, RTL, and open-source EDA validation. The host Windows Vivado installation is the FPGA/Xilinx exception, and paper LaTeX work uses the Windows host LaTeX installation.

The Docker environment is persistent. Build it once as `mico-eda:ubuntu24.04`; the wrapper scripts reuse that local image and persistent Docker volumes on later runs instead of reinstalling the toolchain.

The Dockerfile defaults to Tsinghua University TUNA mirrors:

- Ubuntu apt: `http://mirrors.tuna.tsinghua.edu.cn/ubuntu`
- PyPI: `https://pypi.tuna.tsinghua.edu.cn/simple`

Rust uses RsProxy:

- rustup dist: `https://rsproxy.cn`
- rustup init/update: `https://rsproxy.cn/rustup` and `https://rsproxy.cn/rustup-init.sh`
- Cargo crates.io sparse index: `sparse+https://rsproxy.cn/index/`

Override the Ubuntu mirror only when needed:

```bash
docker build --build-arg UBUNTU_MIRROR=http://archive.ubuntu.com/ubuntu -f docker/eda/Dockerfile -t mico-eda:ubuntu24.04 .
```

The apt mirror intentionally uses HTTP for the bootstrap phase because the
minimal Ubuntu base image does not have `ca-certificates` before the first
`apt-get install`. Rustup and PyPI still use HTTPS after certificates are
installed.

Override Rust sources only when needed:

```bash
docker build \
  --build-arg RUSTUP_DIST_SERVER=https://static.rust-lang.org \
  --build-arg RUSTUP_UPDATE_ROOT=https://static.rust-lang.org/rustup \
  --build-arg RUSTUP_INIT_URL=https://sh.rustup.rs \
  -f docker/eda/Dockerfile -t mico-eda:ubuntu24.04 .
```

## Image

The image is based on Ubuntu 24.04:

```bash
docker build -f docker/eda/Dockerfile -t mico-eda:ubuntu24.04 .
```

Rust is installed with rustup using the required bootstrap command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The Dockerfile installs it non-interactively under `/opt/rust`, adds `rustfmt` and `clippy`, and defaults to Rust 1.96.0. Rust 1.96.0 is the current stable release as of June 13, 2026, per the official Rust release announcement.

## Included Tools

Required apt tools include:

- Rust toolchain via rustup: `rustc`, `cargo`, `rustfmt`, `clippy`
- RTL/EDA: `yosys`, `yosys-smtbmc`, `sby` (SymbiYosys driver), `verilator`, `iverilog`, `ghdl`, `tclsh`, `z3`
- Build/runtime: `build-essential`, `cmake`, `ninja`, `python3`, `pip`, `git`
- Python EDA helpers: `cocotb`, `edalize`, `fusesoc`, `jsonschema`,
  `pytest`, `pyyaml`
- LLM provider smoke tests: OpenAI Python SDK, configured by `$opencode-go-provider`

The Dockerfile also installs optional Ubuntu packages when available, such as `boolector`, `berkeley-abc`, `gtkwave`, `nextpnr-*`, `fpga-icestorm`, `openocd`, and `ghdl-yosys-plugin`. Ubuntu 24.04 does not package the SymbiYosys `sby` driver, so the Dockerfile installs it from the upstream `YosysHQ/sby` repository during image build.

## Windows Usage

From PowerShell:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo test --workspace"
.\scripts\eda-docker.ps1 bash -lc "yosys -V && verilator --version"
```

The first command builds `mico-eda:ubuntu24.04` if it is missing. Later runs reuse the image. To force a rebuild after changing the Dockerfile:

```powershell
$env:MICO_EDA_REBUILD = "1"
.\scripts\eda-docker.ps1 mico-verify-tools
Remove-Item Env:\MICO_EDA_REBUILD
```

Run an interactive shell:

```powershell
.\scripts\eda-docker.ps1
```

## Linux Or WSL Usage

```bash
./scripts/eda-docker.sh mico-verify-tools
./scripts/eda-docker.sh bash -lc "cd rust_project && cargo fmt --check && cargo test --workspace"
```

Force a rebuild:

```bash
MICO_EDA_REBUILD=1 ./scripts/eda-docker.sh mico-verify-tools
```

## Persistence

Docker persists the configured environment locally:

- image: `mico-eda:ubuntu24.04`
- Cargo registry cache volume: `mico-cargo-registry`
- Cargo git cache volume: `mico-cargo-git`
- workspace build outputs: mounted repository paths such as `rust_project/target/`
- apt package downloads: BuildKit cache mounts in `docker/eda/Dockerfile`
- Rust toolchain and Cargo mirror settings are configured after the apt layer, so changing Rust version or Rust mirror build args does not invalidate the apt install layer.

The helper scripts do not rebuild the image when it already exists, unless `MICO_EDA_REBUILD=1` is set. Remove the image or volumes only when intentionally resetting the environment:

```powershell
docker image rm mico-eda:ubuntu24.04
docker volume rm mico-cargo-registry mico-cargo-git
```

## Compose Usage

```bash
docker compose -f docker/eda/compose.yaml build
docker compose -f docker/eda/compose.yaml run --rm eda mico-verify-tools
docker compose -f docker/eda/compose.yaml run --rm eda bash -lc "cd rust_project && cargo test --workspace"
```

## Vivado Exception

Vivado is not installed in the Docker image. Use the Windows host installation:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\path\to\flow.tcl
```

This repository pins the allowed host Vivado root to `D:\Application\vivado\2025.2\Vivado`. The launcher writes Vivado journal and log files under ignored `build/reports/vivado-host/` by default and exports `MICO_VIVADO_REPORT_DIR` for Tcl scripts.

```powershell
.\scripts\run-vivado-host.ps1 -Source .\path\to\flow.tcl
```

`VIVADO_BIN` may only be used to point inside the same required Vivado root.

The current representative Vivado QoR/timing subset is:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

It writes only ignored report files under `build/reports/vivado-host/`.

## Paper LaTeX Exception

Do not compile the paper in Docker. The authoritative IEEE-style paper entrypoint is `paper/main.tex`, with body sections under `paper/sections/`; use the Windows host LaTeX distribution:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

If `latexmk` is unavailable, use the host `xelatex`/`bibtex` fallback documented in `$paper-latex-writer`.

## Policy

- Use Docker for Rust builds, tests, Yosys, Verilator, Icarus Verilog, GHDL, Tcl validation, Python EDA helpers, and open-source synthesis/formal checks.
- Use Windows host Vivado only for Vivado-specific flows, FPGA project synthesis, implementation, bitstream generation, or Xilinx IP handling.
- Use Windows host LaTeX for paper writing and PDF compilation.
- Do not require developers to install open-source EDA tools directly on Windows.
