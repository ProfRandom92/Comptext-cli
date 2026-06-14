[$ctxt-runtime](C:\Users\contr\Desktop\comptext-cli-agent-work\.agents\skills\ctxt-runtime\SKILL.md) [$ctxt-antigravity-runtime](C:\Users\contr\Desktop\comptext-cli-agent-work\.agents\skills\ctxt-antigravity-runtime\SKILL.md) [$cli-developer](C:\Users\contr\.agents\skills\cli-developer\SKILL.md)

SECTION: Task

Upgrade README.md into a polished, community-friendly v0.1.0 release candidate README.

SECTION: Current repository

Path:
C:\Users\contr\Desktop\comptext-cli-agent-work

GitHub:
https://github.com/ProfRandom92/comptext-cli

Latest synced commit:
3706133 Local snapshot: add phase 5e review workflow contract

Current status:
main synced with origin/main

Validation evidence:
- cargo fmt --all --check green
- cargo check green
- cargo test green
- cargo clippy -- -D warnings green
- ctxt --json validate --run green
- 37 unit tests passed
- 83 smoke tests passed

SECTION: Current identity

CompText ctxt v0.1.0 is a deterministic contract runtime for agent-friendly local review workflows.

It exposes stable JSON contracts for:
- runtime self-reporting
- schema introspection
- capabilities introspection
- proposal artifact evidence
- review artifact evidence
- deterministic subagent role contracts
- startup readiness
- startup flow
- deterministic review workflow planning
- local validation

SECTION: Required safety sentence

Include this exact sentence near the top:

It does not execute external agents, use network, call providers, apply proposals or reviews, or run subagents.

SECTION: Important correction

The current README is stale. It still describes an older Phase 17 / Antigravity Plugin Bundle / Review-Gate status.

Remove or replace stale Phase 17 positioning with v0.1.0 contract-runtime positioning.

SECTION: Style target

Community-friendly, confident, visual, readable.

Use:
- centered hero block
- badges
- table of contents
- GitHub-compatible Mermaid diagrams
- command matrix
- capability matrix
- safety boundary matrix
- quickstart
- validation evidence
- roadmap
- contributing
- license
- visual asset plan

SECTION: Badges

Use honest shield badges only.

Allowed badge concepts:
- License MIT
- Rust stable
- CLI ctxt
- JSON contracts stable
- Network deny by default
- External agents disabled
- Apply disabled
- Subagent runtime disabled
- MCP server not implemented
- Windows validated
- v0.1.0 release candidate
- Deterministic contracts

Do not add CI badge unless an existing CI workflow is confirmed locally.

SECTION: Required diagrams

Add at least three GitHub-compatible Mermaid diagrams.

Diagram 1:
Architecture overview:
User or Agent -> ctxt CLI -> JSON contracts -> local artifacts / validation output

Diagram 2:
Deterministic review workflow:
startup readiness -> startup flow -> schema -> capabilities -> subagents -> proposals -> reviews -> review workflow -> validate --run -> user summary

Diagram 3:
Safety boundary:
Allowed read-only contracts vs disabled gates:
network, providers, external agents, apply, subagent execution, MCP server, hooks, plugins, arbitrary shell

Use GitHub-compatible Mermaid syntax. Keep labels readable.

SECTION: Figma / visual asset plan

Add a short section called Visual asset plan.

Explain:
- README diagrams stay as Mermaid for version control and GitHub rendering.
- Figma can be used later for GitHub README header, social preview card, footer strip, architecture poster, and launch image.
- Do not add generated asset references unless matching files already exist.

SECTION: Stable commands to document

Document these commands:

cargo run --bin ctxt -- --json self report
cargo run --bin ctxt -- --json schema
cargo run --bin ctxt -- --json capabilities
cargo run --bin ctxt -- --json startup readiness
cargo run --bin ctxt -- --json startup flow
cargo run --bin ctxt -- --json review workflow
cargo run --bin ctxt -- --json subagents list
cargo run --bin ctxt -- --json proposals list
cargo run --bin ctxt -- --json proposals inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json proposals validate latest
cargo run --bin ctxt -- --json reviews list
cargo run --bin ctxt -- --json reviews inspect latest --max-bytes 12000
cargo run --bin ctxt -- --json reviews validate latest
cargo run --bin ctxt -- --json agent discover
cargo run --bin ctxt -- --json runs list
cargo run --bin ctxt -- --json runs read latest --max-bytes 12000
cargo run --bin ctxt -- --json validate --run

SECTION: Quickstart commands

Include PowerShell examples for:

cargo fmt --all --check
cargo check
cargo test
cargo clippy -- -D warnings
cargo run --bin ctxt -- --json self report
cargo run --bin ctxt -- --json startup readiness
cargo run --bin ctxt -- --json startup flow
cargo run --bin ctxt -- --json review workflow
cargo run --bin ctxt -- --json validate --run

SECTION: Allowed files

Edit only:
- README.md

Do not edit:
- src/
- tests/
- docs/
- .agents/
- Cargo.toml
- Cargo.lock
- proposals/
- reviews/
- reports/
- assets/
- workflows
- release notes
- tags

SECTION: Do not

Do not commit.
Do not push.
Do not tag.
Do not create release.
Do not create Figma files.
Do not modify assets.
Do not claim production readiness.
Do not claim MCP server implementation.
Do not claim external execution.
Do not claim provider integration.
Do not claim network-enabled runtime.

SECTION: Validation after edit

Run:

git --no-pager diff -- README.md

Select-String -Path README.md -Pattern "production-ready", "external execution enabled", "network enabled", "applies proposals", "applies reviews", "subagents executed", "MCP server implemented", "provider calls enabled", "token passthrough enabled"

git --no-pager status --short --branch

SECTION: Return format

Return:
- changed file
- summary
- badges added
- Mermaid diagrams added
- stale README claims removed
- safety claims checked
- validation output
- git status
- concerns
