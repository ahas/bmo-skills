# Vue / Nuxt

## SSR Safety
- SSR safety is mandatory.
- Direct access to `window` / `document` / `navigator` is forbidden without guards.

## Composition API / Macros
- Use Composition API with `<script setup>`.
- Props/Emits are typed via `defineProps` / `defineEmits` (or `defineModel` for v-model).

### v-model Proxy Priority
1. Prefer `defineModel`.
2. If unavoidable, use `props` + `update` event.
3. Use a `computed` proxy (avoid when possible).

### defineModel Rules
- Template generics are forbidden.
- `type` option is mandatory.

## Nuxt Auto Imports
- Use Nuxt auto-imports.
- Do not manually import composition APIs like `ref`, `computed` (exception only when necessary).

## State / Reactivity
- Scalars use `ref`; grouped objects use `reactive`.
- Prefer `computed` over watchers.
- Keep template logic minimal; move branching/calculation into `computed`.

## Async Data
- Use `useAsyncData` or module composables.
- Ad-hoc fetches are forbidden.

## Accessibility
- Provide labels/aria for interactive elements.
- Tooltips/buttons must use meaningful text.

## Style / UI
- Preserve existing Tailwind/utility patterns.
- Use class binding instead of inline styles.
- For state management, use installed **Pinia**.
- For UI composition, use the project’s primary UI framework.

## Props / Emits / Template Access
- Declare props/emits as separate `interface`s.
- When accessing props in templates, use the `props.` prefix.
- Component tags use PascalCase.

## Template Attribute Order (Strict)
1. directives
2. `ref`
3. `key`
4. `class`
5. `style`
6. `v-model`
7. static props
8. bound props
9. `v-bind` spread
10. events

Exceptions:
- If `v-bind` overwrites props, place it **between style and props**.
- If it overwrites events, place it **between props and events**.

## Macro Placement Rules (Strict)
- Macro placement/adjacency rules for `defineProps` / `defineEmits` / `defineSlots` / `defineModel` follow the source of truth in `skill/vue-sfc-critical.md`.

## Template Ref / Naming Rules
- Component refs and naming conventions (`useTemplateRef`, `Ref` suffix) follow the source of truth in `skill/vue-sfc-critical.md`.

## Boolean Ref Naming Rules
- Do not use boolean ref prefixes like `is`, `can`.
  - `isLoading` → `loading`
  - `canModify` → `modifiable`
  - `isDialogOpen` → `showDialog`
- Props may keep original names; rename only to avoid reserved word collisions.

## Readability / Spacing
- Use blank lines to separate multi-line behavior blocks.
- Template conditional spacing and `<script setup>` internal spacing follow the source of truth in `skill/vue-sfc-critical.md`.

## Bound Prop Variable Naming
- Bound prop variables should match the prop key.
  - Example: if prop is `items`, use `visibilityItems`.

## Page Composition Policy
- Pages should remain a single component when possible (except the outer shell like `NuxtPage`).
- `<script>` should focus on composition.
- Use only a few child components when unavoidable.
- Do not split small pages.
- If multiple components must share data, declare only the minimal shared variables outside the page.

## API / Data
- Use schema-based APIs (e.g., the owner's own API package).
- Preserve query normalization (pagination defaults, sort keys, etc.).
- Align SSR/client defaults to avoid hydration mismatch (initialize via route/query).
- Dates/times: use `dayjs` with i18n formatting.
- Prefer util/composable functions from `packages/asa`.
- If `packages/asa` lacks a suitable implementation, choose between `vueuse` / `remeda` as appropriate.
- If platform-specific libs (ThreeJS, CesiumJS, OpenSeadragon, ...) provide suitable built-ins, prefer built-ins over custom utils.
- i18n: user-facing strings **MUST** use `t()`; no hardcoded strings.

## Testing & Quality
- Run `bun check` by default.
- Add Vitest when logic is complex.
- Keep components small.
- If too large, split into subcomponents, but avoid excessive fragmentation.
- Keep comments minimal and only when necessary.

## Frontend UX
- Respect the existing design system (ASA/Nuxt UI).
- Preserve spacing/typography/interaction patterns (unless a redesign is explicitly requested).
- Avoid “AI slop”:
  - Intentional layout
  - Clear hierarchy
  - Purposeful spacing
- Prefer a small number of meaningful motions over excessive micro-animations.
