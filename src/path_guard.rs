use std::path::Path;

pub struct PathVerdict {
    pub is_fatal: bool,
    pub reason: String,
}

pub fn check_path(file_path: &str, workspace_paths: &[String]) -> Option<PathVerdict> {
    let normalized = file_path.replace('\\', "/");

    // 1. Path traversal attacks
    if normalized.contains("../") || normalized.contains("/..") || normalized.contains("%2e%2e") {
        return Some(PathVerdict {
            is_fatal: true,
            reason: format!("Path traversal blocked: target '{}' attempts directory escape", file_path),
        });
    }

    // 2. Sensitive system directories
    let forbidden_prefixes = [
        "/etc/", "/var/", "/usr/", "/bin/", "/sbin/", "/System/", "/Library/",
        "~/.ssh/", "~/.aws/", "~/.gnupg/", "~/.bashrc", "~/.zshrc"
    ];
    for prefix in forbidden_prefixes {
        if normalized.starts_with(prefix) {
            return Some(PathVerdict {
                is_fatal: true,
                reason: format!("Modifying system or credential path '{}' is blocked", file_path),
            });
        }
    }

    // 3. Git internals and hook tampering
    if normalized.contains(".git/hooks/") || normalized.contains(".git/config") {
        return Some(PathVerdict {
            is_fatal: true,
            reason: format!("Tampering with git hooks or internal config '{}' is blocked", file_path),
        });
    }

    // 3b. Python .pth backdoor persistence
    if normalized.ends_with(".pth") && !normalized.contains("/tests/") && !normalized.contains("/test/") {
        return Some(PathVerdict {
            is_fatal: false,
            reason: format!("Writing Python .pth file '{}' executes code on startup; confirmation required", file_path),
        });
    }

    // 4. Verify boundary: if absolute path, must be inside workspace or allowed system/artifact directories
    if Path::new(file_path).is_absolute() {
        // System temp and agent scratch/artifact directories are permitted
        if normalized.starts_with("/tmp/")
            || normalized.starts_with("/private/tmp/")
            || normalized.contains(".gemini/antigravity-cli/brain")
            || normalized.contains(".gemini/brain")
        {
            return None;
        }

        if !workspace_paths.is_empty() {
            let is_inside = workspace_paths.iter().any(|ws| {
                let ws_norm = ws.replace('\\', "/");
                normalized.starts_with(&ws_norm)
            });
            if !is_inside {
                return Some(PathVerdict {
                    is_fatal: true,
                    reason: format!(
                        "Path guard: '{}' is outside active workspace boundaries",
                        file_path
                    ),
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_blocked() {
        let v = check_path("../../etc/passwd", &[]).unwrap();
        assert!(v.is_fatal);
        assert!(v.reason.contains("Path traversal"));
    }

    #[test]
    fn test_git_hook_blocked() {
        let v = check_path(".git/hooks/pre-commit", &[]).unwrap();
        assert!(v.is_fatal);
        assert!(v.reason.contains("git hooks"));
    }

    #[test]
    fn test_workspace_boundary() {
        let ws = vec!["/Users/srafiz/GithubProjects/app".to_string()];
        assert!(check_path("/Users/srafiz/GithubProjects/app/src/main.rs", &ws).is_none());
        assert!(check_path("/Users/srafiz/.ssh/id_rsa", &ws).is_some());
    }

    #[test]
    fn test_pth_file_detection() {
        let ws = vec!["/Users/srafiz/GithubProjects/app".to_string()];
        let v = check_path("/Users/srafiz/GithubProjects/app/backdoor.pth", &ws).unwrap();
        assert!(!v.is_fatal);
        assert!(v.reason.contains(".pth"));
    }
}
