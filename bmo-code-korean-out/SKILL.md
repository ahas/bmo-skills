---
name: "bmo-code-korean-out"
bundle: worker
version: "1.1.0"
description: "Taking the Korean out of a bundle of code files as the Korean-in-the-code ledger asks: a fixed sentence a person reads into the bot's language pack, text only an agent reads and comments into English, test input into neutral English or a check by key, data left alone. Use when an item hands you a bundle of files or a directory to clear of Korean, or before writing any Korean into a code file."
---

# Moving Korean out of code

The code is written in English. bmo-web's `docs/language-pack.md` is where the words go and how they
are read; bmo-testbed's `docs/checks.md` (「The quick checks」, `korean`) is the ledger that holds every
code file to the Korean it still has. Read both before the first change. Close one bundle — a directory or a short list of files —
before taking another, so merges stay small.

## Sort every Korean line first

List the bundle's lines with `bmo check korean <path prefix>`, each classed as a comment, test input, a console's word, a literal elsewhere, or a language no parser here reads.
Then sort each literal by who reads it:

1. **A person reads it** — a line in a room, an answer or a refusal the owner or the chat agent sees, a note
   in a run's log, a commit message the bot writes, a label on the console handed out of the bot. It
   goes to the bot's language pack.
2. **Only an agent reads it** — a prompt, a tool's answer to a job, a usage line a job reads. It is
   rewritten in English in place. No pack, no policy.
3. **The code reads it** — a pattern matched against Korean input, the words a room types to a game,
   Korean grammar such as choosing a particle, a site's label the code parses. It stays. A list of
   such words the owner may change belongs in the shared policy, never in the pack.
4. **It is data** — a room's or a person's name, a recorded message, anything in a JSON, YAML or TOML
   data file, a seed, a snapshot or a fixture. It stays exactly as recorded. Never translate data.

## Bundle kind A: a fixed sentence a person reads

1. Find the feature's part in bmo-web's language pack, `server/lang/ko/` of that repository, or add
   one file named for the feature. Rules for a part and a key are in bmo-web's
   `docs/language-pack.md` (「A part is a file」): one sentence to a key, `{name}` marks, every key
   commented, particles chosen by the code.
2. Move each sentence there word for word. The line a person reads must not change.
3. On the console side read the part the way bmo-web's `docs/language-pack.md` 「How each side reads
   it」 says. Do not name the import the same as a local variable of the module.
4. On bmo-web's server the part gets a module in `server/src/lang/` with the `part!` macro, every
   mapping denying unknown keys, and the server fills it with its `fill_marks`. Both sides read the
   one file; neither keeps a copy.
5. Another unit never imports the pack: it answers with an English message and a status code, and
   a line a person reads is rendered by bmo-web, as its `docs/language-pack.md` 「What other units
   read」 says.
6. Repair the document describing the feature where it says where its words stand.
7. Tests asserting the moved line check it by key (bundle kind B).

## Bundle kind B: test input and expectations

1. An expected line a person reads is built from the pack: import the part, fill the key with the
   values the test gave. The test then survives a wording fix.
2. Input that is not about Korean becomes neutral English: names like Bob, Carol, Dave, rooms like
   Alpha Room, words like hello. Real people's or rooms' names and ids never stand in a test.
3. Input the behaviour under test reads as Korean — a particle helper, a Korean word list, a Korean
   site's page — stays Korean, with neutral Korean names (홍길동, 철수) instead of real ones.
4. Run the test file and its twin, if any, before and after; the count of passing tests stays.

## Bundle kind C: a comment

1. Rewrite it in English. The owner's name becomes the owner. A Korean term quoted only to name a thing is said
   in English; a doc section's title cited in 「」 may stay as written.
2. Change no code in a comment bundle.

## Bundle kind D: a language no parser here reads

Python, shell, Kotlin and C# are counted line by line. Sort each line as above: comments and agent
text become English, a person's sentence goes to the console's or the bot's pack only where that
program can read it, and otherwise is left with its reason written beside it as a present
constraint.

## Bundle kind E: text only an agent reads

Rewrite it in English where it stands, keeping every command, path and identifier exactly. Update the
test expectations and the document that quotes it.

## Checking and closing every bundle

1. `bunx tsc --noEmit`, and `bmo check quick`: its key check fails on a key the pack or the
   policy lacks, its dash and wording checks read the pack. For Rust:
   `bmo cargo clippy -p <crates> --all-targets -- -D warnings` and `bmo cargo fmt`.
2. `bmo check korean --settle` lowers the ledger; commit it with the change.
3. The tests the change touched, one file to a process: `bun test <file>`; the suite runs each file
   alone, and so do you.
4. Update the graph nodes of every file whose words moved, and close as `bmo-closing-a-job` says.

## Example commits

- A: the running report, the TODO readings and verbs, the word chain, a work order's edits and
  checkout hold, a job's question, the resident turn's notes, the control routes' refusals and the
  Telegram message kinds moved into the pack — the commit titled 「#1479 사람이 읽는 문장을 언어팩으로」.
- B: `chat/chat/kinds.test.ts` checking by key with neutral input, in the same commit.
- C: the commit titled 「#1479 예시(주석): 주석의 오너를 the owner로」.
- D: the commit titled 「#1479 예시(읽지 못하는 언어): KakaoChatServer.cs의 주석과 메시지를 영어로」.
- E: the tool traffic reading's report and usage in English, in the commit titled
  「#1479 TODO 읽기·동사와 툴 트래픽의 한국어 정리」.
