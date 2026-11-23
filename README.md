**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

# bmo-skills

The skills a job worker is given, one folder per skill named by its scope, `bmo` or `ahas`, and its category first, and the placer that
hands them over. Each skill is a `SKILL.md` with the files it names; its front matter says which bundle
it belongs to: the `worker` bundle every general job worker is given, the `coder` bundle a coder's
coding and design runs are given, the `producer` bundle a producer's video runs are given, or `project`,
a skill one repository's checkout keeps as its own. The repository runs nothing: a consumer's deployment runs `bmo-skills place`, which writes the worker
plugin in Claude Code's standard plugin layout, or one skill into its own folder, into the consumer's tree.

## Layout

| Folder | Holds |
|---|---|
| `.claude-plugin/` | The manifest of the `bmo-worker` plugin every placed bundle is written as |
| `ahas-code-boundaries/` | Where a variation belongs, where a list a person edits is kept, and how a base library is contributed to |
| `ahas-code-clean/` | The line-level pass of a refactoring, with its principles and its Rust and TypeScript notes |
| `ahas-code-design-patterns/` | The structural pass of a refactoring, with its pattern catalogues |
| `ahas-code-refactoring/` | The module and function pass of a refactoring, with its catalogue and smells |
| `ahas-code-test-writing/` | The rules a test is written to |
| `ahas-docs-git/` | The git workflow, documentation and changelog rules |
| `ahas-tools-find-skills/` | Finding a skill for a task among the public skills |
| `ahas-tools-playwright-cli/` | Driving a browser and writing Playwright tests, with its references |
| `ahas-ux-design-rules/` | The design rules for every user interface, by area |
| `ahas-ux-nuxt-style/` | The coding style for Node.js, TypeScript and Vue or Nuxt projects |
| `ahas-ux-nuxt-ui/` | Building interfaces with Nuxt UI, with its component and layout references |
| `ahas-video-code-rendered/` | Making a video as code, every frame a function of time, with a treatment, measured timing, scene modules and an offline render |
| `ahas-video-shotcraft/` | Making a video with the video-shotcraft studio (Remotion), from the setup to the render and the licence lines |
| `bmo-asking/` | Asking the supervisor what a job cannot settle itself, and filing a document for the owner |
| `bmo-beyond-the-checkout/` | What a job owns once its work reaches past its own checkout |
| `bmo-closing-a-job/` | How a job ends: the close, what it may leave and the report |
| `bmo-code-korean-out/` | Taking the Korean out of a bundle of code files |
| `bmo-docs-conventions/` | Where and how new information is recorded in a repository |
| `bmo-docs-eli5/` | Explaining a topic in pictures, as simply as it goes |
| `bmo-docs-plain-writing/` | Writing any text a person reads so it does not read as machine-written |
| `bmo-open-source-audit/` | What a public repository never carries, and the checks before a commit or a publication |
| `bmo-research-deep-dives/` | How a deep dive frames its question, reads a made thing's maker first and has its page reviewed |
| `bmo-research-economy-brief/` | Writing a brief of a day's economic news and figures |
| `bmo-research-investment/` | Answering where to put money: strategies, picks, entry and exit |
| `bmo-research-kinds/` | What any search checks and attaches by kind of question, and the shape of its result |
| `bmo-research-products/` | Finding a thing to buy: the seller order, the line's news, stock, real discounts and landed prices |
| `bmo-research-references/` | Gathering reference pictures for a drawing into a dossier |
| `bmo-research-restaurants/` | Recommending places to eat or food to order, by the asker's own rule or the default rule |
| `bmo-research-routine/` | Researching, measuring and reporting on a thing somebody made |
| `bmo-research-security-advisories/` | Answering a vulnerability or an advisory: exposure, fix and patch steps |
| `bmo-research-study-material/` | Writing study material that teaches a subject |
| `bmo-tools-researching-and-benching/` | Researching, benching or tuning something the lab runs |
| `bmo-ux-wording/` | Where the console's UI wording guide lives, how a term is added and how a use is fixed |
| `bmo-waiting-on-long-work/` | Starting work that outlasts a few minutes in the background and waiting on it with one bounded wait |
| `bmo-warm-prefix/` | Changing what a running session holds as its cached prefix |
| `src/` | The `bmo-skills` placer: reading the skills, listing them and placing them |
| `tests/` | The placer's tests and the checks that hold the skills to their format |
| `docs/` | The canonical statements of the format, the boundary and the inventory |

At the root stand the Cargo manifest, the version, `testbed.toml` with its Korean ledger, `rustfmt.toml`,
`LICENSE` and `CLAUDE.md`. The repository is MIT-licensed.

## Documents

| Document | States |
|---|---|
| `docs/format.md` | The shape of a skill folder, its front matter and its bundle, and what a skill may carry |
| `docs/boundary.md` | The placer's verbs, the manifest, which unit receives which skills from where, what each skill calls on another unit, and the version rule |
| `docs/inventory.md` | Every file with its source and place, what was left to other units, the tests ported and every name replaced |
| `docs/PERSONAL-DATA.md` | What personal data never goes into the repository, where a value lives instead, the two allowed values, and the check that holds it out |

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
