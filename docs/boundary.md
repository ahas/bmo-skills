# Boundary

What this repository offers the other units, how each of them receives it, and what each skill calls on
another unit. What is not here is internal and may change in any release.

## What this repository is

- **A data repository with one tool:** the skills, one folder each (`format.md`), the bmo-worker plugin's
  manifest, and the `bmo-skills` binary that places them. It runs no server, exposes no API and depends on
  no other repository.
- **No unit's code reads this repository or its paths.** A consumer receives the skills as a copy the
  `bmo-skills` binary places in that consumer's own tree. The copy is the consumer's deployment's step, run
  from a checkout of this repository at a tagged release; this repository's testbed declaration declares
  no deployment of its own.

## The `bmo-skills` binary

One binary, whose first word is the verb. Every verb reads the skills from the folder `--from` names, the
current directory where it is left out, and every line it prints is English.

### `list`

```
bmo-skills list [--json] [--from <root>]
```

- Prints one line per skill folder, in name order: the name and the bundle.
- **`--json`** prints one array instead, one object per skill: `name`, `bundle` and `files`, the folder's
  file paths relative to it, in order.

### `place --bundle`

```
bmo-skills place --bundle worker --into <dir> [--from <root>]
```

- Writes the bmo-worker plugin into `<dir>` in Claude Code's standard plugin layout: the manifest at
  `.claude-plugin/plugin.json`, and every skill whose bundle is `worker` at `skills/<name>/`, each file of
  its folder byte for byte. Nothing else stands in `<dir>` afterwards.
- `<dir>` is replaced whole, and only where it is absent, empty, or an earlier placement: a directory whose
  `.claude-plugin/plugin.json` names the plugin `bmo-worker`.
- Prints one line naming how many skills it placed and where.

### `place --skill`

```
bmo-skills place --skill <name> --into <dir> [--from <root>]
```

- Writes the one skill's folder at `<dir>/<name>/`, each file byte for byte; `<dir>` is made where it is
  absent, and nothing else in it is touched.
- `<dir>/<name>` is replaced only where it is absent, empty, or an earlier placement of that skill: a folder
  whose `SKILL.md` names it.
- Prints one line naming the skill and where it went.

### Placing

- **A placement is built beside its target and swapped in,** so a reader never sees half a bundle, and a
  placement that fails leaves the target as it was.
- **Every skill is read before anything is written.** A folder without a readable `SKILL.md`, a name that
  differs from its folder, or a missing or unknown bundle fails the verb, naming the folder.

### Exit codes

| Code | Meaning |
|---|---|
| 0 | Done |
| 1 | Failed: a skill that does not read as `format.md` says, or a write that did not go through; the line names it |
| 2 | Refused: words that do not read, a skill name with no folder, or a target that is neither absent, empty nor an earlier placement; nothing is written |

## The manifest

- **The plugin is named `bmo-worker`,** and every skill of the `worker` bundle is called as
  `bmo-worker:<name>`.
- **Its version is the repository's `VERSION`.**
- **It names no skill folder,** since a placement puts the skills in the standard layout's own folder.

## Who receives what

| Consumer | What it receives | From where | How it reads it |
|---|---|---|---|
| bmo-agency | the `worker` bundle | its own plugin directory, placed with `place --bundle worker` at its deployment | its worker launch line hands that directory to Claude Code as the plugin; its co-worker definitions read the manifest's name and the names under `skills/`, and keep only the preloaded skills the plugin carries; its prefix digest reads the manifest and every `skills/<name>/SKILL.md` |
| every job worker | the `worker` bundle | the agency's plugin directory, through its launch line | Claude Code lists the skills to the job, and a job whose line shows no listing reads them at the skills path its trailer names |
| the agency's `builder` co-worker | `design-patterns`, `refactoring`, `clean-code`, preloaded in that order | the same plugin | its definition names them as `bmo-worker:<name>` |
| bmo-job | nothing placed | | its closing policy takes the word `ui-design-rules` in a report as the mark that a changed screen was swept against the design rules |
| bmo-song | `song-cover` | its own checkout's `.claude/skills/song-cover/`, placed with `place --skill song-cover` | a song job reads it as the skill its item names; worker launch lines load no checkout's project skills, so it is read as a file at that path |
| the agency's chat procedures | nothing placed | | the song procedure names `song-cover` when it files a song item, and the TODO procedure names the builder's three skills |
| chat sessions and gates | none | | |

A consumer that keeps a skill's name in its own text or policy relies on that name: renaming or removing
a skill is a major change (「Versions」).

## What each skill calls on another unit

A skill reaches another unit only through words it tells a job to run or to read. A `bmo` command is
bmo-cli's, which hands it to the unit that serves it, as bmo-cli's own boundary document states: `job
ask`, `job borrow`, `job wait`, `job close`, `todo` and `work-order` go to bmo-job; `archive doc` to
bmo-artifact; `check` and `cargo` to bmo-testbed; `reload` to bmo-agency; `graph`, `query` and `voice
brief` stand there as not served yet. The graph tools a job calls are the agency's worker tools.

| Skill | What it calls | The unit that answers | Skills it names |
|---|---|---|---|
| `asking-the-owner` | `job ask`, both to ask and to answer a talk; `archive doc`; the deep dive rule file the agency deploys | bmo-job; bmo-artifact; bmo-agency | `closing-a-job`, `docs-conventions`, `researching-and-benching` |
| `beyond-the-checkout` | `job borrow`; the close's `--unfinished`; the checkout's own `CLAUDE.md`; the owner's infrastructure repository | bmo-job; bmo-job; the checkout; outside every unit | |
| `clean-code` | | | `design-patterns`, `refactoring` |
| `closing-a-job` | `graph alias`; the graph tool `graph_impact`; `todo` with `--done`, `--stop-here`, `--add` and `--resolve`; `work-order --todo`; the close and the suite it runs; the document reload; `STRUCTURE.md` | bmo-graph, not served yet; bmo-agency's worker tools; bmo-job; bmo-job; bmo-job; bmo-agency; the umbrella repository | `beyond-the-checkout`, `researching-and-benching`, `ui-design-rules` |
| `design-boundaries` | | | |
| `design-patterns` | | | `clean-code`, `refactoring` |
| `docs-conventions` | the checkout's own `AGENTS.md`, `CHANGELOG.md`, `CLAUDE.md` and `.claude/rules/` | the checkout | `asking-the-owner` |
| `eli5` | `archive doc --html` | bmo-artifact | |
| `find-skills` | the skills.sh directory and its `npx skills` commands; this repository's rule for writing a skill | outside every unit; this repository | |
| `marking-a-wait` | `job wait`, over a unit or a command | bmo-job | |
| `moving-korean-out-of-code` | `check korean` and its `--settle`; `check quick`; `cargo clippy` and `cargo fmt`; the language pack and its document | bmo-testbed; bmo-testbed; bmo-testbed; bmo-web | `closing-a-job` |
| `my-git-docs` | the checkout's own `AGENTS.md`, `CHANGELOG.md` and `README.md` | the checkout | |
| `my-nuxt-style` | | | `refactoring` |
| `nuxt-ui` | the Nuxt UI MCP server's `search-components`, `search-composables` and `search-icons`; the launch line document that says which line carries it; the outside documentation sites it links | outside every unit, handed over by the agency's launch line; bmo-agency; outside every unit | `warm-prefix` |
| `playwright-cli` | the `playwright-cli` program, `npx` and `npm` | outside every unit | |
| `refactoring` | | | `clean-code`, `design-patterns` |
| `researching-and-benching` | `job ask`; `archive doc` with its narration; `query`; `voice brief`; the brain's knowledge chunk on the compute nodes; a card's hold through its node's arbiter; the deep dive rule file the agency deploys | bmo-job; bmo-artifact; not served yet; not served yet; bmo-brain; bmo-arbiter; bmo-agency | `asking-the-owner`, `marking-a-wait` |
| `test-writing` | | | |
| `ui-design-rules` | | | `nuxt-ui` |
| `warm-prefix` | the launch lines and the document reload | bmo-agency | |
| `song-cover` | the song bench, its commands and its canonical document | bmo-song | `asking-the-owner`, `marking-a-wait` |

## Words other units match

The skills keep these exactly as written, since another unit reads them as written.

- **The close's strand marks,** read by bmo-job: `미결`, and the two marks that open each strand of one,
  `비모 몫:` and `하스 몫:`.
- **The item states and the stop mark the close and the hold verbs read,** bmo-job's: `재개 대기`, `중지`
  and `[멈춤 요청]`.
- **The command forms** of every `bmo` command in the table above, as bmo-cli and the unit behind it read
  them.

## Paths named as they stand

Three paths stand in the skills literally, so the units behind them keep them reachable:

- **`~/.local/share/bmo/deploy/bin/bmo`,** the installed entry point bmo-cli provides, which the skills
  run every `bmo` command through.
- **`~/.local/share/bmo-agency/deploy/prompts/rules/deep-dives.md`,** the deep dive rule file in the
  agency's own deploy tree.
- **`~/.local/share/bmo-script/songs/`,** where the song bench keeps each song's work folder, which
  `song-cover` names.

## Versions

`VERSION` holds the repository's version in semantic versioning, and the manifest carries the same.

- **Major:** a skill renamed or removed, a skill moved to another bundle, a word another unit matches
  changed, or a change to the `bmo-skills` grammar, its exit codes or the fields of its JSON.
- **Minor:** a skill added, or a field added to the JSON.
- **Patch:** a skill's text changed, every name and matched word left as it was.
