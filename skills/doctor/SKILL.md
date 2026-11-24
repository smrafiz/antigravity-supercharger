---
name: doctor
description: Run a comprehensive diagnostic check of the Supercharger installation, active hooks, statusline health, and configuration.
---

# Supercharger Doctor

Diagnose the health of the Antigravity Supercharger installation, verify binary responsiveness, check hook registration, and inspect active guardrails.

## Diagnostic Steps

1. **Verify Binary & Execution**:
   - Run `supercharger status` to ensure sub-millisecond hook execution.
   - Check statusline output responsiveness and quota bucket parsing.

2. **Verify Plugin Registration**:
   - Run `agy plugin validate` on the Supercharger plugin directory.
   - Confirm hooks (`PreToolUse`, `PostToolUse`, `Stop`) are actively intercepting calls.

3. **Inspect Active Guardrails**:
   - Verify safety modes (`autopilot`, `readonly`, `strict`, `economy`).
   - Confirm workspace path boundaries and credential isolation.

4. **Output Verdict**:
   - Lead with a one-line PASS/FAIL verdict.
   - List any warnings or missing components with single-step remediation.
