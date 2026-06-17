# CompText CLI Docs Index

This directory contains project documentation for the `ctxt` Rust CLI runtime, safety model, release preparation, and agent-facing guidance.

`PROJEKT.md` remains the current project tracker and source of truth for active task state. Files under `reports/` preserve historical implementation evidence and may refer to older phase numbers.

## Start Here

| File | Purpose |
| --- | --- |
| [`../PROJEKT.md`](../PROJEKT.md) | Current phase/task tracker and validation baseline. |
| [`../AGENTS.md`](../AGENTS.md) | Repository agent rules, allowed modes, stop conditions, and Git safety gates. |
| [`../README.md`](../README.md) | Public project overview and v0.1.0 release-candidate positioning. |
| [`AGENT_RUNTIME_CLI.md`](AGENT_RUNTIME_CLI.md) | Agent-facing `ctxt --json ...` runtime behavior and phase-contract notes. |
| [`RELEASE_CHECKLIST.md`](RELEASE_CHECKLIST.md) | Local release-candidate checklist and release-boundary reminders. |

## Runtime and Safety Contracts

| File | Purpose |
| --- | --- |
| [`AGENT_RUNTIME_CLI.md`](AGENT_RUNTIME_CLI.md) | Deterministic startup, readiness, review workflow, proposal/review artifact, and agent-discovery contracts. |
| [`MCP_PROVIDER_BOUNDARY.md`](MCP_PROVIDER_BOUNDARY.md) | MCP/provider boundary wording and safety constraints. |
| [`MCP_AND_SKILLS_PLAN.md`](MCP_AND_SKILLS_PLAN.md) | Planning notes for MCP-style and skill-related surfaces. |
| [`PERMISSIONS_MODEL.md`](PERMISSIONS_MODEL.md) | Permission and boundary model for local runtime work. |
| [`LONG_RUN_AUTONOMY.md`](LONG_RUN_AUTONOMY.md) | Long-running autonomous execution constraints and safeguards. |

## Skills, Governance, and Antigravity

| File | Purpose |
| --- | --- |
| [`SKILL_AUTHORING_GUIDE.md`](SKILL_AUTHORING_GUIDE.md) | Guidance for repo-local skill files. |
| [`SKILL_BUNDLE_REGISTRY.md`](SKILL_BUNDLE_REGISTRY.md) | Skill bundle registry documentation. |
| [`ANTIGRAVITY_PLUGIN_BUNDLE.md`](ANTIGRAVITY_PLUGIN_BUNDLE.md) | Antigravity plugin-bundle skeleton documentation. |

## Release and Evaluation

| File | Purpose |
| --- | --- |
| [`RELEASE_CHECKLIST.md`](RELEASE_CHECKLIST.md) | Checklist before tag, release, or publication. |
| [`RUST_CRATE_EVALUATION.md`](RUST_CRATE_EVALUATION.md) | Notes on Rust crate choices and future dependency candidates. |
| [`TOKEN_ECONOMY.md`](TOKEN_ECONOMY.md) | Token/context-economy notes and positioning. |

## Historical Evidence

Historical status reports live under [`../reports/`](../reports/). They are evidence records, not the active planning source. Current active work should be read from:

1. [`../PROJEKT.md`](../PROJEKT.md)
2. the task file referenced by `PROJEKT.md`
3. relevant `.agents/skills/*` files only when needed

## Legacy Bootstrap Material

Root-level bootstrap files such as [`../MANIFEST.json`](../MANIFEST.json) and [`../CREATE_REPO.md`](../CREATE_REPO.md) describe repository genesis/setup history. They are not current runtime source-of-truth files and should not be used as active setup instructions for an already-created repository.
