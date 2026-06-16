# MICO LLM v3 Artifact README

Snapshot date: 2026-06-16.

This note describes the sanitized v3 LLM artifacts used by the current bounded
Branch A paper claim. The artifacts are generated evidence under ignored
`build/` paths and are not committed.

## Scope

- Provider: OpenCode Go through the OpenAI-compatible chat completions API.
- SDK: OpenAI Python SDK.
- Config template: `config/llm-provider.example.yaml`.
- Local credentials: `config/llm-provider.local.yaml` or
  `OPENCODE_GO_API_KEY`; neither is committed or bundled.
- Profiles: `smoke`, `low_cost_crosscheck`, `quality_code`.
- Baselines: `direct_verilog`, `sv_interface`, `mico_source`,
  `mico_json_ast`, `mico_json_ast_repair`.
- Splits: 62-task public-development and 20-task held-out manifests.
- Supplemental realism: deterministic-only; it is not part of the v3 LLM
  claim.

## Current Evidence Files

| Artifact | Purpose | SHA-256 |
|---|---|---|
| `build/llm/bench_execute_public_dev_v3.json` | Sanitized public-development execute record | `f1c5aa83d2527241eae0b8efbae788b6ae630a8c4ef01de6d9f322fbc5ba9513` |
| `build/llm/bench_execute_heldout_v3.json` | Sanitized held-out execute record | `44f249e02b6e5df0bbb70dfefdbb9ab07f340ff306c780ab2b88b8a395af06ff` |
| `build/bench/aggregate_llm_v3.json` | Aggregated summaries, paired tests, repair, token/cost, and failure taxonomy | `123f8296533f5e07312873c547e8e598454704fb2605e406f977af403c7aedbd` |
| `build/release/llm_evidence_hashes.json` | Release sidecar for sanitized LLM evidence | `07e217f985dbea56818efcfd7602712b091e5a418b80f1b0eddfbaaa806de15d` |

Manifest bindings:

- Public-development manifest:
  `3b8c412659b22fcbcdb5954fc299a6250ecd237988ff85bc398f096b61bf2957`.
- Held-out manifest:
  `8aebbfe8b1c1f9cf67cbb112e6fa9d439e4cded2fd8cf8b270041c8b37380ffd`.

## Regeneration Commands

Run all commands in the Docker EDA environment:

```powershell
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_public_dev_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 scripts/run_llm_bench.py --manifest benchmarks/module_compose_bench_heldout.yaml --config config/llm-provider.local.yaml --execute --profiles smoke,low_cost_crosscheck,quality_code --baselines direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair --output build/llm/bench_execute_heldout_v3.json"
.\scripts\eda-docker.ps1 bash -lc "python3 benchmarks/aggregate_results.py --bench-result build/bench/seed_results.json --llm-result build/llm/bench_execute_public_dev_v3.json --llm-result build/llm/bench_execute_heldout_v3.json --out-json build/bench/aggregate_llm_v3.json --out-dir build/bench/llm_v3 --paper-table-dir build/paper_tables/llm_v3"
.\scripts\eda-docker.ps1 python3 scripts/validate_json_schemas.py --no-generate-smoke --bench-manifest benchmarks/module_compose_bench_heldout.yaml --bench-result build/bench/seed_results.json --bench-result build/bench/heldout_results.json --llm-run build/llm/provider_validate.json --llm-bench build/llm/bench_execute_public_dev_v3.json --llm-bench build/llm/bench_execute_heldout_v3.json --aggregate-result build/bench/aggregate_llm_v3.json
.\scripts\eda-docker.ps1 python3 scripts/write-llm-evidence-hashes.py --require --output build/release/llm_evidence_hashes.json
```

Execute-mode reruns may reuse the local cache. The sanitized records retain
`provider_requests` and `cache_hits` counters so reviewers can distinguish the
execute-mode record from cache reuse. Replaying provider calls without cache is
not required for artifact review.

## Sanitization Policy

The committed repository and release bundle must not include API keys, local
provider YAML, raw provider payload caches, non-v3 execute records, logs, or
provider response directories. Sanitized v3 execute records include profile and
model names, prompt hashes, request parameters, JSON validity, compiler and EDA
outcomes, repair provenance, token counts, and cost status.

## Claim Boundary

The paper may claim only the tested-profile v3 result: JSON AST repair reaches
36/36 public positive final pass, 10/10 held-out positive final pass, 26/26
public unsafe rejection, and 10/10 held-out unsafe rejection for the tested
profiles. Accepted repair wins are limited to zero free-form patch wins and 14
deterministic adapter-instance fallback wins accepted through compiler gates.
