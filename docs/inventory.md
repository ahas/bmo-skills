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
| Files read from the source tree | 71 | `git ls-tree` at the source commit, as above |
| of which worker skill files (`prompts/agents/worker/plugin/skills/` → `<skill>/`) | 68, in 20 folders | the same listing |
| of which the plugin manifest | 1 | the same listing |
| of which the project skill `song-cover` | 1 | the same listing |
| of which the rule for writing a skill | 1 | the same listing; it becomes text of this repository's own documents rather than a file of its own |
| Crates | 0 | #301: a data repository takes no code; the code this repository carries is its own placer and checks |
| Packages | 0 | #301, as above |
| Policy files | 0 | the reader trace of this item: no policy file is read for a skill; bmo-job's closing policy names the skill `ui-design-rules` and is a consumer, not taken (`boundary.md`) |
| Tests | 26 cases ported | the test trace of this item, 「Tests」 below |
| Service units | 0 | #301, as above |

## Worker skills

`prompts/agents/worker/plugin/skills/<skill>/` → `<skill>/` at this repository's root, 68 files in 20
folders, every relative path inside a folder unchanged. Each folder's `SKILL.md` gains its bundle,
`worker` (`format.md`).

| Skill | Files | Paths inside the folder |
|---|---|---|
| `asking-the-owner` | 1 | `SKILL.md` |
| `beyond-the-checkout` | 1 | `SKILL.md` |
| `clean-code` | 3 | `SKILL.md`, `references/principles.md`, `references/typescript.md` |
| `closing-a-job` | 1 | `SKILL.md` |
| `design-boundaries` | 1 | `SKILL.md` |
| `design-patterns` | 5 | `SKILL.md`, `references/behavioral.md`, `references/beyond-gof.md`, `references/creational.md`, `references/structural.md` |
| `docs-conventions` | 1 | `SKILL.md` |
| `eli5` | 1 | `SKILL.md` |
| `find-skills` | 1 | `SKILL.md` |
| `marking-a-wait` | 1 | `SKILL.md` |
| `moving-korean-out-of-code` | 1 | `SKILL.md` |
| `my-git-docs` | 5 | `SKILL.md`, `skill/agent-enforcement.md`, `skill/changelog.md`, `skill/documentation.md`, `skill/git-workflow.md` |
| `my-nuxt-style` | 6 | `SKILL.md`, `skill/agent-enforcement.md`, `skill/node-base.md`, `skill/typescript.md`, `skill/vue-nuxt.md`, `skill/vue-sfc-critical.md` |
| `nuxt-ui` | 15 | `SKILL.md`, `references/components.md`, `references/guidelines/component-selection.md`, `references/guidelines/conventions.md`, `references/guidelines/design-system.md`, `references/guidelines/forms.md`, `references/layouts/chat.md`, `references/layouts/dashboard.md`, `references/layouts/docs.md`, `references/layouts/editor.md`, `references/layouts/landing.md`, `references/recipes/auth.md`, `references/recipes/data-tables.md`, `references/recipes/navigation.md`, `references/recipes/overlays.md` |
| `playwright-cli` | 10 | `SKILL.md`, `references/element-attributes.md`, `references/playwright-tests.md`, `references/request-mocking.md`, `references/running-code.md`, `references/session-management.md`, `references/storage-state.md`, `references/test-generation.md`, `references/tracing.md`, `references/video-recording.md` |
| `refactoring` | 3 | `SKILL.md`, `references/catalog.md`, `references/smells.md` |
| `researching-and-benching` | 1 | `SKILL.md` |
| `test-writing` | 1 | `SKILL.md` |
| `ui-design-rules` | 9 | `SKILL.md`, `controls.md`, `floating.md`, `layout.md`, `lists.md`, `loading.md`, `motion.md`, `notices.md`, `wording.md` |
| `warm-prefix` | 1 | `SKILL.md` |

## The manifest, the project skill and the rule

| Old path | New place | What changes |
|---|---|---|
| `prompts/agents/worker/plugin/.claude-plugin/plugin.json` | `.claude-plugin/plugin.json` | the description names this repository instead of an agent; the version is the repository's (`boundary.md`) |
| `.claude/skills/song-cover/SKILL.md` | `song-cover/SKILL.md` | its bundle, `project`; names replaced (below); its bench and its document are named as the song unit's |
| `.claude/rules/skills.md` | the 「Writing a skill」 section of `CLAUDE.md`, and `format.md` | the rule's path scoping goes, since every folder here is a skill; names replaced (below) |

## Left to other units

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
| `prompts/agents/worker/graph.md` | bmo-agency |

bmo-agency also holds a copy of the plugin and its 68 skill files inside its own prompts tree. This
repository is their one owner; the agency receives the bundle as `boundary.md` says.

## Tests

### Ported here

26 cases of the agent documents' test file in the source tree (`bot/bot/agents.test.ts`) check what a
skill says. Several also check the worker's trailer; that half stays with bmo-agency, which owns the
trailer, and the skill half is ported to `tests/` under the same name. A case that expects a Korean
literal from a skill checks the same rule by an English anchor from the same passage instead, since no
test here carries Hangul; the literal each such case drops is named in the table.

| Old test name | Skills it reads | Korean literal dropped |
|---|---|---|
| sends a job to the worker's skills for the rules only some jobs meet, by their folder | `closing-a-job` | |
| makes the checking the job's own work | `closing-a-job` | |
| closes a job by handing its commit over, after the whole suite where the change reaches it | `closing-a-job` | |
| excuses only what no machine can reach, and still demands an account of it | `closing-a-job` | |
| reports nothing unchecked that a check could settle, and names only what is beyond reach | `closing-a-job` | |
| keeps a task handed to the owner out of 미결 | `closing-a-job` | |
| makes closing with no 미결 the ordinary ending rather than a blank field | `closing-a-job` | |
| keeps a row from becoming the other way out of finishing it | `closing-a-job` | |
| keeps a structural fix the work needs inside the job, and leaves only what it cannot reach | `closing-a-job` | |
| leaves no improvement behind as a proposal, and names what only the owner can give | `closing-a-job` | |
| sends the job to finish it in the same pass, or to raise a row, instead of writing a note | `closing-a-job` | |
| and keeps the measurements out of the prompt, where the rule alone belongs | `closing-a-job` | |
| sends a job that changed a screen back through the design rules before it closes | `closing-a-job` | |
| tells every job to ask the graph and to bring it up to date before the commit | `closing-a-job` | |
| and to write down the word the owner used where the graph does not carry it yet | `closing-a-job` | |
| the worker's own rules stand in the worker's own texts, and the skill says where each rule belongs | `docs-conventions` | |
| the restructuring passes are found by the owner's words for the work, and skipped where nothing is restructured | `design-patterns` | |
| the warm-prefix skill gives a job the launch line its item needs, and what a warm document costs to edit | `warm-prefix` | |
| and the worker's beyond-the-checkout skill carries the same rule from its own side | `beyond-the-checkout` | |
| with the estate to be read rather than remembered, and no credential of its own | `beyond-the-checkout` | |
| the rules only some jobs meet stand in skills of their own, each saying when it is read | `asking-the-owner`, `researching-and-benching`, `marking-a-wait`, `test-writing`, `beyond-the-checkout`, `design-boundaries`, `refactoring`, `clean-code` | |
| the design rules carry the sweep the closing skill sends a job back to | `ui-design-rules` | |
| the design rules keep their core under 500 lines and name every area file beside it | `ui-design-rules` | |
| the three restructuring skills each name the order and their place in it | `design-patterns`, `refactoring`, `clean-code` | |
| and they are carried by a plugin the launch line can name | the manifest | |
| a skill copied from elsewhere keeps its origin and licence beside it | `eli5` | |

### Left to the units that own the mechanism

| Old test | What it checks | Unit |
|---|---|---|
| the two prefix tests of the server's launch line prefix module | the prefix digest's reading of the plugin folder and its prompt-shaping paths | bmo-agency, which already holds them |
| a skills line is read as a list, and a document without one carries none | the co-worker definition reader | bmo-agency |
| and the builder is told the same order it is handed the three skills in | the builder's definition | bmo-agency |
| and the skills rule is the one place that states what a skill carries and where its material comes from | the source tree's scoped rule files | none: the rule is text of this repository's own documents |
| every rule scoped to part of the tree names its paths, and AGENTS.md names every rule | the source tree's scoped rule files and its AGENTS.md | none: the source tree's own layout |
| the repository wording check's test (`repo/checks/wording.test.ts`) | the wording check's targets, one of which is `ui-design-rules/wording.md` left out | not this unit: the wording check is the testbed's mechanism, and this repository declares none (`format.md`) |
| the markup formatter's test (`markup/markup/format.test.ts`) | formatting of a sample reply that mentions skill folders | not this unit: the markup library |

## Names

A per-file regex sweep over the 71 files for person names, agent names, machine names, account-shaped
tokens, home paths, internal addresses and room ids. 16 files carry one or more, totalling 88 of the
owner's name, 62 of the chat agent's name in Latin letters, 11 of the same name in Hangul, one machine
name, one machine pool name, one home path and one account-shaped package scope. No IP address, room id
or other machine name occurs; a second sweep for node-shaped tokens found the pool name and nothing
else.

| File | Owner's name | Agent name, Latin | Agent name, Hangul | Other |
|---|---|---|---|---|
| `.claude/rules/skills.md` | | 2 | | |
| `.claude/skills/song-cover/SKILL.md` | 13 | 21 | | a machine name, 1 |
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
| `my-nuxt-style/skill/vue-nuxt.md` | | | | an account-shaped package scope, 1 |
| `researching-and-benching/SKILL.md` | 3 | 4 | | a machine pool name, 1 |
| `ui-design-rules/SKILL.md` | 2 | | | |
| `ui-design-rules/wording.md` | 39 | 5 | | |

- **What replaces them.** The owner's name becomes the owner; an agent acting becomes the chat agent;
  an agent name standing for the whole system becomes the system; the machine name becomes a neutral
  example host, and the machine pool name the GPU node pool; the home path and the package scope become a description of the thing they point at.
- **What stays as written.** The keywords a process matches on: 미결 and the two strand marks that
  open each of its strands, which bmo-job's close reads (`boundary.md`). The 15 deploy paths under `~/.local/share/bmo`, which
  name the installed `bmo` entry point and an agent rule file, and carry no account or machine name.
