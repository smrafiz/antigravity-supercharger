use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct StatusLineInput {
    pub session_id: Option<String>,
    pub conversation_id: Option<String>,
    pub current_dir: Option<String>,
    pub project_dir: Option<String>,
    pub cwd: Option<String>,
    pub plan_tier: Option<String>,
    pub transcript_path: Option<String>,
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
pub struct AntigravityQuotaBucket {
    pub remaining_fraction: Option<f64>,
    pub reset_in_seconds: Option<u64>,
    pub reset_time: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
pub struct QuotaInfo {
    // Claude direct format
    pub session_percentage: Option<f64>,
    pub session_resets_in_seconds: Option<u64>,
    pub weekly_percentage: Option<f64>,

    // Antigravity buckets
    #[serde(rename = "gemini-5h")]
    pub gemini_5h: Option<AntigravityQuotaBucket>,
    #[serde(rename = "gemini-weekly")]
    pub gemini_weekly: Option<AntigravityQuotaBucket>,
    #[serde(rename = "3p-5h")]
    pub three_p_5h: Option<AntigravityQuotaBucket>,
    #[serde(rename = "3p-weekly")]
    pub three_p_weekly: Option<AntigravityQuotaBucket>,
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

pub fn get_session_duration_secs(
    cwd: &str,
    session_id: Option<&str>,
    cost_dur_ms: Option<u64>,
) -> u64 {
    if let Some(ms) = cost_dur_ms {
        if ms > 0 {
            return ms / 1000;
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let sid = session_id.unwrap_or("default");
    let start_file = std::env::temp_dir().join(format!(".supercharger-start-{}", sid));

    if let Ok(content) = fs::read_to_string(&start_file) {
        if let Ok(st) = content.trim().parse::<u64>() {
            if now >= st {
                return now - st;
            }
        }
    }

    // Try finding earliest timestamp from .supercharger/audit.jsonl
    let audit_file = Path::new(cwd).join(".supercharger").join("audit.jsonl");
    if let Ok(content) = fs::read_to_string(audit_file) {
        if let Some(first_line) = content.lines().next() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(first_line) {
                if let Some(ts) = v.get("timestamp").and_then(|t| t.as_u64()) {
                    if now >= ts {
                        let _ = fs::write(&start_file, ts.to_string());
                        return now - ts;
                    }
                }
            }
        }
    }

    let _ = fs::write(&start_file, now.to_string());
    0
}

pub fn calculate_equiv_cost(
    model_id: &str,
    total_in: u64,
    cache_read: u64,
    total_out: u64,
) -> f64 {
    let id_lower = model_id.to_lowercase();
    let (in_rate, cache_rate, out_rate) = if id_lower.contains("opus") {
        (15.0, 1.5, 75.0)
    } else if id_lower.contains("sonnet") || id_lower.contains("3p") {
        (3.0, 0.30, 15.0)
    } else if id_lower.contains("gpt-4") {
        (2.5, 1.25, 10.0)
    } else if id_lower.contains("pro") && !id_lower.contains("flash") {
        (1.25, 0.3125, 5.0)
    } else {
        // Default to Flash rates ($0.075 input, $0.01875 cache, $0.30 output per 1M)
        (0.075, 0.01875, 0.30)
    };

    let uncached_in = total_in.saturating_sub(cache_read);
    let cost = (uncached_in as f64 * in_rate / 1_000_000.0)
        + (cache_read as f64 * cache_rate / 1_000_000.0)
        + (total_out as f64 * out_rate / 1_000_000.0);

    cost
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
        .cwd
        .as_deref()
        .or(input.project_dir.as_deref())
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

    let session_id = input.session_id.as_deref().or(input.conversation_id.as_deref());
    let dur_ms = input.cost.as_ref().and_then(|c| c.total_duration_ms);
    let dur_secs = get_session_duration_secs(cwd, session_id, dur_ms);
    let dur_mins = dur_secs / 60;
    let time_str = if dur_mins >= 60 {
        format!("{}h {}m", dur_mins / 60, dur_mins % 60)
    } else {
        format!("{}m {}s", dur_mins, dur_secs % 60)
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

    // Determine cost: use explicit total_cost_usd if provided, else compute API-equivalent
    let cost_val = if let Some(usd) = input.cost.as_ref().and_then(|c| c.total_cost_usd) {
        if usd > 0.0 {
            usd
        } else {
            calculate_equiv_cost(model_name, ctx_used, cache_read, session_out)
        }
    } else {
        calculate_equiv_cost(model_name, ctx_used, cache_read, session_out)
    };

    let sub_str = if sub_tokens > 0 || sub_cost > 0.0 {
        format!(" {dim}(incl. sub {}/~${:.2}){reset}", fmt_tokens(sub_tokens), sub_cost)
    } else {
        String::new()
    };

    // Extract real quota / rate limit information
    let mut session_pct: Option<f64> = None;
    let mut resets_sec: Option<u64> = None;
    let mut weekly_pct: Option<f64> = None;

    if let Some(q) = &input.quota {
        if q.session_percentage.is_some() || q.weekly_percentage.is_some() {
            session_pct = q.session_percentage;
            resets_sec = q.session_resets_in_seconds;
            weekly_pct = q.weekly_percentage;
        } else {
            // Antigravity bucket format
            let id_lower = model_name.to_lowercase();
            let is_3p = id_lower.contains("claude")
                || id_lower.contains("sonnet")
                || id_lower.contains("opus")
                || id_lower.contains("gpt");

            let (h5_bucket, wk_bucket) = if is_3p {
                (&q.three_p_5h, &q.three_p_weekly)
            } else {
                (&q.gemini_5h, &q.gemini_weekly)
            };

            if let Some(b) = h5_bucket {
                if let Some(rf) = b.remaining_fraction {
                    session_pct = Some(((1.0 - rf) * 100.0).max(0.0));
                }
                resets_sec = b.reset_in_seconds;
            }

            if let Some(b) = wk_bucket {
                if let Some(rf) = b.remaining_fraction {
                    weekly_pct = Some(((1.0 - rf) * 100.0).max(0.0));
                }
            }
        }
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
    fn test_antigravity_quota_format() {
        let raw = r#"{
            "model": {"display_name": "Gemini 3.8 Flash (High)"},
            "context_window": {
                "total_input_tokens": 176910,
                "total_output_tokens": 155602,
                "context_window_size": 1048576,
                "current_usage": {
                    "input_tokens": 2677,
                    "cache_read_input_tokens": 175968
                }
            },
            "quota": {
                "gemini-5h": {
                    "remaining_fraction": 0.6777696,
                    "reset_in_seconds": 12676
                },
                "gemini-weekly": {
                    "remaining_fraction": 0.9079342,
                    "reset_in_seconds": 599476
                }
            }
        }"#;

        let input: StatusLineInput = serde_json::from_str(raw).unwrap();
        render(&input);
    }
}
