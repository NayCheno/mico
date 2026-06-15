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

The Docker gate writes `build/release/full_check_manifest.json` and
`build/release/deterministic_evidence_hashes.json`. These files are generated
build artifacts and must not be committed. After the final gate passes, package
the review archive with:

```powershell
.\scripts\make-release-bundle.ps1
```

The bundle and its `.sha256` sidecar are also generated build artifacts under
`build/release/`.

## Evidence Storage Policy

Release evidence is generated under ignored `build/` paths and should be
archived as an external release artifact, not committed to the source tree. The
manifest is source-commit sensitive: rerun the gate after the final release
commit so `source_commit_hash`, prompt hashes, benchmark result hashes, and LLM
validate-only hashes describe the reviewed revision.

If a future review requires result JSON in git, create a dedicated
`artifacts/results/` policy first, store only redacted schema-valid records, and
keep provider caches, API keys, local configs, logs, PDFs, and Vivado outputs
out of the commit.

## Metadata To Record

The release manifest records:

- host Docker version when the PowerShell wrapper provides it;
- Docker image tool versions from `mico-verify-tools`;
- Rust `rustc` and `cargo` versions;
- open-source EDA tool versions for Verilator, Icarus, Yosys, SymbiYosys, and Z3;
- Python version;
- prompt SHA-256 hashes for files under `prompts/`;
- selected LLM model/profile metadata with API keys redacted;
- public-development and held-out benchmark manifest SHA-256 hashes;
- result JSON SHA-256 hashes for deterministic, held-out, LLM validate-only, provider validate-only, and aggregate outputs;
- deterministic evidence sidecar hashes for public-development and held-out
  manifests, deterministic benchmark JSON, aggregate JSON, and generated tables;
- optional v3 authenticated LLM execute and aggregate hashes when those ignored artifacts are present;
- optional Vivado subset summary hashes when host Vivado evidence exists;
- final paper PDF SHA-256 hash when `-WithLatex` is used;
- current source commit hash and latest paper commit hash.

The release bundle manifest records:

- source archive hash;
- full-check manifest hash;
- final `paper/main.pdf` SHA-256 hash;
- included schema, prompt, benchmark manifest, deterministic result, held-out result, validate-only LLM result, sanitized LLM summary, Vivado summary, table, and reproduction-guide file hashes;
- generated bundle SHA-256 sidecar path.

## Required Checks

- `mico-verify-tools` passes in Docker.
- `cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace` passes in Docker.
- `bash scripts/eda-smoke.sh` passes in Docker.
- `python3 benchmarks/run_bench.py --output build/bench/seed_results.json` passes in Docker.
- `python3 benchmarks/run_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --output build/bench/heldout_results.json` passes in Docker.
- `python3 scripts/llm-provider-smoke.py --config config/llm-provider.local.yaml --profile smoke --validate-only --output build/llm/provider_validate.json` passes in Docker without provider requests.
- `python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --profiles smoke,low_cost_crosscheck --output build/llm/bench_validate.json` passes in Docker without provider requests.
- `python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_validate.json --out-json build/bench/aggregate_results.json` passes in Docker.
- `python3 benchmarks/aggregate_results.py --bench-result build/bench/heldout_results.json --manifest benchmarks/module_compose_bench_heldout.yaml --out-json build/bench/aggregate_heldout_results.json --out-dir build/bench/heldout_tables --paper-table-dir build/paper_tables/heldout` passes in Docker.
- `python3 scripts/write-deterministic-evidence-hashes.py --output build/release/deterministic_evidence_hashes.json --full-check-manifest build/release/full_check_manifest.json` passes in Docker.
- `python3 -m json.tool` validates every generated JSON result used by the release manifest.
- Host `latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex` passes when `-WithLatex` is requested.
- `.\scripts\make-release-bundle.ps1` passes after the final source commit and writes the artifact ZIP plus `.sha256` sidecar.
- `git status --short` is empty before publishing the release branch or tag.

## Authenticated LLM Result Handling

The historical low-cost matrix summary is committed as
`docs/16_llm_matrix_results.md`; the v2 authenticated full matrix is retained
as historical evidence in `docs/22_llm_full_matrix_v2.md`. The current
manifest-bound v3 matrix and Branch A decision are summarized in
`docs/24_llm_matrix_v3.md`. Response caches, prompts, and per-attempt scratch
artifacts stay under ignored `build/llm/` paths. The default bundle may include
sanitized v3 execute records and aggregate summaries when present, but it does
not include provider caches, raw response text, local YAML, or API keys.

After the final source commit for a release candidate, rerun:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

The generated `build/release/full_check_manifest.json` must describe that final
commit hash before a release branch is published or an immutable tag is created.
Run `.\scripts\make-release-bundle.ps1` only after that final full-check pass so
the bundle manifest, paper PDF hash, and bundle sidecar all describe the same
source revision.

## Do Not Commit

Do not commit `build/`, `target/`, `rust_project/target/`, `logs`, `reports`, Vivado project output, generated PDFs, LaTeX temporaries, local YAML files, API keys, provider responses, or caches. The full-check script also fails if any generated-output path from the release policy is already tracked by Git.

## Release Branch Or Tag

Use a release branch for this candidate:

```powershell
git switch -c codex/release-candidate-2026-06-14
```

Create an immutable tag only after the release branch has been reviewed and the final artifact bundle is approved.
