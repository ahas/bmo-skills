# Structural Patterns

How objects and types are composed into larger things. Structural typing makes most of these
cheaper in TypeScript than where they were catalogued: nothing declares that it implements anything.

## Adapter

- **Problem:** two interfaces that do not fit have to be joined.
- **Use when:** an external SDK or legacy code is fitted to your own port. **Not when:** both sides
  are yours to change — change the call site.
- **Cost:** one layer of indirection.
- **TypeScript:** one conversion function, or an object literal satisfying the interface; no
  `implements`.

## Bridge

- **Problem:** an abstraction and its implementation grow along two axes, and subclassing both
  explodes the class count.
- **Use when:** the two really grow independently. **Not when:** only one varies.
- **Cost:** the split is designed up front, and two hierarchies are kept. **Instead:** plain
  composition.
- **TypeScript:** inject the implementation side as a field of an interface type or a function
  argument.

## Composite

- **Problem:** parts and wholes of a tree are handled alike.
- **Use when:** file trees, UI trees, syntax trees. **Not when:** the structure is not a tree.
- **Cost:** the shared interface tends to over-generalise.
- **Instead, and in TypeScript:** recursive data and a recursive function —
  `type Node = { kind: "leaf"; … } | { kind: "group"; children: Node[] }`.

## Decorator

- **Problem:** behaviour is layered on at runtime without breaking the code already using it.
- **Use when:** layers such as logging, caching, retry or compression are combined, or extending by
  inheritance would be awkward. **Not when:** there is one fixed layer.
- **Cost:** deep wrapping makes debugging harder. **Instead:** higher-order functions, middleware.
- **TypeScript:** composed higher-order functions — `withRetry(withCache(fetchX))`. The language's
  decorator syntax is a separate thing, not this pattern.

## Facade

- **Problem:** a complicated subsystem needs one simple entrance.
- **Use when:** several module calls make up one use case. **Not when:** the subsystem is already
  simple.
- **Cost:** the facade can swell into a god object.
- **TypeScript:** a module's `index.ts` exporting only a few functions is the facade.

## Flyweight

- **Problem:** very many objects hold state that could be shared to save memory.
- **Use when:** objects number in the hundreds of thousands — glyphs, particles. **Not when:** there
  are few.
- **Cost:** extrinsic state is managed apart, and the code grows harder. **Instead:** indices into
  arrays, data-oriented design.
- **TypeScript:** intern immutable objects in a `Map` cache; with first-class types the pattern
  disappears.

## Proxy

- **Problem:** access control, deferred loading, a remote call or a cache stands in front of the
  real object.
- **Use when:** lazy initialisation, permissions, remote stubs. **Not when:** all it does is
  delegate.
- **Cost:** indirection, and an illusion of transparency. **Instead:** an explicit wrapper function.
- **TypeScript:** the built-in `Proxy` object, or a wrapper object of the same shape.
