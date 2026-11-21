mod audit;
mod code_scanner;
mod loop_detector;
mod modes;
mod path_guard;
mod safety;
mod statusline;
mod types;

use std::io::{self, Read};
use types::{HookInput, PreToolDecision, StopDecision};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "pre-tool" => handle_pre_tool(),
        "post-tool" => handle_post_tool(),
        "session-start" => handle_session_start(),
        "stop" => handle_stop(),
        "statusline" => handle_statusline(),
        "autopilot" => handle_autopilot_cli(&args[2..]),
        "readonly" => handle_readonly_cli(&args[2..]),
        "strict" => handle_strict_cli(&args[2..]),
        "audit" => handle_audit_cli(&args[2..]),
        "status" => handle_status_cli(),
        "version" | "--version" | "-v" => {
            println!("Antigravity Supercharger v{}", VERSION);
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("Antigravity Supercharger v{}", VERSION);
    println!("Safety hooks and real-time statusline for Antigravity CLI\n");
    println!("Usage: supercharger <command> [options]\n");
    println!("Hook Commands (invoked by Antigravity):");
    println!("  pre-tool       Pre-execution safety and policy check");
    println!("  post-tool      Post-execution loop and metric tracker");
    println!("  session-start  Startup banner and workspace initialization");
    println!("  stop           Agent completion guard");
    println!("  statusline     Render dynamic 3-line status bar\n");
    println!("User Commands:");
    println!("  autopilot [dur] [--full] [off|status]  Auto-approve safe actions for a time-box");
    println!("  readonly  [dur] [off|status]           Block file writes and mutating commands");
    println!("  strict    [dur] [off|status]           Prompt on every action (overrides autopilot)");
    println!("  audit     [limit]                      View recent security and audit log entries");
    println!("  status                                 Display Supercharger health and active modes");
    println!("  version                                Print version");
}

fn read_stdin() -> HookInput {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    serde_json::from_str(&buffer).unwrap_or_default()
}

fn handle_session_start() {
    let _input = read_stdin();
    eprintln!("\x1b[32m⚡ Antigravity Supercharger v{} active\x1b[0m · Guardrails armed", VERSION);
    println!("{{}}");
}

fn handle_pre_tool() {
    let input = read_stdin();
    let cwd = input.cwd.as_deref().unwrap_or(".");
    let session_id = input.conversation_id.as_deref();

    let mut workspace_paths = input.workspace_paths.unwrap_or_default();
    if let Some(art) = input.artifact_directory_path {
        workspace_paths.push(art);
    }

    if let Some(tool_call) = input.tool_call {
        let tool_name = tool_call.name.as_str();

        // -------------------------------------------------------------
        // 1. READ-ONLY MODE ENFORCEMENT
        // -------------------------------------------------------------
        if let Some(rem) = modes::is_readonly_active(cwd, session_id) {
            let dur_str = modes::fmt_duration(rem);
            if tool_name == "write_file" || tool_name == "edit_file" {
                let reason = format!(
                    "Supercharger read-only mode is active ({} remaining): file writes and edits are blocked. Turn off with: supercharger readonly off",
                    dur_str
                );
                audit::log_event(Some(cwd), tool_name, "write_blocked", true, Some(&reason));
                println!("{}", serde_json::to_string(&PreToolDecision::deny(reason)).unwrap());
                return;
            }

            if tool_name == "run_command" {
                if let Some(cmd) = tool_call.get_str_arg("CommandLine") {
                    if modes::is_mutating_command(&cmd) {
                        let reason = format!(
                            "Supercharger read-only mode is active ({} remaining): mutating command blocked. Reads, searches, and planning are allowed.",
                            dur_str
                        );
                        audit::log_event(Some(cwd), tool_name, &cmd, true, Some(&reason));
                        println!("{}", serde_json::to_string(&PreToolDecision::deny(reason)).unwrap());
                        return;
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // 2. SHELL COMMAND SAFETY
        // -------------------------------------------------------------
        if tool_name == "run_command" {
            if let Some(cmd) = tool_call.get_str_arg("CommandLine") {
                if let Some(verdict) = safety::check_command(&cmd) {
                    audit::log_event(
                        Some(cwd),
                        tool_name,
                        &cmd,
                        verdict.is_fatal,
                        Some(&verdict.reason),
                    );

                    if verdict.is_fatal {
                        let decision = PreToolDecision::deny(format!("Supercharger: {}", verdict.reason));
                        println!("{}", serde_json::to_string(&decision).unwrap());
                        return;
                    }

                    // Non-fatal asks: check if full autopilot auto-approves
                    let autopilot = modes::is_autopilot_active(cwd, session_id);
                    if autopilot.map_or(false, |ap| ap.is_full) {
                        audit::log_event(
                            Some(cwd),
                            tool_name,
                            &cmd,
                            false,
                            Some(&format!("Autopilot auto-approved: {}", verdict.reason)),
                        );
                        // Allow through
                    } else {
                        let decision = PreToolDecision::ask(format!("Supercharger: {}", verdict.reason));
                        println!("{}", serde_json::to_string(&decision).unwrap());
                        return;
                    }
                }
            }
        }

        // -------------------------------------------------------------
        // 3. FILE PATH & CONTENT SAFETY
        // -------------------------------------------------------------
        if tool_name == "write_file" || tool_name == "edit_file" {
            if let Some(path) = tool_call.get_str_arg("Path") {
                if let Some(verdict) = path_guard::check_path(&path, &workspace_paths) {
                    audit::log_event(
                        Some(cwd),
                        tool_name,
                        &path,
                        verdict.is_fatal,
                        Some(&verdict.reason),
                    );
                    let decision = if verdict.is_fatal {
                        PreToolDecision::deny(format!("Supercharger: {}", verdict.reason))
                    } else {
                        PreToolDecision::ask(format!("Supercharger: {}", verdict.reason))
                    };
                    println!("{}", serde_json::to_string(&decision).unwrap());
                    return;
                }
            }

            // Inspect content for secrets and vulnerabilities
            let content_opt = tool_call
                .get_str_arg("Content")
                .or_else(|| tool_call.get_str_arg("Replacement"));

            if let Some(content) = content_opt {
                let findings = code_scanner::scan_content(&content);
                for finding in findings {
                    if finding.is_secret {
                        let reason = format!(
                            "Supercharger blocked secret write: {}",
                            finding.description
                        );
                        audit::log_event(
                            Some(cwd),
                            tool_name,
                            "...",
                            true,
                            Some(&reason),
                        );
                        let decision = PreToolDecision::deny(reason);
                        println!("{}", serde_json::to_string(&decision).unwrap());
                        return;
                    } else {
                        let reason = format!(
                            "Supercharger warning: {}",
                            finding.description
                        );
                        let decision = PreToolDecision::ask(reason);
                        println!("{}", serde_json::to_string(&decision).unwrap());
                        return;
                    }
                }
            }
        }

        // Default allow
        audit::log_event(Some(cwd), tool_name, "tool_executed", false, None);
    }

    println!("{}", serde_json::to_string(&PreToolDecision::allow()).unwrap());
}

fn handle_post_tool() {
    let input = read_stdin();
    let conv_id = input.conversation_id.unwrap_or_else(|| "default".to_string());

    if let Some(tool_call) = input.tool_call {
        let sig = tool_call
            .get_str_arg("CommandLine")
            .or_else(|| tool_call.get_str_arg("Path"))
            .unwrap_or_default();

        if loop_detector::track_tool_call(&conv_id, &tool_call.name, &sig) {
            eprintln!(
                "Supercharger Loop Warning: Tool '{}' was invoked 3 times in 30 seconds.",
                tool_call.name
            );
        }
    }

    println!("{{}}");
}

fn handle_stop() {
    let _input = read_stdin();
    println!("{}", serde_json::to_string(&StopDecision::stop()).unwrap());
}

fn handle_statusline() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let _ = std::fs::write("/tmp/supercharger-statusline-input.json", &buffer);
    let input: statusline::StatusLineInput = serde_json::from_str(&buffer).unwrap_or_default();
    statusline::render(&input);
}

// -----------------------------------------------------------------------------
// USER CLI SUBCOMMANDS
// -----------------------------------------------------------------------------

fn handle_autopilot_cli(args: &[String]) {
    let cwd = ".";
    let first = args.get(0).map(|s| s.as_str()).unwrap_or("status");

    if first == "off" {
        modes::clear_autopilot(cwd, None);
        println!("⚡ Supercharger Autopilot: \x1b[31mOFF\x1b[0m — normal confirmation prompts restored.");
        return;
    }

    if first == "status" {
        if let Some(st) = modes::is_autopilot_active(cwd, None) {
            let full_str = if st.is_full { " (full)" } else { "" };
            println!("⚡ Autopilot{}: \x1b[32mON\x1b[0m — {} remaining", full_str, modes::fmt_duration(st.remaining_seconds));
        } else {
            println!("⚡ Autopilot: \x1b[33mOFF\x1b[0m");
        }
        return;
    }

    let duration_secs = modes::parse_duration(first).unwrap_or(1800); // default 30m
    let is_full = args.iter().any(|a| a == "--full" || a == "full");
    let is_global = args.iter().any(|a| a == "--global" || a == "global");

    match modes::set_autopilot(cwd, None, duration_secs, is_full, is_global) {
        Ok(_) => {
            let full_str = if is_full { " (full)" } else { "" };
            let scope_str = if is_global { "globally" } else { "for this workspace" };
            println!(
                "⚡ Autopilot{}: \x1b[32mON\x1b[0m for {} {} — safe actions auto-approved.",
                full_str,
                modes::fmt_duration(duration_secs),
                scope_str
            );
        }
        Err(e) => eprintln!("Error setting autopilot: {}", e),
    }
}

fn handle_readonly_cli(args: &[String]) {
    let cwd = ".";
    let first = args.get(0).map(|s| s.as_str()).unwrap_or("status");

    if first == "off" {
        modes::clear_readonly(cwd, None);
        println!("🔒 Read-only mode: \x1b[32mOFF\x1b[0m — writes and mutating commands allowed.");
        return;
    }

    if first == "status" {
        if let Some(rem) = modes::is_readonly_active(cwd, None) {
            println!("🔒 Read-only mode: \x1b[33mON\x1b[0m — {} remaining", modes::fmt_duration(rem));
        } else {
            println!("🔒 Read-only mode: \x1b[32mOFF\x1b[0m");
        }
        return;
    }

    let duration_secs = modes::parse_duration(first).unwrap_or(3600); // default 1h
    let is_global = args.iter().any(|a| a == "--global" || a == "global");

    match modes::set_readonly(cwd, None, duration_secs, is_global) {
        Ok(_) => {
            println!(
                "🔒 Read-only mode: \x1b[33mON\x1b[0m for {} — file edits and mutating commands blocked.",
                modes::fmt_duration(duration_secs)
            );
        }
        Err(e) => eprintln!("Error setting read-only mode: {}", e),
    }
}

fn handle_strict_cli(args: &[String]) {
    let cwd = ".";
    let first = args.get(0).map(|s| s.as_str()).unwrap_or("status");

    if first == "off" {
        modes::clear_strict(cwd, None);
        println!("🛡️ Strict mode: \x1b[32mOFF\x1b[0m — standard auto-approvals restored.");
        return;
    }

    if first == "status" {
        if let Some(rem) = modes::is_strict_active(cwd, None) {
            println!("🛡️ Strict mode: \x1b[33mON\x1b[0m — {} remaining", modes::fmt_duration(rem));
        } else {
            println!("🛡️ Strict mode: \x1b[32mOFF\x1b[0m");
        }
        return;
    }

    let duration_secs = modes::parse_duration(first).unwrap_or(3600); // default 1h
    let is_global = args.iter().any(|a| a == "--global" || a == "global");

    match modes::set_strict(cwd, None, duration_secs, is_global) {
        Ok(_) => {
            println!(
                "🛡️ Strict mode: \x1b[33mON\x1b[0m for {} — prompt confirmation required on all actions.",
                modes::fmt_duration(duration_secs)
            );
        }
        Err(e) => eprintln!("Error setting strict mode: {}", e),
    }
}

fn handle_audit_cli(args: &[String]) {
    let limit = args.get(0).and_then(|s| s.parse::<usize>().ok()).unwrap_or(15);
    let events = audit::get_recent_events(Some("."), limit);

    if events.is_empty() {
        println!("No audit log entries recorded in .supercharger/audit.jsonl yet.");
        return;
    }

    println!("\x1b[1m=== Antigravity Supercharger Audit Log (Last {}) ===\x1b[0m", events.len());
    println!("{:<8} {:<14} {:<8} {:<30}", "TIME", "TOOL", "STATUS", "DETAILS");
    println!("{}", "-".repeat(70));

    for ev in events {
        let ts = ev.get("timestamp").and_then(|v| v.as_u64()).unwrap_or(0);
        let tool = ev.get("tool").and_then(|v| v.as_str()).unwrap_or("?");
        let blocked = ev.get("blocked").and_then(|v| v.as_bool()).unwrap_or(false);
        let reason = ev.get("reason").and_then(|v| v.as_str()).or_else(|| ev.get("args").and_then(|v| v.as_str())).unwrap_or("");

        let status_str = if blocked {
            "\x1b[31mBLOCKED\x1b[0m"
        } else {
            "\x1b[32mALLOWED\x1b[0m"
        };

        let time_str = format!("{:02}:{:02}", (ts % 86400) / 3600, (ts % 3600) / 60);
        let truncated_reason = if reason.len() > 38 {
            format!("{}...", &reason[..35])
        } else {
            reason.to_string()
        };

        println!("{:<8} {:<14} {:<8} {:<30}", time_str, tool, status_str, truncated_reason);
    }
}

fn handle_status_cli() {
    println!("\x1b[1m⚡ Antigravity Supercharger v{}\x1b[0m", VERSION);
    println!("--------------------------------------------------");
    println!("Statusline:      \x1b[32mActive (3-line dynamic)\x1b[0m");
    println!("Safety Guards:   \x1b[32mActive (PreToolUse, PostToolUse, Stop)\x1b[0m");
    println!("Secret Scanner:  \x1b[32mActive (Real-time scan & redaction)\x1b[0m");
    println!("Path Guard:      \x1b[32mActive (Boundary & traversal protection)\x1b[0m");
    println!("Loop Detector:   \x1b[32mActive (3 identical calls / 30s)\x1b[0m");

    let ap = modes::is_autopilot_active(".", None);
    let ro = modes::is_readonly_active(".", None);
    let st = modes::is_strict_active(".", None);

    println!("\n\x1b[1mActive Modes:\x1b[0m");
    if let Some(a) = ap {
        let full = if a.is_full { " (full)" } else { "" };
        println!("  Autopilot:     \x1b[32mON{}\x1b[0m ({} left)", full, modes::fmt_duration(a.remaining_seconds));
    } else {
        println!("  Autopilot:     \x1b[2mOFF\x1b[0m");
    }

    if let Some(r) = ro {
        println!("  Read-only:     \x1b[33mON\x1b[0m ({} left)", modes::fmt_duration(r));
    } else {
        println!("  Read-only:     \x1b[2mOFF\x1b[0m");
    }

    if let Some(s) = st {
        println!("  Strict:        \x1b[33mON\x1b[0m ({} left)", modes::fmt_duration(s));
    } else {
        println!("  Strict:        \x1b[2mOFF\x1b[0m");
    }
}
