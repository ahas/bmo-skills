---
name: "bmo-waiting-on-long-work"
bundle: worker
version: "1.2.0"
description: "How a job runs and waits on work that may outlast a few minutes — the change check, the close, a test or build through the testbed, a task launched on another service: why a quiet call in the foreground is cut, how such work is started in the background, the one bounded wait on it, and what counts as its end. Use before running any command that may run past three minutes, and before writing any wait, poll or loop that watches other work."
---

# Waiting on long work

## Why a quiet call is cut

- **Every shell call a job makes in the foreground is held to the call clock.** Past three minutes, a
  call whose processes burn no processor time, start no new process and write no output for five
  minutes is killed without asking, and the tool answers with a failed result. A call that writes no
  output and stands nearly idle for five minutes is put to the supervisor as overtime, who keeps it or
  cuts it.
- **The check, the close and a test or build run through the testbed write while they wait.** Their
  steps run on another machine or wait in the testbed's line, and the process here burns next to nothing
  meanwhile, but its output grows: the check says once a minute which step still runs, the close says
  once a minute where it stands, a testbed run waiting for its turn says its place every thirty seconds,
  and a run on another machine passes its output back as it comes and says once a minute that it still
  runs there where nothing came back, through its copies, its locks and its line there alike. So the
  clock leaves them running in the foreground, however long the work behind them takes.
- **The clock counts only output that reaches a file:** the call's own output, or a file the command
  writes to with `>`. A check piped into `tail` or `grep` writes nothing the clock can read until it
  ends, so it is cut like a silent one: send the output to a file and read the file.
- **A wait that says nothing is still cut,** a sleep, a read on a pipe or a poll that writes nothing:
  in the foreground it is killed at about five minutes, or put to the supervisor where it starts a
  process on every pass.
- **A shell started with the Bash tool's `run_in_background` is not held to the clock.** It runs until
  its command ends, and the tool sends a notice when it does.
- **A foreground wait on that work is not held either,** while the wait's command names, written out
  in full, a file that the background work still holds open for writing. The work must have started in
  an earlier message than the wait: a process started within three seconds of the waiting call is
  counted as the call's own, and the wait loses the exemption.

## Starting the work

- **Work that may run past three minutes is started with `run_in_background: true` where other work
  can go on meanwhile:** `bmo check`, `bmo job close`, a test or build through `bmo-testbed`. Where
  nothing else is left to do, it may run in the foreground instead, with the Bash tool's largest
  `timeout` and its output going to the call or to a file, since it writes as it waits. A wait on a task of another service writes nothing, so it is
  always started in the background. Work started in the background puts its output and its exit status
  into one file under the scratchpad, named by its absolute path:

  ```
  bmo check > /abs/scratchpad/check.out 2>&1; echo "exit=$?" >> /abs/scratchpad/check.out
  ```

- **The `exit=` line is the end.** The shell writes it once the command has ended, however it ended:
  passed, failed, ended as hung by the testbed (exit 124, with no `log:` line), or killed by a signal.
  A hung or failed check is an end like any other: read the file, fix what it names, start it again.
- **A command that burns processor time here the whole way,** a local build or a search over a tree,
  may run in the foreground. Where unsure, start it in the background.

## Waiting for it

1. **First wait for nothing.** Go on with the work that does not hang on the result: documents,
   lessons, the report's draft, the next step's reading. The tool's notice says when the task ended;
   then read the file once.
2. **Where nothing else is left, block with this wait,** in a later message than the one that started
   the work, with the file's path written out in full in both places and never through a variable:

   ```
   timeout 3600 zsh -c 'until rg -q "^exit=" /abs/scratchpad/check.out || ! fuser -s /abs/scratchpad/check.out; do sleep 15; done'; echo "wait=$?"; tail -n 30 /abs/scratchpad/check.out
   ```

   It ends within fifteen seconds of the `exit=` line, or of the work's last process letting go of the
   file without one, and at the latest at its bound.
3. **Read what it printed.** An `exit=` line is the work's result. No `exit=` line with `wait=0` means
   the work was ended from outside: read the file and start the work again. `wait=124` means the bound
   passed with the work still running: read the file and judge, never start the same wait again
   unchanged.

## A task on another service

- **It is waited on by the id its launch returned,** never by a time window, a name prefix or the
  newest entry: the launch's own answer is the only thing that names it.
- **Any state but the ones the service names as still going is the end,** done, failed and an id the
  service no longer knows alike. The field and every still-going state are read from the service's
  own schema before the wait is written, never guessed: a task service may name half a dozen, held,
  reserved and halting among them, and a wait that lists too few ends at the first one it misses. An
  empty state is a read that failed: look at the service before starting the wait again.
- **The wait runs in the background** like the work above, polling that one id at a bound and writing
  its end into its file, and is waited on the same way. `.status` and `(pending|processing)` stand for
  the schema's own field and its whole list of still-going states:

  ```
  timeout 3600 zsh -c 'while s=$(curl -fsS "https://service.example/tasks/<id>" | jq -r .status); [[ $s == (pending|processing) ]]; do sleep 20; done; echo "state=$s"' > /abs/scratchpad/task.out 2>&1; echo "exit=$?" >> /abs/scratchpad/task.out
  ```

## What never stands in for it

- **`pgrep -f` or `pkill -f` with a pattern:** the pattern stands in the waiting shell's own command
  line, so the loop always finds itself and never ends.
- **`tail -f` piped to `grep -m1`:** grep ends at its match, but tail lives on until the file is
  written again, so the call waits on after its line has come.
- **A line the work may never write,** such as the check's `log:`: a check ended as hung writes none.
  Wait for the `exit=` line your own command writes.
- **`$!` and `wait`:** every call is a fresh shell, so a process put in the background with `&` belongs
  to no later call, and `wait` in the same call is a foreground wait, cut as above.
- **A loop with no bound, or a `sleep` standing in for a condition.**
