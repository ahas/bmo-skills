---
name: "ahas-code-clean"
bundle: worker
version: "2.3.0"
description: "The line-level pass of a refactoring: names, functions, comments, formatting, objects and data structures, error handling, boundaries, tests, classes and concurrency, judged one line at a time against SOLID, DRY, KISS, YAGNI, the Law of Demeter, least astonishment and the other standing principles. Use third and last, after the ahas-code-design-patterns pass has settled the structure and the refactoring pass has moved the parts, and on its own whenever code is being read for readability, a Rust crate is laid out or linted, or logic more than one runtime needs is being written."
---

# Clean Code — The Line Pass

## Where this pass sits

- **Last of three: design patterns, then refactoring, then clean code.** The order, why it runs by
  reach, and the rule never to start later or reorder are in
  `ahas-code-design-patterns` 「Where this pass sits」. This pass, the narrowest, settles a name, a line, a
  comment, a signature.
- **It is also run on its own:** reading existing code for readability, reviewing a diff, or
  finishing a change that needed no restructuring is this pass alone, and skipping the first two is
  correct there.

## How to hold these rules

- **Trade-offs, not laws.** Every principle has a harm when overdone, which its reference entry
  names beside it.
- **Numeric rules are signals, never gates.** A line count, an argument count, one assertion per
  test each say "look here", not "split this"; mechanically small functions are shallow, their logic
  entangled across them.
- **The measure is the reader's load:** how much a developer must hold in mind to change this, and
  how plainly it is shown. A deep module, a simple interface over much hidden work, beats many
  shallow ones.
- **Principles mean what they first meant,** narrower than the folklore: single responsibility is
  one actor's reason to change, not "one thing"; DRY is about knowledge, not text; YAGNI is about
  speculative features, not refactoring or tests.
- **Where a rule and the codebase's own idiom disagree, follow the code around it** and report the
  disagreement.

## How the pass runs

1. Read the unit top to bottom as a reader who has never seen it.
2. Walk the checklist over what the change actually touched.
3. Fix what you find in the same pass; a rule you can meet is never left for somebody else.
4. Where a rule cannot be met, or meeting it would do the harm its entry warns of, say where and why.

## The checklist

Each area's demand, broken shape, overuse and exceptions, and TypeScript form are in
`references/principles.md`; the TypeScript forms with the most leverage in
`references/typescript.md`; a Rust crate's layout, its lint standard and its Rust forms in
`references/rust.md`, read before writing or reviewing Rust.

- **Names** — intent-revealing, pronounceable, searchable; one word per concept; constants for magic
  values fixed by their nature; no encodings or repeated context; not so long they read awkwardly. A
  name says what the thing is, never a fact that can change — the `ahas-code-refactoring` skill's
  「Names that stay true」 is that rule and its test.
- **Functions** — one job at one level of abstraction; few arguments, no flag arguments; no hidden
  side effects; command or query, not both; return values, not output arguments. Tightly bound
  steps stay together.
- **Comments** — what code can say is said in code; redundant, stale and commented-out code is
  deleted. Kept: intent, a warning of a consequence, an interface's contract the signature cannot
  give — format, ordering, behaviour on duplicates, side effects, argument limits.
- **Formatting** — top to bottom; related code close; variables near their use; callers above
  callees; the formatter settles the rest.
- **Objects and data structures** — objects hide data and expose behaviour, data structures expose
  data and carry none; no hybrids. Chains through service objects are the smell, through plain data
  not.
- **Error handling** — never swallow an error; expected failures as values, invariant breaks and
  unrecoverable faults as thrown `Error`s; no `null` passed around.
- **Boundaries** — wrap outside code; parse outside input once at the edge; never assert an outside
  shape with `as`.
- **Tests** — fast, independent, repeatable, self-validating, timely; one concept each; names read
  as requirements.
- **Classes and modules** — small, cohesive, one actor's reason to change; a base knows nothing of
  what derives from it; a module of exported functions often serves where a class would.
- **Concurrency** — kept apart from other code; shared state minimised; every promise awaited,
  returned or explicitly voided.
- **Design heuristics** — configurable data at a high level, without over-configurability; value
  objects; no negative conditionals; leave the code cleaner than found.

## The standing principles

- **SOLID, DRY, KISS, YAGNI, the Law of Demeter, Tell Don't Ask, command-query separation, least
  astonishment, and parse, don't validate with illegal states made unrepresentable** — each with its
  demand, its limit when overdone and its TypeScript form in `references/principles.md`.
  Duplication is far cheaper than the wrong abstraction.
- **What settles what is written at all,** in the same file: one implementation per piece of logic,
  built for WebAssembly where another runtime needs it rather than written again in its language; the
  maintained library first for anything standard; and markdown for people, structured data for
  whatever a program reads.
- **Held as direction rather than verdict:** composable, simple in the Unix sense, predictable,
  idiomatic, domain-based.

## What this pass hands on

- The lines finished, and where a rule was not met, the place and the reason.
