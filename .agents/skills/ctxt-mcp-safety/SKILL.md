---
name: ctxt-mcp-safety
description: Build safe MCP-facing CompText tools with allowed roots, path traversal blocking, max file size limits, and read-only behavior.
---

Use this skill when implementing MCP server or file analysis behavior.

Hard requirements:
- require allowed_roots
- block path traversal
- block arbitrary absolute reads
- enforce max file size
- read-only analyzer only
- no destructive tool side effects
