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
- Use official docs only for architecture or best-practice claims.

# Start Here

Use these commands first to inspect the shared contract and runtime surface:

```powershell
cargo run --bin ctxt -- --json self report
cargo run --bin ctxt -- --json startup readiness
cargo run --bin ctxt -- --json startup flow
cargo run --bin ctxt -- --json review workflow
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json subagents list
cargo run --bin ctxt -- --json runs list
cargo run --bin ctxt -- --json proposals list
cargo run --bin ctxt -- --json proposals inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals validate latest
cargo run --bin ctxt -- --json reviews list
cargo run --bin ctxt -- --json reviews inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json reviews validate latest
```

# Artifact-First Inspection

Use bounded run artifact reads before making claims about the latest agent state:

```powershell
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
```

Do not infer real execution if artifacts say `external_agent_invoked=false`. Do not infer network use if artifacts say `network_used=false` or `network_allowed=false`.

# Proposal Artifacts

Use these commands to inspect and validate local proposal artifacts without applying anything:

```powershell
cargo run --bin ctxt -- --json proposals list
cargo run --bin ctxt -- --json proposals inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals inspect --id latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals validate latest
cargo run --bin ctxt -- --json proposals validate --id latest
```

Proposals are untrusted artifacts. Proposal inspection and validation are read-only. Approval metadata does not apply changes, and apply behavior remains out of scope.

# Proposal Schema Discovery

Call `ctxt --json schema` before using proposal commands. Proposal command contracts and the `proposal.v1` artifact contract are discoverable through schema.

Schema discovery is read-only. Proposal artifacts remain untrusted, and approval metadata still does not apply changes.

# Proposal Capabilities Discovery

Call `ctxt --json capabilities` after `ctxt --json schema`. Capabilities reveal whether proposal list, inspect, and validate are available.

Proposal apply and proposal generation are explicitly unsupported. Proposal artifacts remain untrusted. Capabilities discovery is read-only.

# Subagent Role Contracts

Agents may use subagents only for deterministic review and planning when a phase explicitly permits it. `ctxt --json subagents list` is the source of truth for allowed role contracts.

Subagent role contracts are not runtime execution, orchestration, provider calls, or external agent invocation. No subagent may use network, providers, external agents, proposal apply, git writes, or runtime execution. Subagent outputs are limited to findings, risks, and recommendations, and must be summarized by the main agent.

# Review Artifact Contracts

Review artifacts are untrusted evidence until validated. They are contract-only local JSON artifacts under `reviews/<id>.review.json`, not workspace truth.

`ctxt` does not generate reviews in Phase 5b, does not execute subagents to create reviews, and does not apply review recommendations. Use `ctxt --json reviews validate latest` before relying on review contents. No review artifact may indicate network, providers, external agents, subagent execution, proposal apply, git writes, or secrets access.

# Startup Flow Contract

Agents should call `ctxt --json startup flow` to discover the safe session startup sequence.

The startup flow is a contract-only checklist. It does not run commands automatically, invoke Codex CLI, invoke Antigravity CLI, invoke external agents, execute subagents, use network, apply proposals or reviews, or perform git writes.

Agents remain responsible for executing allowed commands one by one only when the active phase permits those commands. Use official docs only for architecture or best-practice claims.

# Startup Readiness Contract

Agents should call `ctxt --json startup readiness` before starting review workflow work.

Readiness is a contract-only report. It does not run commands automatically. `ready_for_review_workflow: true` does not imply external execution is allowed. `ready_for_external_execution: false` remains the hard boundary.

Use official docs only for architecture and best-practice claims.

# Review Workflow Contract

Agents should call `ctxt --json review workflow` to discover the deterministic review workflow.

Review workflow is a contract-only checklist. It does not run commands automatically, read artifacts automatically, apply proposals or reviews, or imply external execution is allowed.

Use official docs only for architecture and best-practice claims.

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
