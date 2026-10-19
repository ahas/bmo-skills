# TypeScript

## TypeScript Rules
- TypeScript-first.
- Public surfaces (props, emit payloads, return types) **MUST** use explicit types.
- Locals may use inference, but for multi-branch expressions, prefer explicit types.
- Use type aliases as needed to manage complexity.
- Prefer explicit return types for functions/arrow functions when practical.

## TypeScript Imports

### Import Order (Strict)
1. CSS / static assets (e.g., `import "styles.scss";`)
2. *(blank line)*
3. SFC components
4. *(blank line)*
5. Module imports
6. *(blank line)*
7. Type-only imports

Exceptions:
- If type usage is small, it may be kept in the module import using `type` specifier.
  - Example: `import { a, type B } from "module";`
- PascalCase filename imports are treated as SFC component imports.
- Otherwise, treat as normal module imports.

### Import Blank Lines (Strict)
- Do **NOT** insert a blank line between every import statement.
- Keep consecutive imports with the same statement form together.
- Insert one blank line **only when the import statement form changes**.
- The primary statement-form boundaries are:
  - `import * as Foo from "module";`
  - normal value/module imports such as `import { a } from "module";` or `import Foo from "module";`
  - `import type { A } from "module";`
- Multi-line imports are still a single import statement, so they do not justify an extra blank line by themselves.

Example:
```ts
import * as THREE from "three";

import { buildThing } from "./build-thing";
import { createThing } from "./create-thing";

import type { Thing } from "./types";
import type { ThingOptions } from "./types";
```

### Quotes
- Prefer double quotes.
- Use single quotes only when the string contains many double quotes.

## Control Flow & Declarations

### Control Flow Spacing / Style
- Insert one blank line before control statements (`if`, `switch`, `while`, `for`, ...).
- Inline control statements are generally forbidden.

Exception (all must hold):
- Fits within Prettier line width.
- Single simple statement.
- Multiple consecutive control statements.

### switch Rules
- Each `case` block **MUST** use braces.

### Variable Declaration Rules
- Multi-declarations like `const a = 10, b = 20;` are forbidden.
- One declaration per line.
- Prefer `const`; use `let` only when necessary.

### Safe Operators / Functions
- Always use safe operators/functions.
  - Use `===` instead of `==`.
  - Use `includes` instead of `~indexOf`.
  - Use Math utility functions instead of `~~value`.

### Watcher Argument Naming
- Name watcher arguments based on the subject.
  - Example: `goodValue` → `(newGoodValue, oldGoodValue)`.

### Blank Lines Between Control Sets
- Insert one blank line between distinguishable control-flow sets.
