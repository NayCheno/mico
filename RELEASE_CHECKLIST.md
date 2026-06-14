# MICO Release Candidate Checklist

This checklist freezes a reproducible "engineering + experiment + paper" release candidate. All Rust, Python, benchmark, LLM validate-only, and open-source EDA checks must run in the repository Docker environment. The only host exceptions are Windows-host LaTeX for `paper/main.tex` and Vivado through `scripts/run-vivado-host.ps1`.

## One-command Gate

Windows:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

Linux/WSL:

```bash
./scripts/eda-docker.sh bash scripts/full-check.sh --llm-config config/llm-provider.local.yaml
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

The Docker gate writes `build/release/full_check_manifest.json`. This file is a generated build artifact and must not be committed.

## Metadata To Record

The release manifest records:

- host Docker version when the PowerShell wrapper provides it;
- Docker image tool versions from `mico-verify-tools`;
- Rust `rustc` and `cargo` versions;
- open-source EDA tool versions for Verilator, Icarus, Yosys, SymbiYosys, and Z3;
- Python version;
- prompt SHA-256 hashes for files under `prompts/`;
- selected LLM model/profile metadata with API keys redacted;
- benchmark manifest SHA-256 hash;
- result JSON SHA-256 hashes for deterministic, LLM validate-only, provider validate-only, and aggregate outputs;
- current source commit hash and latest paper commit hash.

## Required Checks

- `mico-verify-tools` passes in Docker.
- `cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace` passes in Docker.
- `bash scripts/eda-smoke.sh` passes in Docker.
- `python3 benchmarks/run_bench.py --output build/bench/seed_results.json` passes in Docker.
- `python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only --output build/llm/provider_validate.json` passes in Docker without provider requests.
- `python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json` passes in Docker without provider requests.
- `python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_validate.json --out-json build/bench/aggregate_results.json` passes in Docker.
- `python3 -m json.tool` validates every generated JSON result used by the release manifest.
- Host `latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex` passes when `-WithLatex` is requested.
- `git status --short` is empty before publishing the release branch or tag.

## Do Not Commit

Do not commit `build/`, `target/`, `rust_project/target/`, `logs`, `reports`, Vivado project output, generated PDFs, LaTeX temporaries, local YAML files, API keys, provider responses, or caches. The full-check script also fails if any generated-output path from the release policy is already tracked by Git.

## Release Branch Or Tag

Use a release branch for this candidate:

```powershell
git switch -c codex/release-candidate-2026-06-14
```

Create an immutable tag only after the release branch has been reviewed and the final artifact bundle is approved.
