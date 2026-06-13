---
name: ctxt-antigravity-runtime
description: Safe Antigravity-facing workflow for using the CompText ctxt CLI as the common source of truth, with JSON schema, capabilities, artifact reads, proposal-only agent plans, and strict no-network/no-apply/no-direct-execution boundaries.
---

# Purpose

This skill is for Antigravity sessions working in this repository.

It is an adapter to `ctxt`, not a separate runtime, policy, or execution system. Antigravity must not directly execute task workflows. It must route through the same safe `ctxt` JSON commands used by Codex.

# Hard Safety Rules

- Do not use network.
- Do not apply proposals.
- Do not use full-access mode.
- Do not push.
- Do not pull.
- Do not invoke external agents.
- Do not execute real Codex workflows.
- Do not execute real Antigravity workflows.
- Do not bypass `ctxt` with direct Antigravity CLI task execution.
- User validates externally in PowerShell on this Windows machine.

# Start Here

Use these commands first to inspect the shared contract and runtime surface:

```powershell
cargo run --bin ctxt -- --json self report
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json runs list
```

# Artifact-First Inspection

Use bounded run artifact reads before making claims about the latest agent state:

```powershell
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
```

Do not infer real execution if artifacts say `external_agent_invoked=false`. Do not infer network use if artifacts say `network_used=false` or `network_allowed=false`.

# Plan Artifacts Only

For Antigravity plan artifacts only, use:

```powershell
cargo run --bin ctxt -- --json agent run --kind antigravity --task "..." --allow-external --proposal-only
```

This creates proposal-only planning artifacts through `ctxt`. It must not invoke Antigravity as an external agent, use network, or apply changes.

# Validation

For validation, ask the user to run externally in PowerShell:

```powershell
cargo run --bin ctxt -- --json validate --run
```

Codex Desktop command execution may fail on this machine with:

```text
CreateProcessAsUserW failed: 5
```

Treat the user's external PowerShell validation as the source of truth.
