---
name: "bmo-docs-conventions"
bundle: worker
version: "1.6.0"
description: "Where and how to record new information in a repository (working rules and whose they are, design, follow-up tasks, technical facts, progress), the language and shape of a document, and what a document or a code comment must never carry. Use before changing code that a document under docs/ describes, and when leaving a note, documenting a design, writing a comment, storing something you learned, cleaning up documentation, or whenever a document is about to carry a change record, a source location, or a pasted snippet."
---

# Documentation Conventions — What Goes Where

Put something newly learned where its nature says. **The repository is the source of truth and
agent memory stays minimal** — memory is local, not shared across teammates or machines.

## Where to record

| Nature | Location |
|---|---|
| This repository's own working rules (what its code is made of, how it is built, run and checked) | Repository root guidance file (`CLAUDE.md`, `AGENTS.md`, per tool) |
| A working rule of the repository that binds only the work on one part of its tree | A rule file scoped to those paths (`.claude/rules/` with `paths:`), each named with its paths from the root guidance file, so a tool reading the root file alone still finds it |
| How a worker works in any checkout (when it may speak, the shell it runs in, how it closes a change, what it may not alter) | The worker's own common trailer and the worker's own skills, never a repository's guidance file (「A worker's rule and a repository's」 below) |
| Design and planning (SSOT for large work in progress) | Design document under `docs/` |
| A design, a comparison or an explanation put in front of the owner to read or decide on | the deployment's 아카이브, filed the way the `bmo-asking-the-owner` skill says — never a page hosted outside it: no claude.ai artifact, no Claude Docs document, no gist, no paste |
| Operational procedures (deployment, restart, bootstrap) | Runbook under `docs/` |
| Follow-up tasks, bugs, issues | TODO under `docs/` or the issue tracker |
| Parameters, quality criteria, development pitfalls | Technical note under `docs/` |
| **Only external facts that cannot be derived** from code, git, or docs | Agent memory (minimal) |

## A worker's rule and a repository's

- **The worker's own rules stand in the worker's own texts, whatever repository the work is in**: the
  common trailer (`prompts/agents/worker/trailer.md` in bmo-agency's repository) and the worker's
  skills (one folder each in the bmo-skills repository).
- **A repository's own documents hold that repository's own business** — its `CLAUDE.md`, its
  `AGENTS.md` or whatever it calls its rulebook, its `docs/`: what its code is made of, how it is
  built, run and checked, its commands, paths and own conventions.
- **How a worker works is the same in every checkout and written in the worker's texts alone** —
  when it may speak, what it checks its own work with, how it closes a change, what it may never
  alter, what it writes in its report.
- **A worker rule is never copied into a repository's document**, not even as a line pointing back:
  one of two copies is already wrong, and a worker with a repository of its own reads only its own.
- **Which side a rule falls on:** ask whether a worker in another checkout, doing other work, needs
  it — if so the rule is the worker's; if only a reader of that code needs it, the repository's.
- **A worker rule found in a repository's document is moved** to the worker's texts when that
  repository is the one the job runs in, and named in the report when it is not.

## Principles

- **Never record anew what the repository already holds** — code structure, history and
  completed progress have git as their source; commit messages say what was completed when.
- **Memory holds only facts you cannot pull from the code** — never design, progress or rules;
  everything to be shared goes to the repository.
- **The design document is the SSOT.** Update it as work proceeds, and mark its status or remove it
  once complete; progress logs never accumulate as memory files.
- Asked to "remember" something, first find its place in the table above: usually a repository
  document, memory last.

## Language and shape

These bind every document written into the repository — `docs/`, the README, design notes and
runbooks, the documents a project's own agents keep — but not an answer or report addressed to a
person.

- **Write in English.** Headings, list labels, statements and index entries are English.
- **Keep what translation would break in its original language:** names, nicknames and forms of
  address; examples of how someone talks; short quotes and every passage quoted from a source;
  domain terms bound to their language (a room's own words, an internal name, a product or statute
  name). Code, commands, paths, links, identifiers and error messages stay as written.
- **A tilde in the words is written `\~`,** a Korean range ("3\~5개") as much as any other:
  GitHub strikes through the words between two single tildes of one paragraph. A tilde in code or
  a path stands as it is.
- **A document you touch that is still in another language** is carried into English whole, every
  statement kept.
- **Sections first,** each short, so a reader finds one thing fast and loads only that part.
  **Top-level sections are broad, self-contained units** — each could stand as a tab.
- **Bullet-style:** clipped statements, not long paragraphs; facts as numbered or bulleted items,
  one thing per item. **A long item gets a summary title** apart from its body, so the title alone
  tells a reader whether to read on.
- **Structure is drawn, not told.** How parts connect, the order work flows in, a thing's stages and
  what depends on what are a diagram, with a line of prose for what the picture cannot say.
  - In markdown, a fenced `mermaid` block flowing top to bottom (`flowchart TD`), a few words to a
    box, so it reads at a phone's width.
  - In an HTML page, inline SVG in the page's own colours, scaled by its view box.

## What a document must not contain

The standing rules every job is handed bar a change history and a code quotation from every
document and code comment; this is what goes in their place.

| Instead of | Write |
|---|---|
| A change record in a document | Nothing — the commit message and `CHANGELOG.md` already hold it |
| A rule explained through the design it replaced | The rule as a present constraint, with its reason stated in the present tense |
| A pasted snippet of the project's own source | The behavior in prose, or a sketch in pseudocode that stands on its own |
| A file path and line used to point at something | The name of the thing and what it does. A coordinate ties the document to something it cannot follow |
| A signature or config fragment copied into prose | What the thing takes and what it gives back |

- A quotation, a coordinate and a recorded history each **fork the canonical text**: a copy or a
  pointer that stops matching its source once the source moves.
- **Pseudocode does not fall under this:** it reproduces no source, so nothing goes stale. Use it
  when a shape's intent matters and its exact form does not.
- A document whose subject *is* the history — `CHANGELOG.md`, a release note, a postmortem — is the
  exception: there the history is the content.
- An error message, a command to run, or a passage quoted from an external spec is not a code
  quotation; the source-material rule governs reproducing those.

## Before you change code

The standing rules every job is handed make the `docs/` document the source the code answers to,
read and repaired first. What this adds:

**Finding the right one.** Prefer the `docs/` nearest the code being changed — a workspace member's
own directory outranks the repository root. Scan file names and headings before opening anything,
and read the one that claims the area, not everything adjacent. A design document covering the
work in progress is the SSOT and comes first.

**Deciding whether it is now wrong.** It needs repair when the change falsifies something it
states: a documented behavior, a contract, a boundary, a default, an operational procedure, a
quality criterion — not because code moved, was renamed or was refactored behind an unchanged
statement.

**Repairing it.** State the new truth in place of the old, never a correction stacked on what it
replaces or both left standing. Where several documents state the same thing, repair the one that
is the source and remove the claim from the others rather than editing in parallel.

Document and code go in the same commit: one that changes behavior and leaves the document behind
publishes two sources that disagree.

## When cleaning up

- Completed and verified progress records → discard (they are in git history), after extracting
  any **pitfalls and lessons** worth reusing into a technical note.
- Do not migrate a note containing incorrect data (such as a bad measurement); point at the
  corrected source document instead.
- Keep only the remaining pure facts in the memory index, with a header comment pointing at the
  authoritative locations (repository guidance, `docs/`).
