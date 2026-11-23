# Inventory

What this repository takes from the single repository it is split out of, where each file goes, and
what is changed on the way in.

## Source

- **The source tree** is the single repository this unit is split out of, at commit `3e4c032da`.
  Every file below is listed by
  `git ls-tree -r --name-only 3e4c032da prompts/agents/worker/plugin .claude/skills .claude/rules/skills.md`,
  which answers 71 paths.
- **The split decision** is archive document #301: it places this unit among the data repositories, the
  skill bundle, one repository with one folder per skill and no process of its own.
- **The working branch of the earlier restructuring** (`work/1703`) changes one of these files in one
  commit: a single example path in `moving-korean-out-of-code`, naming where a test file of the old
  tree moved inside that tree. It names a path of another unit's tree and is not taken.

## Counts

| What | Count | Source |
|---|---|---|
| Files taken from the source tree | 70 | `git ls-tree` at the source commit answers 71 paths; the design rules' wording link is not carried (「Worker skills」 below) |
| of which worker skill files (`prompts/agents/worker/plugin/skills/` → `<skill>/`) | 67, in 20 folders | the same listing, the wording link left out |
| of which the plugin manifest | 1 | the same listing |
| of which the project skill `song-cover`, `bmo-song-cover`, which the song unit's repository now holds | 1 | the same listing |
| of which the rule for writing a skill | 1 | the same listing; it becomes text of this repository's own documents rather than a file of its own |
| The UI wording guide | 0 carried | bmo-web's `WORDING.md` at that repository's root, read through `bmo-ux-wording`; this repository holds no copy (「Worker skills」 below) |
| Crates | 0 | #301: a data repository takes no code; the code this repository carries is its own placer and checks |
| Packages | 0 | #301, as above |
| Policy files | 0 | the reader trace of this item: no policy file is read for a skill; bmo-job's closing policy names the design rules skill, here `ahas-ux-design-rules`, and is a consumer, not taken (`boundary.md`) |
| Tests | 24 cases ported, 2 left with the trailer | the test trace of this item, 「Tests」 below |
| Service units | 0 | #301, as above |

## Worker skills

`prompts/agents/worker/plugin/skills/<skill>/` → a folder at this repository's root, 67 files in 20
folders, every relative path inside a folder unchanged. Each skill takes its scope and category first in
its name (`format.md`, 「Names」), and its folder with it, as the table below pairs them; this document
names a skill by its source-tree name wherever it speaks of the source tree. Each folder's `SKILL.md`
gains its bundle, `worker` (`format.md`), and the names below are replaced. Where a skill names a file
of another unit, it names that unit with it: the language pack and its document are bmo-web's, the Korean ledger's check
is bmo-testbed's, the launch lines and the worker's trailer are bmo-agency's, and the skills themselves
are this repository's.

- **The design rules carry no wording file.** The UI wording guide is bmo-web's `WORDING.md` at that
  repository's root, kept by hand; this repository holds no copy. The `bmo-ux-wording` skill says where
  the guide stands, how a term is added, and how a use is reported and fixed.

| Skill in the source tree | Folder here | Files | Paths inside the folder |
|---|---|---|---|
| `asking-the-owner` | `bmo-asking` | 1 | `SKILL.md` |
| `beyond-the-checkout` | `bmo-beyond-the-checkout` | 1 | `SKILL.md` |
| `clean-code` | `ahas-code-clean` | 3 | `SKILL.md`, `references/principles.md`, `references/typescript.md` |
| `closing-a-job` | `bmo-closing-a-job` | 1 | `SKILL.md` |
| `design-boundaries` | `ahas-code-boundaries` | 1 | `SKILL.md` |
| `design-patterns` | `ahas-code-design-patterns` | 5 | `SKILL.md`, `references/behavioral.md`, `references/beyond-gof.md`, `references/creational.md`, `references/structural.md` |
| `docs-conventions` | `bmo-docs-conventions` | 1 | `SKILL.md` |
| `eli5` | `bmo-docs-eli5` | 1 | `SKILL.md` |
| `find-skills` | `ahas-tools-find-skills` | 1 | `SKILL.md` |
| `marking-a-wait` | `bmo-marking-a-wait` | 1 | `SKILL.md` |
| `moving-korean-out-of-code` | `bmo-code-korean-out` | 1 | `SKILL.md` |
| `my-git-docs` | `ahas-docs-git` | 5 | `SKILL.md`, `skill/agent-enforcement.md`, `skill/changelog.md`, `skill/documentation.md`, `skill/git-workflow.md` |
| `my-nuxt-style` | `ahas-ux-nuxt-style` | 6 | `SKILL.md`, `skill/agent-enforcement.md`, `skill/node-base.md`, `skill/typescript.md`, `skill/vue-nuxt.md`, `skill/vue-sfc-critical.md` |
| `nuxt-ui` | `ahas-ux-nuxt-ui` | 15 | `SKILL.md`, `references/components.md`, `references/guidelines/component-selection.md`, `references/guidelines/conventions.md`, `references/guidelines/design-system.md`, `references/guidelines/forms.md`, `references/layouts/chat.md`, `references/layouts/dashboard.md`, `references/layouts/docs.md`, `references/layouts/editor.md`, `references/layouts/landing.md`, `references/recipes/auth.md`, `references/recipes/data-tables.md`, `references/recipes/navigation.md`, `references/recipes/overlays.md` |
| `playwright-cli` | `ahas-tools-playwright-cli` | 10 | `SKILL.md`, `references/element-attributes.md`, `references/playwright-tests.md`, `references/request-mocking.md`, `references/running-code.md`, `references/session-management.md`, `references/storage-state.md`, `references/test-generation.md`, `references/tracing.md`, `references/video-recording.md` |
| `refactoring` | `ahas-code-refactoring` | 3 | `SKILL.md`, `references/catalog.md`, `references/smells.md` |
| `researching-and-benching` | `bmo-tools-researching-and-benching` | 1 | `SKILL.md` |
| `test-writing` | `ahas-code-test-writing` | 1 | `SKILL.md` |
| `ui-design-rules` | `ahas-ux-design-rules` | 8 | `SKILL.md`, `controls.md`, `floating.md`, `layout.md`, `lists.md`, `loading.md`, `motion.md`, `notices.md` |
| `warm-prefix` | `bmo-warm-prefix` | 1 | `SKILL.md` |

**The UI wording skill has no source-tree folder.** `bmo-ux-wording` is written here, one file, `SKILL.md`,
and it names bmo-web's `WORDING.md` as the guide a job reads, follows, adds a term to and fixes a use
from.

## The manifest, the project skill and the rule

| Old path | New place | What changes |
|---|---|---|
| `prompts/agents/worker/plugin/.claude-plugin/plugin.json` | `.claude-plugin/plugin.json` | the description names this repository instead of an agent; the version is the repository's (`boundary.md`) |
| `.claude/skills/song-cover/SKILL.md` | the song unit's repository, `skills/bmo-song-cover/SKILL.md`; none here | its bundle, `project`; names replaced (below); its bench and its document are named as the song unit's |
| `.claude/rules/skills.md` | the 「Writing a skill」 section of `CLAUDE.md`, and `format.md` | the rule's path scoping goes, since every folder here is a skill; names replaced (below) |

## Research skills

The research unit's rulebooks, taken from bmo-research's `prompts/rules/` at its commit `485fa9c`, 11
files, each a skill of the `worker` bundle in the `research` category, or `docs` for plain writing. Each
`SKILL.md` gains its front matter, and its body is the rulebook's text with what the table and the list
below change. bmo-research keeps the platform's rule against destroying the system under its own
`prompts/rules/`, which is no research skill.

| Old path in bmo-research | New place |
|---|---|
| `prompts/rules/search-kinds.md` | `bmo-research-kinds/SKILL.md` |
| `prompts/rules/product-search.md` | `bmo-research-products/SKILL.md` |
| `prompts/rules/restaurant-search.md` | `bmo-research-restaurants/SKILL.md` |
| `prompts/rules/investment-search.md` | `bmo-research-investment/SKILL.md` |
| `prompts/rules/study-material.md` | `bmo-research-study-material/SKILL.md` |
| `prompts/rules/deep-dives.md` | `bmo-research-deep-dives/SKILL.md` |
| `prompts/rules/research.md` | `bmo-research-routine/SKILL.md` |
| `prompts/rules/security-advisory.md` | `bmo-research-security-advisories/SKILL.md` |
| `prompts/rules/economy-brief.md` | `bmo-research-economy-brief/SKILL.md` |
| `prompts/rules/references.md` | `bmo-research-references/SKILL.md` |
| `prompts/rules/plain-writing.md` | `bmo-docs-plain-writing/SKILL.md` |

- **A rulebook named by its file** is named by its skill's name, and the chat agent's answer-shape
  rulebook, which no skill carries, by its description.
- **The owner's personal restaurant checklist leaves the skill.** Its five checks, the price limit among
  them, are his private data, a rule of his own in the research unit's people's rules under the data root
  (bmo-research's `docs/settings.md`). The skill states the rule's shape instead: a person may keep a
  restaurant rule of their own, the asker's own stands where they keep one, and the default rule, the
  four checks without a price limit, stands otherwise.
- **A preference told as the owner's** is told as no one's: what products are compared on, how a reader
  judges places by setting like against like, and whom the economy brief is gathered for.


The other 9 files of `prompts/agents/worker/` in the source tree are the agent engine's, which already
holds them: they define the worker and its co-workers, not a skill.

| Old path | Unit |
|---|---|
| `prompts/agents/worker/coworkers/builder.md` | bmo-agency |
| `prompts/agents/worker/coworkers/digest.md` | bmo-agency |
| `prompts/agents/worker/coworkers/scout.md` | bmo-agency |
| `prompts/agents/worker/coworkers/screener.md` | bmo-agency |
| `prompts/agents/worker/coworkers/verifier.md` | bmo-agency |
| `prompts/agents/worker/settings.json` | bmo-agency |
| `prompts/agents/worker/trailer.md` | bmo-agency |
| `prompts/agents/worker/standing-rules.md` | bmo-agency |

bmo-agency also holds a copy of the plugin and its 68 skill files inside its own prompts tree. This
repository is their one owner; the agency receives the bundle as `boundary.md` says.

## Tests

### Ported here

26 cases of the agent documents' test file in the source tree (`bot/bot/agents.test.ts`) read the
worker's trailer, its skills or both. The 24 that read a skill are ported to `tests/` under the same name,
their skill passages as the name map leaves them; the trailer's half of a case stays with bmo-agency,
which owns the trailer, and so do the two that read the trailer alone (「Left to the units that own the
mechanism」). A case that expects a Korean literal from a skill checks the same rule by an English anchor
from the same passage, since no test here carries Hangul; the literal each such case drops is named in the
table. A test name holding a Korean word takes the English for it as its function name.

| Old test name | Skills it reads | Korean literal dropped |
|---|---|---|
| makes the checking the job's own work | `bmo-closing-a-job` | |
| closes a job by handing its commit over, after the whole suite where the change reaches it | `bmo-closing-a-job` | |
| excuses only what no machine can reach, and still demands an account of it | `bmo-closing-a-job` | the quoted request to have it checked, which the skill forbids; the passage's opening sentence stands for it |
| reports nothing unchecked that a check could settle, and names only what is beyond reach | `bmo-closing-a-job` | |
| keeps a task handed to the owner out of 미결 | `bmo-closing-a-job` | 미결, opening the passage; the rest of its sentence is held |
| makes closing with no 미결 the ordinary ending rather than a blank field | `bmo-closing-a-job` | |
| keeps a row from becoming the other way out of finishing it | `bmo-closing-a-job` | |
| keeps a structural fix the work needs inside the job, and leaves only what it cannot reach | `bmo-closing-a-job` | |
| leaves no improvement behind as a proposal, and names what only the owner can give | `bmo-closing-a-job` | 미결, inside the refusal's sentence; the sentence is held on either side of it |
| sends the job to finish it in the same pass, or to raise a row, instead of writing a note | `bmo-closing-a-job` | |
| sends a job that changed a screen back through the design rules before it closes | `bmo-closing-a-job` | |
| the worker's own rules stand in the worker's own texts, and the skill says where each rule belongs | `bmo-docs-conventions` | |
| the restructuring passes are found by the owner's words for the work, and skipped where nothing is restructured | `ahas-code-design-patterns` | the owner's three Korean words for the work; the clause they stand in is held from its English end |
| the warm-prefix skill gives a job the launch line its item needs, and what a warm document costs to edit | `bmo-warm-prefix` | |
| and the worker's beyond-the-checkout skill carries the same rule from its own side | `bmo-beyond-the-checkout` | |
| with the estate to be read rather than remembered, and no credential of its own | `bmo-beyond-the-checkout` | |
| the rules only some jobs meet stand in skills of their own, each saying when it is read | `bmo-asking`, `bmo-tools-researching-and-benching`, `bmo-marking-a-wait`, `ahas-code-test-writing`, `bmo-beyond-the-checkout`, `ahas-code-boundaries`, `ahas-code-refactoring`, `ahas-code-clean` | the two Korean rules of `ahas-code-boundaries`; their section's English heading stands for them |
| the design rules carry the sweep the closing skill sends a job back to | `ahas-ux-design-rules` | |
| the design rules keep their core under 500 lines and name every area file beside it | `ahas-ux-design-rules` | |
| the three restructuring skills each name the order and their place in it | `ahas-code-design-patterns`, `ahas-code-refactoring`, `ahas-code-clean` | |
| and they are carried by a plugin the launch line can name | the manifest | |
| a skill copied from elsewhere keeps its origin and licence beside it | `bmo-docs-eli5` | |

Beside them, the repository's own checks hold the rest of `format.md`: every name's scope and category, every
skill's description, the manifest's version against `VERSION`, every file of a skill named by its
`SKILL.md` and every file it names there, every skill a skill names, and no personal value by its shape.

### Left to the units that own the mechanism

| Old test | What it checks | Unit |
|---|---|---|
| the two prefix tests of the server's launch line prefix module | the prefix digest's reading of the plugin folder and its prompt-shaping paths | bmo-agency, which already holds them |
| sends a job to the worker's skills for the rules only some jobs meet, by their folder | the trailer's pointer to the skills folder | bmo-agency, which owns the trailer |
| and keeps the measurements out of the prompt, where the rule alone belongs | the trailer's figures | bmo-agency, which owns the trailer |
| a skills line is read as a list, and a document without one carries none | the co-worker definition reader | bmo-agency |
| and the builder is told the same order it is handed the three skills in | the builder's definition | bmo-agency |
| and the skills rule is the one place that states what a skill carries and where its material comes from | the source tree's scoped rule files | none: the rule is text of this repository's own documents |
| every rule scoped to part of the tree names its paths, and AGENTS.md names every rule | the source tree's scoped rule files and its AGENTS.md | none: the source tree's own layout |
| the repository wording check's test (`repo/checks/wording.test.ts`) | the wording check's targets in the source tree | not this unit: the wording check is the testbed's mechanism, and this repository declares none (`format.md`) |
| the markup formatter's test (`markup/markup/format.test.ts`) | formatting of a sample reply that mentions skill folders | not this unit: the markup library |

## Names

A per-file regex sweep over the 70 files this repository carries from the source tree for person names,
agent names, machine names, project names, account-shaped tokens, home paths, internal addresses and room
ids, run again without regard to case and for node-shaped tokens. 18 files carry one or more, totalling
49 of the owner's name, 57 of the chat agent's name in Latin letters, 11 of the same name in Hangul, one
of another agent's name inside an engine's name, two of the owner's account name in a heading, one
machine name, one machine pool name, one project name standing for a kind of node, one home path and one
account-shaped package scope. No IP address, room id or other machine name occurs.

| File in the source tree | Owner's name | Agent name, Latin | Agent name, Hangul | Other |
|---|---|---|---|---|
| `.claude/rules/skills.md` | | 2 | | |
| `.claude/skills/song-cover/SKILL.md` | 13 | 21 | | a machine name, 1; a project name for a kind of node, 1 |
| `prompts/agents/worker/plugin/.claude-plugin/plugin.json` | | 2 | | |
| `asking-the-owner/SKILL.md` | 2 | 4 | | |
| `beyond-the-checkout/SKILL.md` | 3 | 2 | 5 | a home path, 1 |
| `closing-a-job/SKILL.md` | 20 | 14 | 6 | |
| `design-boundaries/SKILL.md` | 2 | | | |
| `design-patterns/SKILL.md` | 2 | | | |
| `docs-conventions/SKILL.md` | | 2 | | |
| `eli5/SKILL.md` | | 1 | | |
| `marking-a-wait/SKILL.md` | | 4 | | |
| `moving-korean-out-of-code/SKILL.md` | 2 | 1 | | |
| `my-git-docs/SKILL.md` | | | | the owner's account name in its heading, 1 |
| `my-nuxt-style/SKILL.md` | | | | the owner's account name in its heading, 1 |
| `my-nuxt-style/skill/vue-nuxt.md` | | | | an account-shaped package scope, 1 |
| `refactoring/SKILL.md` | | | | another agent's name inside an engine's name, 1 |
| `researching-and-benching/SKILL.md` | 3 | 4 | | a machine pool name, 1 |
| `ui-design-rules/SKILL.md` | 2 | | | |

- **What replaces them.** The owner's name becomes the owner, and 오너 in a Korean sentence; an agent
  acting becomes the chat agent; an agent name standing for the whole system becomes the system; the
  machine name becomes a neutral example host, the machine pool name the GPU node pool, and the node
  kind a node; the home path and the package scope become a description of the thing they point at;
  the account name leaves the two headings; the engine's name becomes the neutral `local-llm`.
- **The agent engine's copy as the model.** bmo-agency's copy of the worker skills had already replaced
  these names in 17 of the files; its wording is taken as it stands, but for the sentences where it
  named its own repository, which name the repository that owns each thing instead.
- **What stays as written.** The keywords a process matches on: 미결 and the two strand marks that
  open each of its strands, which bmo-job's close reads (`boundary.md`). The 12 paths of the installed
  `bmo` entry point, which carry no account or machine name.
  The deep dive rule path gives way to the research skills' names (「Research skills」).
