# Format

The shape a skill takes in this repository. The repository's tests hold every skill to it, and the
placer reads it, so a consumer that keeps to `boundary.md` receives each skill the way the source tree's
readers did.

## One folder per skill

- **Every skill is one folder at the repository's root,** named by the skill. There is no second level of
  folders holding the skills: the folder beside `docs/` is the skill.
- **The folder holds `SKILL.md`** and whatever else the skill is made of, under any relative path inside
  it. A folder at the root without a `SKILL.md` is not a skill; the root's other folders are `docs/`,
  `src/`, `tests/` and `.claude-plugin/`.
- **A file inside a folder is taken whole.** Consumers receive every file of the folder byte for byte, so
  a skill's own text decides what it is, and nothing outside the folder adds to it.

## `SKILL.md`

A markdown file opening with a YAML front matter block between two lines of three dashes. The body is
the skill's rules, read whole when the skill is loaded.

| Key | Required | Meaning |
|---|---|---|
| `name` | yes | The skill's name, equal to its folder's name; consumers call it by this name |
| `description` | yes | When the skill is read: the situation it is for and what it covers. It is what an agent sees before loading the skill, so it names the situation rather than summarising the rules |
| `bundle` | yes | Which consumers receive the skill (「Bundles」 below): `worker` or `project` |
| `version` | no | The skill's own version, in semantic versioning, raised when its rules change |
| `license` | only for a skill copied from elsewhere | The licence and the origin: the maker, the package and the place it was copied from. A skill written here carries none |
| `allowed-tools` | no | The tools the skill may use without asking, in Claude Code's own form |

No other key is read by this repository; Claude Code reads the ones it knows and passes over the rest.

## Bundles

A skill states the one bundle it belongs to, and nothing outside its folder lists it.

- **`worker`:** the bmo-worker plugin, which every job worker is given. Its skills are called as
  `bmo-worker:<name>`.
- **`project`:** a skill one repository's checkout receives as its own project skill, under its
  `.claude/skills/<name>/`. Which checkout receives it is that repository's own deployment's choice
  (`boundary.md`).

## Area and reference files

- **A skill's other files sit beside its `SKILL.md`,** at the folder's top or in folders of their own
  (`references/`, `skill/` and the like), and **`SKILL.md` names each one** by its path relative to the
  folder, saying when to read it. A file nothing names is never read.
- **A skill names another skill by its name,** and every skill so named exists in this repository.

## `ui-design-rules`

- **Its core stays under 500 lines.** Loading the skill loads `SKILL.md` whole, so the rules every screen
  answers to stand there, and the detail of each area stands in an area file beside it.
- **The core names every area file beside it,** one entry each under its section of the rules by area,
  and each area file opens by saying it is part of the skill. Every entry names a file that is there.
- **`wording.md` is no area file:** it is the list of banned terms and the one word each screen concept
  takes, which the core sends a job to read before it writes Korean text.

## What a skill carries

- **The rules it teaches, and nothing about where they came from:** no source list, no bibliography and no
  licence in its body. A skill copied from elsewhere keeps its origin and licence in the `license` key
  alone, the one part no prompt loads whole.
- **Its own language.** A skill is content: one written in Korean stays Korean, and the keywords another
  unit matches on stay exactly as that unit matches them (`boundary.md`, 「Words other units match」).
- **No personal value and no agent name.** The owner is the owner, the chat agent is the chat agent, and a
  machine, an account, a home path or an internal address appears only as a neutral example.
- **An outside address is public by nature:** a name reserved for examples, or a public site, package
  scope or code keyword of the software the skill teaches. The personal-value check holds which those
  are, and refuses any other host, address, id or handle by its shape.

## The manifest

`.claude-plugin/plugin.json` is the bmo-worker plugin's manifest, in Claude Code's plugin manifest
format: its `name`, a `description` and a `version` equal to the repository's `VERSION`. It names no
skill folder: the placer writes the bundle in Claude Code's standard plugin layout, where the skills
folder is found by default (`boundary.md`).
