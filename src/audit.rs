use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_event(cwd: Option<&str>, tool: &str, args_preview: &str, blocked: bool, reason: Option<&str>) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let audit_dir = if let Some(dir) = cwd {
        Path::new(dir).join(".supercharger")
    } else {
        std::env::temp_dir().join(".supercharger")
    };

    let _ = fs::create_dir_all(&audit_dir);
    let log_file = audit_dir.join("audit.jsonl");

    let event = serde_json::json!({
        "timestamp": now,
        "tool": tool,
        "args": crate::code_scanner::redact_secrets(args_preview),
        "blocked": blocked,
        "reason": reason,
    });

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_file) {
        let _ = writeln!(file, "{}", event);
    }
}

pub fn get_recent_events(cwd: Option<&str>, limit: usize) -> Vec<serde_json::Value> {
    let audit_dir = if let Some(dir) = cwd {
        Path::new(dir).join(".supercharger")
    } else {
        std::env::temp_dir().join(".supercharger")
    };
    let log_file = audit_dir.join("audit.jsonl");

    let file = match File::open(log_file) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let start = if lines.len() > limit { lines.len() - limit } else { 0 };

    lines[start..]
        .iter()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .collect()
}
