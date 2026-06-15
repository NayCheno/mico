# Reproduction Workflow

This is the shortest path from a clean checkout to a locally reproducible MICO artifact. The default validation surface is the persistent Ubuntu 24.04 Docker EDA image. Windows-host tools are exceptions only for Vivado-specific flows and paper LaTeX. Numeric result claims in this workflow are tracked in `docs/release_claim_table.md`.

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

## Full Release Gate

Run the full release-candidate gate from Windows:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

The PowerShell wrapper prints the host Docker version, runs
`scripts/full-check.sh` inside the Docker EDA image, and optionally compiles the
paper with host LaTeX. The Docker gate verifies tools, runs Rust
fmt/check/tests, runs EDA smoke, runs the deterministic benchmark, validates the
LLM provider config without provider requests, plans the full LLM baseline
matrix without provider requests, runs the held-out benchmark split,
regenerates aggregate result tables for both public-development and held-out
results, validates JSON outputs against the repository JSON Schemas, and writes
`build/release/full_check_manifest.json`. With `-WithLatex`, the wrapper also
updates the manifest with the final paper PDF hash after the host LaTeX build.
The Docker gate also writes
`build/release/deterministic_evidence_hashes.json`, a sidecar for public and
held-out manifests, deterministic benchmark JSON, aggregate JSON, generated
tables, and tool versions.

Linux/WSL equivalent:

```bash
./scripts/eda-docker.sh bash scripts/full-check.sh --llm-config config/llm-provider.local.yaml
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

Read the top-level `RELEASE_CHECKLIST.md` before publishing a release branch or
tag. The generated release manifest records tool versions, prompt hashes,
selected model/profile metadata, public-development and held-out benchmark
manifest hashes, result JSON hashes, optional Vivado subset hashes, the latest
paper commit hash, and the final paper PDF hash when available; it is an
ignored build artifact and must not be committed. The deterministic evidence
sidecar records the non-LLM release evidence hashes used by M1. Package the
review ZIP and bundle sidecar with:

```powershell
.\scripts\make-release-bundle.ps1
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
.\scripts\eda-docker.ps1 bash -lc "cd rust_project && cargo run -q -p mico_cli -- verify --eda --json --artifact-dir ../build/mico-verify/stream_fifo_cli --schema-path ../schemas examples/stream_fifo.mico | python3 -m json.tool >/dev/null"
```

Expected behavior:

- valid examples exit with status 0;
- `examples/invalid_width.mico` exits nonzero and reports a semantic diagnostic;
- generated SystemVerilog is written to stdout unless redirected under `build/`.
- `verify --eda` writes emitted wrapper/SVA and per-tool stdout/stderr files
  under the requested ignored artifact directory.

## RTL And EDA Smoke

Run the open-source RTL smoke flow in Docker:

```powershell
.\scripts\eda-docker.ps1 bash -lc "bash scripts/eda-smoke.sh"
```

The smoke flow emits wrappers and SVA skeletons for stream FIFO, CDC FIFO, and width adapter examples. It checks wrappers with Verilator, Icarus, and Yosys, checks SVA skeletons with Verilator, and runs a minimal SymbiYosys smoke proof. The CDC leaf is smoke-only collateral, not a formal CDC proof.

For a single-example CLI entry point, use `mico_cli verify --eda`; for the
multi-example smoke plus SymbiYosys entry point, keep using `scripts/eda-smoke.sh`.

## ModuleComposeBench

Run the deterministic benchmark runner:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --output build/bench/seed_results.json"
```

Expected current benchmark result:

- `expected_outcome_pass: 62/62`
- `compose_pass_1: 36/36` for positive tasks
- `lint_pass: 36/36` for positive tasks
- `sim_pass: 36/36` for positive tasks; all 36 use committed directed
  Icarus testbenches
- `formal_pass: 31/31` for single-clock formal smoke tasks; all 31 use
  committed directed monitors
- `qor_available: 9/9` for positive tasks with committed reference wrappers;
  availability includes structural and generic-mapped Yosys stat reports
- `unsafe_rejection: 26/26` for negative tasks
- `json_ast_path: 62/62` for source-to-AST-to-check equivalence
- CDC formal proof, full timing closure, technology-mapped delay, and broad
  Vivado QoR remain outside the deterministic benchmark runner. A separate
  representative Vivado subset is documented below.
- T058--T062 provide dedicated streaming, width-bridge, register/status,
  protocol-bridge, and telemetry subsystem case studies; broader latency and
  bus IP studies remain future work.
- `build/bench/qor_summary.csv` and `build/bench/qor_summary.tex` are generated
  from the benchmark JSON and remain ignored build artifacts.

Run the held-out split separately:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json"
```

Expected held-out result:

- `expected_outcome_pass: 20/20`
- `compose_pass_1: 10/10`
- `lint_pass: 10/10`
- `sim_pass: 10/10`
- `formal_pass: 9/9`
- `qor_available: 3/3`
- `unsafe_rejection: 10/10`
- `json_ast_path: 20/20`
- mode split: 10 declared and 0 generated simulations, plus 9 declared and 0
  generated single-clock formal checks; the explicit CDC held-out case remains
  formal not-run

Both public-dev and held-out benchmark JSON records include the manifest path
and manifest SHA-256.

Generate aggregate CSV and paper-table snippets from deterministic results:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --bench-result build/bench/seed_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_validate.json --aggregate-result build/bench/aggregate_results.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/heldout_results.json"
```

The schema validation command also checks
`benchmarks/module_compose_bench_manifest.yaml` against
`benchmarks/manifest_schema.json`.

This writes `build/bench/aggregate_results.json`, deterministic CSVs under
`build/bench/`, and LaTeX snippets under `build/paper_tables/`. The
deterministic aggregation includes the main result table, per-level breakdown,
unsafe diagnostic taxonomy, structural and generic-mapped QoR rows, and ablation/counterfactual
rows.

The current authenticated low-cost LLM matrix summary is documented in
`docs/16_llm_matrix_results.md`. Raw result JSON and response caches remain
ignored build artifacts.

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

Optionally write the validate-only metadata record under ignored `build/llm/`:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only --output build/llm/provider_validate.json
```

Run the cheap provider smoke test:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --output build/llm/provider_smoke.json
```

The output schema is `mico.llm.run.v0` in `schemas/llm_run.schema.json`. It records prompt hash, model/profile, repair turns, optional compiler and EDA result JSON, usage, and configured cost estimates without storing or printing API keys. Repository example cost rates are intentionally `null`; keep real rates in the ignored local YAML if needed.

Use low-cost profiles first: `smoke`, then `low_cost_crosscheck`. Escalate to higher-cost profiles only after compiler acceptance and RTL validation pass.

Plan the full LLM benchmark matrix without making provider requests:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json"
```

Exercise the runner's compiler and open-source EDA scoring path without paid
requests:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke --task-id T004_direct_stream --task-id T005_invalid_width_no_adapter --offline-fixture --output build/llm/bench_offline_fixture.json"
```

Merge deterministic and LLM batch artifacts into one aggregate result:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_validate.json --llm-result build/llm/bench_offline_fixture.json"
```

When LLM result files are supplied, the aggregator also emits LLM summary,
repair-turn distribution, token/cost, paired comparison, and failure taxonomy
CSV/TeX outputs. Validate-only runs are marked as not scored and are not counted
as failed pass-rate evidence.

Authenticated benchmark execution is opt-in and writes ignored artifacts under
`build/llm/`. Run full matrices only when local cost settings and budget are
intentional:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke --baselines mico_source --task-id T004_direct_stream --execute --output build/llm/bench_execute_smoke.json"
```

## Vivado Host Exception

Use Vivado only for Xilinx-specific project synthesis, implementation, bitstream generation, or IP handling. Run it on the Windows host:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\path\to\flow.tcl
```

The host launcher is pinned to `D:\Application\vivado\2025.2\Vivado\bin\vivado.bat` by default and rejects Vivado paths outside `D:\Application\vivado\2025.2\Vivado`. It writes journals and logs under ignored `build/reports/vivado-host/` unless `-ReportDir` is set to another ignored output directory. Keep Vivado project directories, bitstreams, and generated reports out of source control unless a reviewer explicitly requests a small text report.

For the current representative QoR/timing subset, run:

```powershell
.\scripts\run-vivado-host.ps1 -Source .\scripts\vivado-qor-subset.tcl
```

The subset targets `xc7a35tcpg236-1` and writes
`vivado_qor_subset_summary.json`, `vivado_qor_subset_summary.csv`, and
`vivado_qor_subset_delta.csv` under ignored `build/reports/vivado-host/`.
It uses build-only measurement copies for all 12 QoR-enabled public and
held-out tasks (`T001`--`T004` and `T058`--`T065`) and does not claim
board-level implementation, route timing closure, or all-task Vivado QoR.

## Paper Build

The paper's main deterministic result table is read from generated aggregate
output under `build/paper_tables/`. Run the benchmark aggregation command before
compiling from a clean checkout:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json"
```

Compile the paper with the Windows-host LaTeX distribution:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

If `latexmk` is unavailable, run the documented `pdflatex`/`bibtex` fallback from the paper workflow. Do not compile the paper in Docker under the current repository policy.

## Release Candidate Checklist

Before publishing a result or submission artifact:

- `.\scripts\full-check.ps1 -WithLatex` passes, or the Docker gate plus host
  LaTeX commands above pass separately.
- `git status --short` shows no unstaged or staged source changes.
- Docker tool verification passes.
- Rust format, check, and tests pass.
- EDA smoke passes with Verilator, Icarus, Yosys, and the SymbiYosys smoke proof.
- ModuleComposeBench runner writes a JSON result under `build/bench/` with
  expected, lint, simulation, selected formal, QoR, case-study, and unsafe-rejection
  summaries.
- Benchmark aggregation writes `build/bench/aggregate_results.json` plus CSV and
  TeX snippets under ignored build directories.
- The representative Vivado subset passes if host Vivado is available at the
  pinned path; generated reports remain ignored.
- LLM provider validate-only passes; the batch LLM runner can generate the
  validate-only matrix and offline-fixture smoke outputs; authenticated smoke
  runs only when a local key and budget are configured.
- Paper LaTeX build completes on the Windows host.
- No generated outputs, local configs, logs, PDFs, or secrets are staged.
