---
name: autopilot
description: Time-boxed auto-approval window for Antigravity Supercharger. Use to toggle auto-approval of safe operations so you are not interrupted by confirmation prompts.
---

# Supercharger Autopilot

Temporarily auto-approves safe tool permissions so you aren't interrupted by repetitive confirmation prompts during autonomous tasks.

## Usage

Run the `supercharger autopilot` command via the terminal:

- **Activate for a duration (default 30m)**:
  ```bash
  supercharger autopilot 30m
  supercharger autopilot 1h
  supercharger autopilot 2h --full
  ```
- **Turn off**:
  ```bash
  supercharger autopilot off
  ```
- **Check status**:
  ```bash
  supercharger autopilot status
  ```

## Options
- `[duration]`: e.g. `15m`, `30m`, `1h`, `2h`.
- `--full`: Also auto-approves guard asks (e.g., git branching or force asks). Fatal safety blocks (`rm -rf /`, secret leaks) are **never** bypassed.
- `--global`: Applies to all sessions on the machine instead of just the current workspace.
