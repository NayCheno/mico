# MICO Deterministic Evidence Seal 2026-06-17

This report records the P2 deterministic evidence seal for the expanded
public-development, held-out, and supplemental realism splits. Numeric claims
remain governed by `docs/release_claim_table.md`; generated JSON, CSV, TeX,
PDF, and release sidecars stay under ignored `build/` and `paper/*.pdf` paths.

## Commands

The full release-candidate gate was run from the repository root:

```powershell
.\scripts\full-check.ps1 -WithLatex
```

The P2 aggregate command is now supported directly by
`benchmarks/aggregate_results.py`:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --bench-result build/bench/realism_results.json"
```

`scripts/full-check.sh` writes `build/bench/aggregate_results.json` with the
same three deterministic bench inputs, plus the validate-only LLM batch record
used by the release gate.

## Evidence Inputs

| Artifact | SHA-256 |
|---|---|
| `build/bench/seed_results.json` | `a99afa29d04b6e4a95522173ef998dd5ae20c1eca19ecc2452bb7585e6151c6c` |
| `build/bench/heldout_results.json` | `f9520d217f294aaea9b6928dc52b07cd1593ffd505854af1c1960984ce7534bd` |
| `build/bench/realism_results.json` | `6015baea58ef9dc2ce2cf9e184f46e9ac3bdae076c15d7338d37fd22ef9b6f96` |
| `build/bench/aggregate_results.json` | `e129b5f821a6bfefe6340eed1ced0d5e7ae7328c733dd882d099d6249b56dd9e` |
| `build/bench/aggregate_heldout_results.json` | `fb1923bea852a91636094070bc1f334461dd553f8ab25421b41c7adb1faee97a` |
| `build/bench/aggregate_realism_results.json` | `059d67a2663d622cc231251514b752c61b86ca2c51d70e9ac412c7fb12603dcf` |
| `build/release/full_check_manifest.json` | `1684540addefd9c7f9c1e67bbd541d5fac071703f1dcb6ee475b7871f38f365a` |
| `build/release/release_claim_table.json` | `2b645765769a4073aa83dfe6b2ec44b1afb146516b530c29b6f856ce4c777280` |
| `paper/main.pdf` | `a916e64a22f7d7f0a17e6c831fca5d537bad9d0f0a8a05c9cf54f06020270262` |

## Deterministic Results

| Split | Expected | Compose | Lint | Simulation | Formal | QoR | Unsafe | JSON AST |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Public-development | 83/83 | 46/46 | 46/46 | 46/46 | 40/40 | 11/11 | 37/37 | 83/83 |
| Held-out | 40/40 | 20/20 | 20/20 | 20/20 | 17/17 | 6/6 | 20/20 | 40/40 |
| Supplemental realism | 30/30 | 15/15 | 15/15 | 15/15 | 13/13 | 4/4 | 15/15 | 30/30 |

The combined aggregate summary in `build/bench/aggregate_results.json` reports
153/153 expected outcomes, 81/81 positive compose/lint/simulation passes,
72/72 unsafe rejections, 153/153 JSON AST path passes, 70/70 bounded formal
passes, and 21/21 structural QoR rows.

## Guardrails

- `build/bench/aggregate_results.json` records all three benchmark inputs under
  `inputs.bench_results` and all three manifests under `inputs.manifests`.
- The aggregate ablation rows keep evidence scopes separate from task IDs:
  JSON AST schema, compiler feedback, repair boundary, adapter/contract, SV
  lint/elab, and prompt policy.
- LLM provider validation used `config/llm-provider.local.yaml` without
  committing or printing API key material; authenticated LLM claims remain
  bounded by `docs/submission_claim_lock_2026Q3.md`.
- Vivado remains a host-tool exception. All Rust, Python, open-source EDA,
  benchmark, LLM smoke, and release-table generation steps ran through the
  repository Docker wrapper.

## Validation

The following gates passed during this P2 seal:

- `.\scripts\full-check.ps1 -WithLatex`
- Docker tool verification
- `git diff --check`
- `python3 scripts/check-doc-claims.py`
- `cd rust_project && cargo fmt --check && cargo check --workspace && cargo test --workspace`
- `bash scripts/eda-smoke.sh`
- `python3 scripts/validate_json_schemas.py` for seed, held-out, realism, LLM
  validate-only, and aggregate records
- host `latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex`
