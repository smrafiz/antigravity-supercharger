---
name: security
description: Deep security audit of the workspace or diff against OWASP Top 10, secrets leaks, injection sinks, and permission escalations.
---

# Supercharger Security Audit

Perform a structured security review of changes or workspace files to identify exploitable vulnerabilities, verified by refutation.

## Audit Scope

- **F1: Injection & Code Execution**: SQL injection, OS command injection, template injection, unsafe deserialization (`pickle.load`).
- **F2: Client-side Security**: XSS (`dangerouslySetInnerHTML`), CSRF, unsafe origin messaging.
- **F3: Auth & Permissions**: Broken authorization, missing tenant isolation, hardcoded secrets, unprotected routes.
- **F4: Files & Cloud Infra**: Path traversal (`../`), destructive cloud commands (`terraform destroy`, `aws s3 rb`), unencrypted sensitive stores.
- **F5: Supply Chain & Actions**: Insecure GitHub Actions workflows, untracked credentials, dangerous script executions.

## Procedure

1. Inspect modified lines and trace taint paths from user inputs to execution sinks.
2. Verify existing sanitizers and middleware before reporting.
3. Classify findings strictly by severity (Critical, High, Medium, Low) with concrete file links and line ranges.
