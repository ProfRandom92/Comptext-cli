# VHS Demo Asset Plan

## Phase

VHS Demo Asset Plan

## Mode

visual-assets

## Read-First Files

- `AGENTS.md`
- `PROJEKT.md`
- `tasks/VHS_DEMO.md`
- `.agents/skills/ctxt-runtime/SKILL.md`
- `.agents/skills/ctxt-antigravity-runtime/SKILL.md`
- `C:\Users\contr\.agents\skills\cli-developer\SKILL.md`

## Goal

Prepare a small VHS-based terminal demo plan for CompText CLI without changing runtime code, tests, release metadata, workflows, provider configuration, or secrets.

## Allowed Files

- `tasks/VHS_DEMO.md`
- `docs/DEMO.md`
- `assets/demo/.gitkeep`
- `assets/demo/ctxt-demo.tape`
- `README.md` only for a short link to `docs/DEMO.md`

## Forbidden Files

- `src/**`
- `tests/**`
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `assets/brand/**`
- `AGENTS.md`
- `PROJEKT.md`
- `.agents/**`
- `reports/**`
- `RELEASE_NOTES_v0.1.0.md`
- provider configs
- secrets or credential-bearing files
- release tags, release objects, crate publication, or generated large media assets

## Implementation Rules

- Use VHS only as a local terminal-demo renderer.
- Keep the tape README-friendly and short.
- Do not add runtime behavior.
- Do not invoke providers.
- Do not enable network access from the demo commands.
- Do not read or print secrets.
- Treat rendered GIF output as optional and exclude it from commit if rendering fails or the file is larger than 5 MB.

## Validation

```powershell
git --no-pager diff --stat
git --no-pager diff -- tasks/VHS_DEMO.md docs/DEMO.md assets/demo/ctxt-demo.tape README.md
git --no-pager status --short --branch
Select-String -Path README.md,docs/DEMO.md,tasks/VHS_DEMO.md -Pattern `
  <forbidden release and runtime assurance claims from the phase prompt>
```

## Return Schema

```text
PHASE: VHS Demo Asset Plan
STATUS: <success | blocked>
FILES_CHANGED: <list of changed files>
COMMANDS_RUN: <list of commands executed>
VALIDATION: <validation output summary>
ARTIFACTS: <list of generated artifacts>
GIT: <commit SHA(s) and push status if explicitly authorized>
NETWORK: <offline-only | allowed-external for git fetch/push only>
SECRETS: <secrets status>
POLICY_DECISIONS: <policy status>
RISKS: <remaining risks or release blockers>
NEXT: <next action>
```
