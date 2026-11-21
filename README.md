# Antigravity Supercharger

[![CI](https://github.com/smrafiz/antigravity-supercharger/actions/workflows/ci.yml/badge.svg)](https://github.com/smrafiz/antigravity-supercharger/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Blazing fast, memory-safe guardrails, dynamic statusline, and workflow modes for Google Antigravity CLI (`agy`), built in Rust.**

---

## Highlights

- **Sub-millisecond execution (< 1ms)**: Built in Rust with zero runtime dependencies. No Python or Node.js startup lag.
- **Dynamic 3-Line Statusline**: Real-time context usage bar, actual cache hit ratio with saved token metrics, live git branch and uncommitted numstat diff (`+X/-Y`), session duration, and quota burn.
- **Workflow Modes**:
  - **Autopilot (`/autopilot`)**: Time-boxed auto-approval of safe operations so you aren't interrupted by repetitive prompts during autonomous runs.
  - **Read-Only (`/readonly`)**: Time-boxed "look, don't touch" review mode that blocks file writes and mutating commands while allowing inspections and searches.
  - **Strict (`/strict`)**: Time-boxed "ask me everything" mode requiring user confirmation on all actions (overrides autopilot).
- **Destructive Command Prevention**: Blocks `rm -rf /`, `DROP TABLE`, `mkfs`, fork bombs, and curl-pipe-to-shell before execution outside model prompt control.
- **Git Guardrails**: Blocks force-pushing to `main`/`master`, warns on `git reset --hard` and `git clean -f`.
- **Path & Boundary Guard**: Enforces workspace directory boundaries, blocks directory traversal (`../`), and protects `.git/hooks` and system credential folders (`~/.ssh`, `~/.aws`).
- **Code Security & Secret Scanner**: Scans writes and edits in real-time for hardcoded API keys (AWS, OpenAI, GitHub, Slack, Private Keys) and dangerous patterns (`pickle.load`, raw SQL injection).
- **Loop Detection**: Automatically flags repetitive tool loops (3 identical tool calls in 30 seconds).
- **Integrated Audit Trail**: Redacts secrets and logs all tool executions to `.supercharger/audit.jsonl`.

---

## 3-Line Statusline Preview

```
[Gemini 3.8 Flash] antigravity-supercharger | main | Eco: Minimal · Concise | ⚡ Supercharger: active | +28/-14
██░░░░░░░░░░░░░░░░░░ Context: 11% (125.4K/1.0M) | Time: 2m 22s | cache 100% (~99.0K saved)
Tokens: 126.6K/$0.18 equiv | Session: 25% (resets: 3h 0m) · Weekly: 14%
```

- **Line 1**: Model | Workspace name | Git branch | Economy tier | Autopilot badge | Uncommitted diff (`+X/-Y`)
- **Line 2**: Context usage progress bar | Exact used/max token ratio | Elapsed time | Dynamic cache hit ratio & tokens saved
- **Line 3**: Total tokens / USD cost | Subagent token & spend breakdown (when active) | Real-time session & weekly quota resets

---

## Installation & Automatic Activation

### 1-Step Install (Globally Activated Across All Projects)

```bash
git clone https://github.com/smrafiz/antigravity-supercharger.git
cd antigravity-supercharger
bash install.sh
```

`install.sh` builds the optimized release binary, symlinks the plugin into Antigravity's global plugin store (`~/.gemini/config/plugins/supercharger`), and configures `statusLine.command` in `~/.gemini/antigravity-cli/settings.json`.

Verify installation:
```bash
agy plugin list
# Output:
# Installed plugins:
# - supercharger (enabled, 0.1.0)
#     Components: 7 skills, 1 rule, 1 hook
```

---

## Commands & Modes

### CLI Commands

```bash
# Check Supercharger status and active modes
supercharger status

# Auto-approve safe actions for 30 minutes
supercharger autopilot 30m
supercharger autopilot 1h --full
supercharger autopilot off

# Lock workspace to read-only for 1 hour
supercharger readonly 1h
supercharger readonly off

# Enable strict confirmation mode
supercharger strict 45m
supercharger strict off

# View recent blocked actions in the audit log
supercharger audit 15
```

### Slash Skills (in Antigravity Chat)

- **`/autopilot`**: Toggle time-boxed auto-approval.
- **`/readonly`**: Toggle "look, don't touch" mode.
- **`/strict`**: Toggle strict confirmation prompts.
- **`/sc-status`**: Check health of guardrails and active modes.
- **`/audit`**: Audit the workspace for uncommitted risk and secrets.
- **`/interview`**: Deep interactive requirement interview before coding.
- **`/devlog`**: Generate structured development log entries.

---

## Architecture

```
Antigravity CLI (agy)
       │
       ├── PreToolUse    ──▶ bin/supercharger pre-tool     ──▶ Allow / Deny / Ask
       ├── PostToolUse   ──▶ bin/supercharger post-tool    ──▶ Loop Detection & Audit
       ├── SessionStart  ──▶ bin/supercharger session-start──▶ Startup Banner
       ├── Stop          ──▶ bin/supercharger stop         ──▶ Quality Gate
       └── statusLine    ──▶ bin/supercharger statusline   ──▶ 3-Line Dynamic Status
```

---

## License

MIT License — Developed by [smrafiz](https://github.com/smrafiz).
