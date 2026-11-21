use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct CallEntry {
    tool: String,
    hash: String,
    timestamp: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct State {
    calls: Vec<CallEntry>,
}

fn get_state_file(conv_id: &str) -> PathBuf {
    let sanitized: String = conv_id.chars().filter(|c| c.is_alphanumeric() || *c == '-').collect();
    std::env::temp_dir().join(format!("agy_sc_loop_{}.json", sanitized))
}

pub fn track_tool_call(conv_id: &str, tool: &str, signature: &str) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let state_file = get_state_file(conv_id);
    let mut state: State = if state_file.exists() {
        fs::read_to_string(&state_file)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    } else {
        State::default()
    };

    // Filter out entries older than 30 seconds
    state.calls.retain(|c| now.saturating_sub(c.timestamp) < 30);

    // Compute simple signature
    let sig_hash = format!("{:x}", md5_hash(signature));

    // Count identical calls in the last 30 seconds
    let repeat_count = state.calls.iter().filter(|c| c.tool == tool && c.hash == sig_hash).count();

    state.calls.push(CallEntry {
        tool: tool.to_string(),
        hash: sig_hash,
        timestamp: now,
    });

    let _ = fs::write(&state_file, serde_json::to_string(&state).unwrap_or_default());

    repeat_count >= 2 // Current call is the 3rd repetition
}

fn md5_hash(s: &str) -> u64 {
    // Fast 64-bit FNV-1a hash
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_detection() {
        let conv = "test-conv-123";
        let state_file = get_state_file(conv);
        let _ = fs::remove_file(&state_file);

        assert!(!track_tool_call(conv, "run_command", "git status"));
        assert!(!track_tool_call(conv, "run_command", "git status"));
        assert!(track_tool_call(conv, "run_command", "git status"));

        let _ = fs::remove_file(&state_file);
    }
}
