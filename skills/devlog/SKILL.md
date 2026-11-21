---
name: devlog
description: Generate structured development logs documenting decisions, changes, bugs fixed, and remaining tasks.
---

# Development Log Procedure

Use this skill when recording session milestones, documenting architectural decisions, or writing changelog summaries.

## Procedure

1. **Inspect Session Changes**:
   - Run `git status` and `git diff --stat` to review all files touched during the session.
   - Review recent commits.

2. **Draft the Devlog Entry**:
   - Structure the entry:
     * **Date & Goal**: What task was being solved.
     * **Key Decisions**: Why specific approaches were selected over alternatives.
     * **Changes Made**: Bulleted list of modified modules and new files.
     * **Testing & Verification**: How correctness was proven.
     * **Next Steps**: What remains for follow-up sessions.

3. **Persist the Log**:
   - Append to `DEVLOG.md` or the project's documentation folder.
