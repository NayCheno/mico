# MICO Release Gate Report 2026-06-15

This report records the M1 release-gate hardening validation run. Generated
outputs stayed under ignored `build/` paths and are not committed.

## Scope

The M1 change hardened the release gate by:

- running `git diff --check` inside the Docker release gate;
- running `scripts/check-doc-claims.py` inside the Docker release gate;
- allowing a clean checkout without `config/llm-provider.local.yaml` to use
  `config/llm-provider.example.yaml` for validate-only LLM checks;
- making authenticated v3 LLM evidence hashes optional by default while keeping
  `scripts/write-llm-evidence-hashes.py --require` for the final M5 evidence
  seal.

## Command

Run from `main` with M1 source changes present before the M1 commit:

```powershell
.\scripts\full-check.ps1 -Manifest build/release/m1_full_check_manifest.json
```

This was a functional gate validation, not the final release evidence. The
final release manifest must be regenerated after the final source commit.

## Result

The Docker release-candidate gate passed.

Observed gate results:

- Docker tool verification passed with Rust 1.96.0 and the repository EDA
  image tools.
- `git diff --check` passed.
- `scripts/check-doc-claims.py` passed.
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
- LLM provider validate-only and LLM batch validate-only passed without
  provider requests.
- JSON schema validation passed for deterministic, held-out, realism, LLM
  validate-only, and aggregate records.
- Release manifest, release claim-table JSON, LLM evidence sidecar, paper table
  snippets, and deterministic evidence sidecar generation passed.

## Follow-Up

After all source commits for the release candidate are complete, rerun:

```powershell
.\scripts\full-check.ps1 -WithLatex
.\scripts\make-release-bundle.ps1
```

The final manifest and bundle must describe the final source commit and clean
tree state before any external archive or tag is created.
