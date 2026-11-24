---
name: think
description: Deep deliberation and counter-argument analysis before executing high-complexity decisions.
---

# Supercharger Deep Deliberation

Formulate and stress-test multiple distinct solutions to a complex engineering problem before writing code.

## Protocol

1. **Problem Framing**:
   - What are the explicit requirements and implicit constraints?
   - What is the failure mode if we do nothing or take the naive path?

2. **Divergent Paths**:
   - Generate at least 2 distinct technical approaches (e.g. In-memory vs Persistent, Sync vs Async, Library vs Hand-rolled).
   - Compare operational complexity, developer experience, and long-term maintenance costs.

3. **Steel-manning & Adversarial Critique**:
   - Identify the strongest counter-argument against the preferred approach.
   - What assumptions could prove false under production load?

4. **Verdict & Justification**:
   - Select the optimal approach with crisp justification.
   - Define unambiguous milestones for execution.
