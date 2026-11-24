---
name: cache-clear
description: Safely clear local Supercharger audit logs and temporary statusline cache.
---

# Supercharger Cache Clear

Reset local session audit trails and temporary state without modifying repository files.

## Usage

Run the cache clear command via the terminal:

```bash
supercharger cache-clear
```

## Cleared Targets

- `.supercharger/audit.jsonl`: Local workspace audit trail.
- `/tmp/supercharger-statusline-input.json`: Ephemeral statusline input snapshot.
