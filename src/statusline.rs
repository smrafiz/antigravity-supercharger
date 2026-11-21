use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct StatusLineInput {
    pub session_id: Option<String>,
    pub current_dir: Option<String>,
    pub project_dir: Option<String>,
    pub workspace: Option<WorkspaceInfo>,
    pub model: Option<ModelInfo>,
    pub cost: Option<CostInfo>,
    pub vcs: Option<VcsInfo>,
    pub context_window: Option<ContextWindowInfo>,
    pub quota: Option<QuotaInfo>,
    pub rate_limits: Option<RateLimitsInfo>,
    pub subagents: Option<Vec<SubagentInfo>>,
    pub terminal_width: Option<usize>,
    pub conversation_title: Option<String>,
    pub output_style: Option<OutputStyleInfo>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct WorkspaceInfo {
    pub current_dir: Option<String>,
    pub project_dir: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct OutputStyleInfo {
    pub name: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct ModelInfo {
    pub display_name: Option<String>,
    pub id: Option<String>,
    pub effort: Option<String>,
    pub role: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct CostInfo {
    pub total_cost_usd: Option<f64>,
    pub total_duration_ms: Option<u64>,
    pub total_lines_added: Option<u64>,
    pub total_lines_removed: Option<u64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct VcsInfo {
    pub branch: Option<String>,
    pub dirty: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct ContextWindowInfo {
    pub used_percentage: Option<f64>,
    pub context_window_size: Option<u64>,
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub current_usage: Option<CurrentUsageInfo>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct CurrentUsageInfo {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct QuotaInfo {
    pub session_percentage: Option<f64>,
    pub session_resets_in_seconds: Option<u64>,
    pub weekly_percentage: Option<f64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct RateLimitsInfo {
    pub five_hour: Option<RateLimitWindow>,
    pub seven_day: Option<RateLimitWindow>,
    pub spend_limit: Option<RateLimitWindow>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct RateLimitWindow {
    pub used_percentage: Option<f64>,
    pub resets_at: Option<f64>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct SubagentInfo {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub model: Option<String>,
    pub total_usd: Option<f64>,
    pub total_tokens: Option<u64>,
}

pub fn fmt_tokens(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

pub fn get_git_info(cwd: &str) -> (String, u64, u64) {
    let mut branch = String::new();
    let mut added = 0;
    let mut removed = 0;

    // Get current branch
    if let Ok(out) = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(cwd)
        .output()
    {
        if out.status.success() {
            branch = String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
    }

    // Get uncommitted numstat
    if let Ok(out) = Command::new("git")
        .args(["diff", "HEAD", "--numstat"])
        .current_dir(cwd)
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let (Ok(a), Ok(r)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                        added += a;
                        removed += r;
                    }
                }
            }
        }
    }

    (branch, added, removed)
}

pub fn render(input: &StatusLineInput) {
    // ANSI formatting
    let bold = "\x1b[1m";
    let dim = "\x1b[2m";
    let cyan = "\x1b[36m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    let cwd = input
        .project_dir
        .as_deref()
        .or_else(|| input.workspace.as_ref().and_then(|w| w.project_dir.as_deref()))
        .or(input.current_dir.as_deref())
        .or_else(|| input.workspace.as_ref().and_then(|w| w.current_dir.as_deref()))
        .unwrap_or(".");

    let project_name = Path::new(cwd)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string());

    let (git_branch, git_added, git_removed) = get_git_info(cwd);
    let branch = if !git_branch.is_empty() {
        git_branch
    } else {
        input
            .vcs
            .as_ref()
            .and_then(|v| v.branch.clone())
            .unwrap_or_else(|| "main".to_string())
    };

    let model_name = input
        .model
        .as_ref()
        .and_then(|m| m.display_name.as_deref().or(m.id.as_deref()))
        .unwrap_or("Gemini");

    let cost_val = input.cost.as_ref().and_then(|c| c.total_cost_usd).unwrap_or(0.0);
    let dur_ms = input.cost.as_ref().and_then(|c| c.total_duration_ms).unwrap_or(0);
    let dur_mins = dur_ms / 60000;
    let dur_secs = (dur_ms % 60000) / 1000;
    let time_str = if dur_mins >= 60 {
        format!("{}h {}m", dur_mins / 60, dur_mins % 60)
    } else {
        format!("{}m {}s", dur_mins, dur_secs)
    };

    // Lines changed
    let lines_str = if git_added > 0 || git_removed > 0 {
        format!(" {dim}|{reset} {green}+{git_added}{reset}{dim}/{reset}{red}-{git_removed}{reset}")
    } else {
        String::new()
    };

    // Autopilot check
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let autopilot_paths = [
        Path::new(cwd).join(".supercharger").join(".autopilot-until"),
        Path::new(cwd).join(".supercharger").join("autopilot-until"),
        Path::new(".supercharger").join(".autopilot-until"),
        Path::new(".supercharger").join("autopilot-until"),
        Path::new(&home).join(".gemini/config/plugins/supercharger/scope/.autopilot-until"),
        Path::new(&home).join(".claude/supercharger/scope/.autopilot-until"),
    ];

    let mut best_remaining: Option<u64> = None;
    for ap_path in autopilot_paths {
        if let Ok(content) = fs::read_to_string(ap_path) {
            if let Ok(until) = content.trim().parse::<u64>() {
                if until > now {
                    best_remaining = Some((until - now) / 60);
                    break;
                }
            }
        }
    }

    let autopilot_str = if let Some(mins) = best_remaining {
        format!(" {dim}|{reset} {yellow}⚡ Autopilot (full): {mins}m left{reset}")
    } else {
        format!(" {dim}|{reset} {green}⚡ Supercharger: active{reset}")
    };

    // Economy & Output style
    let mut eco_str = format!(" {dim}|{reset} {dim}Eco:{reset} Minimal {dim}·{reset} Concise");
    if let Some(style) = input.output_style.as_ref().and_then(|s| s.name.as_deref()) {
        if style.to_lowercase() != "default" {
            eco_str = format!(" {dim}|{reset} {dim}Style:{reset} {style}");
        }
    }

    // ==========================================
    // LINE 1: Model | Project | Branch | Eco | Autopilot | Diff
    // ==========================================
    println!(
        "{cyan}[{model_name}]{reset} {bold}{project_name}{reset} {dim}|{reset} {yellow}{branch}{reset}{eco_str}{autopilot_str}{lines_str}"
    );

    // ==========================================
    // LINE 2: Context Bar | Time | Cache
    // ==========================================
    let ctx = input.context_window.as_ref();
    let ctx_max = ctx.and_then(|c| c.context_window_size).unwrap_or(0);

    let usage = ctx.and_then(|c| c.current_usage.as_ref());
    let in_tok = usage.and_then(|u| u.input_tokens).unwrap_or(0);
    let cache_read = usage.and_then(|u| u.cache_read_input_tokens).unwrap_or(0);
    let cache_create = usage.and_then(|u| u.cache_creation_input_tokens).unwrap_or(0);
    let out_tok = usage.and_then(|u| u.output_tokens).unwrap_or(0);

    let ctx_used = if let Some(tot_in) = ctx.and_then(|c| c.total_input_tokens) {
        if tot_in > 0 {
            tot_in
        } else {
            in_tok + cache_read + cache_create
        }
    } else {
        in_tok + cache_read + cache_create
    };

    let ctx_pct = if let Some(p) = ctx.and_then(|c| c.used_percentage) {
        p.round() as u64
    } else if ctx_max > 0 && ctx_used > 0 {
        ctx_used * 100 / ctx_max
    } else {
        0
    };

    let filled_blocks = (ctx_pct.min(100) as usize * 20 / 100).min(20);
    let empty_blocks = 20 - filled_blocks;
    let bar_color = if ctx_pct >= 90 {
        red
    } else if ctx_pct >= 70 {
        yellow
    } else {
        green
    };
    let bar = format!("{}{}", "█".repeat(filled_blocks), "░".repeat(empty_blocks));

    let ctx_display = if ctx_max > 0 && ctx_used > 0 {
        format!("{ctx_pct}% ({}/{})", fmt_tokens(ctx_used), fmt_tokens(ctx_max))
    } else if ctx_used > 0 {
        format!("{ctx_pct}% ({})", fmt_tokens(ctx_used))
    } else if ctx_max > 0 {
        format!("0% (0/{})", fmt_tokens(ctx_max))
    } else {
        "0%".to_string()
    };

    let cache_total = cache_read + cache_create;
    let cache_str = if cache_total == 0 {
        format!("{dim}cache: n/a{reset}")
    } else if cache_read == 0 {
        format!("{dim}cache: warming{reset}")
    } else {
        let cache_pct = (cache_read as f64 / cache_total as f64 * 100.0).round() as u64;
        let cache_saved = (cache_read as f64 * 0.9) as u64;
        if cache_saved > 0 {
            if cache_pct < 50 {
                format!("{red}cache {cache_pct}%{reset} {dim}(~{} saved){reset}", fmt_tokens(cache_saved))
            } else if cache_pct < 70 {
                format!("{yellow}cache {cache_pct}%{reset} {dim}(~{} saved){reset}", fmt_tokens(cache_saved))
            } else {
                format!("cache {cache_pct}% {dim}(~{} saved){reset}", fmt_tokens(cache_saved))
            }
        } else {
            format!("cache {cache_pct}%")
        }
    };

    println!(
        "{bar_color}{bar}{reset} {dim}Context:{reset} {ctx_display} {dim}|{reset} {dim}Time:{reset} {time_str} {dim}|{reset} {cache_str}"
    );

    // ==========================================
    // LINE 3: Tokens / Cost | (incl. sub) | Quotas
    // ==========================================
    let mut sub_tokens: u64 = 0;
    let mut sub_cost: f64 = 0.0;
    if let Some(subs) = &input.subagents {
        for sub in subs {
            sub_tokens += sub.total_tokens.unwrap_or(0);
            sub_cost += sub.total_usd.unwrap_or(0.0);
        }
    }

    let session_out = ctx.and_then(|c| c.total_output_tokens).unwrap_or(out_tok);
    let total_tokens = ctx_used + session_out + sub_tokens;

    let sub_str = if sub_tokens > 0 || sub_cost > 0.0 {
        format!(" {dim}(incl. sub {}/~${:.2}){reset}", fmt_tokens(sub_tokens), sub_cost)
    } else {
        String::new()
    };

    // Extract real quota / rate limit information (only displayed when present)
    let mut session_pct: Option<f64> = None;
    let mut resets_sec: Option<u64> = None;
    let mut weekly_pct: Option<f64> = None;

    if let Some(q) = &input.quota {
        session_pct = q.session_percentage;
        resets_sec = q.session_resets_in_seconds;
        weekly_pct = q.weekly_percentage;
    }

    if let Some(rl) = &input.rate_limits {
        if let Some(fh) = &rl.five_hour {
            if session_pct.is_none() {
                session_pct = fh.used_percentage;
            }
            if resets_sec.is_none() {
                if let Some(reset_at) = fh.resets_at {
                    let now_f = now as f64;
                    if reset_at > now_f {
                        resets_sec = Some((reset_at - now_f) as u64);
                    }
                }
            }
        }
        if let Some(sd) = &rl.seven_day {
            if weekly_pct.is_none() {
                weekly_pct = sd.used_percentage;
            }
        }
    }

    let mut quota_parts = Vec::new();
    if let Some(sp) = session_pct {
        let s_round = sp.round() as u64;
        let s_color = if s_round >= 80 { red } else if s_round >= 50 { yellow } else { dim };
        let reset_label = if let Some(rsec) = resets_sec {
            let rh = rsec / 3600;
            let rm = (rsec % 3600) / 60;
            if rh > 0 {
                format!(" (resets: {rh}h {rm}m)")
            } else {
                format!(" (resets: {rm}m)")
            }
        } else {
            String::new()
        };
        quota_parts.push(format!("{s_color}Session: {s_round}%{reset_label}{reset}"));
    }

    if let Some(wp) = weekly_pct {
        let w_round = wp.round() as u64;
        let w_color = if w_round >= 80 { red } else if w_round >= 50 { yellow } else { dim };
        quota_parts.push(format!("{w_color}Weekly: {w_round}%{reset}"));
    }

    let quota_str = if !quota_parts.is_empty() {
        format!(" {dim}|{reset} {}", quota_parts.join(&format!(" {dim}·{reset} ")))
    } else {
        String::new()
    };

    println!(
        "{dim}Tokens:{reset} {}/{yellow}${:.2} equiv{reset}{sub_str}{quota_str}",
        fmt_tokens(total_tokens),
        cost_val
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_format() {
        assert_eq!(fmt_tokens(0), "0");
        assert_eq!(fmt_tokens(500), "500");
        assert_eq!(fmt_tokens(201_100), "201.1K");
        assert_eq!(fmt_tokens(1_048_576), "1.0M");
    }

    #[test]
    fn test_empty_input_no_fake_numbers() {
        let empty_input = StatusLineInput::default();
        // Should render without panic, with 0% context, cache: n/a, $0.00, and no fake quotas
        render(&empty_input);
    }

    #[test]
    fn test_render_with_real_metrics() {
        let input = StatusLineInput {
            current_dir: Some("/Users/srafiz/GithubProjects/zenith".to_string()),
            model: Some(ModelInfo {
                display_name: Some("Gemini 3.8 Flash".to_string()),
                ..Default::default()
            }),
            cost: Some(CostInfo {
                total_cost_usd: Some(0.42),
                total_duration_ms: Some(125_000), // 2m 5s
                ..Default::default()
            }),
            vcs: Some(VcsInfo {
                branch: Some("main".to_string()),
                dirty: Some(false),
            }),
            context_window: Some(ContextWindowInfo {
                used_percentage: Some(15.0),
                context_window_size: Some(1_048_576),
                total_input_tokens: Some(157_286),
                current_usage: Some(CurrentUsageInfo {
                    input_tokens: Some(10_000),
                    cache_read_input_tokens: Some(140_000),
                    cache_creation_input_tokens: Some(7_286),
                    output_tokens: Some(2_500),
                }),
                total_output_tokens: Some(2_500),
            }),
            quota: Some(QuotaInfo {
                session_percentage: Some(30.0),
                session_resets_in_seconds: Some(7200),
                weekly_percentage: Some(12.0),
            }),
            subagents: Some(vec![SubagentInfo {
                name: Some("researcher".to_string()),
                display_name: Some("Researcher".to_string()),
                model: None,
                total_usd: Some(0.08),
                total_tokens: Some(45_000),
            }]),
            ..Default::default()
        };
        render(&input);
    }
}
