# CompText CLI v0.1.0 Release Notes

Status: GitHub pre-release candidate only. Tag only after CI is green.

CompText CLI `ctxt` is a local-first Rust CLI for deterministic, schema-oriented review workflow contracts. The v0.1.0 release candidate focuses on local evidence, explicit disabled gates, and contract-first startup behavior.

This release candidate is not published to crates.io yet.

## Highlights

- Local-first Rust CLI `ctxt`.
- Deterministic JSON contracts for machine-readable runtime state.
- Startup, readiness, review workflow, and capability contracts.
- Proposal, review, run, and validation artifact inspection contracts.
- Experimental local runtime contract covering command output, JSON behavior, bounded local reads, and MCP-style stdio errors.
- DSL validation for the `local-fixture-v1` subset only.
- Encode JSON shape coverage for the current symbolic command vocabulary.
- Clean trace heuristic coverage for traces without deterministic finding phrases.
- Release-readiness blocker cleanup for clippy, Cargo package metadata, package exclusions, and README MCP wording.
- README brand assets for the v0.1.0 release-candidate presentation.
- Binary README and media assets are excluded from context packing.
- Offline/local-first safety boundary by default.

## Boundaries

- GitHub pre-release candidate only; no crates.io publish has occurred.
- MCP-style local stdio adapter only.
- No production MCP support claim.
- No full MCP compliance claim.
- No external agent execution claim.
- No provider gateway or live provider claim.
- No proposal or review auto-apply claim.
- No claim of hidden network activity.

## Runtime Contract Scope

- `ctxt parse`, `ctxt encode`, `ctxt batch`, `ctxt dsl validate`, `ctxt evidence hash`, `ctxt mcp serve --allowed-root`, and `ctxt detect-illegible-cot` are documented as local runtime surfaces.
- DSL validation is limited to `local-fixture-v1`; it does not execute skills, tools, tasks, providers, shell commands, OAuth flows, network resources, or MCP tool definitions.
- MCP behavior is limited to the local stdio adapter contract and JSON-RPC-style error shapes under an explicit allowed root.
- Correctness claims are bounded to local command outputs, JSON contracts, deterministic hashes, exact exit codes, smoke tests, and `cargo test`.

## Validation Commands

```powershell
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run --bin ctxt -- --json validate --run
```
