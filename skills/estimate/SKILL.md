---
name: estimate
description: Estimate token consumption, context window headroom, and equivalent API costs before executing large operations.
---

# Supercharger Token & Cost Estimator

Assess context usage, token footprint, and model cost projections to optimize prompt economics and avoid context overruns.

## Estimation Process

1. **Inspect Context Window**:
   - Check current context fill percentage from the Supercharger statusline.
   - Warn if context usage exceeds 50% (~500K tokens on 1M models) to recommend compaction.

2. **File Size & Token Conversion**:
   - Rule of thumb: ~4 characters per token for source code and prose.
   - Estimate input token footprint for target files before loading entire directories.

3. **Cost Projection**:
   - Calculate estimated cost based on active model rates (e.g. Gemini 3.8 Flash: $0.075 / 1M input, $0.30 / 1M output).
   - Recommend the `minimal` or `lean` Economy tier when processing batch tasks.
