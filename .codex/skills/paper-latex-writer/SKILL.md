---
name: paper-latex-writer
description: Write, edit, and compile the MICO research paper as TeX sources with the Windows host LaTeX toolchain. Use when working under paper/, editing paper/main.tex or BibTeX, converting Markdown notes into LaTeX, compiling PDF output, or checking academic paper formatting; do not use Docker for LaTeX unless the user explicitly changes the paper toolchain policy.
---

# Paper LaTeX Writer

## Overview

Use this skill for paper work in `paper/`. The policy for this repository is: `paper/main.tex` is the authoritative IEEE-style paper entrypoint, section bodies live under `paper/sections/`, and LaTeX is compiled on the Windows host using the locally installed LaTeX distribution. Rust and non-Vivado EDA validation stay in Docker.

## Workflow

1. Inspect `paper/main.tex`, the relevant `paper/sections/*.tex` file, and `paper/related_work.bib` before editing the paper.
2. Keep `paper/main.tex` as the template/preamble/section-order entrypoint; put substantive body text in chapter/section files under `paper/sections/`.
3. Treat Markdown files under `paper/` as historical drafts or notes only; convert substantive paper content into `.tex` before editing.
4. Keep citations in `paper/related_work.bib` unless a task requires a new bibliography file.
5. Prefer `latexmk` when available because it handles BibTeX/Biber and reruns.
6. Use the default IEEE pdfLaTeX path for the English paper. Use `xelatex` only when Chinese text or mixed CJK/English content is intentionally introduced.
7. Keep generated files out of source control: `.aux`, `.bbl`, `.blg`, `.fdb_latexmk`, `.fls`, `.log`, `.out`, `.synctex.gz`, `.xdv`, and generated PDFs unless explicitly requested.

## Host Commands

Use Windows host LaTeX:

```powershell
latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
```

Fallback when `latexmk` is unavailable:

```powershell
Push-Location paper
pdflatex -interaction=nonstopmode -halt-on-error main.tex
bibtex main
pdflatex -interaction=nonstopmode -halt-on-error main.tex
pdflatex -interaction=nonstopmode -halt-on-error main.tex
Pop-Location
```

Do not run these commands through `scripts/eda-docker.ps1` unless the repository policy is changed.

## Output Discipline

- Report the host LaTeX command used.
- Summarize warnings that affect bibliography, cross references, missing figures, overfull boxes, or failed CJK fonts.
- Do not claim PDF compilation succeeded unless the command actually completed successfully.
