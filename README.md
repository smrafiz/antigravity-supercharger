# Antigravity Supercharger

**Blazing fast, memory-safe guardrails and security hooks for Google Antigravity CLI (`agy`), built in Rust.**

---

## Highlights

- **Sub-millisecond execution (< 1ms)**: Built in Rust with zero runtime dependencies. No Python or Node.js startup lag.
- **Destructive Command Prevention**: Blocks `rm -rf /`, `DROP TABLE`, `mkfs`, fork bombs, and curl-pipe-to-shell before execution.
- **Git Guardrails**: Blocks force-pushing to `main`/`master`, warns on `git reset --hard` and `git clean -f`.
- **Path & Self-Modification Guard**: Enforces workspace directory boundaries and blocks tampering with `.git/hooks`, system folders (`/etc`), and security configs.
- **Code Security & Secret Scanner**: Scans writes and edits in real-time for hardcoded API keys (AWS, OpenAI, GitHub, Slack, Private Keys) and dangerous AST patterns (`pickle.load`, raw SQL injection).
- **Repetitive Loop Detection**: Automatically alerts when an agent repeatedly executes the exact same tool with the same arguments.
- **Progressive Disclosure Skills**: Ships built-in skills for `/interview`, `/devlog`, and `/audit`.
- **Integrated Audit Trail**: Redacts secrets and logs all tool executions to `.supercharger/audit.jsonl`.

---

## Architecture

Antigravity Supercharger integrates directly with Antigravity CLI's plugin and lifecycle hook architecture:

```
Antigravity CLI (agy)
       │
       ├── PreToolUse  ──▶ bin/supercharger pre-tool  ──▶ Allow / Deny / Ask
       ├── PostToolUse ──▶ bin/supercharger post-tool ──▶ Loop Detection & Audit
       └── Stop        ──▶ bin/supercharger stop      ──▶ Quality & Test Verification
```

---

## Installation & Build

### Requirements
- Rust 1.80+ (`cargo`, `rustc`)
- Antigravity CLI (`agy`)

### Build
```bash
git clone https://github.com/smrafiz/antigravity-supercharger.git
cd antigravity-supercharger
bash install.sh
```

### Enable in Antigravity CLI

#### Option A: Workspace-specific (Recommended)
Symlink or copy the directory into your project's `.agents/plugins/` directory:
```bash
mkdir -p .agents/plugins
ln -s /path/to/antigravity-supercharger .agents/plugins/supercharger
```

#### Option B: Global (Applies to all projects)
Symlink into your global Antigravity config:
```bash
mkdir -p ~/.gemini/config/plugins
ln -s /path/to/antigravity-supercharger ~/.gemini/config/plugins/supercharger
```

---

## Included Skills

- **`interview`**: Run requirement-gathering interviews before coding.
- **`devlog`**: Generate structured development log entries.
- **`audit`**: Run an audit on the current workspace for hardcoded secrets and uncommitted risk.

---

## License

MIT License — Developed by Starter Developer ([smrafiz](https://github.com/smrafiz)).
