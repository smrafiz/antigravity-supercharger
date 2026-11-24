---
name: design
description: Structured architecture and system design review before implementing complex architectural changes or new modules.
---

# Supercharger System Design Review

Conduct an interactive architectural review to evaluate system boundaries, trade-offs, scaling bottlenecks, and failure modes before writing code.

## Evaluation Areas

1. **Component Architecture**:
   - Single responsibility and interface clarity.
   - Coupling, data flow direction, and state management.

2. **Error Handling & Failure Modes**:
   - Graceful degradation under rate limits or network failures.
   - Fallback strategies and retry budgets.

3. **Performance & Resource Footprint**:
   - Sub-millisecond latency requirements for hooks.
   - Memory allocation and zero-copy data structures.

4. **Security & Guardrails**:
   - Boundary enforcement and permission requirements.
   - Audit trail logging and secret isolation.
