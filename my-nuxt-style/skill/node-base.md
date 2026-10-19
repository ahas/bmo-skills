# Node Base (Node.js runtime / tooling / project ops)

## Package Manager / Commands
- Default package manager is **`bun`**, but if the project already uses another package manager, **the project’s existing one wins**.
- Prefer the existing project scripts (e.g., `check`, `test`, `format`) over ad-hoc commands.
- Because tools may provide default build commands, every build script in `package.json` **MUST** specify its build target using a colon (`:`).
  - Examples: `build:nuxt`, `build:manifest`

## Characters / Comments
- Avoid non-ASCII unless it already exists in the codebase or is explicitly required.
- Comments are **English-only** and **only when necessary**.
- Keep comments short and strictly essential.

## Declarations / Structure
- After `import`s, declare interfaces and types, and **declare interfaces first**.
- Vue SFC source code **MUST NOT** contain business logic.
- Business logic **MUST** be separated into abstracted units under `app/domain/`.
- To prevent duplication and context loss, utility functions **MUST** live in a shared directory and be referenced.
- Hard-coded constants are **strictly discouraged**.
- Within the same domain scope, manage values as config/constants and define them with explicit interfaces.

