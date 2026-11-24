---
name: "bmo-open-source-audit"
bundle: coder
version: "1.1.0"
description: "What a public BMO repository must never carry and how to check before a commit, a push or a repository going public: secrets, personal data, internal addresses, the names of the owner's other projects and machines, and any runtime default that would send an outside user's machine to the owner's own hosts, read from a private list kept outside every repository. Use before committing in any public BMO repository, before changing a repository's visibility, when writing tests, fixtures, docs or examples that need a sample value, when adding a default address, an installer, an updater or a download, and when a scan finds a hit."
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
- **A default that reaches the owner's own hosts.** The owner's servers cannot carry outside traffic, so nothing an
  outside user installs may send their machine there on its own: no code constant or compiled-in value, default
  setting, example config an install recipe copies as it stands, installer or updater feed, download or model URL,
  tool source template, plugin or skill marketplace address, telemetry endpoint, or document telling a reader to
  install from such an address. The hosts are listed in the same private file under `[hosts]`. Such a default
  becomes either no default, the program refusing to start or asking, with a message naming the setting, or the
  public service the program is meant to use (GitHub Releases, Hugging Face), whichever the unit's documents make
  right. The owner's own value moves into his deployment's settings under the data root, and is written there
  before the change can deploy, so his deployment keeps working. GitHub, Hugging Face and the static project site
  are not his servers; a link a person clicks is no runtime reach.

## How to check

1. **Before each commit** in a public repository, run the repository's own check (`bmo check`), whose
   personal-data step holds the allowed values, and then the forbidden-names scan over what you are committing:
   `git diff --cached -U0 | grep '^+' | grep -i -E -f <(sed '/^\[private\]/,$d; /^#/d; /^$/d; s/ *=>.*//' "$BMO_DATA_DIR/env/oss-forbidden-names")`.
   No output is the pass.
   Then the hosts scan over the whole tree, since a default may already stand there:
   `git grep -n -I -E -f <(sed -n '/^\[hosts\]/,/^\[/{/^\[/d; /^#/d; /^$/d; p}' "$BMO_DATA_DIR/env/oss-forbidden-names")`.
   Every hit is read: a runtime default as above is a finding and is fixed before the repository is published or
   pushed; the owner's host named in prose, in a deployment document of his own or in history is the names rule's
   business, not this one's.
2. **Before a repository goes public, and after any doubt,** scan what has not been scanned since the last
   audit: the commits since the commit the last audit of that repository recorded in
   `$BMO_DATA_DIR/oss-audit/<repository>` (one line, the commit it reached), and the repository's root files
   (README, LICENSE, its settings files) as they stand. A history once audited and cleaned is not read again;
   the whole history is scanned only where no audit is recorded, or where a doubt names it. Each scan:
   - secrets: `gitleaks git --redact <repo>`, with `--log-opts="<recorded>..HEAD --all"` where an audit is
     recorded; a finding is checked against the real secret files without printing it, and a test stand-in is no
     finding;
   - patterns: the platform's privacy scan in the umbrella repository's `tools/privacy-scan` folder, run on the
     checkout (it prints counts only);
   - names: `git log --all -p -U0 <recorded>..` (the whole log where none is recorded) `| grep '^+' | grep -i -E
     -f <the list>`.
   An audit that ends clean writes the commit it reached into that file, so the next one starts there.
3. **A hit in the tree** is replaced in a commit of its own with the list's neutral word or a documentation-range
   sample, and the check run again. **A hit in the history** of a repository that is or will be public needs the
   history rewritten: that is the owner's step (a forced push rewrites a repository's history); prepare the
   rewritten copy and its verification, and hand him the one command.
4. **Never print a secret** while checking: compare values in a program and print only where they matched.

## What may stay

- English words that match a name by chance (an emotion called joy, a verb), upstream code of a fork and its
  own history (bmo-note carries AFFiNE's), and names the list marks as public (the account framework the console
  is built on, the hub, the platform's own domains for its public site and console).
