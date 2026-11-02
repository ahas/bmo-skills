---
name: "bmo-marking-a-wait"
bundle: worker
version: "1.1.0"
description: "Marking a deliberate wait on the job's TODO item so the chat agent does not take a quiet job for a dead one. Use before standing still on purpose for minutes: waiting on a unit, another process or a long training, judging or checking run, or anything one tool call will sit through for minutes."
---

# Marking a wait

- **A job quiet for five minutes is reported to the chat agent as standing still**, and the chat agent then has to look
  whether it is dead. A tool call whose own processes keep spending CPU or starting new ones counts
  as work, so a busy test run or build needs no mark; a wait where nothing of the job's own works —
  a unit, another process, a run on another machine — does.
- **So mark the wait before you stand still on purpose:**
  `~/.local/share/bmo/deploy/bin/bmo job wait --todo <id> --what '<무엇을 기다리나>' --unit <유닛> --until 40m`.
  - `--pid <pid>` in place of `--unit` for a process.
  - The end is a length or a Korean clock time (`2h`, `14:30`).
- **A wait that is one command** is handed after `--` instead of a target:
  `~/.local/share/bmo/deploy/bin/bmo job wait --todo <id> --what '<무엇>' --until 30m -- <명령…>`.
  The mark stands on that run and clears the moment it ends, its exit code passed through.
- **Otherwise clear it** with `--clear` once the wait is over.
- **The mark stands while its target lives.** A target that ends lets it go by itself. The end you
  give paces the watch, never the wait: a wait outliving it is told to the chat agent once as exactly
  that, its target still alive, and nothing ends the wait for it. So give the end you actually expect.
