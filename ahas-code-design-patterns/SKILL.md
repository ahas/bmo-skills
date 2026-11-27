---
name: "ahas-code-design-patterns"
bundle: coder
version: "2.2.0"
description: "The structural pass of a refactoring: judging whether the shape a piece of code already has is the right one for what actually varies in it, across the GoF patterns and the enterprise, dependency-injection and functional patterns beyond them. Use first, before the refactoring and ahas-code-clean passes, whenever code is to be restructured — the owner calls it 리팩토링, and 정리, 클린코드 and reviewing a design mean the same — a new module's shape is being decided, or an existing structure is under review."
---

# Design Patterns — The Structural Pass

## Where this pass sits

- **Three passes run in a fixed order: design patterns, then refactoring, then clean code.** The
  order is the owner's own and is strict. Never start at a later pass and never reorder them. A change
  that restructures nothing reads none of the three.
- **The order is by reach.** A pattern settles whole modules and the seams between them, a
  refactoring moves a module, a class or a function, an ahas-code-clean rule settles a name, a line, a
  comment: widest first. Out of order, the later passes are wasted: names polished and functions
  extracted inside a structure that turns out wrong, or names polished inside a function the
  refactoring pass then splits, are thrown away with them.
- **This pass changes nothing by itself.** It gives a verdict per site, and the refactoring pass
  carries it out: a pattern is installed by small refactorings, never by a rewrite.

## What this pass is for

- **Never "which pattern can I add".** For each place where behaviour varies: what varies, who owns
  the variation, and whether the shape there is the one those forces ask for.
- **The problem first, never the pattern name;** then the TypeScript primitive that answers it (a
  function, a union, a module), and a class-based pattern only where those fall short.
- **Most verdicts are "leave it":** a structure fighting nothing is right, whatever it is called.

## How the pass runs

1. **List the axes of variation,** one line each: everything that differs per type, case, consumer
   or environment.
2. **Name who owns each axis today:** a branch on a type tag, a table keyed by concrete types, a
   flag argument, an `if` ladder, a subclass.
3. **Name the pattern the code is a bent version of:** an `if` ladder over a tag is a bent Strategy
   or State, a wrapper adding behaviour a Decorator or Proxy, an object assembled field by field a
   Builder.
4. **Check the forces against that entry's "use when" and "not when".** A mismatch either
   way — a pattern the forces do not ask for, or forces fighting a missing one — is the finding.
5. **Write the verdict per site,** naming the axis it answers: keep; replace with a named target
   shape; or remove the pattern and inline it.
6. **Hand the sites to the refactoring pass** with the target shape named, not the diff.

## The cost, which is always paid

- **A pattern buys flexibility with indirection.** Where the flexibility is not needed, only the
  indirection lands: more interfaces and classes, harder tracing.
- **The catalogues warn against themselves.** Strategy over two rarely changing algorithms needs
  only a set of functions; Abstract Factory multiplies interfaces and classes past the problem;
  Singleton breaks single responsibility, hides bad design and resists testing; Visitor makes every
  visitor change when an element type is added or removed.
- **Most of the catalogue is a missing language feature.** With first-class types and functions,
  sixteen of the twenty-three GoF patterns become invisible or simpler, and Abstract Factory,
  Flyweight, Factory Method, State, Proxy and Chain of Responsibility vanish outright. TypeScript's
  first-class functions and modules make that hold here.
- **One case is not a family.** An abstraction built before the second case exists is Speculative
  Generality, which the refactoring pass tears out.
- **A pattern name in a type name is not a pattern:** `FooFactory` or `BarStrategy` over one
  concrete thing is naming, not structure.

## What TypeScript already gives

- **Structural typing:** an adapter or a role interface is an object literal or a type, not a class
  with `implements`.
- **Functions and closures:** the one-method Strategy, Command, Template Method, Decorator and
  Observer are a function, a function passed in, a composed wrapper, a callback list.
- **Discriminated unions with an exhaustive `switch`:** State, Visitor, Interpreter and serialisable
  Command collapse into a union with a literal tag, exhaustive through `never`, so a new member makes
  every unhandled `switch` a compile error.
- **Modules:** a top-level binding is a singleton; an `index.ts` exporting a few functions is a
  Facade.
- **Built-ins:** `Symbol.iterator` and generators are Iterator, the `Proxy` object Proxy, spread and
  `structuredClone` Prototype, `EventTarget` Observer.

## The unions-or-classes trade

- **A closed set of cases whose operations keep growing** is a discriminated union with functions
  over it: a new operation is one function, and a missing case is a compile error.
- **An open set of cases whose operations are fixed** is an interface with implementations: a new
  case is one file, and no central `switch` changes.
- Choose by which of the two grows here, and write down which it was.

## The catalogue

Each entry — problem, use when, not when, cost, what to use instead, TypeScript form — is in the
references. Open only the file for the family being judged.

- `references/creational.md`: Factory Method, Abstract Factory, Builder, Prototype, Singleton.
- `references/structural.md`: Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy.
- `references/behavioral.md`: Chain of Responsibility, Command, Interpreter, Iterator, Mediator,
  Memento, Observer, State, Strategy, Template Method, Visitor.
- `references/beyond-gof.md`: the enterprise patterns (Transaction Script, Domain Model, Service
  Layer, Active Record and Data Mapper, Repository, Unit of Work, Query Object, DTO, Gateway, Value
  Object, Special Case, Registry and Plugin, offline locks, controllers); dependency injection; CQRS,
  Event Sourcing and DDD tactics; the functional patterns; and the shared "do not use" list every
  entry is measured against.

## A change that meets running work

Every change to a setting or a state the system acts on — a model, an agent, a node, a provider, a role, a label,
a limit, a deployment, a queue's state — meets the work already running under it. Decide, for each kind of change,
exactly one of three branches, and write it in the document that states the change:

- **Run to the end, the new setting from the next run.** The default: what runs finishes as it started, and
  the change takes the next run.
- **Graceful resume.** Only where the running work cannot go on — the node under it died, its session is
  replaced, the service restarts for a deployment — and then by a checkpoint or a handover the next run takes up.
- **Stop.** Only where somebody said in so many words to stop it, or where going on is unsafe.

A change never ends running work as a side effect: a limit lowered under running jobs leaves them running and
holds the jobs past the new limit in the queue, rather than letting them fail on the next request. The verb's
answer and the notice it raises say which branch the change took.

## Verbs and notices

- **A noun the system keeps has its whole set of verbs:** create, list, read, change by `PATCH` naming only the
  fields it changes, and remove. A verb the caller may not use is refused with a permission refusal whose
  sentence says who may and what to do next, never left out or answered as not found.
- **A noun whose rows are a record is closed or archived,** never deleted: its removal verb ends it and keeps it.
- **A notice carries the fact and the next step it calls for:** the data of what changed, then one line of what
  its reader does now, computed where the system can compute it ("launch #12 on coder-01"), and the standing its
  reader needs to judge it.

## What this pass hands on

- A list of sites, each with its axis of variation, the shape found, the verdict and, where the
  verdict is a change, the target shape by name.
- A move goes to the refactoring pass untouched; a name, a comment or a line to the ahas-code-clean pass.
