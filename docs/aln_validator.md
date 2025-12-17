# ALN Validator Scaffold

This document describes the placeholder ALN validation stage integrated into the CI pipeline replacing former Python lint/test jobs.

## Purpose
Provide deterministic, policy-aligned checks for:
- NO-PYTHON-STRICT enforcement
- DID presence in each `.aln` module
- Forbidden Python-like constructs (`def`, `import`)
- Presence of critical policy and rights framework manifests

## Current Implementation (Placeholder)
The GitHub Actions job `aln_validate` performs simple textual scans:
- Grep for forbidden tokens
- Verify DID inclusion
- Produce `aln-validation.json` artifact

## Future Enhancements
1. Formal ALN grammar parser (EBNF) for syntax correctness.
2. Static semantic checks (block uniqueness, field types, trigger references).
3. Cross-module dependency graph and cycle detection.
4. Consent & rights semantic validation (multi-party hooks resolved).
5. Hash integrity and signature verification (SHA3-512 + optional PQ signatures).
6. Energy optimization rule validation (ALN-SCHEDULER-V3 parameter bounds).
7. Ledger anchoring simulation: dry-run recording of audit events.

## Suggested Container Design
```Dockerfile
FROM alpine:3.20
RUN apk add --no-cache bash grep coreutils jq
COPY validator.sh /usr/local/bin/validator.sh
ENTRYPOINT ["/usr/local/bin/validator.sh"]
```

`validator.sh` would:
```bash
#!/usr/bin/env bash
set -euo pipefail
DID="did:ion:EiD8J2b3K8k9Q8x9L7m2n4p1q5r6s7t8u9v0w1x2y3z4A5B6C7D8E9F0"
FILES=$(find . -name "*.aln")
for f in $FILES; do
  grep -q "$DID" "$f" || { echo "DID missing in $f"; exit 1; }
  grep -q "def " "$f" && { echo "Python def found in $f"; exit 1; }
  grep -q "import " "$f" && { echo "Python import found in $f"; exit 1; }
  echo "Validated: $f"
done
jq -n --arg ts "$(date -u +%Y-%m-%dT%H:%M:%SZ)" '{status:"passed",timestamp:$ts}' > aln-validation.json
```

## Invocation Pattern
Add a dedicated workflow or reuse `ci-cd.yml` job:
```yaml
jobs:
  aln_validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: bash validator.sh
      - uses: actions/upload-artifact@v3
        with:
          name: aln-validation
          path: aln-validation.json
```

## Metrics to Output (Roadmap)
- Module count
- Average lines per ALN file
- Forbidden pattern incidents
- DID coverage percentage
- Energy optimization directive coverage

## Rollback Strategy
If validation fails:
1. Quarantine commit via branch protection + manual approval.
2. Generate forensic JSON-ALN export referencing violation file set.
3. Escalate to director council if repeated (>3 within 24h).

## Forensic Export Example
See `manifests/templates/forensic_export.example.json-aln` for structural pattern.

## Ledger Integration (Future)
Record each validation pass/fail into QUORUMX ledger including:
- Commit hash
- Validator version
- DID list
- Policy compliance flags

---
**Status:** Placeholder validator active; full semantic engine pending design phase.
