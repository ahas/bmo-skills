# UI Design Rules — Forms, buttons, menus and search

Part of the `ahas-ux-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Forms

- Where a label stands beside its field rather than above it, the labels share a column of their
  own.
  1. The column is as wide as the longest label on one line. No label wraps, and none is shortened.
  2. The field takes the rest of the row.
  3. Every row of the form uses that one column width, so the fields start on one line down the
     form.
- A text box that shares a row with anything else must be allowed to shrink with the row: its
  default width is, on a narrow screen, wider than the whole row and pushes what stands beside it
  off the edge.
- A form page must fit its container exactly.
- A form page's action buttons must sit in a footer attached to the bottom of the container.
- A dialog's action buttons must sit in the dialog's footer.
  - Exception: a modal that is a page of its own, with several sections of its own, keeps each
    action beside the section it acts on — every action but saving, which the next rule keeps in the
    footer.
- A modal or a form that edits stored values must have exactly one save button, in the footer fixed
  at its bottom.
  1. No section, field, list or switch inside it saves on its own, and none carries a save button of
     its own.
  2. Every change is held in the form until that one save sends all of them together.
  3. Whether the form holds a change is read by comparing each field with its saved value, never by
     a flag set on input: a field typed back to what is saved is no change.
  4. The save is enabled only while something differs from what is saved, and disabled otherwise.
  5. The save goes at once. No confirmation stands between the press and the save, whatever the save
     sets off; what a field's change sets off — something restarted, reloaded or reset — is said
     beside that field while it differs.
  6. A save that went through makes what was saved the new saved value, from what the server
     answered: the form then holds no change, the save is disabled again, and closing or leaving
     asks nothing.
  7. Closing or leaving asks whether to throw changes away only while a change is held — the close
     button, Esc, a press outside, the back gesture, another page or tab. With nothing changed it
     closes at once.
  8. A modal opened from inside the form to write one piece of it hands the piece back to the form
     with a confirm button, never saves it itself.
  9. A refused save keeps every change as written, with the reason beside the field it names, and
     what it saved before the refusal stands as saved.
  10. Not a form, and taking effect as it is done: a table whose cells are each their own editor, a
     page of standing switches, and the records of a list each stored on its own — adding, changing,
     moving or removing one. A list that is itself one setting, as an order, is a field of the form,
     and a switch shown inside a form is part of it; both wait for its save.
  11. The dialog and the page frame hold the guard and the save once for the whole interface; no form
     writes its own.
  - Wrong: a settings modal with a save button under each section, or a switch inside it saving the
    moment it is turned while the rest waits for the footer.
  - Wrong: 저장 asking "저장할까요?" before it saves; a dialog that, closed right after a save went
    through, asks whether to throw changes away.
- A text box must grow with its text, and a screen has only one scroll.
  1. A long body input — in a form, a panel, a dialog, anywhere inside a scrolling region — has no
     maximum height at any width and never scrolls inside; only the region holding it scrolls.
  2. It is measured again whenever its width changes as well as its text — a scrollbar coming up, the
     window resized, a font landing — so text wrapped onto more lines never leaves it short with a
     scroll of its own.
  3. Growing never moves the scroll around it: the caret stays on screen while typing, and the page
     does not jump.
  4. Only an input pinned outside the scrolling region, where the screen is split between a view and
     an input area, stops at a maximum height and scrolls inside beyond it; on mobile even that one
     has none, so the page scroll is never captured by the box.
  5. One component sizes every such box; no screen measures its own.

## Buttons and menus

- Action buttons must always sit at the right of their container.
- A single-line text field and its button may share a row when there is room.
- Under a text box of two or more lines, the button must go on the next row.
- A group of buttons must be all icon buttons or all text buttons, never a mix.
- The number of buttons in a group decides their shape:
  1. Three or fewer: icon buttons, each with a tooltip.
  2. Four or more: one vertical-dots icon button opening a dropdown menu.
  3. On mobile, a vertical-dots dropdown in most cases.
- An icon button must always have a tooltip naming what it does — alone or in a group, in a toolbar,
  a header or a dialog, and as the button that opens a menu. Its text is the button's accessible
  name.
- A button whose words go when its container folds is an icon button from that moment, and carries
  that tooltip.
  1. This covers everything the fold strips to a mark: every row, the button that folds and unfolds
     the panel, a menu button at the panel's foot, a status line reduced to its icon.
  2. The tooltip says the words the button showed spread, and is its accessible name while folded.
  3. Spread, the words stand and the tooltip is not shown: the same name is never said twice.
- **A button that carries a word must never wrap that word.** Its label stays on one line at every
  width, on a phone and inside a panel dragged narrow as much as on a wide screen.
  1. A two-line label makes the button taller than its row, and a Korean label breaks between any
     two characters, where nothing would break a word.
  2. Where the width runs out, the answer is the one its container already has for overflow —
     scrolling sideways, an ellipsis, folding the group behind a menu — never a second line.
  3. The rule is set once for the container that holds the buttons, never button by button. In a
     toolbar that container is the toolbar itself, at the one place that already tells an item
     button from the rest, where its square floor is set too (the Toolbar rules in `layout.md`).
  4. A rule that fixes a button's width must be a floor rather than a fixed size, so a longer label
     pushes the button wider instead of folding into it. A stylesheet reads a button's elements, not
     its text, so a rule meant for icon-only buttons also catches a button whose word sits straight
     beside its icon; a floor keeps that button whole.
  - Wrong: a toolbar's 범례 button squeezed to a square, its two characters stacked one above the
    other.
- A bar of actions must decide whether to gather by the room the bar itself has, never by the
  window's width. A bar inside a side panel, one half of a splitter or over content in a reading
  column is as narrow as that part however wide the window is.
  1. Short of that room the bar keeps standing only what is reached for first, and folds the rest
     into one more-menu at the end of the row.
  2. The actions are written in the order they are reached for, and the first of them stand.
  3. How much room is enough, and how many stand on either side of it, is decided once for every
     bar of the interface, not per screen.
  4. A control the reader must have at every width — the one that chooses every row, the one that
     leaves a mode — never gathers.
  5. Flowing sideways is left for a bar that has gathered all it can and is still wider than its
     container; it is never the first answer.
  - Wrong: a toolbar in a panel a third of the screen wide standing its full row of actions,
    because the window it is in is a desktop's.
- One screen must never show two vertical-dots menus at once. Where one region already folds its
  buttons into that menu, a neighbouring group of three or fewer stays as icon buttons, on mobile
  too.
- A choice among options that may grow in number — a search scope, a mode, a view — must be a
  dropdown, not a row of toggles or a button group.
- The chosen option of a switch or a button group is marked by its colour or its background, never
  by an underline.
- Buttons of one group each take their own content's width with the same inner padding, so the group
  reads even; none is stretched to fill a share of the row, and none is narrower than its content.
- A dropdown must be its content's own width — at least as wide as its longest item, so no item is
  cut — capped at the screen less a small gutter each side, so it is never wider than the screen.
  1. Only a control that shows the value it drops for — a select, an input with a list — floors that
     width, at its own, so the list lines up under what it replaces. Nothing else does: not the
     button a menu drops from, whose width says nothing of the menu's content; not the fixed minimum
     a component library gives every menu; never a minimum of "fit content". The first two leave a
     short menu standing half empty, and the last, since a minimum always beats a maximum, runs the
     list off a narrow screen.
  2. A column an item reserves for a check, a key or a submenu arrow takes no room, gap included,
     on a row where it holds nothing.
  3. Past the cap the items scroll horizontally together, uncut; none ends in an ellipsis.
  4. The rule is set once for every select, dropdown menu and context menu, not per screen.
- Only one context menu may stand at a time. Pressing for a menu elsewhere closes the open one first.
- A dropdown or popover must be aligned to an edge of the button that opened it.
- A button that floats over a scrolling list — one fixed to a corner of the screen above the
  content — must have room kept for it at the foot of that list: the list's scrolling area ends with
  padding at least as tall as the button and the gap it keeps from the edge, so the last item,
  scrolled to its end, stands clear of the button.

## Search

- A list's search must match the text actually shown in the list as well as the stored data.
  1. Every piece of text shown on a row — item number, status, stage, model, badge and chip text —
     must be searchable.
  2. The part of a result row that matched must be highlighted.
  3. When the match is in data not shown on the row, the matching part must be shown in one line
     under the row, so it is clear where it matched.
  4. List search must not be built separately per screen: it must use the project's shared search
     module.
