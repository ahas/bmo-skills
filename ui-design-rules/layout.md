# UI Design Rules — Layout, scrolling and the bars

Part of the `ui-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Page layout and content width

- The horizontal margins of a content area must follow what the content is:
  - **Reading and entry content** — chat messages, forms, settings pages: the content must have a
    maximum width of 768px and be centered (`mx-auto` or equivalent). The 768px is the content's
    own width: a container's left and right padding is added outside it, never taken out of it. A
    highlight, hover or selection box drawn around an item keeps inner padding of its own, also
    outside the 768px, so the item's content keeps its full width and never touches the box's edge.
  - **Repeated, uniform records** — database views, admin tables: the listing component must fill
    its container edge to edge with no margin.
  - **Monitoring dashboards** — tiles and charts of live figures read across, such as a hardware
    monitor: the content fills the container rather than standing in the 768px column.
    1. Filling the container is not stretching every card across it. Tiles of one kind stand as
       many to a row as fit at the width their content reads well at; cards of different weight
       take different widths, the heavier ones the broader column.
    2. How many columns stand is decided by the room the dashboard's own container has, never by
       the window width, since a side panel or a sidebar takes a different share of every window.
- A page that does one thing must show that one component filling the container, with the header,
  footer or toolbar that operates it flush against the container:
  - A table's footer (pagination and the like) must stick to the bottom of the window whatever the
    table's size.
  - An editor with a header must have that header stick to the top of the container and the editor
    fill the container with no margin.
- A data table of roughly thirty rows or more, with pagination or sorting, must use minimal cell
  spacing so that as many rows as possible are visible.
- A side panel that is a fixed region of the service must be resizable with a Splitter. The
  splitter handle must look like a 1px line, and its grab area must be wide.
  1. The line takes only its own pixel in the layout, and the regions on either side reach it.
  2. The grab area is drawn over the regions' edges, never as room beside the line: room there
     reads as a stray margin down both sides of every region.
- A list or a table must reach its region's edges, with no strip of margin down its sides. Its own
  rows keep their inner padding inside that edge.
- A value of its own length shown whole — a path, a breadcrumb trail, an address — must never be cut
  or squeezed on a narrow screen.
  1. Where the row it stands in has no room for it, it takes a line of its own under that row.
  2. There it wraps onto as many lines as it needs, between its steps.
- A side panel that folds to its icons must keep every folded row on one axis.
  1. Each row's icon is centred in the folded width, and every row's icon lands on the same centre
     whatever else that row carries.
  2. A badge or status mark a row carries must not take a cell of its own in a folded row, pushing
     that row's icon off the shared centre; it is drawn over the icon's own corner instead.
  3. The row's name is gone when folded, so each row names itself in a tooltip, as every icon
     button does.
  4. A row that heads a group of rows folds to a mark of its own, and its members open beside it
     under the group's name. A folded group is never a row with nothing in it.
- A list whose rows lose their names must still say which item each row is — wherever a list is
  reduced to marks: a side panel folded to its icons, a rail of icons, a strip with no room for
  words.
  1. Where the rows are one kind of thing and differ only in what they point at, the kind's own icon
     identifies the row and stands as it is.
  2. Where each row is an identity of its own — an account, a person, a group, a workspace — one
     repeated icon tells them apart from nothing: each row stands as that item's own avatar, its
     picture or the first letter of its name on a colour drawn from its own identity.
  3. One identity is always the same letter in the same colour, wherever in the interface it
     stands.
  4. A mark shared by several of the rows — a kind's logo, a group's colour — is not that identity
     and must not be used for this.
  5. It is decided in one place for the whole interface, not per list.
  6. Such a row keeps its badge and its tooltip.
- Regions of different kinds standing side by side — body and source list, list and detail, code
  and preview — must be separate containers:
  1. Each region must scroll on its own; scrolling one must not move the other.
  2. Each region's scrollbar must sit on that region's own edge, never beyond another region.
  3. A Splitter must divide the two and let their widths be adjusted.
  4. Where there is no width to stand them side by side, follow the mobile rules for a body and
     the regions around it.
  - Wrong: inside one scroll container, only the side list is made sticky, so the body's scrollbar
    stands at the far right, outside the side list.
- A page whose sections can grow too long must split them into tabs and offer a search over the
  sections.
- A title row must not share its line with status and buttons: the title takes a whole row, and the
  next row holds the status on the left and the buttons on the right.

## Scrolling

- Any scroll other than the whole page's may use a scroll container such as ScrollArea, and its
  scrollbar must be thin.
- A scrollbar must sit on the edge of the container that scrolls, never floating inside the
  content's side margins or inside a max-width column.
- A scrolling region must fill its parent container. When a panel holds a fixed element such as a
  search bar together with a scrolling tree or list, the two must not share the same outer padding.
  1. A scrolling tree or list is itself a container, and must reach the parent's left and right
     edges with no margin.
  2. Its scrollbar must sit on its own edge, which is the parent's edge; the parent's padding must
     never push it inward.
  3. Items inside the tree or list must be aligned by their own inner padding, and that horizontal
     padding must equal the search bar's padding in the same panel, so the text of both starts on
     one line.
  4. Padding is not set once on the parent: the fixed element gets outer padding, and the
     scrolling region's items get inner padding.
  - Wrong: a side panel whose search bar and tree sit inside the same panel padding, so the tree's
    scrollbar stands away from the panel's edge.
- Lists and panel containers that are not tables must never scroll horizontally. Only a cell
  designated to hold overflowing information inside an item may, and a tree, as the next rule says.
- A tree of nested rows — folders, sections, an outline — must never cut a row's name.
  1. The tree stands as wide as its widest row, keeps every level's indent, and scrolls sideways
     inside its panel when it is wider.
  2. No name ends in an ellipsis: widening the panel must never be the only way to read a row.
- A list that grows at the bottom must keep up with what it adds while the scroll is already at the
  bottom, by what the list is read for.
  1. A list read for its latest entry — a log, a stream of output — keeps its scroll pinned to the
     bottom.
  2. A list read from where the new part begins — a conversation — scrolls only as far as keeps the
     first line of the new part in view: the item the new part begins with is never carried past the
     top of the viewport, clear of anything held at that top. Where the reader sent something, the
     new part begins with the first item after it.
  3. Additions to a conversation rise from the bottom: the list glides up to them rather than
     jumping, and moves at once for a reader who asked for less movement.
  4. A reader scrolled up to read is never moved by an addition, and the shrinking of the viewport
     under a reader at the bottom keeps them at the bottom.
  5. A way down to the end stands only while the reader stands away from the end, and never while the
     list's own glide is carrying the reader to what it added.
     - It goes while the reader scrolls up, slides up from under the input's edge once they scroll down
       short of the end, and slides back down as they reach the end; for less movement it stands and
       goes at once.
     - It stands a small step above the input's top edge, its trailing edge on the send button's, and
       moves with that edge.
     - It counts what arrived at the end while the reader stood away.
- Scroll anchoring: an item whose height the reader changes themselves — opening or folding something
  in it, spreading more of it — or whose picture arrives while it is in view of a reader scrolled away
  from the end keeps its place.
  1. Its top stays where it stood on screen, and what it gains opens downwards.
  2. Where what opened passes the foot of the visible part, the list glides up only until the item's
     top reaches the top, the same stop a new part's first line has.
  3. Such a change is never read as something added to the list: it is told apart from an addition in
     one place, for every kind of change, and only the item the reader acted in is held this way.
  4. A reader following the end is held by the end alone, and anchoring never moves them. What stood
     when they came to the end — opening the list, the way down, sending, their own scroll down — has
     been read: its pictures, cards and players sizing themselves late keep the reader at the very
     end, at once, with no frame drawn off it. Only their own move up hands the place to anchoring.
- A vertically scrolling container must never hold another vertically scrolling container.
  1. Content inside it grows without a height limit, and only the outer container scrolls.
  2. Regions standing side by side behind a Splitter are not nested; each scrolls on its own.
  3. A view that pins its input under a scrolling list lets that list scroll alone, and the
     container around it does not scroll.
  4. The horizontal scroll of an item's overflow cell or of a code block stays allowed.

## Tab bar, view switch, toolbar and top bar

- **Tab bar**
  1. Tabs that open and close — documents, files, sessions — must use the shared tab view
     component, and in-page view tabs a Nuxt UI component. Tabs must never be imitated with a row
     of buttons.
  2. Tab-like components must be stacked at most two levels deep, and one level wherever possible.
  3. A tab bar must never scroll vertically.
     - The bar keeps its own full height: its row has a fixed height and no padding above or below
       it, the tabs fill that height, and the bar stands against the container's edge.
     - A bar squeezed by the container's padding scrolls; hiding that scroll is no fix, and leaves
       the bar floating off the edge.
  4. When a toolbar sits directly under the tab bar, the content's action buttons go into the
     toolbar, not into the tab bar's action area.
  5. Otherwise the content's action buttons must not be scattered; they are gathered at the right
     end of the tab bar row.
- **View switch**
  1. A view that is either on or off — original or translation, wrapped or not — must be a state
     toggle, not a split bar.
  2. A choice of views whose number can grow — layouts of a list, ways of reading a record — must
     be one dropdown that names the view standing, never a row of buttons. Only a fixed choice of
     two or three views of the same content may be a split switch whose options share the width
     equally.
  3. A switch of views for the content under a tab bar goes into the tab bar's action area, not onto
     a tab row of its own.
- **Toolbar**
  1. Buttons and information acting on the same target must be grouped into one toolbar, not
     scattered as separate buttons.
  2. The toolbar must sit directly against its target: a toolbar for an editor goes between the tab
     bar and the editor with no gap.
  3. The toolbar must span the container edge to edge, its top and bottom borders continuing the
     borders of the tab bar and the editor.
  4. Items inside the toolbar are separated by dividers and must not each carry their own border.
  5. An item button whose whole content is an icon must be a square as wide as the toolbar is tall,
     so the row reads as one band of equal cells and a finger has the whole height to aim at. That
     square is a floor, not a fixed width, and no button in a toolbar wraps its word — both set as
     「Buttons and menus」 in `controls.md` says.
  6. A control drawn smaller than the row is tall — a checkbox, a small mark — must stand in a cell
     of that same square, and the whole cell must be pressable and carry the hover and press colors:
     a box of a few pixels is nothing a finger can aim at.
  7. A cell is as wide as what it holds. The room a cell of words keeps on either side must not be
     laid around a cell holding only buttons — a group of actions, the menu button a folded group
     opens from — whose buttons are already the row's squares, nor over a library component's own
     padding. Set once on the toolbar, off what a cell holds.
  8. On mobile the toolbar must be as thin as a single row of compact controls.
  - Wrong: an editor's symbol controls scattered as separate buttons detached from the editor.
- **Properties**
  - A file's or document's details — path, kind, size, author, language and the like — must not be
    laid out above the body but go into a properties viewer opened from a properties button. Above
    the body stands only the title needed to read it.
  - Wrong: a path, a category, a size and a language laid out in a row above a document's body.
- **Top bar**
  1. The top bar must hold only elements that span the whole app and mean the same on every page —
     opening the sidebar, usage, global search.
  2. An action button that concerns only one page's content — fetching the original, refresh, edit —
     must not go in the top bar, but in that page's title area.
  3. Information in the top bar must be a button that opens a popover, unless it is useful to see at
     all times. On mobile it must always be a button with a popover.
- **Bars lying over a conversation**
  1. A view read as a conversation lays its top bar and its input bar over the list: each runs the
     view's full width with no rounding and no margin round it, on a translucent ground that blurs what
     passes behind it, with the list running to the view's top and foot underneath them.
  2. Only such a view's bars are translucent; every other top bar is opaque.
  3. The top bar keeps every top bar's height, padding, title size and leading mark size, and its
     bottom edge lines up with a neighbouring region's header border as any header's does.
  4. A bar under the top bar for a mode of the list — choosing, say — lies the same way.
  5. The list keeps each bar's measured height as room at its top and foot, so nothing — its last
     item, a line under it — is left behind a bar when the reader stands at the top or the end; what
     holds to the foot while scrolling holds a small step above the input bar's top edge and follows
     that edge as the bar grows.
  6. The bars run to the view's edges, their lines meeting the lines beside them, and the list's
     scrollbar runs between them rather than under them: its track starts under the top bar and ends
     over the input bar, on its own edge of the view. What stands in a bar and lines up with the
     list keeps the scrollbar's room as padding at its trailing edge.
  7. The input box inside the bar is one box: the words carry no ground or corners of their own, and
     its buttons stand on the words' last line, the same inset from the box's edge on every side.
