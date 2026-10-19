---
name: "ahas-code-boundaries"
bundle: worker
version: "1.2.0"
description: "Where a variation belongs when the members of a family or the services on a shared base each need their own value or behavior, where a list a person edits is kept, and how a base library is contributed to. Use when factoring a common supertype, designing a framework or base package others build on, introducing a registry or lookup table, adding a branch that turns on a type name or a service name, or adding to a list — workers, rooms, rules — that the owner may later rename, add to or remove from, or when a repository needs something its base library does not serve."
---

# Design Boundaries — Who May Know What

The standing rules every job is handed state the two prohibitions. This is the working material: the
shapes that replace a misplaced table, where a registry is legitimate, and how to unwind one already
in place.

## Replacing a table with a declaration

Give the requirement to the supertype so every member is forced to answer it; a member that cannot
fails to compile, which is the point.

```
# misplaced — a central map enumerates the family
KEY_TABLE = { A: "a-key", B: "b-key", C: "c-key" }
A.key() -> KEY_TABLE[type_of(self)]

# correct — the supertype states the requirement, each member owns its answer
supertype HasKey { required KEY }
A implements HasKey { KEY = "a-key" }
```

Read the member and you know its key; add a member and you touch one file.

## Delegating instead of branching

A base that decides per consumer has taken the policy along with the mechanism. Hand the policy
back.

```
# misplaced — the base decides by the consumer's name
base.configure_permissions(service_name):
    if service_name == "first":  grant(...)
    if service_name == "second": grant(...)

# correct — the base defines the contract and a default; the consumer fills it
base declares contract PermissionPolicy { grants_for(subject) }
base.configure_permissions(policy = SafeDefault)
consumer implements PermissionPolicy and hands it in at startup
```

The default must be expressible without naming anyone; if writing it requires knowing the
consumers, the contract is drawn at the wrong level.

## When a registry is legitimate

- **An external identifier maps to a type** — a serialization tag, a route, a discriminator column.
  The outside world fixed the string, so a reverse lookup is genuinely needed.
- **Membership is unknown at build time** — plugin discovery.
- **Plain data** — translations, rate tables, fixture sets: values, not per-type behavior, and never
  a polymorphism candidate.

In the first two, each member registers itself; a central file enumerating the members is the
violation, whatever the table is for. The boundary in one line: **the owner brings its own entry, or
a central file enumerates other people's entries** — the first is a registry, the second a
misplaced variation disguised as one.

## A list a person edits is a table

- 사람이 고치고 지울 수 있어야 하는 것은 DB에 만들어라.
- 목록에 무언가를 더하는 일을 맡았을 때 — 워커든 방이든 규칙이든 — 그것이 나중에 오너가 이름을
  바꾸거나 하나 더 넣거나 지울 수 있어야 하는 종류라면, 코드에 박힌 상수나 문서에 적힌 줄이 아니라
  아카이브의 표로 만들어라.
- 코드에 박힌 목록은 그가 손댈 수 없고, 문서에 적힌 목록은 곧 코드나 표와 갈라져 두 벌이 된다.
- 코드에는 표가 비어 있을 때 쓸 씨앗만 두고, 화면과 스크립트는 표를 읽는다.
- 표에 넣지 않기로 정했으면 왜인지를 보고에 적어라.
- 이미 표로 선 목록은 — 어느 에이전트가 있고 무엇으로 불리는지처럼 — 표가 원본이니, 문서에 적힌
  목록이 아니라 표를 읽어라.

## Detection signals

- A name of a consuming project or service appears as a string inside the base layer.
- The keys of a map enumerate concrete types that live in the same codebase.
- A branch in a general layer turns on a type name or a service name.
- Adding a derived member requires editing a file other than the new one.
- Members share a value the supertype never asked for, held together only by convention.

## Unwinding one already in place

1. Add the requirement to the supertype and make it mandatory, so nothing can be forgotten silently.
2. Move each row into the member it describes, deleting the row as you go.
3. Delete the table only once it is empty: a half-emptied table is two sources that disagree.
4. For a base branching on consumer names: declare the contract, give it a default that names no
   one, move each branch into the consumer it names, then remove the names from the base.
5. Confirm with the test in the standing rules — add an imaginary member and count the files.

## Contributing to a base library

The same boundary seen from below: a repository built on a base library meets something the base does
not serve.

- **The missing piece is sent to the base,** as a pull request to the base's repository. The lower
  repository keeps no copy of its own, temporary or otherwise, and no workaround beside the base; it
  holds only the part of its work that waits on the piece and carries on with the rest.
- **The pull request carries nothing of the sender's:** no repository, provider or unit name, no branch
  for one consumer, no data or fixture only the sender uses. The feature is written as the general case,
  with the base's own tests for it, and the sender's need is one use of it.
- **It is checked twice:** by the sender against the list above before it is opened, and by the base's
  maintainer in review, who merges it or returns it with the reason. A returned request is fixed and sent
  again, never kept as a local copy instead.
- **A merge raises the base's semantic version,** and the sender pins that version and takes up the part
  it held.

## Sharing a utility is a different thing

A shared helper location collects behavior that does not vary per member — cohesion, unrelated to
this rule. Once something there has one row per concrete type, it is a registry, and the rest of
this skill applies.
