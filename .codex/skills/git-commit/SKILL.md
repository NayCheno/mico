---
name: git-commit
description: Create disciplined Git commits from local changes. Use when the user asks to commit, stage changes, split changes into commits, write a commit message, clean up a branch before pushing, or verify that a commit contains only intentional work.
---

# Git Commit

## Overview

Use this skill to turn a working tree into one or more focused, reviewable commits. Prefer repository conventions over generic rules, protect user changes, and stage only the files or hunks that belong to the requested commit.

## Workflow

1. Inspect repository context before staging:
   - `git status --short`
   - `git diff`
   - `git diff --stat`
   - recent commit style: `git log --oneline -n 8`
   - project guidance when present: `README*`, `CONTRIBUTING*`, PR templates, package scripts, or equivalent local docs
2. Identify intentional changes:
   - group changes by purpose: feature, fix, docs, tests, refactor, build, chore
   - keep unrelated user changes unstaged
   - stop and report if secrets, credentials, private data, generated caches, logs, or local config appear in the diff
3. Stage narrowly:
   - prefer explicit paths: `git add -- path/to/file`
   - use patch staging when only part of a file belongs to the commit
   - do not use `git add .` unless the user explicitly asks and the entire working tree has been audited
4. Review staged content:
   - `git diff --cached`
   - `git diff --cached --stat`
   - verify the staged diff is atomic and matches the requested scope
5. Run relevant validation:
   - choose the smallest checks that cover the staged behavior
   - run formatting/lint/tests/typecheck/build when local project conventions indicate them
   - if validation cannot run, state the concrete reason
6. Commit:
   - use the repository's observed message style
   - if no style exists, use Conventional Commits
   - commit only after staged diff and validation have been reviewed

## Commit Message

Prefer the repo's existing convention. If none is clear, use:

```text
<type>(<scope>): <subject>
```

Common types:

```text
feat, fix, docs, test, refactor, perf, style, build, ci, chore, revert
```

Subject rules:

- be specific and concise
- use imperative mood when natural
- do not end with a period
- avoid vague subjects like `update`, `misc`, `changes`, `fix stuff`, or `wip`

Add a body when the change needs context: motivation, risks, migrations, validation notes, or breaking changes.

For breaking changes:

```text
feat(api)!: remove legacy token endpoint

BREAKING CHANGE: /v1/token has been removed. Use /v1/session.
```

## Splitting Commits

Split commits when changes are independently reviewable or revertable:

- code and tests for the same behavior usually belong together
- docs for the same behavior may belong with the code, unless they are broad documentation work
- mechanical formatting should be separate if it touches unrelated files
- generated lockfile changes belong with the dependency or build change that caused them

Do not create WIP commits unless the user explicitly asks.

## Safety Rules

- Never run history-changing commands such as `git reset --hard`, `git rebase`, `git commit --amend`, or force-push unless the user explicitly requests them.
- Never revert or overwrite unrelated user changes.
- Never stage files outside the requested commit scope.
- Never commit `.env`, credentials, tokens, private keys, or personal data.
- Never claim validation passed unless the command was actually run and succeeded.

## Final Report

After committing, report:

```text
Created commit <sha>: <subject>

Files changed:
- <path>

Validation:
- <command>: passed
- <command>: not run (<reason>)
```
