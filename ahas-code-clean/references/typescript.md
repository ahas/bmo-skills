# Clean Code In TypeScript

The shapes carrying the most weight in TypeScript, each turning a rule the reader would have to
remember into one the compiler or the linter checks.

## Exhaustive unions

- **A closed set of states or messages is a discriminated union** with a literal `kind`, a `switch`
  over it ending in `const _: never = x`: a new member makes every place missing it a compile error.
- **Each state's fields are required in its own member,** so no optional field needs a `!` and no
  impossible combination can be built.
- **The linter's switch-exhaustiveness check** catches a `switch` over a union missing a case.

## Parse once at the edge

- **Outside input arrives as `unknown`,** parsed once by a schema returning the domain type; inside,
  only the refined type is handled.
- **Never `as` an outside shape:** a type assertion is a claim nobody checked.
- **What a check learned stays in the type:** a branded type for a validated string, `[T, ...T[]]`
  for a non-empty array.

## Failures as values

- **Expected failures return a Result union,** `{ ok: true; value: T } | { ok: false; error: E }`,
  so the type makes the caller handle them.
- **Throw only `Error` instances,** and only for broken invariants and unrecoverable faults.
- **Absence is `T | undefined`** under strict null checks, never a `null` handed along.

## Promises

- **No floating promise:** each is awaited, returned, handled with `.catch` or marked `void` on
  purpose, as the linter's no-floating-promises rule enforces.
- **Parallel work goes through `Promise.all`,** `allSettled`, `any` or `race`, never an array of
  promises left unobserved.

## Signatures

- **Several arguments become one object parameter,** destructured in the signature; **a boolean
  flag becomes a union or two functions.**
- **A parameter is typed by what the function uses** — `Pick<T, …>` or a narrow type — not the
  whole object it happens to receive.
- **A contract-critical callback is a property of function type,** not a method signature, so its
  parameters are checked contravariantly.
- **Inputs are `readonly`,** and a changed value is returned rather than written into an argument.

## Modules over classes

- **A module of exported functions** serves where a small class would; a class earns its place with
  identity, state held across calls, or several operations over one invariant.
- **A dependency is a constructor or function argument,** assembled by hand at one composition root.
- **One list, one type:** a union derived from an `as const` array keeps the alternatives and their
  type in one place.

## A WebAssembly build in place of a second copy

The consuming side of 「One implementation per piece of logic」 in `principles.md`: where the logic
also lives in a language that builds to WebAssembly, TypeScript loads that build rather than keeping a
copy of its own.

- **The package becomes a thin wrapper over the build,** keeping its module paths, its exported names
  and its signatures, so no caller changes. It converts at the boundary, plain values in and out, and
  turns the build's errors into the package's own error or Result shape.
- **The existing tests are the contract.** They run unchanged against the wrapper; a test edited so the
  build passes is a contract broken, and the build is what is fixed.
- **The build is instantiated once,** through one shared initialisation the wrapper awaits, never
  once per call; a synchronous export stays synchronous only where the build is ready before its first
  call.
- **Only what truly cannot cross stays in TypeScript:** an API that calls back into its caller for every
  element, behaviour that differs by engine, work on the page itself. The reason stands beside that
  code, and everything else goes to the build.
- **The bundle size and the load time are measured** against the TypeScript copy before it goes: the
  `.wasm` file and its glue as they are served, and the time to the first call.
