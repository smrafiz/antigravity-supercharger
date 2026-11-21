---
name: readonly
description: Time-boxed look-don't-touch mode for Antigravity Supercharger. Blocks all file edits, writes, and mutating commands while allowing reads and searches.
---

# Supercharger Read-Only Mode

Activates a temporary "look, don't touch" guardrail. While active, every file write (`write_file`, `edit_file`) and mutating shell command (`rm`, `mv`, `git commit`, `npm install`, etc.) is blocked. Reads, searches, and inspections pass freely.

## Usage

- **Activate for a duration (default 1h)**:
  ```bash
  supercharger readonly 30m
  supercharger readonly 2h
  ```
- **Turn off**:
  ```bash
  supercharger readonly off
  ```
- **Check status**:
  ```bash
  supercharger readonly status
  ```
