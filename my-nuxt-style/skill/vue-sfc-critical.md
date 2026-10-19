# Vue SFC Structure & Order (Critical)

This is a **critical** rule set and must be followed whenever writing/modifying Vue SFCs.

## SFC Tag Order (Strict)
1. `template`
2. `script`
3. `script setup`
4. `style`

- If both `script` and `script setup` exist, place `script` first (top) and `script setup` after it.
- Having both is allowed **only when unavoidable**.
- A typical unavoidable case is when type declarations must live in the top `script` block.

## Preferred Script / Style Form
- Prefer `setup lang="ts"`.
- Prefer `scoped lang="scss"`.

## `<script setup>` Declaration Order (Strict)
1. `definePageMeta`, `defineOptions`
2. *(blank line)*
3. `defineProps`
4. `defineEmits`
5. `defineSlots`
6. *(blank line)*
7. `defineModel` (no blank lines between consecutive declarations; no template generics; `type` option required)
8. *(blank line)*
9. Component refs via `useTemplateRef` (must use `Ref` suffix)
10. *(blank line)*
11. Utility composables (`useI18n`, `useRoute`, `useQuery`, ...), ordered from more “basic” to more “specific”
12. *(blank line)*
13. `ref` declarations
14. `reactive` declarations
15. *(blank line)*
16. `computed` declarations
17. *(blank line)*
18. `watch` / `watchOnce` declarations
19. *(blank line)*
20. function declarations
21. *(blank line)*
22. event listeners / hooks (`onMounted`, `addEventListener`, `manager.on`, ...)
23. *(blank line)*
24. module definitions (`defineExpose`, `provide`, ...)

## Spacing Rules Inside `<script setup>`
- Keep one blank line between watchers / hooks / functions.
- Use blank lines to separate multi-line behavior blocks.
- Keep one blank line between distinguishable control-flow sets.

## Ref / Reactive Declaration Ordering
- Prefer declaring `ref` / `reactive` from higher-level to lower-level.
  - instance → string → number → boolean
- Exceptions are allowed when there is a dependency relationship.

## Template Condition Branch Spacing
- Do not insert blank lines between conditional branches (`v-if` / `v-else-if` / `v-else`).
