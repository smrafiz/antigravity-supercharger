---
name: profile
description: Inspect and configure Supercharger runtime performance profiles.
---

# Supercharger Performance Profiles

Configure hook evaluation intensity and latency profiles across the workspace.

## Profiles

- **standard (default)**: Full real-time AST scanning, credential path isolation, cloud infra protection, and dynamic statusline.
- **fast**: Skips secondary analytics while maintaining strict security barriers (sub-30µs latency).
- **minimal**: Enforces only fatal destructive command blocks and secret writes.

## Commands

```bash
supercharger status
supercharger doctor
```
