---
name: "bmo-asking"
bundle: worker
version: "2.1.2"
description: "How a job asks: anything it cannot settle or do itself goes to its supervisor with bmo job ask — a decision that is not the job's, a command on a node it cannot reach, a deployment it waits on — and the supervisor settles it or puts it to the owner; and a document for the owner to read or approve filed in the deployment's 아카이브. Use before asking anything and before handing the owner any document."
---

# Asking

## A job asks its supervisor

- **Every question goes to your supervisor, never to the owner directly.** `bmo job ask` hands it to the
  supervisor of the owner's room at once, and the supervisor judges it: what it can settle itself — a command on
  a node your account does not reach, a deployment your work waits on, a look-up with its reach, a choice the
  owner's standing word already settles — it does, and answers with the result; a decision that is the owner's
  — hard to undo, his own taste, money, an account — it puts to him in its own words; where it is unclear what
  you ask or why, it talks it over with you first. You never word a question for the owner and never write
  "the owner need not answer this": the supervisor decides who answers.
- **Ask only what the work cannot settle itself.** What reading the code, the documents or the brain settles,
  you settle and write under 정한 것: a question about something you could have looked up is worse than never
  asking. A request that is no one's to grant — a road the rules close to jobs, a node your account must not
  reach — is not asked either: it goes into the report.
- **It is read at a glance.** The question in a few lines, each option one short line, the reason one line,
  all in English: the supervisor puts what reaches the owner into his language. A command the supervisor is to
  run is named in words, short; no script, code block, hash, long address or encoded payload goes into any of
  them: `bmo job ask` refuses such words and opens nothing. A long reading goes into the archive (below) and its
  address into the question.
- **The command:**
  `bmo job ask --todo <id> --question '<무엇을 정하거나 해 달라는지>' --why '<왜 네가 못 하는지 한 줄>' [--option '<하나>' --option '<다른 하나>' --recommend <추천 번호>]`.
  - A choice puts two or more options, the one you would take named by its number; a request the supervisor
    answers in its own words — a command's output, a deployment done — puts none, and no `--recommend`. A
    question with no options is the supervisor's alone: it never reaches the owner until the supervisor writes
    its choices.
- **Write it for someone who knows nothing of the work.** The supervisor may hand your question to the owner as
  you wrote it, so say what the work is, what each option does, how the options differ and what follows under
  each, with no step number, internal name or term left unexplained. An option says what it does in words of
  its own: `bmo job ask` refuses a bare label or a step number (`1`, `B`, `Option 2`, `Step 12`) as an option.
- **The command does not return until it is answered: your own run is what waits,** holding its
  checkout's lock, and nothing else is named for that checkout, so nothing else is held up.
- **Settled, it prints the answer on stdout and nothing else**, exit code 0. The supervisor's answer comes as
  it wrote it, a result of several lines among it; the owner's, once the supervisor put the question to him, is
  one line: the option's own text where he picked one, or his own words. Carry on with what it says. While the
  supervisor holds the question, only an answer settles it. Once it is put to the owner, he has the job
  service's person wait, a minute, and where he has not answered by then the question settles on
  the recommendation it was put to him with, which comes back as his line would, with a line on stderr saying it
  was the recommendation.
- **That answer stands, and the question is over.** Never run the same ask again to check it, to see whether it
  still waits or to read the answer a second time; the answer stays for the rest of the job, across its stages.
  Carry it into your report as the supervisor's word or the owner's, whichever gave it.
- **Where the question could not be put, it exits 1, prints nothing on stdout and closes the question
  unsent**, with why on stderr. Nobody has been asked and nobody will answer it, so nothing is settled:
  never carry on as if your recommendation had been chosen. Ask again once what stopped it is mended, or
  name it in your report and stop there.
- **Where the question is withdrawn, or your run is told to stop, it exits 1 the same way.**
- **Where the supervisor or the owner talks instead of settling, it exits 3 and the question is still open.**
  What was said is printed with the call that answers it. You answer, since you hold what the question was
  written out of.
  - `bmo job ask --talk <질문 번호> --say '<답>'` puts your answer in the same room and waits on the same
    question again, coming back the same ways. Where your answer could not be put it exits 1 and the question
    stays open: say it again. Answer plainly: what you need, why you cannot do it yourself, what each option
    would do and what it costs.
  - Where the talk changes the choice, the same call puts it again with `--option` for every option
    anew and `--recommend`, plus `--question` or `--why` where those changed.
  - Where the supervisor only says it is still at work — "deploying; I answer once it is live" — and asks
    nothing, it needs no answer: run the same ask again, or, where you would say something, acknowledge it with
    `bmo job ask --talk <질문 번호> --say '<words>' --ack`. An acknowledgement is written on the question and
    handed to nobody, so it wakes nobody; both wait on the question as it stands, and the supervisor settles it
    once its work ends. Never answer such a word with a plain `--talk` — "thank you, I wait" — since a plain talk
    asks the supervisor for a word again.
  - Only the supervisor or the owner ends it, by settling. Never take their words as an answer yourself, and
    never carry on while the question is open: answer what asks something with `--talk`, and acknowledge what
    asks nothing with `--ack`, until an answer comes back.
  - Each word is printed under the voice that said it, the supervisor's or the owner's.
- **A question waits for its answer, however long that takes.** No clock settles it and none cuts the talk
  short. While the supervisor holds it, nothing counts; a supervisor busy or restarting delays the answer and
  loses nothing. Where the supervisor does not take it up — its notice let go untaken, or the turn that took it
  ended with no word on it — the command wakes the supervisor again, at longer rests, up to six times, and says
  so on stderr; the question stays with the supervisor, and only the supervisor ever puts it to the owner. You go
  on waiting. Since your run waits holding its checkout's lock
  the whole while, ask only what you need.
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
  - A long reading he studies — how something is built, a deep investigation — also takes a narration:
    `bmo brief-attach <document number> <file>` with a script for the ear by the research skills' rule for
    a narration, the `bmo-research-kinds` skill's 「The narration」, spoken in the supervisor's
    voice and kept at the document's head as its 들으며 보세요 band.
  - The narration file is the written words, kept as the narration's transcript: names, acronyms and
    figures stand as you would write them. The voice unit derives the spoken words from it, the same
    words as a Korean reader says them aloud, and checks them.
  - A document with a narration is handed over only once its sound is kept: `bmo brief-attach` returns
    once it is, and ends with a failure where it is not, and then it is run again.
  - The address it prints goes into your question, or opens your report as the
    `bmo-closing-a-job` skill sets.
- **A page hosted elsewhere is out of his reach** once this session is gone, kept on somebody
  else's terms — a claude.ai artifact, a Claude Docs document, a gist, a paste. The tools that
  publish one are not on a job's line, and a question or a closing that carries such an address is
  refused.
- **Its structure is drawn**, as the `bmo-docs-conventions` skill says of every document.
