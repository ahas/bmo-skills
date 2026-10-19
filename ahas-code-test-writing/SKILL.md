---
name: "ahas-code-test-writing"
bundle: worker
version: "1.0.0"
description: "The three rules a test is written to, since every test is paid for on every run of the suite after it: no waiting on the real clock, every process it starts taken down with its descendants, and no check made again that another test already makes. Use before writing or changing a test."
---

# Writing a test

- **A test is paid for on every run of the suite after it**, so write it to these three rules.
- **A test never waits on the real clock.**
  - Where the code under it waits on an interval or a deadline, the module owning the wait gives a
    seam — a parameter, or an environment override it reads itself — and the test sets it short, so
    what it checks still holds and only the seconds go.
  - A `sleep` standing in for a condition is replaced by a bounded wait on that condition.
- **A test that starts a process takes it down itself**, with everything that process started — a
  stand-in host, a fake CLI left sleeping, a resident program — since a descendant left running
  holds the file's output open and the suite waits on it as long as it lives.
- **A check another test already makes is not made again in another file.** A script's test checks
  what only the running script can show — its exit, its output, what it sends — and leaves the logic
  under it to the module's own tests.
