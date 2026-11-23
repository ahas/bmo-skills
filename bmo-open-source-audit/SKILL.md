---
name: "bmo-open-source-audit"
bundle: coder
version: "1.0.0"
description: "What a public BMO repository must never carry and how to check before a commit, a push or a repository going public: secrets, personal data, internal addresses, and the names of the owner's other projects and machines, read from a private list kept outside every repository. Use before committing in any public BMO repository, before changing a repository's visibility, when writing tests, fixtures, docs or examples that need a sample value, and when a scan finds a hit."
---

# Open-source audit

The BMO repositories are published under MIT. Anything committed to a public repository is public for good:
a later commit does not take it back, and the history keeps it. So nothing below goes in, ever — not in code,
tests, fixtures, docs, commit messages, branch names or file names.

## What never goes into a public repository

- **Secrets:** keys, tokens, passwords, cookies, session ids, private keys, connection strings with credentials,
  and any file holding one (`.env`, `*.token`, `*.pem`, `*.key`). Secrets live under the data root's `env/` and are
  handed to their consumer by path; a test uses an obvious stand-in (`test-token`), never a value copied from a
  real file.
- **Personal data:** a real person's name, phone, address, mail address, chat id or account, other than the two
  values each repository's personal-data check allows (the owner's handle and his one public mail address).
  Samples are invented: a name such as One, a mail address at `example.com`, a phone number of all zeros.
- **The owner's private room and chat ids,** and any real room, group or chat number.
- **Internal addresses:** the lab's and the home network's addresses and host names. A sample address comes from
  the ranges set aside for documentation (RFC 5737 for IPv4, RFC 3849 for IPv6) and a sample host is under
  `example.org`.
- **The owner's other projects, machines and employers,** by name. The list is private and is not written in any
  repository: read it from `$BMO_DATA_DIR/env/oss-forbidden-names` (one pattern per line, `#` comments, a line
  `name => replacement` giving the neutral word to use). Use the replacement it names, so every repository agrees.
- **Services kept private,** by more than their bare existence: the company-specific services listed in the same
  file under `[private]` are not described, linked or depended on by a public repository.

## How to check

1. **Before each commit** in a public repository, run the repository's own check (`bmo check`), whose
   personal-data step holds the allowed values, and then the forbidden-names scan over what you are committing:
   `git diff --cached -U0 | grep '^+' | grep -i -E -f <(sed '/^\[private\]/,$d; /^#/d; /^$/d; s/ *=>.*//' "$BMO_DATA_DIR/env/oss-forbidden-names")`.
   No output is the pass.
2. **Before a repository goes public, and after any doubt,** scan its whole history:
   - secrets: `gitleaks git --redact <repo>`; a finding is checked against the real secret files without printing
     it, and a test stand-in is no finding;
   - patterns: the platform's privacy scan in the umbrella repository's `tools/privacy-scan` folder, run on the
     checkout (it prints counts only);
   - names: `git log --all -p -U0 | grep '^+' | grep -i -E -f <the list>`.
3. **A hit in the tree** is replaced in a commit of its own with the list's neutral word or a documentation-range
   sample, and the check run again. **A hit in the history** of a repository that is or will be public needs the
   history rewritten: that is the owner's step (a forced push rewrites a repository's history); prepare the
   rewritten copy and its verification, and hand him the one command.
4. **Never print a secret** while checking: compare values in a program and print only where they matched.

## What may stay

- English words that match a name by chance (an emotion called joy, a verb), upstream code of a fork and its
  own history (bmo-note carries AFFiNE's), and names the list marks as public (the account framework the console
  is built on, the hub, the platform's own domains for its public site and console).
