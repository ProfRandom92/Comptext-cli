# CompText CLI v0.1.0 Release Notes

Status: Release candidate, tag only after CI green.

CompText CLI `ctxt` is a local-first Rust CLI for deterministic, schema-oriented review workflow contracts. The v0.1.0 release candidate focuses on stable local evidence, explicit disabled gates, and contract-first startup behavior.

## Highlights

- Local-first Rust CLI `ctxt`.
- Deterministic JSON contracts for machine-readable runtime state.
- Startup, readiness, review workflow, and capability contracts.
- Proposal, review, run, and validation artifact inspection contracts.
- README brand assets for the v0.1.0 release-candidate presentation.
- Binary README and media assets are excluded from context packing.
- Offline/local-first safety boundary by default.

## Boundaries

- No MCP server implementation claim.
- No external agent execution claim.
- No provider gateway or live provider claim.
- No proposal or review auto-apply claim.
- No claim of hidden network activity.

## Validation Commands

```powershell
cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run --bin ctxt -- --json validate --run
```
