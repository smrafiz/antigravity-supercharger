---
name: supercharger
description: Supercharger router and index. Routes situations to the right skill or displays full skill catalogue.
---

# Antigravity Supercharger Command Router

Route to the right Supercharger skill for a situation, or view the complete catalog.

## Quick Index

### Code & Review
- `/supercharger:audit`: Sweep workspace for naming, pattern, documentation, and structural inconsistencies.
- `/supercharger:security`: OWASP-aligned security audit of current changes, sinks, and secret leaks.
- `/supercharger:multi-review`: Multi-perspective adversarial code review (Correctness, Security, Perf, Style).
- `/supercharger:challenge`: Red-team stress-test for design decisions and PRs.
- `/supercharger:think`: Deep reasoning and counter-argument analysis on complex problems.

### Workflow & Execution
- `/supercharger:scope`: Pre-flight boundaries, risk evaluation, and stop conditions.
- `/supercharger:estimate`: Token usage, context headroom, and complexity estimation.
- `/supercharger:cleanup`: Safe dead code, unused imports, and stale branch purge.
- `/supercharger:pr`: Streamlined pull request preparation with automated pre-flight checks.
- `/supercharger:resolve-conflicts`: Guided git conflict resolution with safety verification.
- `/supercharger:handoff`: Session handoff capturing decisions, uncommitted state, and next steps.
- `/supercharger:devlog`: Living architecture journal tracking decisions and bug fixes.
- `/supercharger:interview`: Structured requirements clarification before coding.
- `/supercharger:de-ai`: Strip AI filler, robotic prose, and corporate slop from copy and PRs.

### UI & Design
- `/supercharger:design`: System design, brand tokens, and architectural evaluation.
- `/supercharger:design-review`: UI review for WCAG 2.2 AA contrast, hierarchy, and 3 viewports.
- `/supercharger:unslop`: Redesign generic AI interfaces into distinct, intentional products.

### Diagnostics & Controls
- `/supercharger:sc-status`: Live status of guardrails, active modes, and statusline health.
- `/supercharger:autopilot`: Time-boxed auto-approval of safe operations.
- `/supercharger:readonly`: Look-don't-touch mode blocking mutating tools.
- `/supercharger:strict`: Time-boxed strict mode requiring confirmation on all actions.
- `/supercharger:economy`: Switch token economy tier (`minimal`, `lean`, `standard`).
- `/supercharger:cache-stats`: Prompt cache hit ratio and token savings statistics.
- `/supercharger:cache-clear`: Purge local audit logs and temporary cache files.
- `/supercharger:doctor`: Diagnostic health check of binaries, hooks, and guard latency.
- `/supercharger:perf`: Hook execution timing benchmark and bottleneck analysis.
- `/supercharger:why`: 5-Whys root cause analysis of guard triggers and system failures.

## Situational Routing

| Situation | Primary Skill | Secondary |
|---|---|---|
| "Is this safe to deploy?" | `/supercharger:security` | `/supercharger:multi-review` |
| "Review my code changes" | `/supercharger:multi-review` | `/supercharger:security` |
| "UI looks generic / AI-generated" | `/supercharger:unslop` | `/supercharger:design-review` |
| "Check accessibility / contrast" | `/supercharger:design-review` | `/supercharger:design` |
| "Stuck in a loop / repeated failures" | `/supercharger:stuck` | `/supercharger:why` |
| "Too many confirmation prompts" | `/supercharger:autopilot` | `/supercharger:sc-status` |
| "Prevent any edits or commands" | `/supercharger:readonly` | `/supercharger:strict` |
