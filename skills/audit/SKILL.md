---
name: audit
description: Run an audit of the current workspace against Supercharger safety rules, scanning for secrets, vulnerable dependencies, and untracked changes.
---

# Supercharger Workspace Audit

Use this skill to perform a comprehensive security and health check on the current repository.

## Procedure

1. **Scan for Hardcoded Secrets**:
   - Search for potential keys, tokens, and passwords in tracked files.
   - Check if `.env` files are accidentally tracked in git.

2. **Scan for Common Vulnerabilities**:
   - Inspect recent changes for unsafe shell executions (`child_process.exec`, `os.system(shell=True)`).
   - Check for unparameterized SQL queries.

3. **Verify Git Health**:
   - Check `git status` for untracked sensitive files.
   - Verify that `.gitignore` properly excludes build artifacts and environment files.

4. **Output Findings**:
   - Present a clear table of findings categorized by severity (Critical, Warning, Informational).
