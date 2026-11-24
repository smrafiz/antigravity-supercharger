---
name: ste
description: Self-Testing Engine protocol for rigorous pre-flight test suites and regression verification.
---

# Supercharger Self-Testing Engine (STE)

Verify changes through automated, repeatable test execution before declaring any task complete.

## STE Rules

1. **Never Claim Done Without Execution**:
   - Running tests in thought or assuming "it should work" is strictly forbidden.
   - Execute the actual test suite and observe the exit code and output.

2. **Negative Testing**:
   - Verify that invalid inputs, missing keys, or unauthorized paths trigger expected errors.
   - Confirm safety guards actively block dangerous operations.

3. **Regression Proof**:
   - Run existing unit and integration suites to ensure zero regressions.
   - Verify formatting and linting pass with zero warnings (`cargo clippy -- -D warnings`, `npm run lint`).
