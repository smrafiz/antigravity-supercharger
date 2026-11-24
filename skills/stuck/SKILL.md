---
name: stuck
description: Break out of a debugging loop or cognitive deadlock after repeated failed fixes.
---

# Supercharger Break Out of Debug Loop

Stop retrying. Step back. Look, don't think.

## Protocol

1. **Quote the Raw Error**:
   - Copy the exact, verbatim error message and stack trace. Do not reason over a paraphrased summary.
2. **Make it Fail on Command**:
   - Write the exact reproduction steps. If intermittent, declare it immediately.
3. **What Changed Since it Last Worked?**:
   - Identify the exact diff, commit, environment variable, or dependency version that diverged.
4. **Forbidden List**:
   - Enumerate every approach already attempted. These and their near-variants are strictly off the table.
5. **Separate Facts from Assumptions**:
   - List verified facts with `file:line` evidence. Everything else is an unverified assumption.
6. **Formulate 3 Falsifiable Hypotheses**:
   - Symptom-level, Upstream data-flow, or Environmental configuration.
   - Run the cheapest decisive test to confirm or refute each hypothesis.
