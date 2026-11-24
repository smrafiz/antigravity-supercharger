---
name: resolve-conflicts
description: Guided resolution of git merge, rebase, or cherry-pick conflicts with safety checks.
---

# Supercharger Git Conflict Resolver

Safely inspect, analyze, and resolve git merge or rebase conflicts without losing work.

## Resolution Workflow

1. **Inspect Conflict State**:
   - Run `git status` to identify conflicting paths.
   - Inspect conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).

2. **Analyze Both Sides**:
   - Compare `HEAD` (ours) vs the incoming branch (theirs).
   - Understand the intent of both changes before editing.

3. **Resolve Contiguous Blocks**:
   - Edit conflicting files to preserve intended behavior from both branches.
   - Remove all git conflict markers completely.

4. **Verify & Stage**:
   - Run build and test suites to verify syntactical and logical correctness.
   - Stage resolved files with `git add`.
