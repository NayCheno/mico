#!/usr/bin/env python3
"""Convert the Markdown release claim table into a hashed JSON artifact."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]


def repo_path(value: str | Path) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPO_ROOT / path


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def git_text(args: list[str]) -> str | None:
    try:
        return subprocess.check_output(["git", *args], cwd=REPO_ROOT, text=True, stderr=subprocess.DEVNULL).strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None


def split_markdown_row(line: str) -> list[str]:
    return [cell.strip().replace("\\|", "|") for cell in line.strip().strip("|").split("|")]


def parse_claim_table(path: Path) -> list[dict[str, Any]]:
    entries: list[dict[str, Any]] = []
    section = ""
    headers: list[str] = []
    for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if line.startswith("## "):
            section = line.removeprefix("## ").strip()
            headers = []
            continue
        if not line.startswith("|"):
            continue
        cells = split_markdown_row(line)
        if not cells or all(set(cell) <= {"-", ":"} for cell in cells):
            continue
        if cells[0] == "Claim":
            headers = cells
            continue
        if len(cells) != len(headers) or len(cells) < 6:
            continue
        row = dict(zip(headers, cells, strict=True))
        entries.append(
            {
                "section": section,
                "line": line_no,
                "claim": row["Claim"],
                "current_value": row["Current value"],
                "evidence_artifact": row["Evidence artifact"],
                "schema_or_source": row["Schema or source"],
                "hash_source": row["Hash source"],
                "paper_location": row["Paper location"],
            }
        )
    return entries


def build_payload(source: Path) -> dict[str, Any]:
    return {
        "schema_version": "mico.release.claim_table.v0",
        "source_commit_hash": git_text(["rev-parse", "HEAD"]),
        "source_branch": git_text(["branch", "--show-current"]),
        "source_markdown": {
            "path": source.relative_to(REPO_ROOT).as_posix(),
            "sha256": sha256_file(source),
        },
        "claims": parse_claim_table(source),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", default="docs/release_claim_table.md")
    parser.add_argument("--output", default="build/release/release_claim_table.json")
    args = parser.parse_args()

    source = repo_path(args.source)
    output = repo_path(args.output)
    payload = build_payload(source)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {output.relative_to(REPO_ROOT)}")
    if not payload["claims"]:
        print("release claim table contained no claims")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
