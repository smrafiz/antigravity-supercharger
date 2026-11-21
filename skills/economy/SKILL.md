---
name: economy
description: Switch token economy tier (minimal, lean, standard) to control model output token verbosity and prompt behavior.
---

# Supercharger Token Economy

Dynamically controls model response verbosity, reducing token usage and speeding up generation during coding tasks.

## Tiers
- **`minimal`** *(Default)*: Zero conversational fluff, no greetings or sign-offs. Direct diffs and commands only. Saves up to ~30-40% on output tokens.
- **`lean`**: 1-2 sentence context overview, then direct solution.
- **`standard`**: Full explanations, detailed code walkthroughs, and architecture context.

## Usage
Run:
```bash
supercharger economy minimal
supercharger economy lean
supercharger economy standard
supercharger economy status
```
The active tier is instantly reflected in Line 1 of your statusline:
`Eco: Minimal | Eco: Lean | Eco: Standard`
