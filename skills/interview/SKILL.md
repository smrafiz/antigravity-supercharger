---
name: interview
description: Deep interview to clarify project requirements, trade-offs, and design decisions before writing code or making architectural changes.
---

# Requirements Interview Procedure

Use this skill when the user asks for an interview, wants to clarify ambiguous requirements, or when initiating a complex multi-part architectural feature.

## Procedure

1. **Review Current Codebase & Architecture**:
   - Inspect existing project structure and dependencies.
   - Note potential constraints or conventions already in place.

2. **Conduct the Interactive Interview**:
   - Ask targeted questions focusing on core ambiguities:
     * User personas and workflows.
     * Scale, throughput, and performance targets.
     * Error handling expectations and fallback behaviors.
     * Backwards compatibility and migration requirements.
   - Use the `ask_question` tool if multiple-choice answers help structure the decision.

3. **Synthesize Decisions**:
   - Summarize confirmed choices into an implementation checklist before writing any code.
