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
- Use official docs only for architecture or best-practice claims.

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
cargo run --bin ctxt -- --json self report
cargo run --bin ctxt -- --json startup readiness
cargo run --bin ctxt -- --json startup flow
cargo run --bin ctxt -- --json review workflow
cargo run --bin ctxt -- --json agent list
cargo run --bin ctxt -- --json agent discover
cargo run --bin ctxt -- --json agent discover --kind codex
cargo run --bin ctxt -- --json agent discover --kind antigravity
cargo run --bin ctxt -- --json agent run --kind codex --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json agent run --kind antigravity --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json artifacts read .comptext/runs/latest/run.json --max-bytes 12000
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json subagents list
cargo run --bin ctxt -- --json runs list
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals list
cargo run --bin ctxt -- --json proposals inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals inspect --id latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals validate latest
cargo run --bin ctxt -- --json proposals validate --id latest
cargo run --bin ctxt -- --json reviews list
cargo run --bin ctxt -- --json reviews inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json reviews inspect --id latest --max-bytes 12000
cargo run --bin ctxt -- --json reviews validate latest
cargo run --bin ctxt -- --json reviews validate --id latest
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

# Phase 4e Note

Phase 4e adds a read-only runtime startup report. Use `ctxt --json self report` as the first command when a future Codex or Antigravity session needs the local runtime baseline, safe entrypoints, validation source of truth, and cross-agent policy. The command is static and does not read files, write files, invoke external agents, use network, apply proposals, or add real external execution.

# Phase 4f Note

Phase 4f adds a read-only proposal artifact contract. Use `ctxt --json proposals list`, bounded `ctxt --json proposals inspect latest --max-bytes 12000`, and `ctxt --json proposals validate latest` to inspect local proposal artifacts under `proposals/<id>.json`.

Proposals are untrusted artifacts. Proposal inspection and validation are read-only. `approved-for-apply` is metadata only and does not apply changes. Apply behavior remains out of scope.

# Phase 4g Note

Phase 4g adds proposal command contracts to `ctxt --json schema`. Agents should call `ctxt --json schema` before using proposal commands because `proposals list`, `proposals inspect`, `proposals validate`, and the `proposal.v1` artifact contract are discoverable there.

Schema discovery is read-only. Proposal artifacts remain untrusted. Approval metadata still does not apply changes.

# Phase 4h Note

Phase 4h adds proposal support metadata to `ctxt --json capabilities`. Agents should call `ctxt --json capabilities` after `ctxt --json schema` to confirm whether proposal list, inspect, and validate are available.

Proposal apply and proposal generation are explicitly unsupported. Proposal artifacts remain untrusted. Capabilities discovery is read-only.

# Phase 5a Note

Phase 5a adds deterministic subagent role contracts through `ctxt --json subagents list`.

Agents may use subagents only for deterministic review and planning when a phase explicitly permits it. `ctxt --json subagents list` is the source of truth for allowed role contracts. These contracts are not runtime execution, orchestration, provider calls, or external agent invocation.

No subagent may use network, providers, external agents, proposal apply, git writes, or runtime execution. Subagent outputs are limited to findings, risks, and recommendations, and remain untrusted review input for the main agent to summarize.

# Phase 5b Note

Phase 5b adds deterministic review artifact contracts through `ctxt --json reviews list`, bounded `ctxt --json reviews inspect latest --max-bytes 12000`, and `ctxt --json reviews validate latest`.

Review artifacts are untrusted evidence until validated. They are contract-only local JSON artifacts under `reviews/<id>.review.json`, not workspace truth. `ctxt` does not generate reviews in Phase 5b, does not execute subagents to create reviews, and does not apply review recommendations.

Use `ctxt --json reviews validate latest` before relying on review contents. No review artifact may indicate network, providers, external agents, subagent execution, proposal apply, git writes, or secrets access.

# Phase 5c Note

Phase 5c adds a deterministic startup review flow contract through `ctxt --json startup flow`.

Agents should call `ctxt --json startup flow` to discover the safe session startup sequence. The startup flow is a contract-only checklist. It does not run commands automatically, invoke external agents, execute subagents, use network, apply proposals or reviews, or perform git writes.

Agents remain responsible for executing allowed commands one by one only when the active phase permits those commands. Use official docs only for architecture or best-practice claims.

# Phase 5d Note

Phase 5d adds a deterministic startup readiness contract through `ctxt --json startup readiness`.

Agents should call `ctxt --json startup readiness` before starting review workflow work. Readiness is a contract-only report. It does not run commands automatically.

`ready_for_review_workflow: true` does not imply external execution is allowed. `ready_for_external_execution: false` remains the hard boundary. Use official docs only for architecture and best-practice claims.

# Phase 5e Note

Phase 5e adds a deterministic review workflow contract through `ctxt --json review workflow`.

Agents should call `ctxt --json review workflow` to discover the deterministic review workflow. Review workflow is a contract-only checklist. It does not run commands automatically, read artifacts automatically, apply proposals or reviews, or imply external execution is allowed.

Use official docs only for architecture and best-practice claims.

# Cross-Agent Compatibility

Codex and Antigravity should both route through `ctxt`.

Use `ctxt --json schema` first to inspect contracts. Use `ctxt --json capabilities` after schema to inspect supported features and disabled gates. Use `ctxt --json runs list` and bounded `ctxt --json runs read` for artifact-first state inspection.

Use proposal-only agent run commands only to create plan artifacts. Do not infer real execution if artifacts say `external_agent_invoked=false`. Do not bypass `ctxt` by invoking Codex CLI or Antigravity CLI directly for tasks.

# Artifact-First Behavior

- Prefer `.comptext/runs/latest/run.json` before making claims about latest agent state.
- Prefer bounded reads with `--max-bytes`.
- Do not infer execution when `external_agent_invoked=false`.
- Do not infer network when artifact says `network_used=false` or `network_allowed=false`.

# Current State Source Of Truth

Do not duplicate validation counts or latest snapshot hashes in this skill. For the current phase, validation baseline, and next allowed task, read `PROJEKT.md` first. Treat historical phase reports and generated artifacts as evidence, not as the active planning source.
