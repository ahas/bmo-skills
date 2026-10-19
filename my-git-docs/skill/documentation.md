# Documentation

## README / Doc Comments
- If a change affects the public surface, you **MUST** update `README.md` and relevant doc comments.
- Public surface includes (not limited to): exported APIs, CLI flags, config schema, runtime behavior visible to users, and any documented contract.

## AGENTS
- If `AGENTS.md` exists and the change affects project management state, you **MUST** update `AGENTS.md`.

## Formatting
- If formatting is required and within scope/permissions, you **MUST** apply the project formatter (e.g., Prettier, rustfmt, gofmt).
- If execution/application is not possible, you **MUST** report why.

## Dependency Updates
- Before updating dependencies, verify breaking changes across the latest and intermediate upgrade paths.
- If no breaking changes and applicable, update to the latest stable version.
- If breaking changes exist or cannot be applied immediately, report impact scope and wait.
