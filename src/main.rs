mod audit;
mod code_scanner;
mod loop_detector;
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
        "stop" => handle_stop(),
        "statusline" => handle_statusline(),
        "version" | "--version" | "-v" => {
            println!("Antigravity Supercharger v{}", VERSION);
        }
        _ => {
            eprintln!("Usage: supercharger <pre-tool|post-tool|stop|statusline|version>");
            std::process::exit(1);
        }
    }
}

fn read_stdin() -> HookInput {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    serde_json::from_str(&buffer).unwrap_or_default()
}

fn handle_pre_tool() {
    let input = read_stdin();
    let mut workspace_paths = input.workspace_paths.unwrap_or_default();
    if let Some(art) = input.artifact_directory_path {
        workspace_paths.push(art);
    }

    if let Some(tool_call) = input.tool_call {
        let tool_name = tool_call.name.as_str();

        // 1. Check Shell Commands
        if tool_name == "run_command" {
            if let Some(cmd) = tool_call.get_str_arg("CommandLine") {
                if let Some(verdict) = safety::check_command(&cmd) {
                    audit::log_event(
                        input.cwd.as_deref(),
                        tool_name,
                        &cmd,
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
        }

        // 2. Check File Writes and Edits
        if tool_name == "write_file" || tool_name == "edit_file" {
            if let Some(path) = tool_call.get_str_arg("Path") {
                if let Some(verdict) = path_guard::check_path(&path, &workspace_paths) {
                    audit::log_event(
                        input.cwd.as_deref(),
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

            // Inspect content
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
                            input.cwd.as_deref(),
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
        audit::log_event(input.cwd.as_deref(), tool_name, "tool_executed", false, None);
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
    // Default allow stop
    println!("{}", serde_json::to_string(&StopDecision::stop()).unwrap());
}

fn handle_statusline() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let input: statusline::StatusLineInput = serde_json::from_str(&buffer).unwrap_or_default();
    statusline::render(&input);
}
