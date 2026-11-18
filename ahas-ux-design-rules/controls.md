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
  3. Whether the form holds a change is read as the next rule reads it, against the values it opened
     with.
  4. The save is enabled only while something differs from those values, and disabled otherwise.
  5. The save goes at once. No confirmation stands between the press and the save, whatever the save
     sets off; what a field's change sets off — something restarted, reloaded or reset — is said
     beside that field while it differs.
  6. A save that went through makes what was saved the values the form opened with, from what the
     server answered: the form then holds no change, the save is disabled again, and closing or
     leaving asks nothing.
  7. A modal opened from inside the form to write one piece of it hands the piece back to the form
     with a confirm button, never saves it itself.
  8. A refused save keeps every change as written, with the reason beside the field it names, and
     what it saved before the refusal stands as saved.
  9. Not a form, and taking effect as it is done: a table whose cells are each their own editor, a
     page of standing switches, and the records of a list each stored on its own — adding, changing,
     moving or removing one. A list that is itself one setting, as an order, is a field of the form,
     and a switch shown inside a form is part of it; both wait for its save.
  10. The dialog and the page frame hold the save once for the whole interface; no form writes its
     own.
  - Wrong: a settings modal with a save button under each section, or a switch inside it saving the
    moment it is turned while the rest waits for the footer.
  - Wrong: 저장 asking "저장할까요?" before it saves; a dialog that, closed right after a save went
    through, asks whether to throw changes away.
- A dialog, a sheet or a page form — one that edits and one that creates alike — asks whether to
  throw changes away only while its values differ from the values it opened with.
  1. The values it opened with are read once its data has loaded, never before: a form filled in after
     it is drawn is compared with what it was filled in with, not with the empty fields drawn first. A
     form that creates opens with its blanks and defaults.
  2. Each field is compared with its own opened-with value, never by a flag set on input: a value
     changed and changed back is no change.
  3. Opened and closed untouched, it closes at once and asks nothing. After a save went through,
     closing asks nothing.
  4. Every way out asks alike: the close button, Esc, a press outside, the back gesture, another page
     or tab.
  5. The question is the interface's question dialog (`notices.md`), a floating element of its own
     (`floating.md`): Back, Esc and a press outside answer it as staying, and the form stays open
     under it with every change as written. Only its discard throws the changes away and goes.
  6. It is built only through the shared form-save module, once for the whole interface. In bmo-ui
     that module's `useCloseGuard` guards a dialog or a sheet and `useLeaveGuard` a page, both fed
     by its helper that keeps a form's opened-with values once its data has loaded. No screen keeps a
     dirty flag of its own or writes its own question.
  - Wrong: a dialog opened and closed untouched asking whether to throw changes away, because the
    values it compares against were read before its data arrived.
- A setting's choices must be its real values only; no choice stands for "the default" in place of a
  value.
  1. The value equal to the default carries `(기본값)` after it: in the list of choices, in the cell or
     field showing the chosen value, and in the placeholder of a box whose empty value stands for it.
  2. A setting left unset shows the value it runs on, marked the same way, where that value is known.
     Where it is not known, the field stands empty, with no word standing in for a value.
  3. No column, chip or label says where a value comes from with a word for "default".
  - Wrong: a list of '기본값', '켬', '끔'. Right: '켬 (기본값)', '끔'.
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

- Action buttons must always sit at the right of their container. A submit button, one that commits
  what the reader entered or chose (add, send, approve, save, confirm, apply), holds to this wherever
  it stands, alone or beside other controls.
  1. It stands at the right end of its row or container, never straight after the content on its left
     with empty room trailing it.
  2. Where it shares a row with a field or a select, the row takes one of two layouts: the control on
     its left takes all the room left over, shrinking with the row as 「Forms」 says, or a spacer that
     takes that room stands between the control and the button. The button keeps its own content's
     width either way.
  3. Only a single-line field or a select shares the button's row, and only where there is room.
     Where the row has no room, and under a text box of two or more lines, the button goes on the
     next row, at that row's right end. A box that holds its buttons inside its own edge, as a
     message input does, keeps them there, on its last line at its right end (`layout.md`).
  - Wrong: a field of fixed width with its 추가 button pressed against its right side, and the rest of
    the row empty after them.
  - Right: the field stretched across the row with 추가 at its right end; or a select of its own width
    at the left, a spacer, and 추가 at the right end.
- A button must never stand straight after words whose length changes with state, such as a
  status, a count, a timer or a name. A spacer takes the room between them so the button stays at
  the right end, or the words go on a row of their own. Changing those words must never move the
  button.
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
     toolbar that container is the toolbar itself, where its buttons' small size is set too (the
     Toolbar rules in `layout.md`).
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
- A screen's main create action — the one new item the screen is for, at most one per screen — must
  be the interface's one shared create component, never a button of the screen's own. The screen
  supplies what is created and what pressing it does; the look and the place are the component's.
  1. On a wide screen it is a neutral ghost icon button at the right end of the screen's own row (the
     Top bar rules in `layout.md`). Its tooltip names what it creates, then the key that presses it.
  2. On a phone it is one primary icon button floating at the bottom right of the screen. Only the
     main create takes this form. Its corner is a button's tier, never round, and the list under it
     keeps room at its foot as any floating button's does.
  3. No primary icon button stands in a row, and no create button carries both an icon and a word.
  - Wrong: a filled button with a plus and the word "New" at the head of a list, beside a round
    floating plus on the same screen's phone layout.
- An add that belongs to a row — a tab bar's add at the end of its tabs, the add at the head of a
  list or a sidebar — must stay in that row at every width, as that row's own control. It is not the
  screen's main create, and it never leaves its row for the floating button on a phone.
  - Wrong: a tab bar's add taken off the bar on a phone and floated at the screen's bottom right.

## Search

- A list's search must match the text actually shown in the list as well as the stored data.
  1. Every piece of text shown on a row — item number, status, stage, model, badge and chip text —
     must be searchable.
  2. The part of a result row that matched must be highlighted.
  3. When the match is in data not shown on the row, the matching part must be shown in one line
     under the row, so it is clear where it matched.
  4. List search must not be built separately per screen: it must use the project's shared search
     module.
- A dropdown that picks a value — a select of one value or of several — searches by what its list
  holds, never by a screen's choice.
  1. It searches once it holds more than seven choices, and with seven or fewer it does not, save
     where the next point says. Only choices are counted: a group heading and a separator are none. A
     list read from data takes its search the moment it grows past seven, on every screen at once.
  2. A list found on the server, asked for the words typed, always searches, and so does one that
     takes a value it does not hold, typed new, so there is a field to type into.
  3. The search field stands at the top of the opened list, above the choices, with its search icon
     leading. Opened by a pointer or a key, it takes the focus, so typing searches at once; opened by
     a finger, it does not, so the keyboard covers nothing until the field is pressed.
  4. It matches every text a choice shows, its label and the line under it, through the shared
     search module, and marks the matched part where it stands. A group heading stays while any
     choice under it stays, and a separator stands only between groups that both stay.
  5. Its words are one set everywhere: the search placeholder, the line for nothing matched, and
     the line for a list with no choices, or the screen's own word for its list where the wording
     guide gives one. A list found on the server adds the line for a search that failed.
  6. It is applied once, by the interface's shared select. No screen sets the count its list searches
     at, builds a search field of its own, or words these lines itself.
  - Wrong: a select of three choices opening on a search field, a select of forty without one, or one
    screen's select given a search field above its list by hand.
