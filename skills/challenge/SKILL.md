---
name: challenge
description: Adversarial red-team challenge to stress-test designs, PRs, and architectural assumptions.
---

# Supercharger Architectural Challenge

Critique a proposed technical solution from an adversarial attacker and operational perspective to discover hidden flaws.

## Challenge Dimensions

1. **Scale & Concurrency**:
   - How does this design behave when traffic spikes 100x?
   - What happens under concurrent read/write contention or distributed clock skew?

2. **Partial Failures & Network Partitions**:
   - What happens if downstream service A succeeds but B times out?
   - Are distributed state changes atomic or idempotent?

3. **Security & Boundary Attacks**:
   - Can untrusted input bypass caller validation?
   - Are error responses leaking stack traces or sensitive environment variables?

4. **Deliverable**:
   - Ranked list of structural vulnerabilities and concrete recommendations to harden the design.
