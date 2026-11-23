---
name: bmo-research-study-material
description: "Read when someone asks to be taught a subject (study material, a textbook, lecture notes, a course to learn from) or a research's result shape is learning: the head, the heading levels, the frame its chapters are drawn from, and what the material carries."
bundle: worker
---

# Study material

- This rulebook sets the shape of study material, the document a search writes when it is asked to
  teach a subject. Its deciding section stands in the search runner's prompt and the run reads the
  rest once that section says it applies.
- Study material borrows nothing of the frame `bmo-research-kinds` gives a search's answer: this
  rulebook is the whole shape of its body.

## Whether this rulebook applies at all

- **It applies** where the request asks to be taught a subject (study material, a textbook, material to study,
  lecture notes, a course to learn from) or the launch's result shape is `learning`.
- **It does not apply** to a question about a technical subject for that alone; only a request for
  material to learn it from is study material.

## The head

- **The head says so.** `KIND: learning` stands in the head beside `TITLE:`, `TOPIC:` and
  `KEYWORDS:`. The title names the material as a book is named, never the request restated.
- **No verdict.** No `VERDICT:`: study material is read through, not answered from. The contents are what a reader takes in first.

## The levels and the frame

- **The levels are markdown headings.** `#` a part, `##` a chapter, `###` a section. A subject
  too small for parts starts at chapters: `#` a chapter, `##` a section. No fourth
  level: under a section stand its paragraphs, lists, tables, code and pictures. No `SECTION:`,
  `NAME:`, `===ITEM===` or `MORE:` line: the headings are the structure.
- **The frame the chapters are drawn from**: the skeleton technical books share:
  - **introduction**: who it is for, what to know first, what to have ready, how it is laid out.
  - **Parts by stage, not by topic**: getting started (the whole picture, then one run from
    start to end), the core concepts one chapter each, practice and application (operating,
    deploying, troubleshooting, a case), and an advanced part only where the subject has one.
  - **A chapter** opens with its learning objectives, then the principle, how the thing works, before the
    concept and practice sections with their examples, then the limits and what to weigh, and
    closes with key summary (the chapter's keywords), check questions and references.
  - **The back**: closing with what to learn next, and appendix: setting up, the answers to the
    check questions, and a glossary giving each term with its original.
- **Fit the subject; never stamp the frame.** The frame is a source for chapters, not a form to
  fill. Each chapter and section is named for what it teaches, never overview or body standing for
  anything. A field keeps its own shape where it has one: principles as rules with a case study
  and a checklist, infrastructure as task sections each doing one thing, a first programming
  language as small sections each with its own check, a theory as theory, then implementation,
  then variants. A chapter element with nothing of its own is left out (no exercises, no check
  questions) and a part with one chapter is no part.
- **The size follows the request.** A primer is a handful of chapters, not a thousand-page book.

## What every result carries

- **It still stands here.** Claims cite pages with `[n]` and close with `SOURCE [n]:` lines; a
  diagram or screen worth seeing is an `IMAGE:` line; code stands in fenced blocks.
- **What could not be confirmed is the last top-level heading**, `what could not be confirmed`, a numbered
  list, in place of an `UNCONFIRMED:` line: a structure heading there would stand above the parts
  and break the contents.
