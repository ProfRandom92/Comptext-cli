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

## Future Phases

Later phases may add real Codex CLI or Antigravity CLI invocation behind explicit gates. Those phases should preserve the run artifact, keep JSON output machine-readable, and record provenance before and after external execution.
