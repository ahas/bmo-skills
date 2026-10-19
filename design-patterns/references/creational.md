# Creational Patterns

Who makes an object, and how much the caller must know to make it. In TypeScript each defaults to a
function; a class hierarchy has to earn its place.

## Factory Method

- **Problem:** the concrete type is decided by a subclass or the caller's case, and the call site
  must not know which.
- **Use when:** creation branches and the call site should see only the interface or the union.
  **Not when:** there is one implementation.
- **Cost:** an inheritance hierarchy for what is often one decision. **Instead:** a factory
  function or a constructor argument.
- **TypeScript:** `createX(kind)` returning an interface or a union. A class hierarchy is almost
  never needed; with first-class types the pattern disappears.

## Abstract Factory

- **Problem:** a whole family of products that must stay compatible is swapped at once.
- **Use when:** there really is a matrix of product families by variant. **Not when:** there is one
  variant, or a second only might appear some day.
- **Cost:** interfaces and classes multiply past the problem. **Instead:** a configuration object
  or a record of functions.
- **TypeScript:** a record of creation functions passed in, one per family —
  `{ button: () => …, checkbox: () => … }`.

## Builder

- **Problem:** a construction with many optional parts is split into steps.
- **Use when:** building takes several stages, or one procedure must yield different
  representations. **Not when:** the object has a handful of options.
- **Cost:** one more class and a mutable intermediate state. **Instead:** an options object with
  defaults, merged by spread.
- **TypeScript:** usually `new X({ ...defaults, ...opts })`; a chained builder only where the types
  enforce required steps, as in a query builder.

## Prototype

- **Problem:** copy an object without knowing its concrete class.
- **Use when:** initialising is expensive and copying cheaper. **Not when:** the data is plain and
  copying ordinary.
- **Cost:** deep copies and cyclic references must be handled.
- **Instead, and in TypeScript:** `{ ...obj }` or `structuredClone(obj)`. JavaScript is itself
  prototype-based, so a pattern of its own is rarely called for.

## Singleton

- **Problem:** exactly one instance, with a global access point.
- **Use when:** a shared resource such as a database connection genuinely must be one.
  **Not when:** it stands in for a global variable or hides a dependency.
- **Cost:** breaks single responsibility, hides bad design, complicates concurrent access, makes
  mocking hard. **Instead:** one instance created once and injected.
- **TypeScript:** a module's top-level `export const db = createDb()` already is one. Export the
  creating function too, so a test can make its own.
