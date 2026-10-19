# Clean Code In Rust

The standard a Rust crate is written to: its layout, its lints, the shapes carrying the most weight,
and the producing side of a WebAssembly build. Each turns a rule the reader would have to remember into
one the compiler, clippy or the layout itself holds.

## Layout

- **One crate per service by default:** one Cargo package at the repository's root, its code in `src/`,
  a library in `src/lib.rs` and the binary in `src/main.rs` over it, so the tests reach the logic
  without starting the program.
- **A workspace with `crates/` only where a part must stand apart:** a library another program consumes
  on its own, a part built for another target such as WebAssembly, or a binary whose dependencies the
  rest must not carry. Each member stands at `crates/<name>/`.
- **Never a root `src/` beside `crates/`.** Once there is a workspace, the root manifest is a virtual
  one and every crate is a member under `crates/`; a root package beside the members is two layouts at
  once.
- **Tests stand in `tests/`,** beside the `src/` of the crate they test, and reach it through its public
  interface. A `#[cfg(test)]` module inside a source file is kept for what can only be reached
  privately, and stays small.
- **Shared settings are inherited:** the edition, the version, the lints and the common dependencies
  are declared once in `[workspace.package]`, `[workspace.lints]` and `[workspace.dependencies]`, and
  each member takes them with `workspace = true`.

## Lints and formatting

- **Every crate passes `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`,** tests,
  examples and benches included.
- **Lint levels live in the manifest's `[lints]` table,** never scattered as crate attributes, so one
  place says what the crate holds itself to. `unsafe_code` is forbidden unless the crate's purpose needs
  it, and then each `unsafe` block carries a `// SAFETY:` comment saying why it holds.
- **A lint is silenced on the smallest item that needs it,** with `#[expect(lint, reason = "...")]`
  rather than `#[allow]`, so the reason is written and the attribute fails once the cause is gone. A
  crate-wide allow is a standard lowered, not an exception.

## Errors

- **Expected failures are `Result`s;** a library's error is an enum of its own, derived with
  `thiserror`, and a binary gathers errors with context at its edge, through `anyhow` or the like.
- **No `unwrap` outside tests.** An invariant the code has proved takes `expect` with the invariant as
  its message; a panic is for a broken invariant, never for bad input.
- **`?` carries an error up** with context added where the caller would otherwise not know what failed.

## Types

- **A closed set is an `enum` matched exhaustively;** a `_` arm over the crate's own enum hides the
  next variant from the compiler.
- **A value with a unit or a meaning of its own is a newtype,** an id, a duration, a validated string,
  so two of them cannot be swapped and the check made at construction is never made again.
- **Outside input is parsed once at the edge** with `serde` into the domain type, and the domain
  handles only that type.
- **A boolean flag in a signature becomes an enum,** and several arguments of one purpose a struct.

## Ownership and signatures

- **Borrow what is read, take what is kept:** `&str`, `&[T]` or `impl AsRef<…>` for an input read,
  an owned value for one stored; return owned values.
- **A `clone` that only quiets the borrow checker is a sign** the data's owner is drawn in the wrong
  place.

## Concurrency

- **One async runtime per program;** blocking work goes through the runtime's blocking pool, never onto
  an async task.
- **Every spawned task is awaited, held or explicitly detached,** and its failure reaches somebody.
- **Shared state is minimised:** a message over a channel before a lock, a lock held for the shortest
  span and never across an `.await`.

## Libraries

- **The maintained crate first** (`principles.md`, 「Use what is maintained」): `serde` and `serde_json`,
  with its `preserve_order` feature where key order is kept; `toml` and a maintained YAML crate; `clap`
  for a command line; a maintained date and time crate; `url`; `regex`. A hand-written replacement
  carries the reason no crate fits beside it.

## One implementation, built for WebAssembly

The producing side of 「One implementation per piece of logic」 in `principles.md`, where Rust is the
source language.

- **The logic is a plain library crate,** free of any binding, and it is the one implementation:
  native callers depend on it directly, and every other runtime receives its WebAssembly build.
- **The binding is a thin layer behind a cargo feature,** `wasm` or the like, or a small crate of its
  own under `crates/`: the `wasm-bindgen` exports live there, so a native build carries none of it. The
  binding converts at the boundary and holds no logic.
- **The surface that crosses is plain data:** strings, numbers, byte slices and typed arrays, and
  serialisable values through `serde-wasm-bindgen`. An error crosses as a JavaScript error, through a
  `Result` whose error is `JsError`, carrying the library's own message.
- **The build uses the maintained toolchain:** `cargo build --target wasm32-unknown-unknown` with the
  `wasm-bindgen` CLI, or `wasm-pack` over them, for the target the consumer loads (`web`, `bundler`
  or `nodejs`), and `wasm-opt` to shrink the result.
- **The library's own tests run natively;** `wasm-bindgen-test` covers only what the binding itself
  does, and the consumer's existing tests hold the build to the contract.
- **The size of the build and the time to instantiate it are measured** before the switch and
  recorded with the change: the `.wasm` file and its glue, compressed as they are served.

A worked example of the pattern: a pricing rule is computed on a server and shown live in a browser
cart. The rule lives once, in a library crate; its `wasm` feature exports the function that prices a
cart, taking and returning plain serialisable values; the browser package keeps its module path and its
function's signature and loads the build inside it; the package's existing tests run unchanged against
the build; and the cart's per-line callback for formatting stays in TypeScript, with the reason written
beside it.
