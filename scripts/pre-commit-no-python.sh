#!/usr/bin/env bash
# Pre-commit hook: Enforce NO-PYTHON-STRICT policy
# Blocks any staged *.py files or Python dependency manifests.

set -euo pipefail

PY_FILES=$(git diff --cached --name-only --diff-filter=ACMR | grep -E '\.py$' || true)
BLOCKED_MANIFESTS=$(git diff --cached --name-only --diff-filter=ACMR | grep -E '^(requirements.txt|Pipfile|Pipfile.lock)$' || true)

if [ -n "$PY_FILES" ]; then
  echo "❌ NO-PYTHON-STRICT policy violation: Python files staged:" >&2
  echo "$PY_FILES" >&2
  exit 1
fi

if [ -n "$BLOCKED_MANIFESTS" ]; then
  echo "❌ NO-PYTHON-STRICT policy violation: Python dependency manifests staged:" >&2
  echo "$BLOCKED_MANIFESTS" >&2
  exit 1
fi

echo "✅ Pre-commit check passed: No Python artifacts detected."
exit 0
