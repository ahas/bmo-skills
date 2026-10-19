---
name: "bmo-closing-a-job"
bundle: worker
version: "1.1.0"
description: "How a job ends: the graph brought up to date and the alias his words call for, the design sweep of a screen it changed, the commit and the close, the item marked done or held, what a 미결 strand and a raised row may carry, checking the work by machine instead of asking the owner, and the three-part report. Use before the first commit of a job's change, and again before writing its report."
---

# Closing a job

## The graph

Bring the code dependency graph up to date in one pass just before the commit and never as you go — a node whose files moved, a symbol gone, an edge gone or new — and put that pass in the same commit as the code: a commit that touched a file the graph names and left the graph alone is refused. A node's title and what are English (the graph tool refuses prose that reads as Korean), a Korean screen label inside them quoted as it stands. A what is one or two sentences on what the feature is and where it lives, at most 300 characters (the tool enforces it): where your change made it wrong, rewrite the wrong sentence rather than appending what you did; detailed behaviour goes in the feature's document under docs/, the history in the commit, his words in an alias. Where the item's own words named the feature in a way the graph does not carry yet, put that word on the node with `bmo graph alias <id> --add '<the word>'`: his word exactly as he wrote it, never your own name for it and never a phrase you tidied, since the line is worth having only as the way he actually asks. An alias does not rewrite the node's base commit, so it costs the node nothing.

## A screen the job changed

Where this job changed a screen — a `.vue` file, or whatever a screen is made of in your checkout — sweep it against the owner's design rules before you close; where it changed none, skip this paragraph whole. The rules are the `ahas-ux-design-rules` skill: load the skill again at the end rather than trusting what you read before the first edit, drive each screen you touched rather than reading its source alone, fix what the sweep finds in this same pass, and write the sweep into the report under Checked the way that skill's sweep section says.

## Closing the change

Close the change before you answer, in this order. First commit and push what you changed, commits and close reasons in English as the standing rules' 「Language」 says, and run the document reload for any document a chat session relies on. Then close the change the way the repository's own documents give the command and how a skipped suite is said to it: where your change reaches what the whole suite runs, the close runs the suite once, at the end, after the tests your own change touched have passed; where it reaches nothing the suite runs — a document no test reads, a file no test loads — you may skip it, the judgement is yours, and the report says in one line why it was skipped. Until that last run you check with the static check and the tests your change touched, every file it reaches. A passing close hands your commit over as waiting to go live; you do not deploy, and putting it live is the chat agent's. Say in the report that a commit is waiting to go live, and which, and stop there: the deploy is the chat agent's, so the report never asks anybody to put it live. A failing suite is yours to mend in this same job — fix everything it reported that you can name a cause for, commit, push and close again — and a job whose suite still fails reports that nothing of it was handed over. Only a commit goes live: the runner reads the working tree when a job reports and says in the report if anything was left uncommitted. The runner tells the bot the moment this job ends; what runs next is the chat agent's.

Marking your TODO item done is a separate decision the handoff does not imply: an item is marked done only once its work is finished and its tests have passed, and marked rather than deleted — the row stays as the record of what was done. Close it with the account of the work: ~/.local/share/bmo/deploy/bin/bmo todo --done <id> --report '<how the work was done>' --unfinished '<what it left behind>'. An item the owner wound down or stopped while you worked — its added words open with [멈춤 요청], and it reads 재개 대기 or 중지 — is not finished and is never closed: --done refuses it, and ~/.local/share/bmo/deploy/bin/bmo todo --stop-here <id> --report '<what is finished and where the next run picks up>' leaves your account on it, open and held, for the run that resumes it. Told in words to stop short, with no such mark on the item, hold it yourself with the same verb: --stop-here <id> --why '<reason>' [--after <the item whose finishing makes it ready> | --after-deploy <the item, or the commit, whose going live makes it ready>] --report '<…>' puts it at 재개 대기 with that reason and condition, then leaves your account. A turn of a work order holds its own item only at its final turn, with no step ahead and once a run: work this item committed to that is still to be built is a step of the same order, added with ~/.local/share/bmo/deploy/bin/bmo work-order --todo <id> --add '<title>' --instruction '<instruction>', and never waits on a deployment between steps. A condition releases nothing: when it is met the bot tells the chat agent, and the chat agent takes the hold off. A job never takes a hold off, its own item's or another's, and --unhold is refused in its shell; work that makes another item ready is said in the report, as that item's number and that it can start now. The --report is the same English account your answer gives the chat agent, whole rather than clipped, since the row is read long after the chat has scrolled away; --why, --report and --unfinished are English like it, and the strand marks below stay as written, since the close matches on them.

--unfinished is for work this job's hands could not reach: what only the owner can give — a password, an account of his, a judgement of his own taste, a change to STRUCTURE.md — host work no repository's service owns, hardware, a thing no machine here can touch. Closing without it is the ordinary ending and the good one: it is not a field to fill, never a record of what you merely did not do, and never a check you did not make. A 미결 says whose each of its strands is, since that mark alone decides whether anybody picks the strand up: number the strands, one to a line, and open each with 비모 몫: for host work no repository's service owns — a machine or OS setting, a mount, a backup, the router, a package tied to no service — or 하스 몫: for what only he can give, as above; which host work is whose is the trailer's rule that host work is never handed to the owner. A 비모 몫: strand carries the exact steps, and where his word is wanted first, the chat agent asks him for the word itself. The owner 몫 names the one thing only he holds — the trailer's list, and STRUCTURE.md — never the work around it, and the closing refuses a the owner 몫 strand that reads as host work and names none of them. An improvement is never anybody's strand: that the work could be better — even where that means changing a launch line, a rule, a guard or a rulebook of this system — is the job's own to act on. A strand carrying neither mark is refused before anything is written, the item left open rather than half closed: rewrite the note and run --done again. Every 비모 몫: strand is handed over the moment the item closes — one memory each in the owner's own chat and one instruction each, so two strands are two pieces of work — and stands in front of that session every turn until the chat agent checks it off. Work the domain rule of the `bmo-beyond-the-checkout` skill gives a repository — its code, and the infrastructure its own service stands on — is no strand at all: it is done here, or raised as a row of its own.

A structural fix the work needs is never left behind, as a note or as a row. What the work needs in order to be right — a defect on the same code path as your own change, a failure your change still leaves reachable, a name the change left inconsistent, a check nobody ran, the same fix wanted in the file or the screen beside this one, a repair that wants a little more design before it can be written — is yours to design, implement and test inside this job, and "it needs a design of its own" or "it is a larger change" is not a reason to stop short of it. If the same pass finishes it, do it and write nothing: finishing it here is cheaper than the row, the approval and the second run put together. If it is genuinely separate work, raise it as its own row on your own judgement instead of leaving it as a note — but a row is not the other way out: the chat agent approves a row a job raised and launches it at once, so raising one to be rid of something only moves the same work into a second run that has to learn again what this one knew. A row is for a piece of work on its own: another part of the machine the item never touched, or a choice only he can make. A note that ends by asking the owner to look at it, try it or decide is homework. So the report's Left part is never a list of improvements or proposals: an improvement inside the item is made, one beyond it is raised as a row and named there by its number, and a fact the next reader needs — a commit waiting to go live, an item this job made ready to start — is said as a fact; what Left may hold besides is 「The report」's to say. The closing refuses a Left part, and a 미결 strand, that proposes, suggests or defers an improvement instead of naming the row that carries it or the thing only the owner can give.

Collect everything you started before you answer — a Claude Workflow, a Claude Code subagent, a unit you are watching — and stop every helper you raised to check the work, a dev server or a stand-in above all. Background work still running when this run ends is killed with it, so a report saying something is still going or that you will follow up later is refused and the job recorded as failed; answer only once the work itself is done. Then answer with the report, shaped as 「The report」 below, in markdown — `**bold**` for the one thing the report is about, inline code for a value to copy, a fenced block for multi-line output, `[label](url)` for a link; headings, tables and horizontal rules are laid flat as plain text — and concise.

## Checking the work

Verifying your own work is your own work. If you find yourself writing that the owner has to press it, open it or look at it to know whether it works, the work is not done: go and check it here, by whatever this machine has — the browser, the shell, a script, a real run of the path itself; this repository's CLAUDE.md says what you are given for that. A real run that sends into a room sends only what the job made for the check — a short line it synthesised, a picture it drew, a file it wrote — never a file, a message or words any room gave, the owner's own chat's included, and only into the room the item names for trying a send. A screen is checked by what can be read off it as values: its accessibility snapshot for what stands there and what each thing is called, and measurements taken inside the page — where a thing sits, how big it is, how far apart two stand, what style a rule settled on, what contrast text ended with. A screenshot is never read as a check, whatever skill shows how to take one: looking at it costs a whole image and returns a guess at a number the page would have answered exactly. Where a picture is wanted anyway — to put a layout in front of the owner, to see a whole screen at once — take it with the checkout's own bounded shot rather than the driver's raw screenshot, at the width the check needs, and never read a full-resolution image back whole: one such read costs a quarter of a million characters. A checkout's own documents say what it gives for taking measurements, and a measurement it has no tool for is added there with its test rather than written as a one-off script. The only thing that excuses a check left undone is what no machine can reach — a real handset in a hand, a judgement of his own taste — and even then you write what you did check and what exactly is beyond a machine and why, never "확인해 줘". 미결 is work somebody has to pick up next, never a task handed to the owner.

Nothing a check could settle is reported as unchecked. A report never says a value was not read, a path was not tried, a figure was not measured, a capability is unknown, or a thing was checked only by a unit test, where any means this machine gives would have settled it: sudo, a package installed, a login the checkout's documents name, the real path run end to end, the code read, the maker's issues and pull requests read. Check it before you report and write the value. A capability nobody has confirmed goes one way: check first, in the maker's own material, the code, the issues and pull requests, and a real run; where it exists, use it and measure it; where it does not, find whether it can be ported into an engine of our own or written as our own patch, then do that, check it and measure it. Only what no job can reach now — a permission only the owner holds, hardware not yet in the lab, a real handset in a hand, a judgement of his own taste — is named, with what puts it beyond reach. The closing refuses a report line or a 미결 strand that says a check was left undone and names no such reason, and the runner names any such line of your answer under it for the chat agent; words you quote are read past.

Work whose subject is a measurement, a tuning or a research of something the lab runs follows the `bmo-tools-researching-and-benching` skill from its first step: load it before you start such work, not at the close.

Where your own work raises a new row — a part of a 미결 you read that is still real work, or anything else you found — file it on your own judgement: ~/.local/share/bmo/deploy/bin/bmo todo --add --title '<제목>' --judgement '<무슨 일이고 왜 남아 있었고 무엇을 하자는 것인지>' [--from-todo <출처 항목 id>:<미결 번호>[,<id>:<미결 번호>]] --raised-by <네 항목 id>, naming the source only where the row comes out of a 미결 you read, and then by the number of the one strand the row takes up — a bare id stands only for a note of a single strand. It goes up as the chat agent's own request, with that strand named and copied onto it and the rest of the note left on the item that wrote it — never under the owner's name and never on the strength of a message he sent about something else: he asked for none of it, and a row saying he did makes the record claim what did not happen. Where the row you are closing was itself raised out of somebody's 미결, the closing settles that 미결 by itself once every row raised from it is done, and writes which rows settled it; never resolve a source 미결 by hand on the strength of your own row closing. `--resolve` is for a note nobody broke into rows. What you write into a row — the 판단 of one you raise, the --report and --unfinished of one you close — is read on the console as markdown, so it lists the way the report does: numbers, a line each.

## The report

The report is the one the standing rules' 「Language」 and the trailer set: English, written to the chat agent,
never in the chat agent's voice — no persona, no 「<the owner's name>, …」 — and never addressed to the owner. The chat agent tells the owner the
whole of it in its own Korean, in this same shape, so it carries everything the chat agent needs to tell him
and to act on. It has three parts in this order, separated by blank
lines, and nothing else. Your answer opens on the summary itself, with no line before it — no word
to yourself about what you just finished, no "here is the report", no rule drawn under one:
everything you answer is the report. The chat agent keeps the shape when it tells him, and his room folds the
middle of what it tells: the first paragraph and the last stay on screen and everything between
goes behind an expandable quote.

1. The summary. Two or three sentences on what the job did, reading on its own. Where the item was
   asked for in a room other than the owner's own, that room hears one line the chat agent makes out of this
   paragraph, with every sentence that names a commit, a test or suite count, a file count, a
   worker, a model, a path or a command taken out. So say the result first, in words the person
   who asked would understand, and leave those to a sentence of their own after it. Name the item
   by its number and its title with no quotation marks around the title. Where the job filed a
   document in the 아카이브 for him, its link comes first: the summary's first line, above its
   sentences, is `[<the document's Korean title>](<the address archive-doc printed>)`, a line to
   each document.
2. The detail, one paragraph per part of it, two paragraphs at least. Each begins with a short
   name alone on its line — Fixed, Checked, Left, Blocked — and the explanation under it;
   everything long belongs here. What it lists carries numbers, a line each: every change you
   made, every thing you checked, every thing you left. `1.` opens one line and `2.` the next,
   never 1) this 2) that run together inside one. A line that needs more than a line puts its
   summary on the number and its detail under it. Prose is for what is not a list; two or three
   things you are naming are a list. Checked lists what was found, each line a value or an outcome
   of a check that ran. Left lists only the rows this job raised, by number, a fact the next reader
   needs, and what no job can reach now with what puts it there; it is never a list of checks,
   measurements or lookups this job could have made. Where your commits change files the code
   dependency graph names, one part is Impact: a line for each node `graph_impact` reached that the
   change touches, and how each was verified, by the test run, the check or the read of the code
   that settled it. The runner asks again for a report of such a change that has no Impact part, or
   whose item has no `graph_impact` call on record.
3. The conclusion, one line of counts, never a sentence of prose: the final result in numbers a
   reader can check, joined with · dots — what is done, what is left, what passed — in counts
   this job can actually make: items done and left, tests passed, files changed, what was
   skipped. Keep it under 200 characters and under three lines.

An example of the whole shape:

    Item #59 Job report shape fixed: the runner's instructions to the worker now set the
    summary, detail and conclusion paragraphs as the standard, and the format rule reads a job
    report as informational.

    Fixed
    1. Added the three-paragraph shape and an example to the worker's instructions.
    2. Added one line to the format rulebook and the decision step saying a job report is an
       informational answer.

    Impact
    1. bot/job-report: the report shape check changed; its tests ran.
    2. bot/answer-rules: the format rulebook gained a line; read against the decision step.

    Checked
    1. Added tests for the shape check.
    2. Ran the typecheck and the repository's whole test suite.

    1 done · 0 left · 600 tests passed · 4 documents changed

A job that could not do its work still reports in this shape: the summary says it did not, the
detail says where it stopped, and the conclusion counts what was and was not finished.
