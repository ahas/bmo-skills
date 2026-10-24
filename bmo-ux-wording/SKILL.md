---
name: "bmo-ux-wording"
bundle: worker
version: "1.0.0"
description: "The console's UI wording guide: where WORDING.md stands, when to read it, how a term is added to it, and how a use that departs from it is reported and fixed. Use before writing or changing any Korean text a screen shows, and when a review finds a screen's wording off the guide."
---

# UI wording

The Korean a screen shows is written by one guide, `WORDING.md`, kept by hand at the root of the bmo-web
repository. Read it before writing or changing any UI text. This skill says where the guide stands, how a
term is added to it, and how a use that departs from it is reported and fixed. It carries no copy of the
guide.

## Where the guide stands

- **`WORDING.md` at the root of the bmo-web repository.** That repository's checkout is the one holding
  the console; read the file there.
- **bmo-skills carries no copy, and no other repository receives one.** The guide is not placed by a
  deployment and is not read by a running unit.
- **The words themselves stand in the console's Korean pack, which the guide names.** The guide says
  which word a concept takes and how the sentence is written; the pack shows that word on the screen.
  Change the guide and the pack together.
- **No test reads the guide.** It is kept by hand, and a use is found by reading the screen and the pack,
  not by running a check.

## Reading and following it

- Read `WORDING.md` whole before writing or changing any Korean text a person reads: a label, a button, a
  notice, an empty state, an error, a dialog, a setting, or any other UI text.
- Write the word the guide gives, in every inflection the sentence needs; do not write a form the guide
  says it replaces.
- Where a word keeps its own sense, the guide says so; that use passes.
- A concept the glossary does not carry is written by the guide's principle, and gains its line when it
  is first needed.
- Code identifiers, commands, file and path names, and product, library and protocol names stay as they
  are, as the guide says.

## Adding a term

- Add the line in the same change that first writes the concept, in the guide's 「Console text」 section.
- Follow the shape of the lines already there: the English concept, an arrow, the Korean word, the forms
  the word takes, and the forms it replaces. Add a form to the concept's own line rather than writing a
  second line for the same concept.
- Change the pack with the guide, so the word the guide gives is the word the screen shows.
- A term already in the guide is not added again; where a screen needs another form of it, that form is
  added to the same line.

## Reporting and fixing a use

- A use is a piece of Korean text on a screen that departs from the guide: a word the guide says it
  replaces, another word where the guide gives one, English dropped into a Korean sentence, a mixed
  style, or a dash between words.
- Fix it where it stands, in the same pass: the pack's value and any text the screen writes itself.
  Search the pack and the screen for the word and its forms, so one fix reaches every place the same
  text is shown.
- When the fix shows the guide lacks the concept's word, add the term first, then fix the use.
- Report every use the work fixed in the job's report under Checked: the screen, what it said, what the
  guide gives, and that the pack and the guide now agree.
- A use the job cannot fix — on a screen outside its checkout, or a word only the owner can settle — is
  named in the report with why. Where it is real work somebody must pick up, it goes into `--unfinished`
  or up as its own row.
- Do not add a test or a check for the guide, and do not report a use the guide allows.

## What the guide is not

- It carries the principle and the console glossary. It carries no banned-term list, no allowed uses and
  no section on where a check reads it.
- **The banned-word check over BMO's own speech is another list with its own owner.** It holds what BMO
  says in answers, reports, TODO rows and questions, not what a screen shows. A UI wording change neither
  reads nor changes it.
