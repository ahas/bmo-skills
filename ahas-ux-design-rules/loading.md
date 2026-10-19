# UI Design Rules — Loading skeletons

Part of the `ahas-ux-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Skeletons

- A region that must fetch what it shows must reserve that content's place with a skeleton while it
  fetches, rather than standing empty and jumping when the content lands.
- A skeleton stands only where fetched content will stand.
  1. It stands for what cannot be known without the fetch: a piece of text, a picture, a figure, an
     item of a list.
  2. Everything drawable without the data is drawn at once in its finished form — the layout, a
     card's frame, a section title, a header, a button, a tab, a toolbar, a search field, a label
     beside a field, a region's own padding — and is never covered.
  3. A screen is never covered whole. The skeleton goes inside the region the data lands in, and
     nothing outside that region is touched.
  4. A screen fetching from several places gives each region its own skeleton, and a region that
     already has its data shows it while the others are still waiting.
- A skeleton must be shaped like the thing that replaces it.
  1. Its shape is read off the real thing, never invented: a shape chosen because it looks
     plausible is wrong however well it fills the space.
  2. A list's skeleton is a few copies of one item: the item's own height, number of lines, columns
     and frame, and the list's own gap between them.
  3. A passage of text is lines at the text's own size and line spacing, only the last line short.
  4. A picture is a box at the picture's own size and aspect. A figure is a box the width the
     figure will take, not the width of its cell.
  5. A table's skeleton has that table's columns, at that table's own row height.
  6. How many pieces stand is what the region holds, never more: a skeleton must not make a region
     taller than its content will.
- A skeleton stands only once its fetch has begun.
  1. What the server draws before the browser has asked for anything carries no skeleton; the place
     is kept empty at its full size, so nothing moves when the skeleton comes up.
  2. The frame around it — titles, labels, a card's border — is drawn from the first frame as always.
- A skeleton, once standing, keeps its shape until its content lands.
  1. It keeps the number of pieces and the height it stood with.
  2. It only ever goes away: never into a second skeleton, and never into content that still holds
     skeletons of its own — a place waiting on two fetches waits for both.
  3. No skeleton stands up after any other on the screen has turned into content.
- A skeleton is for the first fetch only.
  1. Content already on the screen stays there while it is read again, never taken down and put
     back as a skeleton.
  2. A fetch that succeeded and returned nothing is content: the empty state stands, and reading it
     again shows no skeleton either.
  3. What says a re-read is going is the spinner on the control that asked for it (「Interaction
     and state」 in `SKILL.md`).
  4. Items appended to a list already shown — a next page, earlier entries — may carry a skeleton of
     the incoming items at the end or the start of that list: they are having their first fetch.
  5. A fetch that failed shows its reason in place of the skeleton, never a skeleton that never
     ends.
- A skeleton carries no interaction and no words.
  1. It holds no button, link or field, and takes no pointer.
  2. It is announced as a busy status under a name, its only text.
  3. It does not animate for a reader who asked for less movement.
