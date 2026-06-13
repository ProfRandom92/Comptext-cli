---
name: ctxt-runtime
description: Safe workflow for working on the CompText ctxt CLI runtime, including validation, artifacts, agent discovery, proposal-only execution plans, and strict no-push/no-network boundaries.
---

# Purpose

`ctxt` is a deterministic safety wrapper around local agent workflows.

It is not a replacement for Codex CLI or Antigravity CLI. It prepares context, records artifacts, supports discovery, and models proposal-only execution plans before any future external agent execution is considered.

# When To Use This Skill

Use this skill when:

- working inside this repository
- editing or reviewing the `ctxt` CLI
- continuing Phase 1-6 runtime work
- safely using Codex or Antigravity through `ctxt`
- inspecting artifacts
- validating runtime state
- preparing agent proposals

# Hard Safety Rules

- Do not push.
- Do not pull.
- Do not commit unless explicitly requested by the user.
- Do not use network.
- Do not invoke external agents.
- Do not invoke Codex CLI directly for tasks.
- Do not invoke Antigravity CLI directly for tasks.
- Do not apply proposals automatically.
- Prefer proposal-only and artifact-based workflows.
- Use JSON commands when available.
- Treat external PowerShell validation as the source of truth on this Windows machine.

# Known Windows Limitation

Codex Desktop command execution may fail with:

```text
CreateProcessAsUserW failed: 5
```

Codex Desktop should usually edit and review only. The user validates externally in PowerShell.

# First Commands To Suggest

Suggest these exact PowerShell commands for validation:

```powershell
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run --bin ctxt -- --json validate --run
```

# Safe ctxt Commands

Use these commands for safe, JSON-oriented runtime inspection:

```powershell
cargo run --bin ctxt -- --json agent list
cargo run --bin ctxt -- --json agent discover
cargo run --bin ctxt -- --json agent discover --kind codex
cargo run --bin ctxt -- --json agent discover --kind antigravity
cargo run --bin ctxt -- --json agent run --kind codex --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json agent run --kind antigravity --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json artifacts read .comptext/runs/latest/run.json --max-bytes 12000
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json runs list
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
```

# Forbidden Commands And Actions

- `git push`
- `git pull`
- direct external Codex task execution
- direct external Antigravity task execution
- network commands
- destructive filesystem commands
- apply without explicit user approval
- full-access mode unless explicitly requested by the user

# Workflow Pattern

- Plan first for non-trivial changes.
- Edit minimal files.
- Return changed files.
- User validates externally in PowerShell.
- If validation fails, fix only the failing issue.
- If validation passes, suggest a local snapshot commit only.
- Do not suggest push by default.

# Phase Map

- Phase 1: local runtime, validate, dry-run wrappers
- Phase 2: execution-plan-only
- Phase 3: discovery-only
- Phase 4a: companion skill
- Phase 4b: agent-friendly CLI polish
- Phase 5: gated external execution
- Phase 6: magic UX with deterministic core

# Phase 4b Note

Phase 4b adds read-only agent-friendly CLI introspection. Use `ctxt --json capabilities`, `ctxt --json runs list`, and bounded `ctxt --json runs read latest --max-bytes 12000` before reaching for raw artifact paths. These commands do not invoke external agents, use network, apply proposals, or add real external execution.

# Phase 4c Note

Phase 4c adds read-only JSON contract introspection. Use `ctxt --json schema` when a future agent needs stable output shape summaries for supported JSON commands. The schema command is static and does not read files, write files, invoke external agents, use network, apply proposals, or add real external execution.

# Cross-Agent Compatibility

Codex and Antigravity should both route through `ctxt`.

Use `ctxt --json schema` first to inspect contracts. Use `ctxt --json capabilities` to inspect supported features and disabled gates. Use `ctxt --json runs list` and bounded `ctxt --json runs read` for artifact-first state inspection.

Use proposal-only agent run commands only to create plan artifacts. Do not infer real execution if artifacts say `external_agent_invoked=false`. Do not bypass `ctxt` by invoking Codex CLI or Antigravity CLI directly for tasks.

# Artifact-First Behavior

- Prefer `.comptext/runs/latest/run.json` before making claims about latest agent state.
- Prefer bounded reads with `--max-bytes`.
- Do not infer execution when `external_agent_invoked=false`.
- Do not infer network when artifact says `network_used=false` or `network_allowed=false`.

# Current Validated Baseline

- 37 unit tests
- 32 smoke tests
- `validate --run` green
- latest local snapshot: `0db56e6 Local snapshot: add phase 3 agent discovery`
