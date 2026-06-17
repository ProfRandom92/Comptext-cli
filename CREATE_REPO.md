# Create GitHub repo and push

> **Legacy bootstrap reference only**
>
> This file documents the original repository-creation flow. The repository already exists. Do not use this file as an active setup, push, or publication instruction for current work. Current project state lives in `PROJEKT.md`; active task scope lives in `tasks/*.md`.

Target:

```text
Repo:   ProfRandom92/comptext-cli
Crate:  comptext-cli
Binary: ctxt
```

Original bootstrap command sequence:

```bash
git init
git branch -M main
git add .
git commit -m "Initialize CompText CLI scaffold"
gh repo create ProfRandom92/comptext-cli --public --source=. --remote=origin --push
```

After initial push, the original bootstrap plan was to open the repo in Antigravity and run:

```text
prompts/AGY_00_REPO_GENESIS.md
```

Then continue with:

```text
prompts/AGY_01_CONTEXT_PACK.md
```

For current work, ignore the bootstrap sequence above and read:

1. `PROJEKT.md`
2. `AGENTS.md`
3. the active task file referenced by `PROJEKT.md`
