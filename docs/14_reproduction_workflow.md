# Reproduction Workflow

This is the shortest path from a clean checkout to a locally reproducible MICO artifact. The default validation surface is the persistent Ubuntu 24.04 Docker EDA image. Windows-host tools are exceptions only for Vivado-specific flows and paper LaTeX.

## Prerequisites

- Docker Desktop or compatible Docker engine.
- PowerShell on Windows, or a POSIX shell on Linux/WSL.
- Windows-host LaTeX distribution for `paper/main.tex`.
- Optional Windows-host Vivado for Xilinx-specific Tcl flows.
- Optional OpenCode Go API key for authenticated LLM smoke tests.

Do not commit generated outputs, local provider config, logs, or credentials. The repository ignores `build/`, `rust_project/target/`, `config/*.local.yaml`, paper PDFs and LaTeX temporary files, and EDA reports.

## Docker EDA Environment

Verify or build the persistent Docker image:

```powershell
.\scripts\eda-docker.ps1 mico-verify-tools
```

Linux/WSL equivalent:

```bash
./scripts/eda-docker.sh mico-verify-tools
```

The image is `mico-eda:ubuntu24.04`; Cargo registry and git caches use Docker named volumes. Force a rebuild only after changing `docker/eda/Dockerfile`:

```powershell
$env:MICO_EDA_REBUILD = "1"
.\scripts\eda-docker.ps1 mico-verify-tools
Remove-Item Env:\MICO_EDA_REBUILD
```

## Rust Compiler

Run the Rust compiler checks inside Docker:

```powershell
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace"
```

Run CLI smoke checks:

```powershell
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- check examples/stream_fifo.mico"
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- emit-sv examples/width_adapter.mico"
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- report examples/cdc_fifo.mico"
```

Expected behavior:

- valid examples exit with status 0;
- `examples/invalid_width.mico` exits nonzero and reports a semantic diagnostic;
- generated SystemVerilog is written to stdout unless redirected under `build/`.

## RTL And EDA Smoke

Run the open-source RTL smoke flow in Docker:

```powershell
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
```

The smoke flow emits wrappers for stream FIFO, CDC FIFO, and width adapter examples, then checks them with Verilator and Yosys against `rtl/examples/mico_example_leafs.sv`. The CDC leaf is smoke-only collateral, not a formal CDC proof.

## ModuleComposeBench Seeds

Run the seed benchmark runner:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

Expected current seed result:

- `compose_pass_1: 3/3`
- `lint_pass: 3/3`
- `sim_pass=false` and `formal_pass=false` in JSON output until simulation and formal harnesses are implemented.

## LLM Provider

Create a local provider config from the template if one does not already exist:

```powershell
Copy-Item config\llm-provider.example.yaml config\llm-provider.local.yaml
```

Set `OPENCODE_GO_API_KEY` in the shell or place `provider.api_key` only in the ignored local YAML file. Never commit or print the key.

Validate config without a paid request:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only
```

Run the cheap provider smoke test:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --output build/llm/provider_smoke.json
```

Use low-cost profiles first: `smoke`, then `low_cost_crosscheck`. Escalate to higher-cost profiles only after compiler acceptance and RTL validation pass.

## Vivado Host Exception

Use Vivado only for Xilinx-specific project synthesis, implementation, bitstream generation, or IP handling. Run it on the Windows host:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\path\to\flow.tcl
```

If Vivado is not on `PATH`, set `VIVADO_BIN` to the local `vivado.bat` path. Keep Vivado journals, logs, project directories, bitstreams, and reports out of source control unless a reviewer explicitly requests a small text report.

## Paper Build

Compile the paper with the Windows-host LaTeX distribution:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

If `latexmk` is unavailable, run the documented `pdflatex`/`bibtex` fallback from the paper workflow. Do not compile the paper in Docker under the current repository policy.

## Release Candidate Checklist

Before publishing a result or submission artifact:

- `git status --short` shows no unstaged or staged source changes.
- Docker tool verification passes.
- Rust format, check, and tests pass.
- EDA smoke passes with Verilator and Yosys.
- ModuleComposeBench seed runner writes a JSON result under `build/bench/`.
- LLM provider validate-only passes; authenticated smoke passes when a local key is configured.
- Paper LaTeX build completes on the Windows host.
- No generated outputs, local configs, logs, PDFs, or secrets are staged.
