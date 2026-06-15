# MICO Anonymous Artifact Quickstart

This archive is a review artifact for the MICO submission candidate. It is
intended to be inspected with the included hash manifests rather than committed
back into the source repository.

## Contents

- `source/`: a `git archive` snapshot for the exact source commit.
- `release/full_check_manifest.json`: tool versions, manifest hashes, result
  hashes, optional Vivado hashes, and the paper PDF hash.
- `release/deterministic_evidence_hashes.json`: deterministic benchmark and
  table hashes.
- `release/release_claim_table.json`: machine-readable claim-to-evidence map
  generated from `docs/release_claim_table.md`.
- `release/llm_evidence_hashes.json`: sanitized hashes and metadata for the
  authenticated v3 LLM evidence.
- `results/`: deterministic, validate-only LLM, optional authenticated LLM,
  aggregate, and Vivado result JSON/CSV/TeX files.
- `tables/`: deterministic aggregate CSV/TeX files plus authenticated v3 LLM
  CSV/TeX table directories when the final LLM matrix artifacts are present.
- `paper/main.pdf`: host-LaTeX-built paper PDF.
- `artifact_manifest.json`: per-file hashes for every file in the archive.

## Expected Runtime

Typical local runtime on the evaluation Windows workstation:

- Docker full-check without LaTeX: about 8 minutes after the image is built.
- Host LaTeX paper build: under 1 minute after TeX Live is installed.
- Host Vivado QoR subset: about 6--7 minutes when the pinned Vivado install is
  available; this step is optional for reviewers without Vivado.
- Bundle creation and SHA-256 sidecar: under 1 minute.
- Authenticated LLM execute reruns can be much longer and may incur provider
  cost; the bundle includes sanitized v3 execute evidence when present, so
  reviewers can validate hashes and aggregate statistics without replaying paid
  provider requests.

## Quick Check

1. Verify the ZIP sidecar:
   `Get-FileHash -Algorithm SHA256 mico-release-candidate-<commit>.zip`.
2. Open `artifact_manifest.json` and compare the `included_files` hashes for
   any evidence file used in review.
3. Check `release/full_check_manifest.json` for the expected source commit,
   Docker/tool versions, manifest hashes, and paper PDF hash.
4. Check `release/release_claim_table.json` before relying on any numeric
   claim in the paper.

## Known Limitations

The artifact does not claim CDC correctness proof, arbitrary LTL, routed timing
closure, broad Vivado QoR beyond the 12-task out-of-context subset, broad
free-form LLM repair, or generalization beyond the tested v3 OpenCode Go
profiles and prompts.

The bundle intentionally excludes local provider configs, API keys, raw provider
caches, Vivado project output, target directories, logs, and source-tree build
artifacts.
