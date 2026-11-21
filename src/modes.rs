use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutopilotStatus {
    pub is_full: bool,
    pub remaining_seconds: u64,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn get_scope_dirs(cwd: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    // 1. Current workspace .supercharger
    dirs.push(Path::new(cwd).join(".supercharger"));
    // 2. Global gemini plugin scope
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(Path::new(&home).join(".gemini/config/plugins/supercharger/scope"));
        dirs.push(Path::new(&home).join(".claude/supercharger/scope"));
    }
    dirs
}

/// Parse duration string like "30m", "2h", "90s", "45" (default minutes)
pub fn parse_duration(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(stripped) = s.strip_suffix('h') {
        stripped.parse::<u64>().ok().map(|n| n * 3600)
    } else if let Some(stripped) = s.strip_suffix('m') {
        stripped.parse::<u64>().ok().map(|n| n * 60)
    } else if let Some(stripped) = s.strip_suffix('s') {
        stripped.parse::<u64>().ok()
    } else {
        s.parse::<u64>().ok().map(|n| n * 60)
    }
}

pub fn fmt_duration(seconds: u64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    if h > 0 {
        format!("{}h {}m", h, m)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}

// -----------------------------------------------------------------------------
// READONLY MODE
// -----------------------------------------------------------------------------

pub fn is_readonly_active(cwd: &str, session_id: Option<&str>) -> Option<u64> {
    let now = now_secs();
    let dirs = get_scope_dirs(cwd);

    let mut best_remaining = None;
    for dir in dirs {
        let mut candidates = vec![dir.join(".readonly-until")];
        if let Some(sid) = session_id {
            candidates.push(dir.join(format!(".readonly-until-{}", sid)));
        }
        for path in candidates {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(until) = content.trim().parse::<u64>() {
                    if until > now {
                        let rem = until - now;
                        best_remaining = Some(best_remaining.map_or(rem, |b: u64| b.max(rem)));
                    }
                }
            }
        }
    }
    best_remaining
}

pub fn set_readonly(cwd: &str, session_id: Option<&str>, duration_secs: u64, is_global: bool) -> Result<u64, String> {
    let now = now_secs();
    let until = now + duration_secs;
    let dirs = get_scope_dirs(cwd);
    let target_dir = if is_global {
        dirs.get(1).unwrap_or(&dirs[0])
    } else {
        &dirs[0]
    };
    let _ = fs::create_dir_all(target_dir);

    let filename = if is_global || session_id.is_none() {
        ".readonly-until".to_string()
    } else {
        format!(".readonly-until-{}", session_id.unwrap())
    };
    let path = target_dir.join(filename);
    fs::write(&path, until.to_string()).map_err(|e| format!("Failed to write readonly flag: {}", e))?;
    Ok(duration_secs)
}

pub fn clear_readonly(cwd: &str, session_id: Option<&str>) {
    let dirs = get_scope_dirs(cwd);
    for dir in dirs {
        let _ = fs::remove_file(dir.join(".readonly-until"));
        if let Some(sid) = session_id {
            let _ = fs::remove_file(dir.join(format!(".readonly-until-{}", sid)));
        }
    }
}

// -----------------------------------------------------------------------------
// STRICT MODE
// -----------------------------------------------------------------------------

pub fn is_strict_active(cwd: &str, session_id: Option<&str>) -> Option<u64> {
    let now = now_secs();
    let dirs = get_scope_dirs(cwd);

    let mut best_remaining = None;
    for dir in dirs {
        let mut candidates = vec![dir.join(".strict-until")];
        if let Some(sid) = session_id {
            candidates.push(dir.join(format!(".strict-until-{}", sid)));
        }
        for path in candidates {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(until) = content.trim().parse::<u64>() {
                    if until > now {
                        let rem = until - now;
                        best_remaining = Some(best_remaining.map_or(rem, |b: u64| b.max(rem)));
                    }
                }
            }
        }
    }
    best_remaining
}

pub fn set_strict(cwd: &str, session_id: Option<&str>, duration_secs: u64, is_global: bool) -> Result<u64, String> {
    let now = now_secs();
    let until = now + duration_secs;
    let dirs = get_scope_dirs(cwd);
    let target_dir = if is_global {
        dirs.get(1).unwrap_or(&dirs[0])
    } else {
        &dirs[0]
    };
    let _ = fs::create_dir_all(target_dir);

    let filename = if is_global || session_id.is_none() {
        ".strict-until".to_string()
    } else {
        format!(".strict-until-{}", session_id.unwrap())
    };
    let path = target_dir.join(filename);
    fs::write(&path, until.to_string()).map_err(|e| format!("Failed to write strict flag: {}", e))?;
    Ok(duration_secs)
}

pub fn clear_strict(cwd: &str, session_id: Option<&str>) {
    let dirs = get_scope_dirs(cwd);
    for dir in dirs {
        let _ = fs::remove_file(dir.join(".strict-until"));
        if let Some(sid) = session_id {
            let _ = fs::remove_file(dir.join(format!(".strict-until-{}", sid)));
        }
    }
}

// -----------------------------------------------------------------------------
// AUTOPILOT MODE
// -----------------------------------------------------------------------------

pub fn is_autopilot_active(cwd: &str, session_id: Option<&str>) -> Option<AutopilotStatus> {
    // Strict mode SUPPRESSES autopilot
    if is_strict_active(cwd, session_id).is_some() {
        return None;
    }

    let now = now_secs();
    let dirs = get_scope_dirs(cwd);

    let mut best_remaining = None;
    let mut is_full = false;

    for dir in dirs {
        let mut candidates = vec![
            (dir.join(".autopilot-until"), dir.join(".autopilot-full")),
        ];
        if let Some(sid) = session_id {
            candidates.push((
                dir.join(format!(".autopilot-until-{}", sid)),
                dir.join(format!(".autopilot-full-{}", sid)),
            ));
        }

        for (ap_path, full_path) in candidates {
            if let Ok(content) = fs::read_to_string(ap_path) {
                if let Ok(until) = content.trim().parse::<u64>() {
                    if until > now {
                        let rem = until - now;
                        best_remaining = Some(best_remaining.map_or(rem, |b: u64| b.max(rem)));
                        if full_path.exists() {
                            is_full = true;
                        }
                    }
                }
            }
        }
    }

    best_remaining.map(|remaining_seconds| AutopilotStatus {
        is_full,
        remaining_seconds,
    })
}

pub fn set_autopilot(
    cwd: &str,
    session_id: Option<&str>,
    duration_secs: u64,
    full: bool,
    is_global: bool,
) -> Result<u64, String> {
    let now = now_secs();
    let until = now + duration_secs;
    let dirs = get_scope_dirs(cwd);
    let target_dir = if is_global {
        dirs.get(1).unwrap_or(&dirs[0])
    } else {
        &dirs[0]
    };
    let _ = fs::create_dir_all(target_dir);

    let ap_filename = if is_global || session_id.is_none() {
        ".autopilot-until".to_string()
    } else {
        format!(".autopilot-until-{}", session_id.unwrap())
    };
    let full_filename = if is_global || session_id.is_none() {
        ".autopilot-full".to_string()
    } else {
        format!(".autopilot-full-{}", session_id.unwrap())
    };

    fs::write(target_dir.join(ap_filename), until.to_string())
        .map_err(|e| format!("Failed to write autopilot flag: {}", e))?;

    if full {
        let _ = fs::write(target_dir.join(full_filename), until.to_string());
    } else {
        let _ = fs::remove_file(target_dir.join(full_filename));
    }

    Ok(duration_secs)
}

pub fn clear_autopilot(cwd: &str, session_id: Option<&str>) {
    let dirs = get_scope_dirs(cwd);
    for dir in dirs {
        let _ = fs::remove_file(dir.join(".autopilot-until"));
        let _ = fs::remove_file(dir.join(".autopilot-full"));
        if let Some(sid) = session_id {
            let _ = fs::remove_file(dir.join(format!(".autopilot-until-{}", sid)));
            let _ = fs::remove_file(dir.join(format!(".autopilot-full-{}", sid)));
        }
    }
}

// -----------------------------------------------------------------------------
// MUTATING COMMAND DETECTION (FOR READ-ONLY MODE)
// -----------------------------------------------------------------------------

pub fn is_mutating_command(cmd: &str) -> bool {
    let trimmed = cmd.trim();

    // 1. Output redirection to a file (excluding /dev/null, /dev/stdout, /dev/stderr, /dev/tty)
    if let Some(idx) = trimmed.find('>') {
        let after = &trimmed[idx + 1..].trim_start_matches(['>', '|', ' ']);
        if !after.starts_with("/dev/null")
            && !after.starts_with("/dev/stdout")
            && !after.starts_with("/dev/stderr")
            && !after.starts_with("/dev/tty")
            && !after.is_empty()
        {
            return true;
        }
    }

    // 2. Common mutating shell tools
    let mutating_tools = [
        "rm ", "rmdir ", "mv ", "cp ", "mkdir ", "touch ", "chmod ", "chown ",
        "truncate ", "dd ", "install ", "sed -i", "perl -i",
    ];
    for tool in mutating_tools {
        if trimmed.starts_with(tool) || trimmed.contains(&format!("; {}", tool)) || trimmed.contains(&format!("&& {}", tool)) {
            return true;
        }
    }

    // 3. Git mutating subcommands (allows status, log, diff, show, branch --show-current, remote)
    let git_mutators = [
        "commit", "push", "add", "mv", "rm", "merge", "rebase", "reset",
        "checkout -b", "restore", "stash", "clean", "apply", "cherry-pick", "revert",
    ];
    if trimmed.contains("git ") {
        for gm in git_mutators {
            if trimmed.contains(&format!("git {}", gm)) || trimmed.contains(&format!("git  {}", gm)) {
                return true;
            }
        }
    }

    // 4. Package manager mutating operations
    let pkg_mutators = [
        "npm install", "npm i ", "npm add", "npm uninstall", "npm update",
        "yarn add", "yarn install", "yarn remove",
        "pnpm add", "pnpm install", "pnpm remove",
        "pip install", "pip uninstall",
        "cargo add", "cargo install",
        "brew install", "brew uninstall",
    ];
    for pm in pkg_mutators {
        if trimmed.contains(pm) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_lifecycle() {
        let temp_dir = std::env::temp_dir().join("sc_test_mode");
        let _ = fs::create_dir_all(&temp_dir);
        let cwd = temp_dir.to_str().unwrap();

        // 1. Readonly lifecycle
        clear_readonly(cwd, None);
        assert_eq!(is_readonly_active(cwd, None), None);
        set_readonly(cwd, None, 1800, false).unwrap();
        assert!(is_readonly_active(cwd, None).unwrap() > 0);
        clear_readonly(cwd, None);
        assert_eq!(is_readonly_active(cwd, None), None);

        // 2. Autopilot & Strict override
        clear_autopilot(cwd, None);
        clear_strict(cwd, None);
        assert_eq!(is_autopilot_active(cwd, None), None);

        set_autopilot(cwd, None, 1800, true, false).unwrap();
        let ap = is_autopilot_active(cwd, None).unwrap();
        assert!(ap.is_full);
        assert!(ap.remaining_seconds > 0);

        // Strict suppresses autopilot
        set_strict(cwd, None, 1800, false).unwrap();
        assert_eq!(is_autopilot_active(cwd, None), None);

        // Clear strict restores autopilot
        clear_strict(cwd, None);
        assert!(is_autopilot_active(cwd, None).is_some());

        clear_autopilot(cwd, None);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_mutating_command_detection() {
        assert!(is_mutating_command("rm -rf target/"));
        assert!(is_mutating_command("git commit -m 'feat'"));
        assert!(is_mutating_command("npm install express"));
        assert!(is_mutating_command("echo hello > out.txt"));
        assert!(!is_mutating_command("echo hello > /dev/null"));
        assert!(!is_mutating_command("git status"));
        assert!(!is_mutating_command("git diff HEAD"));
        assert!(!is_mutating_command("cargo check"));
        assert!(!is_mutating_command("ls -la"));
    }
}
