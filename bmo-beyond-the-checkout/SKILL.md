---
name: "bmo-beyond-the-checkout"
bundle: worker
version: "1.1.0"
description: "What a job owns and where it may write once the work reaches past its own checkout: another repository, another worker's checkout, the infrastructure a service stands on (a public name, DNS record, proxy host, port, firewall opening, unit), or host work no repository's service owns. Use before the first change made outside the checkout the job runs in."
---

# Beyond the checkout

## What a job owns

- **Ownership is settled by the domain the work serves**, never by the directory a file lands in.
- **A service's own infrastructure is its repository's job.** Its public name, the DNS record
  carrying it, the proxy host answering it, the port and firewall opening it needs, the unit running
  it — all of that belongs to the repository whose service it stands up, and that repository's job
  does it, in the infrastructure itself, even though not one byte of it is written inside the
  checkout.
- **Host work no repository's service owns is not the job's**: a machine or OS setting, a network
  drive or a mount, a backup, the router, a package tied to no service.
  - It is named in the report, and in `--unfinished` as a strand opening with `비모 몫:` — which
    hands it to the chat agent — with what exactly is to be done; never `하스 몫:`, by the trailer's rule that
    host work is never handed to the owner.
  - The rest of the item is finished around it.
- **The owner's own word overrides this:** work the owner tells the chat agent to do by hand is the chat agent's, whatever service it
  serves.

## Where a job writes

- **A job writes only in its own checkout.** Another worker's checkout — its base checkout or one of
  its support worktrees — is that worker's, and what is left open in it is parked as its holder's
  own when that holder stops, so your files would leave with it.
- **Another repository is borrowed.** Where the work needs another repository changed, borrow one of
  its checkouts for this job with `~/.local/share/bmo/deploy/bin/bmo job borrow <worker>`: it
  hands you a free one and holds it until the job ends.
  - Work, commit and close there the way that repository closes a job.
  - A support worktree — and a base checkout whose repository has support workers — is put on a
    branch of your item, committed and not pushed, and its close lands it on main.
- The job guard refuses a write into anyone else's checkout and says the same.

## Reaching the infrastructure

- **The estate is written down and you read it rather than remember it**: the owner's infrastructure repository, which the owner's standing rules name, its
  `CLAUDE.md` first and then the document under its `docs/` that covers the area you are about to
  touch.
  - The machines and their names, how each is logged into, what a proxy host is actually made of,
    how the router is read and written, which file holds which credential — all of it stands there
    and none of it is copied here, since a copy is wrong the day the estate moves.
- **A key, a token or a password never appears in a document or in a report.** Which file holds one
  is all you need, and that file's name is how you refer to it.
- **The permission is bounded by the service it was given for.** Outside the checkout you go for
  your own repository's service and nothing else; a machine-level setting nobody's service owns
  stays the chat agent's however near it sits to your work.
- **The infrastructure repository is committed to in its own way:** a file the work leaves there is
  committed in that git repository, apart from your own commit.
- **Your account of the work goes where a job's report goes.** Whether anything is also posted to
  the room the machines' news is kept in is that repository's own rule, written there; that room is
  shut to the system's own doings, so a job in the system's own checkout posts nothing to it.
