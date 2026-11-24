---
name: cache-stats
description: Show prompt cache hit ratio, token savings, and Supercharger audit statistics.
---

# Supercharger Cache Stats

Display real-time prompt cache efficiency, token savings, and security audit metrics.

## Inspection

Run the Supercharger cache statistics CLI:

```bash
supercharger cache-stats
```

## Metrics Breakdown

- **Prompt Cache Hit**: Percentage of tokens served directly from cache (~95%+ on long sessions).
- **Saved Tokens**: Approximate number of input tokens avoided through prefix caching.
- **Audit Operations**: Count of allowed vs blocked security events.
- **Active Economy Tier**: Current token verbosity mode (`minimal`, `lean`, `standard`).
