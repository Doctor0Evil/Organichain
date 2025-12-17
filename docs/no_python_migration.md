# NO-PYTHON Migration & Enforcement Guide

Date: 2025-11-25
Author DID: did:ion:EiD8J2b3K8k9Q8x9L7m2n4p1q5r6s7t8u9v0w1x2y3z4A5B6C7D8E9F0
Policy: NO-PYTHON-STRICT

## Purpose
This document details the full migration from Python-based implementation to a pure ALN (.aln) declarative architecture, enforcing repository governance, cost controls, compliance transparency, and deterministic auditability.

## Scope
- Remove all Python source files (*.py)
- Remove Python dependency manifests (requirements.txt, Pipfile*)
- Replace logic with ALN modules (runtime, rights framework, device manager, API abstraction, workflow examples)
- Update CI/CD to block future Python introductions

## Migration Phases
1. Inventory existing Python artifacts
2. Create ALN replacement modules (functional parity placeholders)
3. Introduce enforcement policy (policy/no_python_strict.aln)
4. Add Copilot instructions excluding Python
5. Gate CI/CD pipeline with no_python_enforcement job
6. Delete Python files from repository history if required
7. Add pre-commit hook to prevent reintroduction
8. Provide forensic export template for violations
9. Document conversion mapping

## Artifact Inventory (Before Removal)
| Original File | Replacement ALN | Notes |
|---------------|-----------------|-------|
| core/aln_runtime.py | core/aln_runtime.aln | Declarative evaluation & audit
| core/rights_framework.py | core/rights_framework.aln | Rights registry, grants, violations
| modules/augmentation/device_manager.py | modules/augmentation/device_manager.aln | Device registration/session
| api/main.py | api/main.aln | API abstraction manifest
| examples/complete_workflow.py | examples/complete_workflow.aln | End-to-end workflow example

## Replacement Strategy
- Structural mapping: Each Python class/function -> ALN function or section block
- State management: Use declarative fields and function return values instead of imperative mutation
- Auditing: Replace logging calls with `AuditLog.record()` blocks inside ALN functions
- Consent & Rights: Inline references to rights and consent flows instead of Python objects

## Enforcement Components
- Policy file: `policy/no_python_strict.aln`
- Copilot config: `.github/copilot-instructions.md`
- CI/CD gate: `no_python_enforcement` job in `.github/workflows/ci-cd.yml`
- Pre-commit hook: `scripts/pre-commit-no-python.sh`
- Forensic export template: `manifests/templates/forensic_export.example.json-aln`

## Developer Workflow Changes
| Previous | New ALN-Only |
|----------|--------------|
| Write Python modules | Author `.aln` declarative modules
| Run pytest | Future: ALN validator / static policy checks
| Import packages | Inline ALN dependency declarations
| Logging via Python | `AuditLog.record()` in ALN
| Packaging via requirements.txt | No packaging; manifests + ALN only

## CI/CD Adjustments
- Early fail if any `.py` or blocked manifests exist
- Future addition: ALN syntax validator container
- Remove Python-specific jobs once ALN validation tooling implemented

## Forensic & Rollback
On violation detection:
1. Quarantine offending files
2. Generate forensic JSON-ALN export
3. Record immutable ledger anchor
4. Notify admin + escalate if repeated

## Future Work
- Implement ALN syntax parser & linter
- Introduce performance benchmarking for ALN evaluation
- Add multi-party consent orchestration module
- Expand device registry to multi-ledger compatibility

## Commands (Local Enforcement)
```bash
# Install pre-commit hook
cp scripts/pre-commit-no-python.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Scan for Python remnants
git ls-files '*.py'

# History rewrite (optional, irreversible)
# Requires backup before executing
git filter-repo --path-glob "*.py" --invert-paths
```

## Risks & Considerations
- Loss of executable runtime until ALN engine tooling implemented
- CI/CD jobs referencing Python will need removal/refactoring
- Contributor retraining on ALN declarative approach

## Approval & Governance
- Enforced by DID authority
- Escalation path: Admin → Compliance Officer → Director Council
- Emergency trigger: `EMERGENCYDETECTED` when critical policy breach occurs

---
**Status:** Migration executed; Python artifacts removed; ALN replacements in place.
