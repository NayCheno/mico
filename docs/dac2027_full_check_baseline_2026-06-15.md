# DAC 2027 Full-Check Baseline 2026-06-15

This records the final local release-candidate procedure for the DAC 2027 plan.
Generated evidence remains under ignored `build/` paths and is not committed.
The authoritative per-run hashes are the `release/full_check_manifest.json` and
`artifact_manifest.json` files inside the generated release ZIP.

## Commands

Run from a clean `main` after the final source commit:

```powershell
git status --short
git diff --check
.\scripts\full-check.ps1 -WithLatex
.\scripts\make-release-bundle.ps1
```

## Expected Gates

- Rust fmt/check/test pass in Docker.
- Docker EDA smoke passes.
- Public-development benchmark passes 62/62 expected outcomes, 36/36
  simulation, 31/31 single-clock formal, 9/9 QoR, and 26/26 unsafe rejection.
- Held-out benchmark passes 20/20 expected outcomes, 10/10 simulation, 9/9
  single-clock formal, 3/3 QoR, and 10/10 unsafe rejection.
- LLM provider validate-only and batch validate-only pass without printing or
  bundling API keys.
- JSON schema validation passes for deterministic, held-out, LLM validate-only,
  and aggregate records.
- Host LaTeX builds `paper/main.tex`.
- Host Vivado subset evidence is present and hashed when available.
- The release bundle rejects local provider configs, build/target paths, logs,
  Vivado project outputs, secret-like strings, and full authenticated execute
  JSON records.

## Archive Target

Release tag: `dac2027-rc-2026-06-15`

GitHub Release URL:
`https://github.com/NayCheno/mico/releases/tag/dac2027-rc-2026-06-15`

The release archive attaches only the generated ZIP and `.sha256` sidecar. Full
authenticated execute-result JSON is hashed in the release manifest when present
but is not bundled by default; publish it separately only after an additional
sanitization review.

## Claim Boundary

The release candidate supports the bounded DAC Branch A claim for the tested
OpenCode Go profiles, public-development and held-out manifests, compiler
gates, EDA smoke/formal/QoR evidence, and recorded repair fallback. It does not
claim arbitrary-model generalization, broad free-form repair, CDC correctness
proof, routed timing closure, full task-specific formal proof, or arbitrary LTL
checking.
