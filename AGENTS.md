# CompText CLI / ctxt Runtime

## Mission

This repository is the target implementation for the next-generation CompText CLI/runtime named `ctxt`.

CompText should become a deterministic symbolic protocol plus audit/evidence layer for agent communication, compact context transfer, MCP-safe tooling, DSL validation, and illegible reasoning trace detection.

## Allowed Work

Implement inside this repository only.

Legacy repositories are located one directory above:

- `../legacy-codex`
- `../legacy-dsl`
- `../legacy-sparkctl`
- `../legacy-mcp`
- `../legacy-v7`

They may be inspected and copied from selectively, but never modified.

## Implementation Targets

Expose these commands:

- `ctxt parse`
- `ctxt encode`
- `ctxt batch`
- `ctxt dsl validate`
- `ctxt evidence hash`
- `ctxt mcp serve --allowed-root`
- `ctxt detect-illegible-cot`

## Reuse Plan

Use `../legacy-codex` for:

- parser
- encoder
- batch syntax
- token-reduction measurement concept
- CLI ideas
- MCP analyzer idea, rebuilt safely

Use `../legacy-dsl` for:

- grammar.ebnf
- `$skill` syntax
- `@resource` syntax
- `tool {}`
- `task {}` as CompText DSL abstraction only

Use `../legacy-sparkctl` for:

- evidence records
- stable hashing
- replay/determinism concepts
- validation contracts

Use `../legacy-mcp` for:

- MCP packaging ideas only
- safe server structure

Use `../legacy-v7` for:

- documentation
- reviewer story
- deterministic audit framing
- showcase wording

## Security Rules

- MCP file access must require explicit `allowed_roots`.
- Block path traversal.
- Block absolute reads outside allowed roots.
- Add max file size checks.
- MCP analyzer must be read-only.
- No credential changes.
- No global machine changes.
- No destructive operations.

## Documentation Rules

- Do not claim universal 94 percent token reduction.
- Token reduction claims must be benchmark-scoped.
- Do not claim `task {}` is an official MCP primitive.
- Describe `task {}` as a CompText DSL abstraction.
- Use `AGENTS.md` for Codex project instructions.
- Use `.codex/config.toml` for Codex project configuration.
- Skills must be folders containing `SKILL.md` with `name` and `description`.

## Testing Requirements

Add or keep tests for:

- parser roundtrip
- batch parse
- invalid command
- invalid language
- invalid modifier
- DSL skill/resource/tool/task validation
- evidence stable SHA256
- MCP allowed path
- MCP blocked traversal
- CLI smoke tests

## Stop Condition

Work is complete only when the following are implemented or explicitly documented as remaining TODO:

- `ctxt parse "C;P:FIB" --json`
- `ctxt encode --command CODE --language PYTHON --task FIB`
- `ctxt batch "B:[D:SUM]|[C;P:FIB]"`
- `ctxt evidence hash README.md`
- `ctxt dsl validate examples/basic.ctxt`
- `ctxt detect-illegible-cot examples/trace.txt`
- tests run
- `git status` shown
