---
name: memory-prune
description: Archive resolved or stale memory entries so they stop loading into session context.
---

# Supercharger Memory Prune

Safely prune resolved, superseded, or obsolete memory entries from `.gemini/rules/` and context index files to reduce per-session token overhead.

## Pruning Protocol

1. **Scan Memory Files**:
   - Inspect `.gemini/rules/`, `GEMINI.md`, and session memory indexes.
   - Categorize entries into Active, Resolved, and Obsolete.

2. **Safety Check**:
   - Never delete unverified active rules.
   - Move resolved project entries into an `archive/` folder or comment out with `# [ARCHIVED: <date>]`.

3. **Context Savings**:
   - Report token savings achieved by pruning stale lines (typically 40-60 tokens per entry).
