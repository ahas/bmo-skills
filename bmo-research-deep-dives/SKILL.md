---
name: bmo-research-deep-dives
description: "Read before a deep dive, a research at the thorough or exhaustive depth, whatever its question: how it frames its question toward a judgement, reads a made thing's maker before anyone writing about it, takes the maker's figures as the baseline, what its document holds, and the review its page is given."
bundle: worker
---

# Deep dives

- This rulebook sets how a deep dive works: how it frames its question, reads the maker of a thing
  before anyone writing about it, sets the maker's figures as the baseline, what its document holds,
  and the review its page is given. Its deciding section stands in the search runner's prompt and
  the run reads the rest once that section says it applies.

## Whether this rulebook applies at all

- **It applies** to every run at the thorough or exhaustive depth: such a run is a deep dive,
  whatever its question. A run at the basic depth is none.
- **How much binds depends on the result.** Every rule here binds a deep dive whose result has a
  body; a brief result and study material keep only 「The maker's word first」 and the done
  condition in 「The question」.

## The question

- **The question is framed toward a judgement, not a collection.** Before the first query, settle
  what the reader will decide from the document. For a thing somebody made (a model, an engine, a
  library, a tool, a product, a service) that is whether we can use it, what of ours it touches
  and what to try next, and the done condition is drawn from that judgement.
- **A deep dive about a thing somebody made is not done while none of its maker's own material
  was read.** Where every attempt failed, the first line is `ANSWERED: no`, and UNCONFIRMED names
  each maker's page tried and what it answered. No count of queries or pages stands in for this.

## The maker's word first

- **The maker is read before anyone writing about it.** Its primary material is found and read
  first: the official documentation, the repository's README and docs, the release notes and
  changelog, the model card, the configuration files, the licence, the reproduction scripts, the
  paper in full, the official blog, the registry page and its discussions. Articles, blogs,
  forums and videos come after, read as what others say about it.
- **Read to the end of what bears on the question:** a model card's every table and its
  footnotes, a paper's method, its evaluation and the conditions under each table, never the
  abstract standing for the paper.
- **A repository is opened, not judged by its front page.** The file tree, the README, the docs
  folder, the examples or cookbook, the wiki, the releases, recent pull requests and issues, and
  the code itself where a claim turns on how something is done. A page a listed site tool answers
  is read with that tool; any other file is read with web_fetch at its raw address.
- **The maker's claims, others' findings and our judgement are told apart sentence by sentence.**
  Each sentence of the maker's carries its source's mark and, where the source is long, the part it
  stands in: a section, a table, a file. A sentence that is our judgement reads as ours. A maker's
  claim no independent source checks says so beside it.
- **Disagreement is set side by side.** Where another source differs from the maker, both stand
  with their sources and the difference is said. A fault raised in public (an issue, a forum
  thread, a registry discussion) is followed to whether it has since been fixed, and the document
  says which.

## The maker's figures as the baseline

- **Every figure of performance carries the maker's own beside it.** For a model or an engine,
  the maker's published figure for the same measure stands as the baseline, with its conditions
  (the benchmark and its split, the settings, the hardware, the version) and who measured it: the
  maker, a third party, or us.
- **A figure stands beside its pair:** one setting against another, the maker's claim against an
  independent measurement, the thing against what we already run where it would take that place.
- **A judgement resting on figures nobody reproduced is marked `no baseline`**: a tuning, a
  comparison, a better or worse that no one here measured under the maker's conditions. It is
  never written as measured.
- **A value nobody measured is a calculation shown whole**, what goes in and the arithmetic, so the
  reader can redo it: the weight bytes ÷ 2^30 and a cache estimate set against a card's memory, an
  index as vectors × dimensions × bytes.
- **Whether we could run it is judged against our own equipment** as the request names it, each
  machine or card coming out as fitting, not fitting, or what blocks it. A request naming no
  equipment leaves the fit in UNCONFIRMED rather than guessed.
- **A value not checked is marked `estimated` or `needs checking`**, its ground in brackets after it.

## What the document holds

- **A deep dive about a thing somebody made takes one of the three shapes `bmo-research-routine` sets** — the
  investigation, the manual or the verification — and that rulebook names its sections; the rules
  below hold inside whichever shape it takes.
- **What the maker states comes first in the middle,** a section of its own: what it says the thing
  supports, its limits and its figures with their conditions, before any section of what others
  found or what we judge.
- **The title names the thing, never the verdict:** a noun phrase of the thing's name and the
  shape, as `bmo-research-routine` closes it and the search task bounds it, whatever the shape; the verdict
  is a judgement that stands in the body and the narration, never in the title.
- **The verdict is a judgement:** whether to use it, for what, and what blocks it.
- **The next step is a reproduction plan** where the answer leans on the maker's figures: which
  figure, on which set, with which settings, about how long it takes, and whether the maker
  publishes the script and the data.
- **What was read and what was not** is the middle's last section: every primary material read,
  and every one that could not be read with what it answered, or none.
- **Tables and lists stand on lines of their own,** never run into a paragraph.
- **A table is written whole:** its header row, then on the next line the separator row, a cell of
  dashes under each header cell, then one line per row; every line opens and closes with a pipe and
  holds as many cells as the header. A table without its separator row is not a table.

## The review

- **A deep dive's page is reviewed once, whole, by the model that wrote it,** after its Korean side
  and its narrations are made and before anyone is told of it. The review reads the page as its
  reader will: the English side, the Korean side and both narrations.
- **The review looks for these faults and no others:**
  - a figure, a unit, a date or a version that differs between the two sides, between a section and
    the narration, between two sections, or between a sentence and what the source it cites is said
    to state;
  - a term mistranslated: a word of the field rendered in a sense it does not carry there;
  - a name translated that stays as written: a model, product, dataset, library, organisation or
    person's name, an acronym, code, a command, a path, an address or a fixed section name;
  - a broken citation: a mark pointing at no source, a source line with no address or with its
    address altered, a link whose text and target disagree;
  - Korean that reads as machine output: word order or particles carried over from English, a
    sentence no Korean writer would write, a register outside the owner's documents', an English
    word left standing where Korean has the word and no name stands.
- **Taste is not a fault.** A correct sentence that could be shorter, an order the reviewer would
  have chosen, a word that is right but not the reviewer's own stay as they are.
- **A fault is fixed where it lies.** A fault of the English side is fixed on the English side, and
  its Korean is translated again from the fix; a fault the Korean side alone holds is fixed on the
  Korean side, the English left as it is. A narration is fixed the same way, by its language.
- **A fix gives its part whole:** a section rewritten from its heading line to its end, every
  sentence of it that holds no fault kept word for word, its heading in the side's own language;
  a narration rewritten whole, kept to `bmo-research-kinds` 「The narration」. Nothing outside the part is
  touched.
- **Nothing is invented in a fix.** A figure is corrected only to the value the page's own sources
  or its other side state; where the right value is not on the page, the fix marks the figure
  `needs checking` rather than guessing one.
- **A page with no fault passes.** A review that finds nothing to fix says so in one word, as the
  question asks.
