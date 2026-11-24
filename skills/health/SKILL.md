---
name: health
description: Comprehensive workspace, binary, and hook health diagnostics.
---

# Supercharger Health Check

Verify end-to-end integration across Antigravity hooks, Rust engine binaries, and active workspace configurations.

## Health Verification

Run the full diagnostic suite:

```bash
supercharger doctor
supercharger status
supercharger cache-stats
```

## Success Indicators

- **Guard Latency**: < 100 µs average evaluation time.
- **Hook Status**: All 3 hooks (`PreToolUse`, `PostToolUse`, `Stop`) registered and responding.
- **Audit Logging**: `.supercharger/audit.jsonl` writable and logging events.
- **Statusline**: Renders live context and quota buckets without errors.
