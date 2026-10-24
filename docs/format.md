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

## Names

- **A skill's name opens with its scope, `bmo` or `ahas`, then its category,** then the skill's own
  words: lower-case words joined by dashes, as `ahas-code-clean` or `bmo-code-korean-out`. The scope
  tells a skill that works in any harness from one tied to this system, and the category says what kind
  of work it is for, so a listing falls into groups by itself and a reader knows both by the name alone.
- **The scopes are these two,** and a skill takes the one its use calls for:

| Scope | Holds |
|---|---|
| `bmo` | A skill that means something only where this system stands: a procedure of its own jobs, or work carried out through its commands, its archive, its services or its worker's texts |
| `ahas` | A skill whose rules hold in any harness and any project without this system: the owner's own standards and the outside tools used anywhere. A line naming how this system carries out one of its rules leaves it `ahas` |

- **The categories are these four,** and a skill takes the one its work belongs to:

| Category | Holds |
|---|---|
| `code` | How code is written, structured, tested and kept, in any project and any language |
| `ux` | How a user interface is designed, and the style and component library its code is written with |
| `docs` | How a document, an explanation or the git record is written |
| `tools` | Driving an outside program, or a kind of work carried out with one |

- **An `ahas` skill always takes a category.** A `bmo` skill takes one where its work belongs to one of
  them, and a procedure of this system's own jobs takes none: its words follow the scope, as
  `bmo-closing-a-job`.
- **A third scope or a fifth category is added here first,** with what it holds, and the structure
  check's lists with it; a skill in a scope and category already here changes nothing outside its
  folder.

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

## `ahas-ux-design-rules`

- **Its core stays under 500 lines.** Loading the skill loads `SKILL.md` whole, so the rules every screen
  answers to stand there, and the detail of each area stands in an area file beside it.
- **The core names every area file beside it,** one entry each under its section of the rules by area,
  and each area file opens by saying it is part of the skill. Every entry names a file that is there.
- **The UI wording guide is not an area file and is not carried here.** The core sends a job to the
  `bmo-ux-wording` skill before it writes Korean text; that skill says where the guide lives, how a term
  is added, and how a use is reported and fixed.

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
format: its `name`, a `description`, a `version` equal to the repository's `VERSION`, and the
repository's licence, `MIT`. It names no skill folder: the placer writes the bundle in Claude Code's
standard plugin layout, where the skills folder is found by default (`boundary.md`).
