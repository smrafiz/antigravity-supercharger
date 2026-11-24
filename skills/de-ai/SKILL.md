---
name: de-ai
description: Strip AI-slop patterns, corporate filler, and robotic conversational prose from text, documentation, PRs, and commit messages.
---

# Supercharger De-AI

Strip AI-slop patterns from a draft without flattening the writer's authentic voice, or detect and flag AI patterns without rewriting.

## Workflow

1. **Input Analysis**:
   - If targeting a file or PR diff, read the text directly.
   - Detect robotic cadence, throat-clearing openers, faux-insight setups, and empty buzzwords.

2. **Words to Cut**:
   - **Banned**: delve, foster, leverage, utilize, facilitate, empower, streamline, robust, cutting-edge, paradigm shift, game changer, this is huge, tapestry, realm, beacon, multifaceted, meticulous, intricate, paramount, transformative, elevate, embark, supercharge, harness, ever-evolving.
   - **Empty Adverbs**: crucially, inherently, fundamentally, truly, honestly, simply.
   - **Filler Phrases**: "it's worth noting", "at the end of the day", "when it comes to", "at its core", "in today's fast-paced world", "in order to".

3. **Patterns to Eliminate**:
   - **Binary Contrasts**: "This is not X. It's Y." -> State Y directly.
   - **Throat-Clearing Openers**: "Here's the thing,", "Let me be clear," -> Remove and state the conclusion.
   - **Colon Reveals**: "The best part: it works." -> "It works."
   - **Superficial Analysis**: Trailing "highlighting / underscoring / showcasing ..." clauses -> Replace with concrete metrics.

4. **Deliverable**:
   - Provide the edited text directly, followed by a concise 2-3 bullet point summary of changes.
