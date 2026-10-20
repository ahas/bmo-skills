# Personal data

What this repository never carries, where a value lives instead, and what holds it out.

## What never goes in

No personal value is written into a skill, `README.md`, a document under `docs/`, the placer's
code or its comments, a check or its fixtures, or a commit message. The values are:

- a person's name, handle or account id;
- an email address;
- a host name, a domain, a device or a network address, a private IP address included;
- a home path or a checkout path that names a person;
- a room member's display name or id;
- a company name;
- a line of a real conversation, whoever spoke it.

## Where a value lives instead

A value a consumer needs is read from that consumer's configuration at run time. An example uses a
neutral name: a reserved example domain, a name such as `someone`, `node-a` or `example-host`, a path
without a person, and a made-up line rather than a real one.

## The values allowed

Two values stand as they are: `ahas`, the name this repository and its skill scope carry, and
`dev@ahas.io`. No other person, account, machine or domain is allowed.

## The check

`tests/personal.rs` reads every skill folder, the manifest, `docs/`, `src/` and the root's markdown,
and reports a value it finds by shape: an email address, a phone number, a token, a home path, an IP
address, and the chat, account, room and user ids, private hosts and account handles it holds. The
public sites, the reserved example names and the code keywords the skills teach are allowed by name.
No personal value is kept in the repository as a term list: the check goes by shape alone.
