# MICO Release Gate Report 2026-06-16

This report records the M1 release-gate run for source commit
`ac27fae603ffcbdd6dec1be2fdf7ea0c7dc13eec` on `main`. Generated evidence
stayed under ignored `build/` paths, and the generated paper PDF stayed ignored
under `paper/main.pdf`.

## Command

Run from a clean working tree:

```powershell
git status --short
git diff --check
.\scripts\full-check.ps1 -WithLatex
```

The release branch step from the planning checklist was intentionally not
performed because the active goal requires all commits to stay on `main`.

## Result

The release-candidate gate passed.

Observed results:

- `git status --short` was clean before the run.
- `git diff --check` passed.
- Docker tool verification passed with Rust 1.96.0 and the repository Ubuntu
  24.04 EDA image.
- `scripts/check-doc-claims.py` passed in Docker.
- `cargo fmt --check`, `cargo check --workspace`, and
  `cargo test --workspace` passed in Docker.
- `scripts/eda-smoke.sh` passed, including the SymbiYosys smoke proof.
- Public-development benchmark passed: 62/62 expected outcomes, 36/36 lint,
  36/36 simulation, 31/31 bounded single-clock formal smoke, 9/9 QoR, 26/26
  unsafe rejection, and 62/62 JSON AST path.
- Held-out benchmark passed: 20/20 expected outcomes, 10/10 lint, 10/10
  simulation, 9/9 bounded single-clock formal smoke, 3/3 QoR, 10/10 unsafe
  rejection, and 20/20 JSON AST path.
- Supplemental realism benchmark passed: 14/14 expected outcomes, 7/7 lint,
  7/7 simulation, 6/6 bounded single-clock formal smoke, 7/7 unsafe rejection,
  and 14/14 JSON AST path.
- LLM provider and LLM batch checks ran in validate-only mode for the release
  gate; no provider request was required for this gate.
- JSON schema validation passed for deterministic, held-out, realism, LLM
  validate-only, and aggregate records.
- Host LaTeX `latexmk -cd -pdf -interaction=nonstopmode -halt-on-error
  paper/main.tex` completed and the release manifest was updated with the paper
  PDF hash.

## Generated Evidence Hashes

These files are release evidence and remain ignored by git:

| Artifact | SHA-256 |
|---|---|
| `build/release/full_check_manifest.json` | `98cfc0bbbed21e84c62a8948384dd4bdcf0775acc89850355587c9828c6b237d` |
| `build/release/deterministic_evidence_hashes.json` | `c552317ab6981dbb577fd439a3933740c7169c712b6db53b5c7c6963cdeb2d18` |
| `build/release/release_claim_table.json` | `15cbd39728001616b76b6693a4a75a031e3c6060c4712dc642b863f100121f60` |
| `build/release/llm_evidence_hashes.json` | `a7f4eec4130d4d4ee91a3fcde6a41f291fd20455b31057ba50273d9bf7aada49` |
| `paper/main.pdf` | `163c0775d33d2d4da0f93c2aa3a1e1ced2d5f21250a2294f1d2c27c347038253` |

## Policy Check

No `build/`, `target/`, logs, Vivado project outputs, paper PDF, local provider
YAML, API keys, or secrets are committed by this report.
