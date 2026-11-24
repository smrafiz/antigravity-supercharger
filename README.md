# Antigravity Supercharger

[![CI](https://github.com/smrafiz/antigravity-supercharger/actions/workflows/ci.yml/badge.svg)](https://github.com/smrafiz/antigravity-supercharger/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Blazing fast, memory-safe guardrails, dynamic statusline, and workflow modes for Google Antigravity CLI (`agy`), built in Rust.**

---

## Highlights

- **Sub-millisecond execution (30 µs latency)**: Built in pure Rust with zero runtime dependencies. Eliminates Python and Node.js process startup lag completely.
- **Dynamic 3-Line Statusline**: Live context bar, accurate prompt cache hit ratio with saved token metrics, uncommitted diff (`+X/-Y`), session stopwatch, API equivalent costs, and rolling quota resets (`gemini-5h` / `weekly`).
- **Workflow Modes**:
  - **Autopilot (`supercharger autopilot [dur] [--full]`)**: Auto-approves safe tool calls during autonomous runs.
  - **Read-Only (`supercharger readonly [dur]`)**: Blocks file writes, modifications, and mutating commands.
  - **Strict (`supercharger strict [dur]`)**: Forces confirmation prompts on every action.
  - **Economy (`supercharger economy [minimal|lean|standard]`)**: Switches model prompt verbosity tier with dynamic rule injection.
- **Safety & Guardrails**:
  - **Destructive Command Guard**: Blocks `rm -rf /`, `mkfs`, fork bombs, SQL database drops, and reverse shells (`nc -e`, `/dev/tcp/`).
  - **Test Mask Guard**: Intercepts shell test masking (`npm test || true`, `pytest || exit 0`, `cargo test ; true`) that hides failures.
  - **Cloud & IaC Protection**: Intercepts `terraform destroy`, AWS EC2/RDS/S3 bulk delete, GCP project/compute deletion, Azure resource group wipe, and Kubernetes namespace removal.
  - **Git Guardrails**: Blocks force-pushes to `main`/`master`, warns on `git reset --hard`, `git commit --amend`, and `git commit/push --no-verify`.
  - **File Clobbering & Exfiltration**: Intercepts shell redirection truncation (`> file.rs`, `truncate -s 0`) and credential exfiltration via `curl`/`wget`.
  - **Secret Scanner**: Real-time scanning & redaction of API keys (AWS, OpenAI, GitHub, Slack, Private Keys, JWTs).
  - **Supply-Chain & Workflow PWN Defense**: Flags `pull_request_target` untrusted PR head checkout vulnerabilities and malicious lifecycle scripts in `package.json`.
  - **Test Integrity Guard**: Detects test suite skip markers (`.skip`, `@pytest.mark.skip`, `#[ignore]`) that bypass test verification.
  - **Prompt Injection Defense**: Scans tool outputs and file edits for adversarial tags (`<IMPORTANT>`, `<SYSTEM>`) and instruction overrides (`ignore all previous instructions`).
  - **Loop Detector**: Intercepts repetitive agent cycles (3 identical tool calls in 30 seconds).

---

## 3-Line Statusline Preview

```text
[Gemini 3.8 Flash (High)] antigravity-supercharger | main | Eco: Minimal | ⚡ Autopilot (full): 479m left
█░░░░░░░░░░░░░░░░░░░ Context: 6% (58.7K/1.0M) | Time: 2m 22s | cache 100% (~47.0K saved)
Tokens: 270.1K/$0.06 equiv | Session: 0% (resets: 4h 56m) · Weekly: 0%
```

- **Line 1**: Model | Current workspace | Git branch | Economy tier | Autopilot/Supercharger badge
- **Line 2**: Context visual bar | Exact used/max token ratio | Session duration | Cache hit % & saved tokens
- **Line 3**: Total tokens & API equivalent cost | Real-time session & weekly quota resets (`gemini-5h`, `weekly`)

---

## CLI Reference

```bash
# Workflow Modes
supercharger autopilot 2h --full        # Enable full autopilot for 2 hours
supercharger readonly 1h                # Enable read-only mode for 1 hour
supercharger strict 30m                 # Enable strict prompt confirmation
supercharger economy minimal            # Switch economy tier (minimal, lean, standard)

# Diagnostics & Auditing
supercharger doctor                     # Run comprehensive diagnostic benchmark
supercharger status                     # Display active modes and guard health
supercharger audit 25                   # Inspect last 25 security audit events
supercharger cache-stats                # Display prompt cache savings and audit metrics
supercharger cache-clear                # Purge local audit trails and temporary cache
supercharger update                     # Check for and apply latest Supercharger updates
```

---

## Skills Catalog (40 Skills Available)

| Skill | Description |
|---|---|
| **audit** | Workspace security audit against Supercharger safety rules |
| **autopilot** | Time-boxed auto-approval of safe operations |
| **cache-clear** | Safely clear local audit logs and temporary cache |
| **cache-stats** | Prompt cache hit ratio, token savings, and audit statistics |
| **challenge** | Adversarial red-team challenge to stress-test designs and PRs |
| **cleanup** | Repository hygiene, stale branch cleanup, and cache pruning |
| **de-ai** | Strip AI buzzwords and robotic corporate filler from text and PRs |
| **design** | Structured architecture and system design review |
| **design-review** | UI review for WCAG 2.2 AA contrast, visual hierarchy, and 3 viewports |
| **devlog** | Structured development logs capturing decisions and changes |
| **doctor** | Diagnostic health check of Supercharger binary and hooks |
| **economy** | Switch token economy tier (minimal, lean, standard) |
| **estimate** | Context window headroom and equivalent API cost estimation |
| **handoff** | Structured session handoff briefing documentation |
| **health** | Comprehensive workspace, binary, and hook health diagnostics |
| **interview** | Interactive requirement and trade-off clarification interview |
| **learn** | Extract reusable developer conventions into Antigravity rules |
| **mcp-audit** | Audit MCP server definitions for hidden instructions |
| **memory-prune** | Session memory and context window optimization |
| **multi-review** | Adversarial 4-lens code review (Security, Correctness, Perf, Tests) |
| **notifier** | Native OS desktop notification integration for completion |
| **perf** | Performance audit and resource bottleneck identification |
| **pr** | Pull request generator with pre-flight branch and secret checks |
| **profile** | Inspect and configure runtime performance profiles |
| **readonly** | Time-boxed read-only mode blocking file modifications |
| **receipts** | Verify cryptographic audit trail records and receipts |
| **reflect** | Score session quality against concrete evidence |
| **resolve-conflicts** | Guided resolution of git merge, rebase, and cherry-pick conflicts |
| **sc-status** | Display Supercharger status and active guardrails |
| **scope** | Scope creep detector and blast-radius evaluator |
| **security** | Deep security audit against OWASP Top 10 and injection sinks |
| **ste** | Self-Testing Engine protocol for pre-flight verification |
| **strict** | Time-boxed confirmation mode overriding autopilot |
| **stuck** | Break out of debugging loops and cognitive deadlocks |
| **supercharger** | Router and command catalog matching situations to skills |
| **think** | Deep deliberation and counter-argument analysis |
| **trust-mcp** | Authorize trusted MCP servers for elicitation |
| **unslop** | Redesign generic AI interfaces into distinct, intentional products |
| **update** | Check for and apply Antigravity Supercharger updates from GitHub |
| **why** | Root-cause causal analysis using the 5-Whys methodology |

---

## Installation

### 1-Step Global Installation

```bash
git clone https://github.com/smrafiz/antigravity-supercharger.git
cd antigravity-supercharger
bash install.sh
```

The installer compiles the release Rust binary, links the plugin globally to `~/.gemini/config/plugins/supercharger`, and activates safety hooks across all your workspaces automatically.

---

## License

MIT License. See [LICENSE](LICENSE) for details.
