---
name: cleanup
description: Repository hygiene, stale branch cleanup, build artifact purge, and git repository optimization.
---

# Supercharger Repository Hygiene

Safely declutter the workspace, purge orphaned build caches, remove merged branches, and optimize git storage.

## Hygiene Checklist

1. **Git Branch Cleanup**:
   - Identify local branches already merged into default branch: `git branch --merged origin/main`.
   - Delete merged local feature branches (excluding protected branches).

2. **Build & Dependency Cache Purge**:
   - Cargo: `cargo clean --release` or targeted cache trim.
   - Node: purge orphaned `.turbo`, `.next/cache`, or stale `node_modules` caches.
   - Python: remove `__pycache__` and orphaned `.pytest_cache` directories.

3. **Untracked File Inspection**:
   - Check `git status -s` for temporary log or scratch files.
   - Update `.gitignore` if recurring scratch files are discovered.

4. **Git Storage Optimization**:
   - Run `git gc --prune=now` to compress loose objects and optimize packfiles.
