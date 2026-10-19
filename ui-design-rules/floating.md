# UI Design Rules — Modals, drawers and floating elements

Part of the `ui-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Modals, drawers and floating elements

- On mobile a bottom drawer is only for a part of the page taken out:
  1. A side panel, or a region that chooses what the body shows — a list, a tree, a table.
  2. An auxiliary region pulled up to consult — sources, references, properties.
  3. A large modal that is a screen of its own — a log, an item's full detail, a record's full view.
- A dialog that is not a part of the page — a confirmation with something to fill in, a small form, a
  rename, a search — must stay a centered modal on mobile too, never a drawer. A yes-or-no question
  with nothing to fill in stands in the message pill instead.
- A list to pick from, opened at a point on the screen, must be a popover at every width.
- A modal on mobile must be centered in the visible part of the screen, so the virtual keyboard never
  covers its fields or its buttons.
- A modal's layout must be designed for the modal. A list/detail layout made for a narrow side
  panel must not be moved into a wide modal as is; the extra width must be used for multiple columns or
  tabs, and a desktop modal must be wide enough for its content.
- Everything floating over the screen must keep its content until it is off the screen.
  1. Closing is animated and watched throughout: what it draws must not be emptied on the closing
     press, or a blank frame slides away and the reader sees what they were reading vanish first.
  2. This holds however the content is drawn: from what is open, from what was chosen, from what
     was loaded for it.
  3. Opening it again on something else shows that straight away; nothing is held past the moment
     it can still be seen.
- Back must undo whatever the reader has to get out of — anything drawn over the screen and equally
  anything the screen has gone into — and the page must stay where it is.
  1. Drawn over the screen: modals, drawers, sheets, popovers, dropdown menus, select lists, context
     menus, lightboxes, command palettes, a yes-or-no question standing in the message pill.
  2. Gone into: a state the screen changed into and the reader has to leave — a selection mode, a
     full-window view, an editing state, a reordering state. A press that was about the mode must
     not take the page with it.
  3. Both are one shared mechanism and one registry, which finds every floating element on its own
     rather than by a registration each screen has to remember: a mode never gets a Back of its own
     written beside the one the overlays use.
  4. When several are stacked, Back undoes the top one only, one per press.
  5. Only when nothing is open and nothing has been entered does Back leave the page.
  6. A tooltip is not included, nor a transient notice, which goes on its own: a Back press spent
     on one would have to be made twice.
- Leaving such a state must not move the keyboard. A region a state stood down is built again when
  the state is left, and nothing in it may take the focus for itself on being built.
  1. A text box takes no focus of its own when built; where the focus belongs to it, the screen
     hands it over.
  2. The focus goes back only where it was there before the state was entered, and to whatever held
     it; otherwise it stays where it is.
  3. This holds for every way out of the state — the back gesture, a cancel, the state emptying
     itself — since they are all one leaving.
  - Wrong: a phone's keyboard rising over the content every time a selection mode is left.
- Opening and closing a floating element must leave the browser history as it was. A choice inside
  one that moves to another page leaves exactly one history step for that move.
- On a touch screen, a tap outside a floating element must only close it: what lies under the tap
  does not act; the next tap does.
- Selecting something in a list must add one step to the browser history, and the selection must be
  kept in the URL, so a shared link opens the same view.

## Tooltips

- Every tooltip must come from one shared tooltip component. A screen supplies its words and whether
  a tap acts on what it stands on; how it opens, how wide it grows, how it moves and how a finger
  reaches it are the component's, never the screen's.
- A tooltip must be reachable on a touch screen, which has no hover.
  1. Where a tap does nothing else, a tap opens it.
  2. Where a tap acts — a link, a row, an item that opens something — a held finger opens it, by the
     interface's one long press, and lifting that finger presses nothing. Where it stands on
     something answering a hold of its own — a menu, the start of choosing — the tooltip comes up
     with it, and goes once the finger moves on.
  3. The next touch anywhere closes it, and a touch on what it stands on does not open it again.
- A tooltip's words must wrap inside a width a phone has room for, never be cut a second time.
- Text cut short to fit its box must show whole in a tooltip, and only where it is actually cut.
  1. Whether it is cut is measured as the tooltip is about to open, so widening the box stops it and
     narrowing it starts it.
  2. Text shown whole carries no tooltip repeating it.
  3. Every cut line of the interface gets it from one layer standing over the whole interface, which
     finds the cut line under the pointer, the focus or the finger itself. A screen writes nothing
     for it, and a line already inside a tooltip of its own is left to that tooltip.
  4. A finger reaches it by holding the line, wherever the line stands, since the layer cannot know
     what a tap does there.
- A record shown by a bare identifier — a number standing for something that has a name — must name
  that thing in a tooltip.
  1. The tooltip carries the name and, where it helps, the thing's owner and state, in the words the
     interface already uses for them.
  2. One shared component takes the identifier, reads the record and attaches the tooltip, so a new
     screen showing such an identifier names it with nothing written for it.
  3. A place showing the whole name beside the identifier, wrapped rather than cut, carries none: an
     identifier with its name beside it never has a tooltip, and its name wraps there rather than
     being cut.
