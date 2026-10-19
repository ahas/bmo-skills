# Beyond the GoF Catalogue

The enterprise, dependency-injection, heavier architectural and functional patterns. Several settle
how a whole application is divided, so they are judged first when the unit under review is that
large.

## Enterprise application patterns

- **Transaction Script** — one procedure per request: the default for simple
  create-read-update-delete; it duplicates as logic grows. TypeScript: a route handler and a service
  function.
- **Domain Model** — objects carrying behaviour and data, only where the rules are complex enough to
  need organising. Data bags with all logic moved into services is the Anemic Domain Model: a domain
  model's cost without its benefit. TypeScript: classes, or immutable records with a module of pure
  functions.
- **Service Layer** — the application boundary coordinating use cases. Kept thin, the core logic in
  the domain; grown thick, it becomes Transaction Script. TypeScript: a module of use-case functions.
- **Active Record and Data Mapper** — an object wrapping one row, against a layer keeping objects and
  database ignorant of each other: Active Record where schema and domain nearly match, Data Mapper
  where they differ, at the cost of the mapper layer.
- **Repository** — a collection-like interface between the domain and the mapping layer. It pays
  where the domain is complex, domain classes many or queries heavy, by removing duplicated query
  logic; a thin wrapper mirroring an ORM client method for method adds indirection and gains
  nothing. TypeScript: a port such as `interface UserRepo { byId(id): Promise<User | null> }` and its
  implementation, with an in-memory one for tests.
- **Unit of Work, Identity Map, Lazy Load** — track changes and write once; load each thing once;
  load on demand. An ORM usually provides them, so they are rarely hand-written; lazy loading risks
  N+1 queries. TypeScript: the ORM's transaction callback, `db.transaction(async tx => …)`.
- **Query Object, DTO, Gateway, Remote Facade** — a query as a value; data carried between
  processes; an external system wrapped; a coarse interface for the network. At boundaries, HTTP and
  external APIs; the cost is DTO and domain types defined twice. TypeScript: a DTO parsed by a schema
  library, its type derived from the schema.
- **Value Object, Money** — a small object whose equality is its value, not its identity: money,
  durations, email addresses, almost always worth it. TypeScript: readonly records, and branded
  types such as `string & { __brand: "Email" }`.
- **Special Case (Null Object)** — the branch on a special case removed, where the default behaviour
  is clear; the risk is hiding an error. TypeScript: `?.`, `??`, or a `{ kind: "none" }` union
  member.
- **Registry, Plugin, Separated Interface** — find shared objects; wire at configuration time; keep
  the interface in another package than its implementation. A registry is effectively global state.
  TypeScript: the interface in the domain module, the implementation in the infrastructure module —
  ports and adapters.
- **Optimistic and Pessimistic Offline Lock** — conflicting business transactions prevented:
  optimistic where conflicts are rare, pessimistic where frequent, at a cost in throughput.
  TypeScript: a `version` column and `WHERE version = ?`.
- **MVC, Front Controller, Page Controller** — UI roles split; one entry for requests. Web frameworks
  implement them, a router being the Front Controller, so they are rarely hand-written.

## Dependency injection

- **A component receives its dependencies rather than creating them,** by constructor injection
  first. A constructor with too many arguments says the object should be split.
- **For testability, injection and a service locator are equal.** A locator becomes the problem in a
  component built for others, who inherit a dependency on the locator.
- **Assemble in code, not configuration files.** In TypeScript, by hand at one composition root,
  through constructor or function arguments. Decorator-driven containers rely on
  `experimentalDecorators` and `emitDecoratorMetadata` and tie the build to compiler support for
  them; reach for one only when hand assembly has become the problem.

## CQRS, Event Sourcing, DDD tactics

- **CQRS** — separate write and read models. For most systems it adds risky complexity, and many
  cases seen in practice went badly: only for a complex bounded context or a large gap between read
  and write load, never across a whole system. It costs eventual consistency; the lighter
  alternative is a reporting database. TypeScript: go no further than separate command and query
  handlers to start.
- **Event Sourcing** — every state change stored as events, so past states can be rebuilt. Where
  auditing, history or retroactive correction is needed; for processing alone, needless
  indirection. TypeScript: events as a discriminated union, state replayed with `reduce`.
- **DDD tactics** — Entity, Aggregate, Domain Event: identity, a consistency boundary, a domain
  happening. For complex domains; too much for plain create-read-update-delete. TypeScript: only the
  aggregate root's functions change its state; events are a union.

## Functional patterns and type-level idioms

- **Higher-order functions, composition, partial application** — injected, reused behaviour, in the
  places of Strategy, Template Method, Decorator and Command. The default; long point-free chains
  read badly and blur stack traces. TypeScript: arrow functions, a `pipe` helper only where it
  helps.
- **Functional core, imperative shell** — pure core logic, a thin input-output shell, wherever there
  is logic to test: the core is tested without mocks and the shell has few conditionals. It costs
  the design effort of carrying data out to the boundary. TypeScript: pure function modules, thin
  handlers.
- **Algebraic data types — make illegal states unrepresentable,** wherever the kinds of state or
  message are finite; network messages and state-management mutations are the handbook cases.
  TypeScript: a `kind` field, a `switch`, the `never` exhaustiveness check.
- **Parse, don't validate** — a check becomes a more precise type rather than being thrown away, at
  every boundary where outside input arrives. TypeScript: a schema parser returning the domain type;
  branded types.
- **Result or Either, Option** — errors as values, where there are expected domain failures;
  wrapping programmer bugs in Results too is overdone. It costs propagation boilerplate or a
  library's learning curve. TypeScript:
  `type Result<T, E> = { ok: true; value: T } | { ok: false; error: E }` by hand; a Result library
  where one is already in use.
- **Higher-kinded abstraction libraries** — monads and type classes: not for new adoption. The older
  library of this kind has merged into a larger effect system, mixed versions can hang the type
  checker, and the team pays the learning curve.
- **Immutability and memoisation** — against shared-mutable-state bugs and repeated computation.
  Memoise only expensive calls whose result depends on the input alone. TypeScript: `readonly`,
  `as const`, a `Map` cache.

## Front end

- **Higher-order components are uncommon in modern component code;** logic is reused through custom
  hooks or composables. **Mixins were abandoned** as causing more trouble than they solve.

## The shared "do not use" list

- **The Anemic Domain Model** — a domain model's cost without its benefit.
- **Singleton abuse** — global state and code that resists testing.
- **An interface hierarchy for a problem two classes solve.**
- **CQRS applied across a whole system.**
