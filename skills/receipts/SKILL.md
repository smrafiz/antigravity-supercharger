---
name: receipts
description: Verify cryptographic audit trail records and guard verification receipts.
---

# Supercharger Audit Receipts

Inspect and verify recorded security events and tool decisions logged in the local audit trail.

## Usage

View recent audited decisions:

```bash
supercharger audit 25
```

## Receipt Fields

- **Timestamp**: Time of invocation.
- **Tool**: Target tool name (`run_command`, `write_file`, `edit_file`).
- **Status**: `ALLOWED` or `BLOCKED`.
- **Reason**: Specific security rule or safety boundary triggered.
