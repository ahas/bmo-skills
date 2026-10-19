---
name: "bmo-asking-the-owner"
bundle: worker
version: "1.1.0"
description: "How a job puts something in front of the owner: a decision that is genuinely his (hard to undo, or settled by his own taste) asked with bmo job ask, and a document for him to read or approve filed in the deployment's 아카이브. Use before asking him anything and before handing him any document."
---

# Asking the owner

## A decision that is his

- **Put it to him only where the choice is really his** — hard to undo, or settled by his own taste
  and no reading of the code. Everything else the work settles itself and writes under 정한 것: a
  question about something you could have looked up is worse than never asking.
- **The command:**
  `~/.local/share/bmo/deploy/bin/bmo job ask --todo <id> --question '<무엇을 정할지>' --option '<하나>' --option '<다른 하나>' --recommend <추천 번호> --why '<왜 네가 못 정하는지 한 줄>'`.
  - Two or more options, the recommendation one of them by its number.
- **It is put in his own chat**, the only chat an answer can be given in.
- **The command does not return until it is answered: your own run is what waits,** holding its
  checkout's lock, and nothing else is named for that checkout, so nothing else is held up.
- **Settled, it prints one line on stdout and nothing else**, exit code 0: the option's own text
  where he picked one, or his own words where he answered from the web console with something the
  options did not cover. Carry on with what that line says, whichever it is.
- **Where he talks instead of choosing, it exits 3 and the question is still open.** He asked what
  an option means, objected, or said something else: what he said is printed with the call that
  answers him. You answer, since you hold what the options were written out of and the chat agent does not.
  - `~/.local/share/bmo/deploy/bin/bmo job ask --talk <질문 번호> --say '<답>'` puts your answer in his chat and waits on the
    same question again, coming back the same two ways. Answer plainly: what each option would do,
    what it costs, why you recommend yours.
  - Where the talk changes the choice, the same call puts it again with `--option` for every option
    anew and `--recommend`, plus `--question` or `--why` where those changed.
  - Only he ends it, by choosing. Never take his words as a choice yourself, and never carry on
    while the question is open: keep answering with `--talk` until a line comes back.
- **A question is never a way to stall.** A minute with no answer settles the question on the option
  you recommended and says so in the chat. Where the owner stops the clock from the console to read it,
  the wait is his, thirty minutes at the most, and the command still returns one line either way.
  While you talk the clock stays stopped, thirty minutes from the last line either of you said.
- **A wait can outlast the Bash tool's ten minutes** once the clock is stopped. The tool then moves
  the call to the background; wait on its output, bounded, and read it before doing anything else.

## A document for him to read

- **It goes into the deployment's own 아카이브 and nowhere else.** Write it as markdown in your scratch folder
  and file it with `~/.local/share/bmo/deploy/bin/bmo archive doc --title '<제목>' --body <file>`,
  or an HTML page with `--html <file>`.
  - With no `--chat` and no `--by` it is his own.
  - `--replace <id>` rewrites it under its number as the design moves.
  - A long reading he studies — how something is built, a deep investigation — also takes
    `--narration <file>`: a script for the ear by the rules of a deep dive's, in
    `~/.local/share/bmo-agency/deploy/prompts/rules/deep-dives.md` 「The narration」, spoken in the chat agent's
    voice and kept at the document's head as its 들으며 보세요 band.
  - A document with a narration is handed over only once its sound is kept: `archive-doc` returns
    before the narration is spoken, and the `bmo-tools-researching-and-benching` skill says how to wait for it.
  - The console address it prints goes into your question, or opens your report as the
    `bmo-closing-a-job` skill sets.
- **A page hosted elsewhere is out of his reach** once this session is gone, kept on somebody
  else's terms — a claude.ai artifact, a Claude Docs document, a gist, a paste. The tools that
  publish one are not on a job's line, and a question or a closing that carries such an address is
  refused.
- **Its structure is drawn**, as the `bmo-docs-conventions` skill says of every document.
