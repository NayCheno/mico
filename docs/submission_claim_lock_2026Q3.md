# MICO Submission Claim Lock 2026Q3

Snapshot date: 2026-06-17.

This file locks the denominator policy for the DAC 2027 research-manuscript
candidate. It is narrower than the roadmap: it defines which numbers may be
used together in README, paper text, release notes, and artifact quickstarts.
Numeric claims still map back to `docs/release_claim_table.md`; LLM-specific
evidence maps to `docs/26_llm_matrix_v4.md`,
`docs/llm_final_matrix_report.md`, and sanitized authenticated records when
they are present under ignored `build/llm/` paths. `docs/24_llm_matrix_v3.md`
is retained as historical pre-expansion evidence only.

## Locked Denominators

| Claim family | Locked scope | Allowed denominator | Source of truth |
|---|---|---:|---|
| Deterministic public-development | Expanded deterministic manifest | 83 tasks: 46 positive, 37 negative | `docs/release_claim_table.md` |
| Deterministic held-out | Expanded deterministic held-out manifest | 40 tasks: 20 positive, 20 negative | `docs/release_claim_table.md` |
| Deterministic realism | Supplemental deterministic realism manifest | 30 tasks: 15 positive, 15 negative | `docs/release_claim_table.md` |
| LLM public-development v4 | Expanded authenticated public-development manifest | 83 tasks; positive rows reported over 46 positives and unsafe rows over 37 negatives | `docs/26_llm_matrix_v4.md` |
| LLM held-out v4 | Expanded authenticated held-out manifest | 40 tasks; positive rows reported over 20 positives and unsafe rows over 20 negatives | `docs/26_llm_matrix_v4.md` |
| LLM realism v4 | Authenticated supplemental realism manifest | 30 tasks; positive rows reported over 15 positives and unsafe rows over 15 negatives | `docs/26_llm_matrix_v4.md` |
| Historical LLM v3 | Locked pre-expansion public/held-out manifests | 62/20 tasks; historical only | `docs/24_llm_matrix_v3.md` |

## Mixing Rule

Do not combine a historical v3 LLM numerator with an expanded deterministic
denominator in the same current claim. In particular:

- do not write that 35--36 LLM public positive passes are out of 46 public
  positives;
- do not write that 9--10 LLM held-out positive passes are out of 20 held-out
  positives;
- do not cite v3 as the current LLM matrix after the v4 expanded rerun.

Acceptable wording separates the scopes:

- "expanded deterministic public-development: 83/83 expected outcomes";
- "expanded v4 LLM public-development: 45--46/46 positive final passes across
  tested profiles";
- "expanded v4 LLM realism: 15/15 positive final passes across tested profiles."

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
