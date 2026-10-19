# UI Design Rules — Tables, list items and conversations

Part of the `ahas-ux-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Tables

- A settings page with two axes must show the current settings at once as a table — service
  providers against purposes, for example — with no pagination.
- A table that fills its container:
  1. Sticks its header to the top of the container.
  2. Scrolls horizontally, so it shows every cell in full and never cuts one.
  3. Fits the container's height exactly even when its rows are few, so the horizontal scrollbar
     sits on the container's bottom edge.
  - Exception: where a screen holds more than one table, or one table above other content, each
    table takes only its own height.
- When a table leads to a detail page per row, and the detail page has action buttons for that
  row, the table must offer the same actions in a more-menu at the right of each row.
- A table that edits settings must edit each value in its own cell.
  1. Pressing a cell opens a dropdown aligned to that cell, holding that cell's value and nothing
     else: a list for a choice, an input for a number or a text.
  2. A cell must never open a modal of the whole row's settings.
  3. A cell that only shows a derived value is not pressable.
- When a table leads to no detail page but needs an edit form of more than one cell's value, the
  form must open in a modal from the row's more-menu.
- A table must have exactly one column that takes the width left over: the one carrying the row's
  title, the words that say which record the row is. Every other column is laid out at its own
  content's width, up to a ceiling of its own, and never stretches to fill what is left.
  1. A column whose cells vary in length keeps that ceiling, and a cell too long for it is cut or
     scrolled inside its own cell.
  2. A column's width is one width down the whole table, so the cells line up: what the column's
     own contents ask for, not what is available.
  3. Dragging a column's edge changes that column alone: the flexible column still takes what is
     left, and the content-width columns still do not stretch.
- A table listing data — rows of records, however many — must let every column's width be adjusted
  by dragging the edge of its column header. A table whose shape its design fixes, such as a
  settings table whose columns are one axis of the setting, is not adjusted this way.
- A header naming the columns stands only where the cells stand side by side on one line; where the
  same content is laid out with its cells stacked, no header is drawn.
- A table of two columns, a name and its value — a list of properties, a record's details, a
  result's figures — must stack on a phone.
  1. At a phone's width each name stands on its own line over its value, never beside it. Only from
     the small breakpoint up do the two stand side by side.
  2. One pair is parted from the next by more than a name is from its value.
  3. Its content is never cut. A value that cannot break keeps its whole width and the table's own
     box scrolls horizontally; no value ends in an ellipsis, and the page never scrolls sideways.
  4. The shape is set once for the whole interface, and every such table carries it.
- Tables of one screen must be built from the project's shared table component, not each on its own.
- In a table with many columns, a status indicator must be compact: an icon and a color only.
  - When icons alone cannot express every status, a colored chip with text is allowed.
  - When there are too many statuses for colors to tell them apart, color may be dropped.
- A table inside reading text — a rendered document, a report, a message — must never squeeze its
  columns to fit a narrow text column.
  1. It is laid out at the width its cells ask for, up to a reading width per column, or at the text
     column's width where that is wider.
  2. A column keeps a readable minimum width, however short its words.
  3. A word in a cell breaks only at a space, never a syllable or a letter to a line; only a run
     longer than its whole column breaks inside itself.
  4. Past the text column's width the table scrolls horizontally inside a box of its own. The page
     never scrolls sideways.
  5. The rule is set once for every place that renders such text, and holds on mobile as on desktop.
- A picture inside reading text stands on a line of its own, centered in the text column, with its
  caption centered under it.
- A diagram inside reading text stands at the size it was laid out at, and is never shrunk until its
  words stop reading.
  1. It stands on a line of its own, centred in the text column.
  2. It may give up a small, fixed share of its size to fit the column. Past that it scrolls sideways
     inside a box of its own, as a wide table does, so its words keep a reading size on a phone. The
     page never scrolls sideways.
  3. A label breaks between words and never inside one.
  4. It is drawn in the theme the screen stands in, and drawn again when the theme switches.
  5. The rule is set once for every place that renders such text.

## List items

- A double press on an item must never select its words. Its text is selectable only where the item
  turns it into a field, as when it is renamed in place.

- An item in a list that is not a data table and carries a status must show an icon with that
  status.
- An item's information is split by importance.
  1. Core: what the item cannot be identified without.
  2. Secondary: what the item can still be recognised without.
- One cell of a row takes the width left over, and it is the title: the words that say which item
  the row is. What does not fit in it is cut, and the cut line shows its whole value as the next
  rule but one says.
- Every other cell — who it is from, a state, a label, a date, a count — is laid out at the size of
  its own content and never stretches to fill what is left.
  1. A cell whose content varies in length keeps a ceiling on its width, and what does not fit is
     cut or scrolled inside that cell rather than taking width from the title.
  2. A cell every row carries in the same place stands in a column of one width down the whole
     list, so the cells line up; that width is what the column's own contents ask for, up to its
     ceiling.
  3. Where the row runs short of width, the cells with a width of their own give way first; the
     title is cut last.
- A line cut short to fit its box must always let the reader see its whole value where it stands,
  in the tooltip every cut line gets by hover, keyboard focus or a held finger (「Tooltips」 in
  `floating.md`), with nothing written for it on the screen. A detail that carries the same value in
  full does not excuse the line: the reader is never sent elsewhere to read what a line cut.
- A cut line may be set moving — a marquee — instead, where its value has a settled form.
  1. A settled form is a name, a path, an identifier, a figure, a fixed vocabulary, so travelling to
     its end shows the whole of it. A sentence somebody wrote has no settled end, so moving it does
     not finish it, and it stays cut and shown in a tooltip.
  2. Importance decides none of it: the value a row is read by is cut like any other where its form
     is not settled, and a value of no importance moves where it is.
  3. A cell the reader scrolls by hand is not covered; this decides only a line that moves on its
     own.
- Core information that is short and of one shape — a number, a state, a time — is given the width
  it needs and is never shortened or cut.
- A header naming the cells goes only over rows whose cells stand side by side on one line; the
  same list laid out with its cells stacked gets no header.
- Numbers — progress, times, token counts — sit at the right end of the line after the free space,
  in tabular figures so they line up down the list.
- When the core information alone does not fit on one line even with the secondary information
  scrolling, move one core item to a row of its own.
- A long status on an item must not be cut with an ellipsis: it is shown in full inside its own
  area, scrolling horizontally there.
- Information every item carries in the same place, such as a time, stays in a fixed cell outside
  any scrolling area.
- Where the width allows, an item stands on one line, its cells on columns of fixed width, so every
  cell stands at the same place down the list. Where it does not, the item breaks into lines as
  above.
- A list that leads to a detail must be found as it was left when the detail is closed: the same
  scroll, the same choice, the same rows already read.
  1. The first way is not to take the list down at all — the detail stands over the list, or the
     router is told to keep the list's screen rather than build it again.
  2. Where the list is drawn again, its state must be restored from the address — the query and
     what else the address carries — and the scroll put back with it.
  3. A screen must not be named to the router by an address that includes the detail, or it is torn
     down and built again for every detail opened from it.
  4. Keeping a screen does not keep its scroll: its element is set aside while away, and a browser
     gives a detached element's offset back as nothing. The offset is written down as it is made and
     put back on the return.
- A navigation list item shows its title and status only; a growing counter that would crowd the
  title must not be shown there.
- A list that chooses what the body shows must show readable titles, not raw file names or keys;
  the raw name is shown in the item's properties.

## Reordering and choosing

- A reorderable list must have a drag handle on the left of each item.
- A list that can choose more than one item must let a drag choose a run of them.
  1. The drag chooses every item it passes, from where it began to where the pointer stands, and
     gives them up again when it comes back.
  2. A drag that reaches the list's top or bottom edge scrolls the list along with it.
  3. On a touch screen the drag begins with a held finger, which also enters the choosing state
     where the list has one.
  4. Choosing is not one drag. A list already choosing must be held again wherever the finger comes
     down: that item is chosen, everything chosen before stays, and a drag from it chooses on the
     same way. So a choice is made in as many holds as the reader needs, scrolling between them.
  5. What ends a press before it becomes a hold is a long press's own cancelling (「Mobile」 in
     `SKILL.md`): the finger sliding beyond the touch distance, or the list starting to scroll under
     it.
  6. A finger that lands on a list still moving under its own momentum — a flick's glide, an
     animated scroll — is stopping it, and is neither a press nor a hold however long it then rests:
     it chooses nothing, enters no choosing state and opens no menu. Whether the list was moving is
     read from its scroll position having changed just before the finger landed. Lifting and pressing
     again on the list now standing is a press as ever.
  7. A list whose items carry words a reader may want to select may keep the hold on an item already
     chosen for the browser's own text selection. Where it does, the hold there is always the
     browser's, and the list takes no other press for itself.
  8. This is one mechanism for the whole interface, not one per list.
- A list that can choose more than one item must offer choosing all of them and giving the whole
  choice up.
  1. The control is on the screen at every width from the moment the choosing state is entered —
     never folded into a menu, never disabled because nothing is chosen yet, never present at one
     width and absent at another.
  2. Where the items are chosen with checkboxes, this control is a checkbox too, at the three states
     the choice can be in: none of the items shown, some of them as the checkbox's own
     indeterminate state, all of them.
  3. A press takes the choice to whichever end it is not already at: anything short of all chooses
     all, and all gives the whole choice up.
  4. The control's name is the end a press would take it to, as the accessible name and the tooltip
     both.
  5. "All" means the items the list is currently showing, not what a page left behind still holds.
  6. This is decided in one place for the whole interface, not per list.
  7. The control must stand on the same vertical line as the controls on the rows it speaks for:
     their leading edges at the same place, the column of them read at a glance as one.
     - The control's box is held to the leading edge of its own cell rather than centred in it, so
       it lands wherever that cell's leading room puts it — the same room a row gives its own
       control — and nothing is measured screen by screen. The cell stays a square of the bar's
       height and the whole of it is pressed.
     - Where the rows stand in a column narrower than the region and the bar does not, the bar's
       contents stand in that same column.
     - Where the rows are inside a scroll and the bar is outside it, the bar keeps at its trailing
       edge the room that scrollbar takes, and the scrolling box reserves that room whether the
       content needs it or not. Otherwise columns centred inside and outside the scroll fall half a
       scrollbar apart, and the line bends the moment the content grows past a screenful.
- Items chosen one after another stand in one box, never a box each.
  1. One edge runs round the whole run, rounded at the large tier only at its two ends, with no line
     between its items and its ground running across any room between them.
  2. The box parts where an item between them is left out, and each item keeps its own checkbox.
  3. The boxes lie behind the items and take no room, so a run joining or parting moves nothing.

## Navigators

- A navigator is a list that walks a hierarchy — folders, sections, levels — one level at a time.
- Going to the level above is an item at the head of the list, `..`, in every view the list has.
  1. It stands only below the top; at the top there is nothing above to go to.
  2. It is never a toolbar button beside the one that goes back: back walks the way the reader came,
     and the level above is a place in the list.
- Every item offers every action that can be done to it through a context menu: a right press, and
  a held finger on a touch screen. The menu over the list's empty space offers what acts on the
  level itself — making, adding, pasting, how the list is laid out and ordered.
- What the toolbar and the context menu offer, the keys its desktop counterpart teaches offer too —
  copy, cut, paste, rename, delete, choose all — each named beside its menu item.
- A change that cannot be taken back, such as deleting, asks first, naming what it takes.

## Conversations

- A conversation is a list of messages read top to bottom, each from a sender, with an input under
  it.
- The measures of a conversation are one set of tokens declared once, which every part of it names:
  bubble corner 8px (the large tier), bubble inner room 8px top and foot and 12px sides, 8px between
  bubbles of a run, 24px between runs, a 32px photo 8px from its bubble, a 14px meta line on a
  20px line with 4px between its parts.
- Messages one after another from the same sender on the same side form a run, read as one block.
  1. Inside a run the bubbles stand a small step apart; between one run and the next stands a larger
     step, the same between every pair of runs.
  2. The sender's name stands once, inside the run's first bubble at its head; the reader's own runs
     carry none.
  3. Every corner of a bubble takes the large tier; where two bubbles of a run meet, the corners on
     the photo's side take the small tier, and the corner its tail hangs from is square.
  5. A run takes whatever each message carries — words, pictures, files, voice — and ends where a
     set time passes between two messages.
  4. Anything that stands between two messages — a date mark, a system line, a note under a message —
     ends the run before it.
- A bubble on either side is at most one width, set by the screen's width.
  1. On a phone it is the row less its own photo column and the choosing column below, running on
     over the rest of the far side's photo column: a phone has no room to keep that column clear.
  2. On a wider screen it is the row less the photo column on both sides, so the far column stays
     clear and the two sides never overlap, and also at most three quarters of the row less its own
     photo column.
  3. Either way the choosing column stays clear on the far side at all times.
- Choosing messages slides them aside and never reflows them.
  1. Entering the choosing state opens a column at the leading edge of every message, the mark's
     width and the photo's gap from it, and each mark slides in from past that edge.
  2. The messages on the leading side, photo and bubble together, slide aside by the column's width
     by transform alone, their bubbles keeping width, shape and lines. Those on the trailing side
     stay; a line centred across the conversation keeps the column clear on both sides and stays.
  3. The slide takes the unfold length on the move easing, leaving runs it back, and with less
     movement asked for the column stands or goes at once.
  4. A chosen message is shown by its mark filled and its line painted across the conversation's
     whole width, never by a box with room added round its bubble. Messages chosen one after
     another are one band.
- Only a run's last bubble carries a tail, hanging off its foot on the photo's side in the bubble's
  own colour, reaching a pixel into the bubble so no seam shows, and that corner is square. A message
  with no bubble — a sticker alone — has no tail.
  1. The tail shares its centre with the photo circle: its concave edge is an arc on that centre, its
     radius the photo's radius plus the photo's gap from the bubble, so it wraps the photo at that one
     gap all the way round.
  2. The arc leaves the bubble's side at the height of the photo's centre, where it runs straight
     down the side, and ends on the bubble's foot line, so the tail is as tall as the photo's radius
     and as wide as the gap.
  3. Its shape is computed from the photo's size and the gap; it carries no measure of its own.
- A picture stands in a bubble too, running to the bubble's edges and cut by its corners, with the
  caption under it in the bubble's inner room.
- A run has one photo, beside its last bubble, its foot level with that bubble's foot.
  1. While the run goes on below the visible part of the list, the photo holds a small step above the
     input bar's top edge, following it as the bar grows, and slides with the scroll until it meets
     the last bubble. It never stands behind the input bar.
  2. It never leaves its run: scrolled past the run's head, it goes up with it.
- Everything a message says of itself stands inside its bubble, on one meta line at its foot.
  1. What the message's turn did and spent stands at the leading end; how long it took, a verdict
     mark, the time and any state mark of the message at the trailing end, the bubble's inner room
     from its edge.
  2. Every part of the line is one size, one line height and one colour on one middle line, and its
     figures are tabular.
  3. A line carrying only its verdict mark, time and state mark stands at the end of the words' last
     line wherever that line leaves it room, and on its own line under the words only where it does
     not, so a short message is one line tall. Chips at its leading end, how long the turn took,
     anything standing between the words and the line, and words ending in a block no line runs
     through put it on its own line, its trailing end at the bubble's trailing edge.
  4. The line breaks between its two ends before it breaks inside its leading part: where they do
     not fit on one line, the leading part keeps a line of its own and the trailing end takes the
     line under it, at the trailing edge.
  5. Over a picture with nothing under it, the line stands on the picture's foot at the trailing end
     as an image overlay chip: black at half opacity in light letters, a small step in from the
     picture's edges, the same room at both ends and one smaller step between its parts, with no room
     kept for parts still to come.
  6. A pressable part of an image overlay chip answers a pointer by its letters alone — dimmed at
     rest, full under the pointer and pressed — and never lightens a box over the picture; its tooltip
     comes up clear of the chip.
  7. What a mark on the line spreads stands inside the bubble under the line, at the bubble's width,
     never as a box of its own under the bubble.
- A row standing for a reply still being written stands where its answer will — the same name,
  photo, words column and meta line — but on no bubble ground, with no tail and no time.
  1. When the answer takes its place, the answer's ground grows from the box the row's words stood in
     to the answer's bubble.
  2. The words are laid out at the bubble's size from the first frame and only move; nothing scales.
- A reply shows what it answers at the head of its bubble: who said it and one line of it, and a small
  preview of what it carried where it carried something to see.
- What arrives at the end of a conversation being read comes in moving, and nothing standing moves.
  1. A message from someone else rises a short way into its place, up the axis the list glides along,
     and never fades (「Motion」).
  2. A message the reader sends rises out of the input bar as its own finished bubble: ground, words,
     pictures and meta line stand at their final size, line breaks and places from the first frame, its
     first part where it stood in the input, and the whole bubble only moves into place, as one block,
     by transform alone. A state mark changing on the way changes on the landed bubble. Its ground never starts at the
     input's width and narrows, which reads as the words breaking anew on the way.
  3. A row taking the place of another — a message on its way becoming the stored one — comes in
     still, and so do rows read in by paging, by opening the list or by going to a message. A reply
     taking the place of the row that stood for it while it was written settles in, as above.
  4. Only the new bubble moves, never the row the list measures to hold the reader's place.
