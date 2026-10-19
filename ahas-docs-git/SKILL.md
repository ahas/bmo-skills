---
name: "ahas-docs-git"
bundle: worker
version: "1.1.0"
description: "Strict git workflow + documentation + changelog rules (language-agnostic)."
---

# Git Workflow & Documentation Skill

## Purpose
Strict rules for git workflow and project documentation/release notes.

- Even if tooling or local conventions differ, **this guide wins**.
- On conflicts, you **MUST report** the conflict point and the reason.
- Applies to: commit message writing, PR/MR summaries, documentation updates, and release notes.

## Sections
- Git Workflow: `skill/git-workflow.md`
- Documentation: `skill/documentation.md`
- Changelog: `skill/changelog.md`
- Agent Enforcement: `skill/agent-enforcement.md`

## Output Checklist
- [ ] I did not overwrite user changes; no destructive git operations without explicit request.
- [ ] Commit message follows Conventional Commits (type/scope/subject) and body formatting rules.
- [ ] If public surface changed, I updated README and doc comments as required.
- [ ] If public surface changed, I updated CHANGELOG for the current semantic version (update existing entry or add new one) with non-empty Release Notes categories and explicit breaking migration steps where applicable.
- [ ] If project management state changed and `AGENTS.md` exists, I updated it.
- [ ] If formatting is required and within scope/permissions, I applied the project formatter; otherwise I reported why not.

## Notes
- This is a **strict standard**, not a suggestion.
- No item may be omitted.
