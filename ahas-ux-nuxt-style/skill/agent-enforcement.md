# Agent Enforcement Rules

Agents MUST perform the following during code generation/modification/review:

1. Apply this guide as the highest priority.
2. If ESLint/Prettier/tsconfig conflicts, explicitly state the conflict point and the reason.
3. Check whether business logic exists inside Vue SFC; if so, apply or propose extraction into `app/domain/`.
4. Check for SSR-unsafe access (`window`, `document`, `navigator`).
5. Check whether public surfaces have explicit types.
6. Check for hard-coded constants; apply or propose moving to domain config/constants.
7. Check for i18n string hardcoding; enforce `t()` usage.
8. Check import order / control flow / spacing rules.
9. Check `Vue SFC Structure & Order` rules.

> Documentation/release notes and git workflow enforcement are handled by separate skills.
