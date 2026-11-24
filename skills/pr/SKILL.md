---
name: pr
description: Prepare a pull request for the current branch with automated pre-flight checks, verified test plans, and clean diff summaries.
---

# Supercharger PR Generator

Prepare a pull request description from git diffs with strict pre-flight checks to prevent leaking secrets or publishing inconsistent changelogs.

## Pre-Flight Checklist

1. **Branch Verification**:
   - Confirm current branch is not `main` or default production branch.
   - Ensure branch has pushed commits to upstream remote.
2. **Secret Scan**:
   - Run `git diff origin/main...HEAD` through the Supercharger secret scanner.
   - Fail immediately if unencrypted tokens or keys are present.
3. **Diff Analysis**:
   - Read the exact commit log and diff stat.
   - Keep description strictly factual to changes present in the diff (no hallucinated features).

## PR Template

```markdown
## Summary
Concise imperative statement of what and why.

## Key Changes
- Traceable bullets grouped by component.

## Verification & Test Plan
- Exact test command run and output (e.g. `cargo test -- 17 passed`).
```
