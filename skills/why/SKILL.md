---
name: why
description: Root-cause debugging and causal analysis using the 5-Whys methodology to identify systemic failures.
---

# Supercharger Root Cause Analysis (5-Whys)

Trace unexpected failures, flaky tests, and logic errors back to their fundamental root cause rather than applying superficial band-aids.

## Debugging Protocol

1. **Define the Symptom**:
   - State the observed failure precisely (exact error message, exit code, unexpected output).
   - Formulate the expected versus actual behavior.

2. **Iterative 5-Whys Inquiry**:
   - Why did the symptom occur? (Immediate mechanical failure)
   - Why did that condition arise? (Direct causal trigger)
   - Why wasn't that prevented by guards/validation? (Defensive gap)
   - Why was the invariant missing or broken? (Architectural/contract assumption)
   - Why did the design permit this state? (Root systemic flaw)

3. **Actionable Fix**:
   - Implement the fix at the deepest preventive layer.
   - Add a regression test that fails before the fix and passes after.
