---
name: trust-mcp
description: Manage trusted MCP servers and inspect permission allowances.
---

# Supercharger MCP Trust Manager

Configure and review trusted Model Context Protocol (MCP) servers and verify credential-access authorizations.

## Trust Commands

- **List Active Servers**: Inspect configured stdio and HTTP/SSE MCP servers in settings.
- **Audit Descriptions**: Scan tool descriptions for unauthorized credential requests.
- **Verify Signatures**: Ensure only explicitly recognized servers are allowed to prompt for sensitive tokens or environment secrets.

## Security Rule

Only trust MCP servers you control or verified third-party tools. Trusted servers bypass credential elicitation warnings.
