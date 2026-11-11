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
bmo-skills place --bundle worker|coder|producer --into <dir> [--from <root>]
```

- Writes the bmo-worker plugin into `<dir>` in Claude Code's standard plugin layout: the manifest at
  `.claude-plugin/plugin.json`, and every skill of the bundle named at `skills/<name>/`, each file of
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

- **The plugin is named `bmo-worker`,** and every skill of any placed bundle is called as
  `bmo-worker:<name>`.
- **Its version is the repository's `VERSION`.**
- **Its licence is `MIT`,** the repository's, stated in the manifest beside the root `LICENSE`.
- **It names no skill folder,** since a placement puts the skills in the standard layout's own folder.

## Who receives what

| Consumer | What it receives | From where | How it reads it |
|---|---|---|---|
| bmo-agency | the `worker`, `coder` and `producer` bundles | its own plugin directories, each placed with `place --bundle worker|coder|producer` at its deployment | each line hands its directory to Claude Code as the plugin; its prefix digest reads the manifest and every `skills/<name>/SKILL.md` |
| every general job worker | the `worker` bundle | the agency's worker plugin directory, through its launch line | Claude Code lists the skills to the job, and a job whose line shows no listing reads them at the skills path its trailer names |
| a coder's run | the `coder` bundle | the agency's coder plugin directory, through its launch line | its coding and design work reads those skills |
| a producer's run | the `producer` bundle | the agency's producer plugin directory, through its launch line | its video work reads those skills |
| bmo-job | nothing placed | | its closing policy takes the word `ahas-ux-design-rules` in a report as the mark that a changed screen was swept against the design rules |
| bmo-song | `bmo-song-cover` | its own checkout's `.claude/skills/bmo-song-cover/`, placed with `place --skill bmo-song-cover` | a song job reads it as the skill its item names; worker launch lines load no checkout's project skills, so it is read as a file at that path |
| the agency's chat procedures | nothing placed | | the song procedure names `bmo-song-cover` when it files a song item, and the TODO procedure names the builder's three skills |
| chat sessions and gates | none | | |

A consumer that keeps a skill's name in its own text or policy relies on that name: renaming or removing
a skill is a major change (「Versions」).

## What each skill calls on another unit

A skill reaches another unit only through words it tells a job to run or to read. A `bmo` command is
bmo-cli's, which hands it to the unit that serves it, as bmo-cli's own boundary document states: `job
ask`, `job borrow`, `job wait`, `job close` and `todo` go to bmo-job; `archive doc` to
bmo-artifact; `check` and `cargo` to bmo-testbed; `reload` to bmo-agency; `graph`, `query` and `voice
brief` stand there as not served yet. The graph tools a job calls are the agency's worker tools.

| Skill | What it calls | The unit that answers | Skills it names |
|---|---|---|---|
| `ahas-code-boundaries` | | | |
| `ahas-code-clean` | | | `ahas-code-design-patterns`, `ahas-code-refactoring` |
| `ahas-code-design-patterns` | | | `ahas-code-clean`, `ahas-code-refactoring` |
| `ahas-code-refactoring` | | | `ahas-code-clean`, `ahas-code-design-patterns` |
| `ahas-code-test-writing` | | | |
| `ahas-docs-git` | the checkout's own `AGENTS.md`, `CHANGELOG.md` and `README.md` | the checkout | |
| `ahas-tools-find-skills` | the skills.sh directory and its `npx skills` commands; this repository's rule for writing a skill | outside every unit; this repository | |
| `ahas-tools-playwright-cli` | the `playwright-cli` program, `npx` and `npm` | outside every unit | |
| `ahas-ux-design-rules` | | | `ahas-ux-nuxt-ui`, `bmo-ux-wording` |
| `ahas-ux-nuxt-style` | | | `ahas-code-refactoring` |
| `ahas-ux-nuxt-ui` | the Nuxt UI MCP server's `search-components`, `search-composables` and `search-icons`; the launch line document that says which line carries it; the outside documentation sites it links | outside every unit, handed over by the agency's launch line; bmo-agency; outside every unit | `bmo-warm-prefix` |
| `bmo-asking-the-owner` | `job ask`, both to ask and to answer a talk; `archive doc`; `brief-attach`; the deep dive rule file the agency deploys | bmo-job; bmo-artifact; the unit command of that name; bmo-agency | `bmo-closing-a-job`, `bmo-docs-conventions`, `bmo-tools-researching-and-benching` |
| `bmo-beyond-the-checkout` | `job borrow`; the close's `--unfinished`; the checkout's own `CLAUDE.md`; the owner's infrastructure repository | bmo-agency; bmo-job; the checkout; outside every unit | |
| `bmo-closing-a-job` | `graph alias`; the graph tool `graph_impact`; `todo` with `--done`, `--stop-here` and `--add`; the close and the suite it runs; the document reload; `STRUCTURE.md` | bmo-graph, not served yet; bmo-agency's worker tools; bmo-job; bmo-job; bmo-agency; the umbrella repository | `bmo-beyond-the-checkout`, `bmo-tools-researching-and-benching`, `ahas-ux-design-rules` |
| `bmo-code-korean-out` | `check korean` and its `--settle`; `check quick`; `cargo clippy` and `cargo fmt`; the language pack and its document | bmo-testbed; bmo-testbed; bmo-testbed; bmo-web | `bmo-closing-a-job` |
| `bmo-docs-conventions` | the checkout's own `AGENTS.md`, `CHANGELOG.md`, `CLAUDE.md` and `.claude/rules/` | the checkout | `bmo-asking-the-owner` |
| `bmo-docs-eli5` | `archive doc --html` | bmo-artifact | |
| `bmo-song-cover` | the song bench, its commands and its canonical document | bmo-song | `bmo-asking-the-owner` |
| `bmo-tools-researching-and-benching` | `job ask`; `archive doc`; `brief-attach`; the brain's knowledge chunk on the compute nodes; a card's hold through its node's arbiter; the deep dive rule file the agency deploys | bmo-job; bmo-artifact; the unit command of that name; bmo-brain; bmo-arbiter; bmo-agency | `bmo-asking-the-owner` |
| `bmo-ux-wording` | bmo-web's `WORDING.md` at that repository's root and the console's Korean pack, which the guide names | bmo-web | |
| `bmo-warm-prefix` | the launch lines and the document reload | bmo-agency | |

## Words other units match

The skills keep these exactly as written, since another unit reads them as written.

- **The close's strand marks,** read by bmo-job: `미결`, and the two marks that open each strand of one,
  `비모 몫:` and `하스 몫:`.
- **The item states and the stop mark the close and the hold verbs read,** bmo-job's: `재개 대기`, `중지`
  and `[멈춤 요청]`.
- **The skill name bmo-job's closing policy reads in a report,** `ahas-ux-design-rules`, as 「Who receives
  what」 says.
- **The command forms** of every `bmo` command in the table above, as bmo-cli and the unit behind it read
  them. A skill names each by the bare `bmo`, the entry point bmo-cli links onto `PATH`.

## Paths named as they stand

Three paths stand in the skills literally, so the units behind them keep them reachable:

- **bmo-web's `WORDING.md` at that repository's root,** the UI wording guide `bmo-ux-wording` sends a job
  to read and to grow; that repository keeps it by hand.
- **`~/.local/share/bmo-agency/deploy/prompts/rules/deep-dives.md`,** the deep dive rule file in the
  agency's own deploy tree.
- **`~/.local/share/bmo-script/songs/`,** where the song bench keeps each song's work folder, which
  `bmo-song-cover` names.

## Versions

`VERSION` holds the repository's version in semantic versioning, and the manifest carries the same.

- **Major:** a skill renamed or removed, a skill moved to another bundle, a word another unit matches
  changed, or a change to the `bmo-skills` grammar, its exit codes or the fields of its JSON.
- **Minor:** a skill added, or a field added to the JSON.
- **Patch:** a skill's text changed, every name and matched word left as it was.
