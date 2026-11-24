---
name: notifier
description: Desktop and terminal notification manager for completed background tasks and long-running jobs.
---

# Supercharger Notifier

Dispatch system and terminal notifications when background commands, long test suites, or CI runs finish.

## Notification Triggers

1. **Background Tasks**: Alert when long-running builds or tests complete.
2. **CI Pipeline**: Notify when GitHub Actions workflow runs succeed or fail.
3. **Safety Blocks**: Surface high-priority desktop alerts when fatal security guards intercept unauthorized operations.

## Notification Channels

- macOS Notification Center (via `osascript` or native binary).
- Terminal audible bell (`\a`) for headless environments.
