---
name: "ahas-ux-nuxt-style"
bundle: worker
version: "2.0.1"
description: "Strict coding style rules for Node.js + TypeScript + Vue/Nuxt projects (git/docs skill separated)."
---

# TypeScript / Vue / Nuxt Coding Style Guide Skill (v2.0.1)

## Purpose
This is a strict standard to keep contributions consistent for Node.js + TypeScript + Vue/Nuxt.

- Even if ESLint / Prettier / tsconfig conflicts with this guide, **this guide wins**.
- On conflicts, you **MUST report** the exact conflict point and the reason.
- This document is used as the Agent Skills `SKILL.md` and applies equally to: code generation, modification, review, and refactoring.

## Sections
- Node Base (Node.js runtime / tooling / project ops): `skill/node-base.md`
- TypeScript: `skill/typescript.md`
- Vue / Nuxt: `skill/vue-nuxt.md`
- Vue SFC Critical (source of truth): `skill/vue-sfc-critical.md`
- Agent Enforcement: `skill/agent-enforcement.md`

## Output Checklist
- [ ] I applied Node Base rules (`skill/node-base.md`).
- [ ] Public surface types are explicit where required (`skill/typescript.md`, `skill/vue-nuxt.md`).
- [ ] Return types are explicit where required (`skill/typescript.md`).
- [ ] No hard-coded constants remain; moved to domain constants/config where required (`skill/node-base.md`, `skill/typescript.md`).
- [ ] Vue SFC contains no business logic (or business logic is moved to `app/domain/`) (`skill/node-base.md`, `skill/vue-nuxt.md`).
- [ ] No SSR-unsafe direct access (`window`, `document`, `navigator`) without guards (`skill/vue-nuxt.md`).
- [ ] Template logic is minimized; `computed` preferred over watchers (`skill/vue-nuxt.md`).
- [ ] User-facing strings use `t()`; no i18n hardcoding (`skill/vue-nuxt.md`).
- [ ] Import order / control-flow / spacing rules are followed (`skill/typescript.md`).
- [ ] SFC tag order + macro placement follow the source of truth (`skill/vue-sfc-critical.md`).
- [ ] Pinia + the project’s primary UI framework are used as required (`skill/vue-nuxt.md`).

## Notes
- This skill is a **strict standard**, not a suggestion.
- Even if automation formats/lints differently, this guide wins and conflicts must be reported.
- No item may be omitted. `Vue SFC Structure & Order (Critical)` is treated as **critical**.
