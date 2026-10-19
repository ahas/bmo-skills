---
name: "ahas-code-refactoring"
bundle: worker
version: "2.3.0"
description: "The module-and-function pass of a refactoring: finding the code smells in a unit and applying the named refactoring each smell asks for, from Martin Fowler's catalogue and its smells. It also carries the rule that a name never becomes a wrong name later: every name, of a file, a unit, a key or a table alike, says what the thing is, never a fact that can change (where it runs, who uses it, an account, an address), and the fact itself is read from its source, never fixed into code. Use second, after the ahas-code-design-patterns pass has settled the structure and before the ahas-code-clean pass, whenever code is being restructured without changing what it does, and before naming anything new."
---

# Refactoring — The Module And Function Pass

## Where this pass sits

- **Second of three: design patterns, then refactoring, then clean code.** The order, why it runs
  by reach, and the rule never to start later or reorder are in
  `ahas-code-design-patterns` 「Where this pass sits」. This pass moves modules, classes, functions and
  statements inside the structure that pass settled.
- **If the structural verdict is not in hand, go back and get it.** The names this pass leaves are
  provisional; the ahas-code-clean pass judges them.

## What refactoring is

- **Structure changes, behaviour does not,** and the tests say the behaviour is the same.
- **A sequence of small, named, individually safe steps.** A large rewrite between two test runs is
  not a refactoring, and no step or commit mixes in a behaviour change.

## The safe step, every time

1. Make one small step.
2. Run the static type check and the tests the step touched. The type checker is the second net:
   turning a `let` into a `const` first lets it catch reassignment, and it catches a use before
   declaration after a statement moves.
3. Green, the step stands; red, undo it rather than debug forward.

- **Prefer the editor's automated moves** — rename, extract function or constant, move to file —
  and step by hand only where none exists.
- **Migrate rather than switch anything with many callers or a public API:** create the new form,
  make the old one delegate to it, move callers one at a time, delete the old one last — or keep it
  as a `/** @deprecated */` delegate where outside callers exist.

## How the pass runs

1. **List the unit's smells** from `references/smells.md`; each names the techniques that answer
   it, by the condition that picks one.
2. **Order them by what blocks what.** Scattered code often has to be gathered — inlined, slid
   together — before it extracts cleanly.
3. **Pick the technique** for each from `references/catalog.md`, and name it in the plan.
4. **Carry each out by its steps,** checking between every one.
5. **Stop when the smells are gone,** not when the code looks different.

## The smells

- **The checklist is `references/smells.md`:** the second edition's twenty-four in the book's
  order, and five first-edition smells still in common use — Switch Statements, Parallel
  Inheritance Hierarchies, Incomplete Library Class, Inappropriate Intimacy, Dead Code.
- **The five groups — Bloaters, Object-Orientation Abusers, Change Preventers, Dispensables,
  Couplers — are a later taxonomy, not Fowler's,** whose list has none; the reference gives them as
  a reading aid only.

## Names that stay true

- **A name is chosen so that it never becomes a wrong name later.** It says what the thing is — its
  role, its work — never a fact that can change: where it stands, who uses it, which account it runs
  under, which address it answers on. When that fact changes, the name is still true.
- **Every name:** files and folders, modules, functions, variables and constants, services and
  units, environment variables, keys, tunnels and tables.
- **The test:** picture the fact changed, and someone meeting this name for the first time after.
  If the name would mislead them, it is a wrong name already. A text model's engine is
  `local-llm`, for what it is, never the node it was first installed on, which another node would
  make a lie.
- **The ahas-code-clean pass judges every name by this rule too;** its entry for names points here.

### Keeping the fact out of the code

- **The changeable fact is not fixed into the code either: the code reads it from its source** —
  the registration, the configuration — when it runs, and is handed it as a value. A fact the code
  only reads is one no name needs to carry, and its change touches nothing but its source.
- **No constant, default or fallback holds it.** Replace Magic Literal ends at a constant only for a
  value fixed by its nature.
- **The check:** change the fact at its source, and nothing else should have to change. Code, a name
  or a test that has to change with it holds the fact fixed.

### Unwinding a name that already lies

1. Make the code read the fact from its source, with today's value set there, so nothing breaks.
2. Rename what was named after the fact for its role, every reader moved in the same commit.
3. Leave no alias on the old name: the deprecated delegate a migration keeps would keep the fact in a
   name.

## The catalogue

`references/catalog.md` holds the sixty-six techniques with motive, safe steps, inverse and
TypeScript shape, in seven groups: a first set of everyday moves, encapsulation, moving features,
organising data, simplifying conditional logic, refactoring APIs, and dealing with inheritance.

- **Ten pairs undo each other,** each entry naming its inverse; the direction is what the smell
  asks. Extract ↔ Inline Function and Class, Hide Delegate ↔ Remove Middle Man, Replace Parameter
  with Query ↔ Replace Query with Parameter, Replace Type Code with Subclasses ↔ Remove Subclass,
  Move Statements into Function ↔ to Callers, Replace Function with Command ↔ Command with Function,
  Change Value to Reference ↔ Reference to Value, Pull Up ↔ Push Down Method and Field.

## TypeScript, across the catalogue

- **Encapsulation:** `#private` or `private`, `readonly`, `get` and `set`, `ReadonlyArray`.
- **Polymorphism:** a class hierarchy where new types keep arriving, a discriminated union with a
  `never` exhaustiveness check where new operations do.
- **Constants and closed sets:** `as const` values and literal unions.
- **An exported name** changes by migration, the old name kept as a deprecated delegate or a
  re-export alias while callers move — except a name that carried a changeable fact, which keeps no
  alias (「Names that stay true」).

## What this pass hands on

- The unit restructured, every step checked, the techniques applied listed by name; names,
  comments and line-level shape left for the ahas-code-clean pass.
