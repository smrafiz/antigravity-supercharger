---
name: scope
description: Scope creep detector and blast-radius evaluator for ongoing tasks and proposed changes.
---

# Supercharger Scope & Blast Radius Evaluator

Evaluate whether current in-flight edits remain tightly aligned with the user's requested goal or have expanded uncontrollably.

## Evaluation Checklist

1. **Request Baseline**:
   - What was the user's original objective?
   - What files were strictly necessary to touch?

2. **Blast Radius Analysis**:
   - Review `git diff --stat` across all modified files.
   - Identify "drive-by refactorings" or stylistic changes unrelated to the core goal.

3. **Recommendation**:
   - If blast radius is excessive, suggest reverting speculative edits or splitting into a follow-up commit.
