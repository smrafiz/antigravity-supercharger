---
name: reflect
description: Score session quality against concrete evidence and record actionable learnings to memory.
---

# Supercharger Session Reflection

Evaluate session execution against objective evidence to capture durable project-specific lessons without conversational fluff.

## Reflection Protocol

1. **Evidence Ledger**:
   - Tests and builds that executed (passed vs failed).
   - Tool errors, permission denials, retried commands.
   - User corrections ("no", "that's wrong", "revert").
2. **Outcome Gap**:
   - Compare intended goal with actual delivered state.
3. **Dimension Scoring (0-3)**:
   - **Accuracy**: First-attempt correctness vs backtracking rework.
   - **Scope Discipline**: Tightly focused vs speculative drift.
   - **Verification**: Verified via test output vs unverified assertion.
   - **Efficiency**: Direct solution path vs redundant tool calls.
4. **Durable Learnings**:
   - Capture only lessons that are grounded, general, checkable, and new.
   - Persist key project conventions to `.gemini/rules/`.
