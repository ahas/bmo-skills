# Agent Enforcement Rules

Agents MUST perform the following during commit/documentation tasks:

1. Apply this guide as the highest priority.
2. If tooling or local conventions conflict, explicitly state the conflict point and the reason.
3. If a change affects public surface, update `README.md` + doc comments.
4. If a change affects public surface, update `CHANGELOG.md` with:
   - the current semantic version from project metadata (for example `package.json` or `Cargo.toml`)
   - update existing version section when present, or add it when missing
   - a `Release Notes` section using only non-empty categories:
     - Breaking
     - Features
     - Bug fixes
     - Reverts
     - Code Refactoring
     - Miscellaneous
   - explicit migration steps when `Breaking` applies.
5. If `AGENTS.md` exists and project management state changed, update it.
6. If formatting is required and within scope/permissions, apply the project formatter; otherwise report why.
7. When asked to write a commit message, enforce Conventional Commits and bullet body formatting rules.
