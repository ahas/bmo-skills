# The Refactoring Catalogue

The second edition's sixty-six techniques in the book's chapter order; the five only in the online
catalogue stand in their group's chapter. First-edition names follow in brackets, so a technique is
recognisable under either. Each entry gives the motive, then the safe steps, the inverse where
there is one, and the TypeScript shape. Check between every step.

## A first set

- **Extract Function** (Extract Method) — separate what the code does, its intent, from how; a
  fragment needing a comment wants extracting. Steps: create the function named for its intent;
  copy the fragment; outside locals become parameters; a value changed inside and used after
  becomes the return value; replace the fragment with the call. Inverse: Inline Function.
  TypeScript: explicit parameter and return types.
- **Inline Function** (Inline Method) — the body says as much as the name, the delegation is
  pointless, or badly split functions are regathered before splitting again. Steps: confirm no
  polymorphic override; replace each call with the body, one at a time; delete the definition.
  Inverse: Extract Function. Stop where an overload or a subclass override exists.
- **Extract Variable** (Introduce Explaining Variable) — name part of a complex expression. Steps:
  confirm no side effect; declare it `const`; replace the expression. Inverse: Inline Variable.
  Inside a class, consider a getter.
- **Inline Variable** (Inline Temp) — the name says no more than the expression. Steps: confirm the
  right-hand side has no side effect; make it `const` so the checker proves it is never reassigned;
  replace the references; delete the declaration. Inverse: Extract Variable.
- **Change Function Declaration** (Rename Function, Add Parameter, Remove Parameter, Change
  Signature) — the name and parameters are the function's joint with everything else. Simple:
  change the declaration and every call at once. Migration: create the new function, make the old
  delegate, move callers one by one, delete the old — deprecated where public. TypeScript: the
  editor's rename; a `/** @deprecated */` wrapper for a public API.
- **Encapsulate Variable** (Encapsulate Field, Self-Encapsulate Field) — widely used data gets one
  point of access, where validation or watching can live. Steps: getter and setter functions;
  replace references one by one; narrow the variable's visibility. TypeScript: a module-scoped
  `let` behind exported get and set functions; in a class, `#field` with `get` and `set`.
- **Rename Variable** — a name is understanding. Steps: encapsulate first where widely used; replace
  references. TypeScript: the editor's rename; an exported name migrates through a re-export alias.
- **Introduce Parameter Object** — arguments always travelling together are one structure. Steps:
  create the type; add it with Change Function Declaration; callers pass it; remove the old
  parameters one by one. TypeScript: `type DateRange = { readonly start: Date; readonly end: Date }`,
  or a class.
- **Combine Functions into Class** — functions on the same data gathered, their parameter lists
  shrinking. Steps: encapsulate the shared record; move each function in as a method.
- **Combine Functions into Transform** — derived values computed in one place, the source kept
  immutable. Steps: a transform deep-copying its input and returning it; add derived fields one at a
  time; callers use the transformed result. TypeScript: the enriched result has its own return type.
- **Split Phase** — two different jobs, parsing and computing say, become two phases. Steps: extract
  the second phase into a function; introduce an intermediate data structure; extract the first
  phase to return it. TypeScript: `parseOrder(s): OrderRecord`, then `price(order, priceList)`, the
  intermediate shape a declared type.

## Encapsulation

- **Encapsulate Record** (Replace Record with Data Class) — hide how the data is stored; access goes
  through methods. Steps: encapsulate the record variable; wrap it in a class; accessors per field;
  move callers.
- **Encapsulate Collection** — a collection changes only through its owner. Steps: `add` and
  `remove` methods; the getter returns a copy or a read-only view; move callers that mutate
  directly. TypeScript: `get courses(): readonly Course[] { return [...this.#courses] }`.
- **Replace Primitive with Object** (Replace Data Value with Object, Replace Type Code with Class) —
  behaviour has gathered around a primitive. Steps: encapsulate the variable; a value class; the
  setter builds it, the getter returns it; move behaviour in. TypeScript: a small class, or the
  lighter branded type `number & { __brand: "Money" }`.
- **Replace Temp with Query** — a computation extracted for others to use. Steps: confirm one
  assignment; extract the right-hand side as a method; inline the variable. TypeScript: a `get`
  accessor.
- **Extract Class** — split a responsibility off a class doing too much. Steps: decide it; create the
  class; hold an instance; Move Field, then Move Function; tidy the interfaces. Inverse: Inline
  Class.
- **Inline Class** — a class not pulling its weight is absorbed, also as a step before splitting
  differently. Steps: delegating methods on the absorbing class; move callers; move members; delete.
  Inverse: Extract Class.
- **Hide Delegate** — clients need not know the delegate's structure. Steps: a delegating method on
  the server; switch clients to it; consider removing the delegate accessor. Inverse: Remove Middle
  Man.
- **Remove Middle Man** — too much plain delegation. Steps: expose the delegate; clients call it
  directly; delete the delegating methods. Inverse: Hide Delegate.
- **Substitute Algorithm** — a clearer algorithm. Steps: isolate it in one function first; write the
  new one; compare them under the same tests.

## Moving features

- **Move Function** (Move Method) — it refers more to another context than its own; moving it lowers
  coupling. Steps: check what it uses and for polymorphic overrides; copy to the target and adapt;
  the original delegates; inline or delete the original. TypeScript: between modules, an import-path
  change; the editor's move to file.
- **Move Field** — data changing together belongs in one record. Steps: encapsulate the source
  field; field and accessors on the target; source accessors point at the target; delete the source
  field.
- **Move Statements into Function** — statements accompanying every call belong inside. Steps: slide
  them next to the call; extract statements and call together; replace callers; inline the old
  function. Inverse: Move Statements to Callers.
- **Move Statements to Callers** — only some callers now want part of the behaviour. Steps: extract
  and rename the part that stays; copy the moving statements into every caller; remove them from the
  function. Inverse: Move Statements into Function.
- **Replace Inline Code with Function Call** — an existing function already does it. Steps: replace
  the inline code with the call.
- **Slide Statements** (Consolidate Duplicate Conditional Fragments) — related code brought together,
  which makes extraction easy. Steps: find the range and destination; check for interference, side
  effects and references; move. TypeScript: the checker catches a use before declaration.
- **Split Loop** — one loop doing two things. Steps: duplicate the loop; remove the duplicated side
  effects from each; extract each into a function.
- **Replace Loop with Pipeline** — the data flow reads as a pipeline. Steps: bind the loop's
  collection to a variable; move each operation into a `filter` or a `map` one by one; remove the
  loop. TypeScript: inference carries through the chain.
- **Remove Dead Code** — code nobody calls still costs understanding. Steps: prove no references,
  by the unused-locals check and the editor's find references; delete. Version control keeps the
  history.

## Organising data

- **Split Variable** (Split Temp, Remove Assignments to Parameters) — one variable, several
  responsibilities. Steps: a new `const` at the first assignment; replace references up to the next
  assignment; repeat.
- **Rename Field** — Steps: encapsulate; rename the private field; rename the accessors with Change
  Function Declaration.
- **Replace Derived Variable with Query** — minimise mutable scope; a computation shows meaning.
  Steps: find where the variable changes; write the computation; assert both agree; replace
  references with it; delete the variable.
- **Change Reference to Value** — immutable values are easier. Steps: confirm it can be immutable;
  remove each setter; value-based equality. Inverse: Change Value to Reference. TypeScript:
  `readonly` fields, replaced wholesale.
- **Change Value to Reference** — copies of one entity drift apart when one is updated. Steps: a
  repository; construction looks the entity up there; replace callers. Inverse: Change Reference to
  Value. TypeScript: a `Map`-backed repository.
- **Replace Magic Literal** (Replace Magic Number with Symbolic Constant) — a literal given its
  meaning. Steps: declare the constant; replace only the literals carrying that same meaning.
  TypeScript: an `as const` constant, or a literal union.

## Simplifying conditional logic

- **Decompose Conditional** — show what the branch is about. Steps: extract the condition, the
  then-part and the else-part each into a function.
- **Consolidate Conditional Expression** — checks with the same result become one. Steps: confirm no
  side effects; join with `||` or `&&`; extract and name. Unrelated checks stay apart.
- **Replace Nested Conditional with Guard Clauses** — the normal path apart from the unusual ones.
  Steps: the outermost unusual condition becomes an early return first, one at a time.
- **Replace Conditional with Polymorphism** — the same branch on type repeats in several places.
  Steps: without a hierarchy, Replace Type Code with Subclasses first; extract the conditional
  method; copy each branch into the matching override; delete the branches; make the base abstract
  if empty. TypeScript: an abstract class, or the equal alternative of a discriminated union, a
  `switch` and a `never` exhaustiveness check.
- **Introduce Special Case** (Introduce Null Object) — repeated checks for a special value gathered
  in one place. Steps: an `isUnknown`-style property; the special-case object; extract the check;
  replace callers. TypeScript: a class, or a literal object of the same shape.
- **Introduce Assertion** — an assumption made explicit. Steps: assert only a condition that must
  always hold; behaviour does not change. TypeScript:
  `function assert(c: unknown, m?: string): asserts c`.
- **Replace Control Flag with Break** (Remove Control Flag) — `break`, `return` or `continue`
  instead of a control flag. Steps: replace each assignment of the flag with the jump; delete it.

## Refactoring APIs

- **Separate Query from Modifier** — a query without side effects is easy to test and reuse. Steps:
  copy the function as a query, its side effects stripped; callers use the query; remove the
  original's return value.
- **Parameterize Function** (Parameterize Method) — functions differing only in a literal become
  one. Steps: make one's literal a parameter; replace callers; replace the others. Opposite in
  spirit: Remove Flag Argument.
- **Remove Flag Argument** (Replace Parameter with Explicit Methods) — a literal flag reads badly.
  Steps: an explicit function per flag value; replace calls passing a literal. TypeScript: split
  even a string-union argument when callers pass it as a literal.
- **Preserve Whole Object** — pass the object, not values pulled out of it. Steps: an empty function
  of the wanted signature calling the old one; move callers; inline. Watch for coupling across
  modules. TypeScript: `Pick<T, "low" | "high">` narrows what is really needed.
- **Replace Parameter with Query** (Replace Parameter with Method) — drop a parameter the function
  can find itself. Steps: move the computation into the body; remove the parameter. Inverse:
  Replace Query with Parameter.
- **Replace Query with Parameter** — cut a global or outside dependency so the function is pure.
  Steps: extract the query to a variable; extract the body as a function; make it a parameter;
  inline the original. Inverse: Replace Parameter with Query.
- **Remove Setting Method** — make it immutable. Steps: take the value as a constructor argument;
  move setter calls to construction; delete the setter. TypeScript: `readonly` fields.
- **Replace Constructor with Factory Function** (Replace Constructor with Factory Method) —
  construction that does more, or deserves a name. Steps: the factory; replace callers; restrict
  the constructor. TypeScript: a plain function, or `private constructor` with `static create()`.
- **Replace Function with Command** (Replace Method with Method Object) — undo, a lifecycle, or a
  body to break into steps. Steps: an empty class; the function moves to `execute`; parameters
  become constructor fields, locals fields; extract methods. Inverse: Replace Command with Function.
- **Replace Command with Function** — a simple one is better as a function. Steps: a function
  wrapping construction and `execute`; inline the methods; fields become parameters; delete the
  class. Inverse: Replace Function with Command.
- **Replace Error Code with Exception** — Steps: the error class; a top-level handler; `throw`
  replaces code returns; remove the code checks. TypeScript: a class extending `Error`; the
  alternative is a Result union.
- **Replace Exception with Precheck** (Replace Exception with Test) — a condition the caller can
  check first needs no exception. Steps: add the precheck; assert inside the `catch`; remove the
  `catch`.
- **Return Modified Value** — return the changed value so the data flow shows. Steps: the function
  returns what it changes; callers assign it; the mutation stays local.

## Dealing with inheritance

- **Pull Up Method** — duplication between subclasses. Steps: make the bodies identical first; pull
  up the fields they use; declare in the parent; delete from subclasses one at a time. Inverse: Push
  Down Method. Where only part matches, an abstract template step.
- **Pull Up Field** — Steps: confirm the uses match; unify the name; declare in the parent; delete
  below. Inverse: Push Down Field.
- **Pull Up Constructor Body** — Steps: the parent constructor; slide the common statements after
  `super`; move them up.
- **Push Down Method** — a method only one subclass uses. Inverse: Pull Up Method.
- **Push Down Field** — a field only one subclass uses. Inverse: Pull Up Field.
- **Replace Type Code with Subclasses** (Extract Subclass, Replace Type Code with State/Strategy) —
  behaviour and fields differ by type; preparation for polymorphism. Steps: encapsulate the type
  code; a factory; a subclass per value overriding the type getter; the factory's `switch` picks;
  delete the type field. Inverse: Remove Subclass. TypeScript: a factory and subclasses, or
  delegation to a strategy.
- **Remove Subclass** (Replace Subclass with Fields) — subclasses doing almost nothing. Steps:
  construction in a factory; the type test moves to a field; delete the subclasses. Inverse: Replace
  Type Code with Subclasses. TypeScript: a `readonly` literal-union field.
- **Extract Superclass** — the common part of two similar classes. Steps: an empty parent; link the
  inheritance; pull up constructor bodies, fields, methods; extract what remains in common. Undone
  by Collapse Hierarchy.
- **Collapse Hierarchy** — parent and child no longer differ. Steps: choose which goes; pull up or
  push down everything; replace references; delete the empty class.
- **Replace Subclass with Delegate** — more than one axis varies, so inheritance falls short; prefer
  composition. Steps: a delegate class; a delegate field on the host; the factory injects it; move
  methods to it; delete the subclass.
- **Replace Superclass with Delegate** (Replace Inheritance with Delegation) — the parent's features
  do not fit the child, as a stack extending a list. Steps: a field holding the former parent; a
  delegating method for each one used; cut the inheritance.
