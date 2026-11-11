# Working in bmo-skills

These rules hold every change to this repository.

## The documents come first

- `docs/` is the canonical statement of what the repository holds and how it is read: `docs/format.md`
  for the shape of a skill and its front matter, `docs/boundary.md` for the placer, who receives which
  skills and what each skill calls on another unit, and `docs/inventory.md` for every file, where it comes
  from and what replaced its personal values.
- A change that would make a document wrong or incomplete repairs the document first, stating what is
  true now, and the skills, the placer or the checks are changed to match. A skill added, removed or
  renamed is added to, removed from or renamed in the README's folder table and the boundary's call
  table in the same commit.
- A document states what is true now: no change history, no rationale told through an earlier design,
  no pasted source, signature or config fragment, and no file-and-line pins.

## Boundaries

- **The repository is content and its placer, not a service.** It runs no server, exposes no API and
  calls no unit. No unit's code reads it by its path: a consumer's deployment runs the placer, which
  writes a bundle or a skill into the consumer's own tree (`docs/boundary.md`).
- **Dependencies:** registry crates, and nothing from another repository, the base libraries included.
  The testbed's `dependencies` check holds this.
- **No hand-written parser for a general format.** A skill's front matter is read by the front matter
  library, the manifest by the JSON library, the command line by clap; only the skills' own prose is
  read by pattern, in the checks.

## One folder per skill

- **Every skill is one folder at the root,** named by the skill, holding its `SKILL.md` and the files it
  names; its front matter states its bundle (`docs/format.md`).
- **A skill's whole is its own folder.** Adding one more skill changes its folder, the README's folder
  table and the boundary's call table, and nothing in `src/` or `tests/`.

## Writing a skill

- **A skill carries the rules it teaches and nothing about where they came from:** no source list, no
  bibliography, no licence in its body. A skill is read before its reader has been asked anything, so every
  line is paid for on every run that holds it. Researched material keeps its sources with the search, in its own rows and document,
  permanently and outside every prompt.
- **A skill copied whole from elsewhere keeps its origin and licence in its front matter alone,** the
  one part no prompt loads whole (`docs/format.md`).
- **A request to research a matter and make a skill of it is two steps, and the first is the search:**
  the searcher runs, and the skill is written out of the document that search wrote. A job whose item
  asks for a skill out of research writes it from the search's document in the 아카이브 the row names
  and opens no source of its own; reading the web itself and leaving only a skill behind is not this
  instruction carried out. A job handed no such document writes no skill from its own reading:
  launching a search is the chat agent's, in chat, so the job names it in its report as the chat
  agent's step and finishes the rest of the item around it.

## Language

- **A skill keeps its own language.** It is written as its readers read it, Korean included, and is never
  translated here; the words another unit matches on stay exactly as that unit matches them
  (`docs/boundary.md`, 「Words other units match」).
- **Everything else is English:** the placer, its messages, the checks, the documents, `README.md`, this
  file, commit messages and branch names. No Hangul in `src/` or `tests/`; the testbed's `korean` check
  holds this.
- **No personal data anywhere.** The owner is the owner and the chat agent the chat agent; no person,
  agent, machine, account or home directory is named. The personal-value check in `tests/` holds the
  shapes such a value takes.

## Layout

- One Cargo package at the root, holding the placer in `src/` and the checks in `tests/`, no other build
  tool's manifest, and no directory named after its parent.
- **`.graph/` is the code dependency graph**, written only by `bmo graph` in this checkout. A change that
  adds, moves or removes a node's files brings its node up to date in the checkout. Its area files are
  generated and untracked, so a fresh checkout builds them again, while `settings.toml` is the repository's
  configuration and stays tracked.

## The stages of a change

1. **Writing:** for the placer and the checks, the only check is `cargo check --all-targets`; for a
   skill's text, none.
2. **Review:** read the whole diff against the document governing it, then `cargo fmt --check` and
   `cargo clippy --all-targets -- -D warnings`.
3. **Verification:** the test targets touched, then `cargo test` once, output written to a file and the
   file read.

A change is not done before stage 3 has passed.
