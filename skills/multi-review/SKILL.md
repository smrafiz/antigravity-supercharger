---
name: multi-review
description: Multi-perspective adversarial code review covering Security, Correctness, Performance, and Conventions.
---

# Supercharger Multi-Review

Conduct an adversarial code review by analyzing changes through four distinct lenses. Every candidate finding must be verified by independent refutation before reporting.

## Review Lenses

1. **Security**:
   - Injection sinks (SQL, OS command, deserialization).
   - Credential exposure, hardcoded tokens, insecure path handling.
   - Trust-boundary crossing without validation.

2. **Correctness & Edge Cases**:
   - Off-by-one errors, null/empty pointer dereferences, unhandled async errors.
   - Concurrency race conditions (TOCTOU) and partial failures.
   - Contract breaks for external callers.

3. **Performance**:
   - N+1 database queries, synchronous I/O inside hot loops.
   - Unbounded memory growth, excessive object cloning, quadratic string algorithms.

4. **Tests & Conventions**:
   - Untested new branches or edge conditions.
   - Mocking tests that pass without verifying actual system logic.
   - Regressions against established repository standards.

## Deliverable

Categorize surviving findings into:
- **MUST FIX**: Blocks release (exploitable vulnerability, data loss, crash).
- **SHOULD FIX**: High-priority defect, serious maintainability concern.
Cite exact `file:line` links with evidence for each finding.
