# bmo-skills

The skills a job worker is given, one folder per skill named by its category first, and the placer that
hands them over. Each skill is a `SKILL.md` with the files it names; its front matter says which bundle
it belongs to: the `worker` bundle, the plugin every job worker is given, or `project`, a skill one
repository's checkout keeps as its own. The repository runs nothing: a consumer's deployment runs `bmo-skills place`, which writes the worker
plugin in Claude Code's standard plugin layout, or one skill into its own folder, into the consumer's tree.

## Layout

| Folder | Holds |
|---|---|
| `.claude-plugin/` | The manifest of the `bmo-worker` plugin the worker bundle is placed as |
| `bmo-asking-the-owner/` | Putting a decision or a document in front of the owner |
| `bmo-beyond-the-checkout/` | What a job owns once its work reaches past its own checkout |
| `bmo-closing-a-job/` | How a job ends: the graph, the close, what it may leave and the report |
| `bmo-marking-a-wait/` | Marking a deliberate wait on a job's item |
| `bmo-song-cover/` | Making a song in the chat agent's voice with the song bench; a project skill |
| `bmo-warm-prefix/` | Changing what a running session holds as its cached prefix |
| `code-boundaries/` | Where a variation belongs, where a list a person edits is kept, and how a base library is contributed to |
| `code-clean/` | The line-level pass of a refactoring, with its principles and its Rust and TypeScript notes |
| `code-design-patterns/` | The structural pass of a refactoring, with its pattern catalogues |
| `code-korean-out/` | Taking the Korean out of a bundle of code files |
| `code-refactoring/` | The module and function pass of a refactoring, with its catalogue and smells |
| `code-test-writing/` | The rules a test is written to |
| `docs-conventions/` | Where and how new information is recorded in a repository |
| `docs-eli5/` | Explaining a topic in pictures, as simply as it goes |
| `docs-git/` | The git workflow, documentation and changelog rules |
| `tools-find-skills/` | Finding a skill for a task among the public skills |
| `tools-playwright-cli/` | Driving a browser and writing Playwright tests, with its references |
| `tools-researching-and-benching/` | Researching, benching or tuning something the lab runs |
| `ux-design-rules/` | The design rules for every user interface, by area, and the wording list |
| `ux-nuxt-style/` | The coding style for Node.js, TypeScript and Vue or Nuxt projects |
| `ux-nuxt-ui/` | Building interfaces with Nuxt UI, with its component and layout references |
| `src/` | The `bmo-skills` placer: reading the skills, listing them and placing them |
| `tests/` | The placer's tests and the checks that hold the skills to their format |
| `docs/` | The canonical statements of the format, the boundary and the inventory |
| `graph/` | The code dependency graph: one node per feature with its files and what it reads, written only by `bmo graph` |

At the root stand the Cargo manifest, the version, `testbed.toml` with its Korean ledger, `rustfmt.toml`
and `CLAUDE.md`.

## Documents

| Document | States |
|---|---|
| `docs/format.md` | The shape of a skill folder, its front matter and its bundle, and what a skill may carry |
| `docs/boundary.md` | The placer's verbs, the manifest, which unit receives which skills from where, what each skill calls on another unit, and the version rule |
| `docs/inventory.md` | Every file with its source and place, what was left to other units, the tests ported and every name replaced |

## Version

- **One version for the repository,** in semver. `VERSION` holds it, and the package's version and the
  manifest's equal it.
- **How a change moves it** is stated in `docs/boundary.md`, 「Versions」.

## Commands

```sh
cargo run -- list
cargo run -- place --bundle worker --into <dir>
cargo run -- place --skill <name> --into <dir>
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
