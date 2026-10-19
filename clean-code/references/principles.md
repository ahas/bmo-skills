# Clean Code Principles

Each entry gives what it demands, what breaking it looks like, the harm of overdoing it and its
exceptions, and its TypeScript form.

## The areas of the code

### Names

- **Demand:** intent-revealing, pronounceable, searchable names. A magic number fixed by its nature
  becomes a named constant; a fact that can change is read from its source and carried by no name
  (the refactoring skill's 「Names that stay true」). One word per concept — never `getUserInfo`,
  `getUserDetails` and `getUserData` side by side. No encodings such as Hungarian notation, no
  repeated context such as `Car.carMake`.
- **Broken:** `a1`, `a2`, `DtaRcrd102`, a bare `86400000`, a service named after the node it runs on.
- **Overdone:** a name carrying every fact grows awkward; a short name with a comment can be better.
- **TypeScript:** the type name is the context (`type Car = { make; model }`); primitives differing
  in unit or meaning are told apart by branded types.

### Functions

- **Demand:** small; one job; one level of abstraction; few arguments; no flag arguments; no side
  effects; command separated from query; a return value instead of an output argument.
- **Broken:** functions hundreds of lines long; a boolean argument forking the behaviour; a function
  that looks like a query and changes state.
- **Overdone:** two-to-four-line functions and one-line `if` bodies make shallow methods entangled
  with each other, their logic passed through shared fields as side effects — the very thing the
  rule forbids. "One thing" is vague: two tightly bound steps, such as taking a lock and the
  critical section, belong in one function. The better measure is depth, a simple interface over
  much hidden function.
- **TypeScript:** many arguments become one destructured object parameter; a flag becomes a
  discriminated union or separate functions; a pure function states its result in its return type.

### Comments

- **Demand:** intent expressed in code; no redundant or obvious comments, commented-out code or
  closing-brace comments. Comments that explain intent, clarify or warn of consequences stay.
- **Broken:** a comment repeating the code; a comment left wrong; code commented out and left.
- **Overdone:** "a comment is a failure" is the most disputed rule of all. An interface comment
  carries the contract a signature cannot — format, ordering, behaviour on duplicates, side effects,
  argument limits — without which a caller reads the implementation and everything it calls. Public
  APIs and interfaces used by other teams need comments on every account.
- **TypeScript:** the type carries part of the contract — nullability, a union's range; the "why" —
  side effects, ordering constraints, a performance reason — goes in a doc comment.

### Formatting

- **Demand:** read like a newspaper, top to bottom; related code vertically close; variables
  declared near their use; a caller above its callee; short lines; no horizontal alignment.
- **Overdone:** most files cannot be ordered cleanly by level of abstraction, and editor navigation
  has lowered the ordering rule's worth.
- **TypeScript:** the formatter and linter end the argument; types and classes in PascalCase,
  variables and functions in camelCase.

### Objects and data structures

- **Demand:** objects hide data and expose behaviour; data structures expose data and have no
  behaviour; no half-and-half hybrids; the Law of Demeter.
- **Broken:** classes of getters and setters assembled from outside; train wrecks such as
  `a.b().c().d()`.
- **TypeScript:** data as `type` or `interface` with `readonly`, behaviour as module functions, a
  common and natural functional style. A discriminated union with a `switch` is the data-structure
  side of the trade: a new operation is easy to add, and a new variant is found missing by the
  compiler.

### Error handling

- **Demand:** exceptions rather than return codes; error handling as its own job; neither return nor
  pass `null`; never ignore a caught error.
- **Broken:** an empty `catch`; a `catch` that logs and swallows; an unhandled promise rejection.
- **Overdone:** an exception is absent from the type signature, so throwing even expected failures —
  a failed validation, a missing record — lets callers forget to handle them.
- **TypeScript:** throw only `Error` instances, made with `new`; expected failures return a Result
  union, `{ ok: true; value } | { ok: false; error }`; exceptions stay for broken invariants and
  unrecoverable faults; a possibly absent value is `T | undefined`, enforced by strict null checks.

### Boundaries

- **Demand:** wrap outside code rather than spreading its interfaces through the system; pin outside
  APIs with learning tests; where an API does not exist yet, define the interface you want and join
  it by an adapter later.
- **Broken:** an outside SDK's types imported all over the domain; an outside response asserted with
  `as` and never checked.
- **TypeScript:** take `unknown` at the edge and parse it once with a schema library, handling only
  the refined type inside. This is parse, don't validate: the check is kept in the type, so nothing
  checks it again.

### Unit tests

- **Demand:** the three laws of test-driven development; tests fast, independent, repeatable,
  self-validating and timely; one concept per test; names revealing intent.
- **Overdone:** one assertion per test, applied mechanically, only splits tests into fragments.
- **TypeScript:** `describe` and `it` names read like requirements; type-level expectations can be
  checked by the type checker as part of the tests.

### Classes

- **Demand:** small; single responsibility and high cohesion; few instance variables; a base class
  knows nothing of its derived classes.
- **Overdone:** a large method broken into several classes hides side effects between the methods
  and entangles them.
- **TypeScript:** a class is not required: a module of a few exported functions does a small class's
  work, and making everything a class breeds verb-named noun classes.

### Concurrency

- **Demand:** concurrency code apart from other code; shared state minimised; copies of data used.
- **TypeScript:** promises and `async`/`await` over callbacks; `Promise.all` for parallel work;
  floating promises forbidden by lint — each awaited, returned, `.catch`-ed or explicitly
  `void`-ed, and an array of them through `Promise.all`, `allSettled`, `any` or `race`.

### Smells and heuristics

- **Smells:** rigidity, fragility, immobility, needless complexity, needless repetition, opacity.
- **Heuristics:** configurable data at high levels; polymorphism over `if` and `switch`; no excessive
  configurability; value objects preferred; no negative conditionals; the boy-scout rule, the
  campground left cleaner than found.
- **Overdone:** polymorphism over `switch` costs in hot loops, where a `switch` or a table measures
  well ahead of virtual dispatch.
- **TypeScript:** a discriminated union with a `switch` is a peer of polymorphism, and the compiler
  reports a missed variant.

## SOLID

- **Single responsibility** — a module has one reason to change, and a reason is a person: one group
  of business actors. Gather what changes for the same reason, separate what changes for different
  ones. Broken: one class computing pay for finance, reporting hours for operations and saving for
  the technical side, so a pay change breaks the audit report. Overdone: read as "one thing", it
  drives over-decomposition. TypeScript: modules split by who changes them — domain computation,
  storage, presentation.
- **Open-closed** — open to extension, closed to modification. Broken: every new kind means edits to
  branches in several files. Overdone: extension points for extensions that do not exist are not
  only wasted but in the way. TypeScript: the strategy as a function or an interface argument; for a
  closed set, a union with an exhaustiveness check has the compiler name every place to change.
- **Liskov substitution** — a subtype satisfies every provable property of its supertype:
  parameters contravariant, return values covariant, no new exceptions, preconditions never
  strengthened, postconditions never weakened. Broken: an override rejecting some input or throwing,
  the square and the rectangle; behavioural subtyping is undecidable in general, so no compiler
  guarantees it. TypeScript: strict function types check parameters contravariantly only for
  function-syntax declarations, while method-syntax parameters stay bivariant and unsafe assignments
  pass; so a contract-critical callback is declared as a property of function type,
  `fn: (x: T) => void`, not a method.
- **Interface segregation** — no client is forced to depend on methods it does not use. Broken: a
  huge interface, unimplemented methods filled by `throw`. Overdone: one-method interfaces
  everywhere; an API so narrow the right combination becomes tacit knowledge. TypeScript: structural
  typing narrows at the call site — `Pick<T, "a" | "b">`, or a parameter type of only the fields
  used.
- **Dependency inversion** — depend on abstractions, not concretions. Broken: `new XmlFormatter()`
  inside the class using it. Overdone: an interface and an inversion-of-control container around a
  single implementation. TypeScript: `constructor(private readonly formatter: Formatter)`, or a
  function argument; no container is required.
- **On SOLID as a whole:** principles harden into pass-or-fail rules. Hold them as properties
  pointing a direction — composable, Unix-philosophy, predictable, idiomatic, domain-based.

## The other principles

- **DRY** — every piece of knowledge has one unambiguous, authoritative representation, schemas,
  builds and documents included; a special case is the single-choice principle, where only one
  module knows the full list of alternatives. Broken: one rule — a validation, a constant, a list —
  copied to several places and fixed in one. Overdone: merging code that merely looks alike makes
  the wrong abstraction: extract; a new need adds a parameter; conditionals pile up; nobody
  understands it. The cure is to inline it back and let the duplication return. Duplication is far
  cheaper than the wrong abstraction: avoid hasty abstraction and wait for the third case.
  TypeScript: the union type derived from an `as const` array, so list and type come from one place.
- **KISS** — keep it simple; cut complexity.
- **YAGNI** — no feature built now because it might be used later. Breaking it costs four ways: of
  building, of delay, of carrying it, which slows every other feature, and of repair; even carefully
  analysed features improve their measures only a third of the time. Exceptions: refactoring,
  self-testing code and continuous delivery are not subject to it; preparation adding no complexity
  is fine; YAGNI holds only while code is easy to change. TypeScript: no unused generic parameters,
  optional fields or abstract interfaces made in advance.
- **Law of Demeter** — a method calls methods only of itself, its parameters, objects it created,
  its own fields and globals: "use one dot". Broken: `a.m().n()`, the caller learning the internal
  structure. Overdone: forwarding wrappers multiply and interfaces bloat, and more methods per class
  goes with more defects. TypeScript: a chain through plain data is no violation; reduce only the
  calls reaching through service objects.
- **Least astonishment** — a component behaves as its user expects; a highly surprising feature is
  redesigned. Broken: an `isX()` that changes state; a side effect the name does not declare; a
  received object silently mutated. TypeScript: inputs `readonly`, new values returned, queries
  pure.
- **Tell, Don't Ask** — rather than pulling data out and deciding outside, tell the object to act;
  data and behaviour stay together. Overdone: it breeds the urge to eradicate every getter, and
  other concerns such as layering can rightly take precedence.
- **Command-query separation** — a function either does something or answers something, not both.
  TypeScript: a query returns a non-`void` value with no side effects; a command returns `void` or a
  Result.
- **Parse, don't validate; make illegal states unrepresentable** — what a check learned is kept in
  the type, so later code never checks again; a partial function becomes total by narrowing its
  input type. Broken: a "can't happen" branch after a check already made; a type of optional fields
  and a `kind` string needing `!` assertions. TypeScript: a discriminated union making each state's
  fields required; parsing at the edge; a non-empty array as `[T, ...T[]]`.
- **The boy-scout rule** — leave the code cleaner than you found it.
