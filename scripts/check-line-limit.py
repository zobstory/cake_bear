#!/usr/bin/env python3
"""Enforce a 500-line maximum on Rust source files in the workspace.

A guardrail to keep modules focused. If a file legitimately wants more
lines, split it into submodules rather than raising the limit.

Run from the repo root:
    python3 scripts/check-line-limit.py

Exits 0 if all files are within the limit, 1 otherwise. In GitHub
Actions, emits `::error` annotations so violations appear inline in
the PR diff.
"""
from __future__ import annotations

import sys
from pathlib import Path

LIMIT = 500
ROOTS = [Path("crates")]
SUFFIXES = {".rs"}


def count_lines(path: Path) -> int:
    with path.open("rb") as f:
        return sum(1 for _ in f)


def main() -> int:
    violations: list[tuple[Path, int]] = []
    for root in ROOTS:
        if not root.exists():
            continue
        for path in root.rglob("*"):
            if not path.is_file() or path.suffix not in SUFFIXES:
                continue
            n = count_lines(path)
            if n > LIMIT:
                violations.append((path, n))

    if not violations:
        roots_repr = ", ".join(str(r) for r in ROOTS)
        print(f"OK: every .rs file under {roots_repr} is within {LIMIT} lines.")
        return 0

    # GitHub Actions annotations (no-op outside Actions).
    for path, n in violations:
        print(f"::error file={path}::file has {n} lines, limit is {LIMIT}")

    print(f"\n{len(violations)} file(s) exceed the {LIMIT}-line limit:", file=sys.stderr)
    for path, n in violations:
        print(f"  {path}: {n} lines", file=sys.stderr)
    return 1


if __name__ == "__main__":
    sys.exit(main())
