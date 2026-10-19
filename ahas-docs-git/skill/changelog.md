# Changelog

## When to Update
- If a change affects the public surface, you **MUST** update `CHANGELOG.md`.
- Use the current project version as the target release entry instead of creating a commit-by-commit log stream.

## Version Source of Truth
- The target version MUST be read from project metadata that matches the project type.
- Typical sources:
  - Node.js: `package.json` -> `version`
  - Rust: `Cargo.toml` -> `[package].version`
  - Python: `pyproject.toml` -> `[project].version` (or Poetry version field when used)
- If multiple version files exist, use the primary package/release metadata for that project.
- If the version cannot be determined, report the block explicitly before editing `CHANGELOG.md`.

## Required Structure
- Each release entry MUST start with a semantic version heading at the top of the entry.
  - Example: `## 1.4.2`
- Under each version heading, use a `### Release Notes` section.
- Under `### Release Notes`, use only applicable non-empty subsections from:
  - Breaking
  - Features
  - Bug fixes
  - Reverts
  - Code Refactoring
  - Miscellaneous
- Do **NOT** create empty subsections.
- Do **NOT** write placeholders such as "None", "N/A", or "없음".

## Update Behavior (Critical)
- If the current semantic version already exists in `CHANGELOG.md`, update that existing version entry.
- If the current semantic version does not exist, add a new version entry.
- Do **NOT** create duplicate entries for the same semantic version.

## Breaking Notes (Critical)
A change MUST be recorded under `Breaking` when it:
- Changes an exported API signature or behavior in an incompatible way.
- Removes or renames an API, route, CLI flag, config key, or event.
- Changes default behavior in a way that can break existing consumers.
- Changes schema/validation rules that can reject previously valid inputs.

### Required Emphasis
- `Breaking` entries MUST be explicit and actionable:
  - What changed
  - Who is impacted
  - Migration steps (minimum viable steps)
- If possible, include “Before/After” snippets or a short migration example.

## Linking With README
- If a `Breaking` note is added to `CHANGELOG.md`, ensure `README.md` is updated (or explicitly state why it is not).
