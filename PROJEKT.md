# PROJEKT.md — CompText CLI Project Tracker

This file serves as the tracking document and roadmap execution record for the **CompText CLI** (`ctxt`) project.

## Project Vision & Architecture
CompText CLI is an experimental terminal context client for building deterministic, schema-checked Context Packs before interacting with local or cloud model providers.
> Models are providers. Context is the product.

### Core Architecture
- **CLI Shell**: Command parser and handler for local, offline-first commands.
- **Context Harvester**: Logic to inspect and aggregate codebase context.
- **Deterministic Context Packs**: Unified payload stored in `.comptext/context_pack.latest.json` with keys sorted recursively and sensitive values redacted.
- **Provider Adapters**: Pluggable provider system (Dummy, Ollama, OpenAI-compatible, etc.) with explicit network and policy boundaries.
- **Proposal/Apply Gate**: Workflows to review planned changes (`propose`) and apply them (`apply`) safely.

---

## Autonomous State Machine Configuration

### Current State
```text
REPOSITORY: ProfRandom92/comptext-cli
LATEST_SYNCED_COMMIT: 3706133 Local snapshot: add phase 5e review workflow contract
BRANCH: main synced with origin/main
CURRENT_PHASE: 5e
CURRENT_TASK: README R1 community upgrade and v0.1.0 release candidate preparation
LAST_GREEN_PHASE: 5e
STATUS: release-candidate-prep
NEXT_ALLOWED_ACTION: Step 1 README community upgrade using tasks/README_R1_COMMUNITY.md
```

### Current Validation Baseline
```text
cargo fmt --all --check: green
cargo check: green
cargo test: green
cargo clippy -- -D warnings: green
cargo run --bin ctxt -- --json validate --run: green
unit tests: 38 green
smoke tests: 83 green
```

### Current Local Working Model
- `PROJEKT.md` is the project source of truth.
- `AGENTS.md` defines repository agent safety rules.
- `tasks/*.md` defines concrete work slices.
- `.agents/skills/*` defines reusable working skills.
- Codex/Antigravity may work only inside declared task scope.
- Commit, push, tag, and release only when explicitly requested.

### Autonomy Contract
- **Allowed Modifications**: May edit source code (`src/**`), tests (`tests/**`), docs (`docs/**`), skills (`.agent/skills/**`, `.agents/skills/**`), prompts (`prompts/**`), and configurations (`Cargo.toml`, `comptext.example.toml`).
- **Allowed Commands**: May run local compilation, lint checks, tests, and formatting validation.
- **Error Remediation**: May automatically modify code to fix local build, format, test, or clippy failures.
- **Phase Transition**: May update project status and phase reports after local validation passes. A local commit requires explicit phase-level authorization. Any remote publication, branch publication, PR creation, or merge requires separate explicit user authorization. If remote interaction is needed without that authorization, halt as `BLOCKED`.

### Forbidden Rules
- **No Secret Material Access**: Forbidden to read or parse secret-bearing local files or private auth material.
- **No Sensitive Output Leakage**: Forbidden to print sensitive values in stdout/stderr or write them to logs/reports/artifacts.
- **No Untrusted Provider Action**: Forbidden to execute real cloud API provider calls during coding/validation phases unless explicitly approved for live integration runs.
- **No Destructive/Out-of-Scope Commands**: Forbidden to run shell operations outside the repo root.
- **No Overwriting Remote History**: Forbidden to run force-push operations unless explicitly approved.
- **No Unsupported Assurance Claims**: Forbidden to make unsupported assurance claims.

### Stop Conditions
The agent must halt execution and yield to the user when:
1. Secret or private auth material is required to proceed.
2. Real cloud provider execution or live network calls are needed.
3. Git merge conflicts arise that cannot be resolved safely.
4. Validation fails and cannot be resolved with small, safe changes.
5. Codebase requirements or user requests are contradictory.
6. Target files outside the repository root need to be accessed or created.
7. Local commit, remote publication, branch publication, PR creation, or merge is needed without explicit authorization for that exact action.

### Global Validation Suite
The agent must run and satisfy the following validation suite before completing any phase:
```bash
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
```

### Git Safety Gate
Passing validation does not imply permission to mutate Git or remote state.

Allowed without separate Git authorization:
- read-only status checks
- read-only diff/stat inspection
- read-only latest-commit inspection

Requires explicit phase-level authorization:
- staging phase changes
- creating a local commit

Requires separate explicit remote authorization:
- publishing to remote
- creating remote branches
- creating PRs
- merging

If remote Git interaction is required without explicit remote authorization, stop and report `BLOCKED`.

---

## Phase Queue & Roadmap Status

| Phase | Description | Goal / Scope | Status |
|---|---|---|---|
| **Phase 5e** | README R1 Community Upgrade & v0.1.0 Release Candidate Preparation | Upgrade README through `tasks/README_R1_COMMUNITY.md`, keep deterministic context and safety boundaries intact, and prepare release-candidate review state | **ACTIVE: release-candidate-prep** |

Historical implementation phases are not the active planning source for this branch snapshot. The current executable work queue is task-scoped and must be read from `tasks/*.md`, with `tasks/README_R1_COMMUNITY.md` as the next allowed task.

---

## Standard Phase Return Format
All phase transitions must output their status report using the following schema:
```text
PHASE: <Phase Number and Title>
STATUS: <success | blocked>
FILES_CHANGED: <list of changed files>
COMMANDS_RUN: <list of commands executed>
VALIDATION: <validation output summary>
ARTIFACTS: <list of generated artifacts>
GIT: <read-only status by default; local commit only if explicitly authorized; remote action only if separately explicitly authorized>
NETWORK: <offline-only | local-only | allowed-external>
SECRETS: <secrets status>
POLICY_DECISIONS: <policy status>
RISKS: <analysis of potential risks>
NEXT: <next action or phase name>
```
