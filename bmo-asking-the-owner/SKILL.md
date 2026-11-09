---
name: "bmo-asking-the-owner"
bundle: worker
version: "1.3.0"
description: "How a job puts something in front of the owner: a decision that is genuinely his (hard to undo, or settled by his own taste) asked with bmo job ask, and a document for him to read or approve filed in the deployment's 아카이브. Use before asking him anything and before handing him any document."
---

# Asking the owner

## A decision that is his

- **Put it to him only where the choice is really his** — hard to undo, or settled by his own taste
  and no reading of the code. Everything else the work settles itself and writes under 정한 것: a
  question about something you could have looked up is worse than never asking.
- **The command:**
  `bmo job ask --todo <id> --question '<무엇을 정할지>' --option '<하나>' --option '<다른 하나>' --recommend <추천 번호> --why '<왜 네가 못 정하는지 한 줄>'`.
  - Two or more options, the recommendation one of them by its number.
- **It is put in his own chat**, the only chat an answer can be given in.
- **The command does not return until it is answered: your own run is what waits,** holding its
  checkout's lock, and nothing else is named for that checkout, so nothing else is held up.
- **Settled, it prints one line on stdout and nothing else**, exit code 0: the option's own text
  where he picked one, or his own words where he answered with something the options did not cover.
  Carry on with what that line says, whichever it is. It is a person's answer every time: the
  recommendation is only shown to him, and comes back only where he settled with it.
- **Where the question could not be put, it exits 1, prints no option on stdout and closes the question
  unsent**, with why on stderr. Nobody has been asked and nobody will answer it, so nothing is settled:
  never carry on as if your recommendation had been chosen. Ask again once what stopped it is mended, or
  name it in your report and stop there.
- **Where the question is withdrawn, or your run is told to stop, it exits 1 the same way.**
- **Where he talks instead of choosing, it exits 3 and the question is still open.** He asked what
  an option means, objected, or said something else: what he said is printed with the call that
  answers him. You answer, since you hold what the options were written out of and the chat agent does not.
  - `bmo job ask --talk <질문 번호> --say '<답>'` puts your answer in his chat and waits on the
    same question again, coming back the same two ways. Where your answer could not be put it exits 1 and
    the question stays open: say it again. Answer plainly: what each option would do,
    what it costs, why you recommend yours.
  - Where the talk changes the choice, the same call puts it again with `--option` for every option
    anew and `--recommend`, plus `--question` or `--why` where those changed.
  - Only he ends it, by choosing. Never take his words as a choice yourself, and never carry on
    while the question is open: keep answering with `--talk` until a line comes back.
- **A question waits for his answer, however long that takes.** No clock settles it for him and none
  cuts the talk short: the command returns only once he has chosen, talked, or the question has closed. Since your run waits
  holding its checkout's lock the whole while, put to him only what is really his.
- **The Bash tool waits for the command however long it runs,** as long as the call names no
  `timeout` or names the largest, 2147483647 ms, which the tool hook asks of this command. Where the
  call does run in the background, wait for it to end — the tool tells you when the task finishes, and
  until then read its output one read to a call — and read what it printed before doing anything else.

## A document for him to read

- **It goes into the deployment's own 아카이브 and nowhere else.** Write it as markdown in your scratch folder
  and file it with `bmo archive doc --title '<제목>' --body <file>`,
  or an HTML page with `--html <file>`. It prints the document's number and the address to hand over.
  - With no `--chat` it is his own.
  - `--replace <id>` rewrites it under its number as the design moves.
- **Its pictures stand in its body, at the place its words speak of them.** A reference you collected, a
  result you drew, a chart, a screenshot: each is drawn there as a markdown picture whose target is the
  file's own name, and the file is handed over in the same call with `--file <path>`, once for each. A
  rewrite hands over only the files the document does not keep yet, and a changed file under a new name.
  - What is compared stands side by side: the pictures of one comparison in one table row, the reference
    first.
  - The files list under the document is where he saves a file, never where he is sent to look. A document
    whose pictures are only attached, its body words and figures alone, is not finished.
  - A file he saves or plays and the body does not draw — a data file, a recording — is handed over the
    same way and named in the body where it is spoken of.
  - A long reading he studies — how something is built, a deep investigation — also takes a narration:
    `bmo brief-attach <document number> <file>` with a script for the ear by the rules of a deep dive's, in
    `~/.local/share/bmo-agency/deploy/prompts/rules/deep-dives.md` 「The narration」, spoken in the chat agent's
    voice and kept at the document's head as its 들으며 보세요 band.
  - A document with a narration is handed over only once its sound is kept: `bmo brief-attach` returns
    once it is, and ends with a failure where it is not, and then it is run again.
  - The address it prints goes into your question, or opens your report as the
    `bmo-closing-a-job` skill sets.
- **A page hosted elsewhere is out of his reach** once this session is gone, kept on somebody
  else's terms — a claude.ai artifact, a Claude Docs document, a gist, a paste. The tools that
  publish one are not on a job's line, and a question or a closing that carries such an address is
  refused.
- **Its structure is drawn**, as the `bmo-docs-conventions` skill says of every document.
