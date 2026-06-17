# MICO Submission Claim Lock 2026Q3

Snapshot date: 2026-06-17.

This file locks the denominator policy for the DAC 2027 research-manuscript
candidate. It is narrower than the roadmap: it defines which numbers may be
used together in README, paper text, release notes, and artifact quickstarts.
Numeric claims still map back to `docs/release_claim_table.md`; LLM-specific
evidence maps to `docs/24_llm_matrix_v3.md`,
`docs/llm_final_matrix_report.md`, and sanitized authenticated records when
they are present under ignored `build/llm/` paths.

## Locked Denominators

| Claim family | Locked scope | Allowed denominator | Source of truth |
|---|---|---:|---|
| Deterministic public-development | Expanded deterministic manifest | 83 tasks: 46 positive, 37 negative | `docs/release_claim_table.md` |
| Deterministic held-out | Expanded deterministic held-out manifest | 40 tasks: 20 positive, 20 negative | `docs/release_claim_table.md` |
| Deterministic realism | Supplemental deterministic realism manifest | 30 tasks: 15 positive, 15 negative | `docs/release_claim_table.md` |
| LLM public-development v3 | Locked pre-expansion LLM-scored public manifest hashes | 62 tasks; positive rows reported over 36 positives and unsafe rows over 26 negatives | `docs/24_llm_matrix_v3.md` |
| LLM held-out v3 | Locked pre-expansion LLM-scored held-out manifest hashes | 20 tasks; positive rows reported over 10 positives and unsafe rows over 10 negatives | `docs/24_llm_matrix_v3.md` |
| LLM realism | Not LLM-scored in v3 | No LLM denominator | `docs/25_realism_supplement.md` |

## Mixing Rule

Do not combine an expanded deterministic denominator with a v3 LLM numerator in
the same claim. In particular:

- do not write that 35--36 LLM public positive passes are out of 46 public
  positives;
- do not write that 9--10 LLM held-out positive passes are out of 20 held-out
  positives;
- do not attach any LLM pass-rate claim to the 30-task realism supplement until
  an authenticated rerun explicitly includes that manifest.

Acceptable wording separates the scopes:

- "expanded deterministic public-development: 83/83 expected outcomes";
- "locked pre-expansion v3 LLM public-development: 35--36/36 positive final
  passes across tested profiles";
- "supplemental realism is deterministic-only until a new LLM matrix reruns it."

## Non-Claims

The submission may not claim broad autonomous repair, CDC correctness proof,
complete formal proof, arbitrary LTL support, routed timing closure,
bitstreams, board-level implementation, or generalization to untested models.
Those phrases may appear only as limitations, unsupported claims, or future
work.

## Required Gate

Run the documentation claim guard before every claim-boundary or release
commit:

```powershell
.\scripts\eda-docker.ps1 python3 scripts/check-doc-claims.py
```

The guard checks manifest denominators, stale public/held-out counts, required
claim-table references, and unsupported affirmative claims. Passing this guard
does not replace the full release gate; it only verifies that claim wording is
consistent with the locked denominator policy above.
