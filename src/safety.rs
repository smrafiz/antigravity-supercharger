use regex::Regex;
use std::sync::OnceLock;

pub struct DangerVerdict {
    pub is_fatal: bool, // true = deny, false = ask
    pub reason: String,
}

static DESTRUCTIVE_REGEX: OnceLock<Vec<(Regex, &'static str, bool)>> = OnceLock::new();

fn get_destructive_patterns() -> &'static Vec<(Regex, &'static str, bool)> {
    DESTRUCTIVE_REGEX.get_or_init(|| {
        vec![
            // Critical destructive system commands (deny)
            (
                Regex::new(r"(?i)\brm\s+-(?:[a-z]*r[a-z]*f|[a-z]*f[a-z]*r)\s+(?:/\s*$|/\*|~\s*$|~\*|\.\s*$)").unwrap(),
                "Destructive command blocked: recursive force-delete on root/home/current directory",
                true,
            ),
            (
                Regex::new(r"(?i)\bmkfs(?:\.[a-z0-9]+)?\b").unwrap(),
                "Destructive filesystem format command blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\bdd\s+if=.*of=(?:/dev/sd[a-z]|/dev/nvme|/dev/disk[0-9]|/dev/zero|/dev/null)").unwrap(),
                "Disk overwrite via dd blocked",
                true,
            ),
            (
                Regex::new(r":\(\)\s*\{\s*:\s*\|\s*:\s*&\s*\}\s*;\s*:").unwrap(),
                "Fork bomb blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\b(?:shutdown|reboot|poweroff|init\s+0)\b").unwrap(),
                "System shutdown/reboot command blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\bchmod\s+(?:-R\s+)?777\s+(?:/|~|\.\.)").unwrap(),
                "Insecure permissive chmod 777 on sensitive directory blocked",
                true,
            ),
            // Unsafe pipe execution (deny)
            (
                Regex::new(r"(?i)(?:curl|wget)\s+[^|]+\|\s*(?:bash|sh|zsh)").unwrap(),
                "Direct curl/wget-to-shell execution blocked",
                true,
            ),
            // SQL destruction (ask/deny)
            (
                Regex::new(r"(?i)\bDROP\s+(?:DATABASE|SCHEMA)\b").unwrap(),
                "Destructive SQL: DROP DATABASE/SCHEMA blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\bDROP\s+TABLE\b").unwrap(),
                "Destructive SQL: DROP TABLE requires explicit user confirmation",
                false,
            ),
            (
                Regex::new(r"(?i)\bTRUNCATE\s+TABLE\b").unwrap(),
                "Destructive SQL: TRUNCATE TABLE requires explicit user confirmation",
                false,
            ),
            // Sensitive credential paths
            (
                Regex::new(r"(?i)\b(?:cat|less|more|head|tail|grep)\s+.*~/(?:\.ssh|\.aws|\.gnupg)/").unwrap(),
                "Accessing sensitive user credentials (~/.ssh, ~/.aws, ~/.gnupg) blocked",
                true,
            ),
        ]
    })
}

static GIT_REGEX: OnceLock<Vec<(Regex, &'static str, bool)>> = OnceLock::new();

fn get_git_patterns() -> &'static Vec<(Regex, &'static str, bool)> {
    GIT_REGEX.get_or_init(|| {
        vec![
            (
                Regex::new(r"(?i)\bgit\s+push\b.*(?:\b(?:main|master|prod|production)\b.*(?:--force|-f|\+refs)|\b(?:--force|-f|\+refs)\b.*(?:\b(?:main|master|prod|production)\b))").unwrap(),
                "Git safety: Force-pushing to protected branch (main/master) is strictly blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\bgit\s+push\s+.*--force\b").unwrap(),
                "Git safety: Force-pushing requires explicit confirmation",
                false,
            ),
            (
                Regex::new(r"(?i)\bgit\s+reset\s+--hard\b").unwrap(),
                "Git safety: 'git reset --hard' wipes uncommitted changes; requires confirmation",
                false,
            ),
            (
                Regex::new(r"(?i)\bgit\s+clean\s+-[a-z]*f\b").unwrap(),
                "Git safety: 'git clean -f' removes untracked files; requires confirmation",
                false,
            ),
            (
                Regex::new(r"(?i)\bgit\s+branch\s+-[dD]\s+(?:main|master|production|release)\b").unwrap(),
                "Git safety: Deleting protected branch blocked",
                true,
            ),
            (
                Regex::new(r"(?i)\bgit\s+stash\s+drop\b").unwrap(),
                "Git safety: 'git stash drop' deletes stashed work; requires confirmation",
                false,
            ),
        ]
    })
}

pub fn check_command(cmd: &str) -> Option<DangerVerdict> {
    for (re, reason, is_fatal) in get_destructive_patterns() {
        if re.is_match(cmd) {
            return Some(DangerVerdict {
                is_fatal: *is_fatal,
                reason: reason.to_string(),
            });
        }
    }

    for (re, reason, is_fatal) in get_git_patterns() {
        if re.is_match(cmd) {
            return Some(DangerVerdict {
                is_fatal: *is_fatal,
                reason: reason.to_string(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rm_rf_blocked() {
        let v = check_command("rm -rf /").unwrap();
        assert!(v.is_fatal);
        assert!(v.reason.contains("Destructive command"));
    }

    #[test]
    fn test_git_force_push_blocked() {
        let v = check_command("git push origin main --force").unwrap();
        assert!(v.is_fatal);
        assert!(v.reason.contains("Force-pushing to protected branch"));
    }

    #[test]
    fn test_git_reset_hard_asks() {
        let v = check_command("git reset --hard HEAD~1").unwrap();
        assert!(!v.is_fatal);
        assert!(v.reason.contains("reset --hard"));
    }

    #[test]
    fn test_safe_command_allowed() {
        assert!(check_command("cargo test").is_none());
        assert!(check_command("git status").is_none());
        assert!(check_command("ls -la").is_none());
    }
}
