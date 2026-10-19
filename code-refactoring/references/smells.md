# Code Smells And What They Call For

Each smell is a shape naming the techniques that answer it, each chosen by the condition written
before it. The group after a name is the later five-group taxonomy, a reading aid only; the second
edition has no groups, and its four new smells belong to none.

## The twenty-four of the second edition

- **Mysterious Name** — a name does not say what the thing does. Change Function Declaration,
  Rename Variable, Rename Field.
- **Duplicated Code** (Dispensables) — the same structure in several places. Identical: Extract
  Function. Similar: Slide Statements, then Extract Function. In sibling subclasses: Pull Up Method.
- **Long Function** (Bloaters; formerly Long Method) — the longer, the harder to understand. Mostly
  Extract Function. Many temps: Replace Temp with Query. Many parameters: Introduce Parameter
  Object, Preserve Whole Object. Still stuck: Replace Function with Command. Conditionals: Decompose
  Conditional. The same switch repeated: Replace Conditional with Polymorphism. A loop doing two
  things: Split Loop.
- **Long Parameter List** (Bloaters) — too many arguments. Obtainable from another argument: Replace
  Parameter with Query. Values pulled from one object: Preserve Whole Object. Always together:
  Introduce Parameter Object. A flag: Remove Flag Argument. Shared by several functions: Combine
  Functions into Class.
- **Global Data** — changed from anywhere, traced from nowhere. Encapsulate Variable, then narrow
  its scope.
- **Mutable Data** — change causes unexpected bugs. Encapsulate Variable. Reassigned for another
  purpose: Split Variable. Side effects separated: Slide Statements with Extract Function; Separate
  Query from Modifier. Remove Setting Method; Replace Derived Variable with Query. Combine Functions
  into Class or into Transform; Change Reference to Value.
- **Divergent Change** (Change Preventers) — one module changes for several reasons. A sequence:
  Split Phase. Calls back and forth: Move Function. Two processes in one function: Extract Function.
  In a class: Extract Class.
- **Shotgun Surgery** (Change Preventers) — one change means edits in many places. Move Function,
  Move Field. Working on the same data: Combine Functions into Class. Transforming or enriching:
  Combine Functions into Transform. Split Phase. Gather with Inline Function or Inline Class, then
  extract again.
- **Feature Envy** (Couplers) — talks more to another's data than its own. Move Function; only part
  of it: Extract Function, then Move Function.
- **Data Clumps** (Bloaters) — the same three or four items travel together. As fields: Extract
  Class. As parameters: Introduce Parameter Object, Preserve Whole Object.
- **Primitive Obsession** (Bloaters) — money, ranges, phone numbers as primitives. Replace Primitive
  with Object. A type code steering branches: Replace Type Code with Subclasses, then Replace
  Conditional with Polymorphism. Clumps: Extract Class, Introduce Parameter Object.
- **Repeated Switches** (OO Abusers; formerly Switch Statements) — the same branching in several
  places. Replace Conditional with Polymorphism.
- **Loops** — a loop where a pipeline reads better. Replace Loop with Pipeline.
- **Lazy Element** (Dispensables; formerly Lazy Class) — a function or class not pulling its weight.
  Inline Function, Inline Class; for a subclass, Collapse Hierarchy.
- **Speculative Generality** (Dispensables) — structure for a future nobody uses. Collapse
  Hierarchy; Inline Function, Inline Class. Unused parameters: Change Function Declaration. Used
  only by tests: Remove Dead Code.
- **Temporary Field** (OO Abusers) — a field filled only in some situations. Extract Class, then
  Move Function; the conditional code: Introduce Special Case.
- **Message Chains** (Couplers) — `a.b().c().d()`. Hide Delegate; better, Extract Function then
  Move Function to push the use down the chain.
- **Middle Man** (Couplers) — half or more of the methods delegate. Remove Middle Man; for a few,
  Inline Function; where behaviour is added, Replace Superclass or Subclass with Delegate.
- **Insider Trading** (Couplers; formerly Inappropriate Intimacy) — modules trading data behind the
  scenes. Move Function, Move Field. A shared concern: a third module, or Hide Delegate. Collusion
  through inheritance: Replace Subclass or Superclass with Delegate.
- **Large Class** (Bloaters) — too many fields, too much code. Extract Class. Where inheritance
  fits: Extract Superclass, or Replace Type Code with Subclasses. Remove duplication inside it.
- **Alternative Classes with Different Interfaces** (OO Abusers) — classes that should be swappable
  differ in interface. Align with Change Function Declaration and Move Function; duplication that
  results: Extract Superclass.
- **Data Class** (Dispensables) — fields and accessors only. Encapsulate Record; Remove Setting
  Method. Move the logic using it in with Move Function, extracting first where needed. An
  immutable result record is the exception.
- **Refused Bequest** (OO Abusers) — a subclass refuses what it inherits. A sibling class with Push
  Down Method and Push Down Field. Refusing the interface too: Replace Subclass or Superclass with
  Delegate. A faint smell can be left alone.
- **Comments** (Dispensables) — deodorant over bad code. Extract Function; Change Function
  Declaration; a stated rule becomes Introduce Assertion. A comment saying why, or what is
  uncertain, stays.

## First-edition smells still in common use

- **Switch Statements** (OO Abusers) — Extract Function then Move Function; Replace Type Code with
  Subclasses; Replace Conditional with Polymorphism; Remove Flag Argument; Introduce Special Case. A
  simple switch, or one selecting in a factory, is left alone.
- **Parallel Inheritance Hierarchies** (Change Preventers) — every subclass in one hierarchy needs
  one in another. Make one hierarchy's instances refer to the other, then Move Function and Move
  Field to dissolve the referred one; back out if it grows uglier.
- **Incomplete Library Class** (Couplers) — a library lacks what is needed. A few methods: a local
  helper function. More: a wrapper module of your own.
- **Inappropriate Intimacy** (Couplers) — the first-edition Insider Trading. Move Function, Move
  Field; Extract Class with Hide Delegate; make a two-way association one-way.
- **Dead Code** (Dispensables) — Remove Dead Code; Inline Class or Collapse Hierarchy; remove unused
  parameters with Change Function Declaration.
