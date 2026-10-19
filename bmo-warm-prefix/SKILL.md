---
name: "bmo-warm-prefix"
bundle: worker
version: "1.0.0"
description: "Changing what a running session holds as its cached prefix: a document that stands in a session's system prompt, or how an agent's Claude process is started — its launch line, model, effort, permissions, tools, system prompt and environment. Use before editing either."
---

# Changing a warm prefix

## A document in a running session's system prompt

- **It is expensive to edit.** The session holds it as a cached prefix, so writing it costs that
  session its whole context on the next turn — for a chat room carrying hundreds of thousands of
  tokens, several dollars of re-prefill per write.
- **So a job that changes such documents writes them once** — every edit together, in one pass, and
  the repository's reload for them run once at the end. Three edits over an evening buy three full
  re-prefills of every session holding it; the last version alone would have cost one.

## A launch line

- **How an agent's Claude process is started is code of the repository that runs the agent** — the
  whole launch line, from the model and the effort to the permissions, the tools, the system prompt
  and the environment.
- **A job whose item is better served by a line carrying something more changes that line as part of
  the item.** A tool an agent ought to have is given to it, never worked around with a second road
  beside the one it should have had.
- **What holds a line steady is the prompt cache,** which matches a byte-identical prefix: the tools
  shown, the system prompt and the machine configuration are that prefix, and changing any of them
  rewrites every affected session's context and costs a chat its warm cache.
- **So a line changes only where the item's work calls for it** — never to ease the job's own run,
  never as a side effect of another change — and the whole of the change goes in one commit, once,
  as a warm document is written once.
