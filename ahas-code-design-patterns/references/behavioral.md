# Behavioral Patterns

How responsibility and control are shared between objects. State, Visitor, Interpreter and a
serialisable Command are, in TypeScript, a discriminated union with an exhaustive `switch`: a new
member makes every place not handling it a compile error.

## Chain of Responsibility

- **Problem:** which of several handlers takes a request is unknown, so it is passed along.
- **Use when:** request pipelines, validation stages. **Not when:** the handler is fixed.
- **Cost:** nothing guarantees a handler takes it, and the flow is hard to trace.
- **TypeScript:** an array of functions walked in order; middleware of the form `(ctx, next) => …`.

## Command

- **Problem:** a request made a value, to be queued, undone or logged.
- **Use when:** undo and redo, job queues, macros. **Not when:** the call runs at once and is done.
- **Cost:** more classes.
- **TypeScript:** a `() => void` closure; where it must be serialised, a union of actions such as
  `{ type: "add", … }`.

## Interpreter

- **Problem:** a small language's grammar is evaluated through a class per rule.
- **Use when:** a simple DSL, a rules engine. **Not when:** the grammar is large or speed matters.
- **Cost:** a class per grammar rule. **Instead:** a parser generator, an existing parser.
- **TypeScript:** the AST as a discriminated union, evaluated by a recursive function over a
  `switch`.

## Iterator

- **Problem:** walk a collection while hiding how it is built.
- **Use when:** custom collections, lazy or infinite sequences. **Not when:** an array is enough.
- **Cost:** almost none.
- **TypeScript:** built in — `Symbol.iterator`, `function*` generators, `for…of`, async iterators.

## Mediator

- **Problem:** many-to-many communication tangled between objects is gathered into one centre.
- **Use when:** interacting form widgets, a chat room. **Not when:** there are two participants.
- **Cost:** the mediator becomes a god object. **Instead:** an event bus, a state store.
- **TypeScript:** a store with a reducer, or one coordinating function.

## Memento

- **Problem:** snapshot and restore state without breaking encapsulation.
- **Use when:** undo, checkpoints. **Not when:** the state is large and changes often, for memory.
- **Cost:** every snapshot's memory. **Instead:** a history of immutable states, an event log.
- **TypeScript:** with immutable state, a reference to the previous state is the memento —
  `history.push(state)`.

## Observer

- **Problem:** a change of state is announced to several subscribers.
- **Use when:** UI reactions, domain events. **Not when:** there is one fixed subscriber — call it.
- **Cost:** leaks from missed unsubscription; ordering and cascades hard to trace. **Instead:** a
  direct call, reactive streams.
- **TypeScript:** `EventTarget` or an emitter, a `subscribe()` returning its own unsubscribe
  function, signals.

## State

- **Problem:** behaviour branching by state is divided into state objects.
- **Use when:** a flow of many states and transitions — an order, a connection. **Not when:** two
  or three states a boolean holds.
- **Cost:** a class per state. **Instead:** a state-machine library.
- **TypeScript:** a union of states and a transition function over a `switch`; impossible states
  are removed from the type.

## Strategy

- **Problem:** a family of algorithms is made interchangeable.
- **Use when:** swapping at runtime, or removing a large conditional. **Not when:** two or so
  algorithms that rarely change.
- **Cost:** more classes and interfaces, and the client must know how the strategies differ.
  **Instead:** anonymous functions.
- **TypeScript:** a function argument such as `(a, b) => number`, or a `Record<Kind, Fn>`.

## Template Method

- **Problem:** an algorithm's skeleton is fixed and subclasses fill in its steps.
- **Use when:** framework hooks. **Not when:** the steps combine in many ways — inheritance explodes.
- **Cost:** inheritance coupling, and a risk to substitutability. **Instead:** Strategy,
  higher-order functions.
- **TypeScript:** a skeleton function taking the step functions as arguments.

## Visitor

- **Problem:** operations are added over a tree of heterogeneous elements without touching their
  classes.
- **Use when:** operations are added often over a fixed set of node kinds. **Not when:** node kinds
  change often — each addition or removal changes every visitor.
- **Cost:** double dispatch, weakened encapsulation. **Instead:** pattern matching.
- **TypeScript:** a discriminated union and an exhaustive `switch` closed with `const _: never = x`;
  the compiler reports a new node kind wherever it is missed.
