use std::fs::{self, OpenOptions};
use std::io::Write;
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
