# Agent Runtime CLI

CompText CLI is not a replacement for Codex CLI or Antigravity CLI. It is a deterministic safety wrapper that prepares and records local agent runs before any external coding agent is allowed to act.

## Phase 1 Behavior

Phase 1 does not execute external agents.

`ctxt agent list` reports the agent kinds known to the runtime:

- `dummy`: local/offline placeholder.
- `codex`: external agent, dry-run-only in Phase 1.
- `antigravity`: external agent, dry-run-only in Phase 1.

`ctxt agent run --kind <kind> --task "<task>"` always creates a Context Pack and writes a run artifact at:

```text
.comptext/runs/latest/run.json
```

The artifact records the task, agent kind, context pack path, network policy, proposal policy, validation commands, timestamp, and safety flags.

## Safety Model

CompText keeps these boundaries in front of future agent execution:

- Context is packed deterministically before an agent run.
- Network remains denied by default.
- Proposal-before-apply remains required.
- Generated proposals are not applied automatically.
- External agents are not invoked unless a future phase adds an explicit execution gate.
- Phase 1 treats Codex and Antigravity runs as dry-run-only unless later phase flags are used.

## Phase 2 Behavior

Phase 2 adds execution plans only. It does not invoke Codex CLI, Antigravity CLI, or any other external agent.

`ctxt agent run --kind codex --task "<task>" --allow-external --proposal-only` and the matching `antigravity` command return an `execution-plan-only` JSON response. The run artifact records the same execution plan and confirms that no external process was invoked.

Proposal-only means:

- no apply
- no external process
- no network

This prepares the contract for future gated execution without changing the Phase 1 safety boundary.

## Phase 3 Behavior

Phase 3 adds discovery only.

`ctxt agent discover` lists the external agent kinds that CompText knows how to discover:

- `codex`
- `antigravity`

`ctxt agent discover --kind <kind>` scans local `PATH` metadata for the matching CLI binary. It does not execute external agents, does not execute version commands, does not use network, and does not apply proposals.

Version detection is deferred to a future gated capability because even a version check would invoke the external binary.

## Phase 4b Behavior

Phase 4b adds read-only, agent-friendly CLI introspection.

`ctxt --json capabilities` reports the stable machine-readable runtime capability surface, including supported phases, safety defaults, feature flags, and safe command families.

`ctxt --json runs list` lists first-class run references. Phase 4b exposes `latest` at:

```text
.comptext/runs/latest/run.json
```

`ctxt --json runs read latest --max-bytes 12000` and `ctxt --json runs read --id latest --max-bytes 12000` read that run artifact through a bounded interface. The default read limit is 12000 bytes.

These commands are read-only. They do not use network, do not invoke external agents, do not execute version commands, do not apply proposals, and do not add real external execution.

## Phase 4c Behavior

Phase 4c adds read-only JSON contract introspection.

`ctxt --json schema` reports stable machine-readable contract summaries for major JSON outputs:

- `capabilities`
- `runs list`
- `runs read`
- `agent discover`
- `agent run --allow-external --proposal-only`
- `validate`

The schema command returns static JSON only. It does not read files, write files, use network, invoke external agents, apply proposals, or add real external execution.

## Phase 4d Behavior

Phase 4d adds cross-agent compatibility guidance only.

`ctxt` is the common source of truth for Codex and Antigravity. Both agents must use the same safe JSON commands rather than separate runtime behavior:

```powershell
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json runs list
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
cargo run --bin ctxt -- --json agent discover
cargo run --bin ctxt -- --json agent run --kind codex --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json agent run --kind antigravity --task "..." --allow-external --proposal-only
cargo run --bin ctxt -- --json validate --run
```

Phase 4d does not add plugin packaging, MCP servers, hooks, network, apply, or real external execution. Antigravity guidance is an adapter to `ctxt`, not a separate runtime, policy, or execution system.

## Future Phases

Later phases may add real Codex CLI or Antigravity CLI invocation behind explicit gates. Those phases should preserve the run artifact, keep JSON output machine-readable, and record provenance before and after external execution.
