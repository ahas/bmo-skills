---
name: "ahas-ux-design-rules"
bundle: worker
version: "1.18.1"
description: "The owner's design rules for every user interface a worker builds, restyles or reviews: the stack and component choice, type, contrast and spacing, layout, forms, states and motion, notices and indicators, controls and bars, tables and lists, navigators, search, overlays, the back gesture, mobile, and UI wording. Use before the first edit of any screen, when reviewing one, and again before the work on a screen is closed, to sweep what the change touched against these rules."
---

# UI Design Rules

These are the owner's own rules, and every one binds. "Must" and "must not" mean exactly that; a rule
that does not fit a screen is reported, never quietly bent. Where a project document or an existing
screen says otherwise, the work follows these rules for what it builds and reports the difference.

## Editing this skill

- Write every rule as a short imperative, under the general condition it applies to, never one
  product's screen.
- Never use a screen, a feature or a label of the service being built as an example. An example,
  when needed, is a generic case any interface could have.
- A rule goes into the file of its area (「The rules by area」); a rule every screen answers to goes
  into this file.

## Sweeping a screen before the work is closed

- A screen is swept against these rules once the work on it is finished, before it is reported or
  the item is closed. Reading the rules before the first edit is not the sweep: once the screen
  works, what was read is no longer in mind, so the skill is loaded again at the end.
- Sweep only the screens this change built or touched, and only the sections that change reaches —
  this file's own and those of every area file the change reaches, each opened again for the sweep.
  An untouched screen is not swept, and an unreachable section is not walked.
- Walk the screen against each of those sections rule by rule, rather than recalling the rules and
  looking at the screen. What goes missing is always a rule nothing on the finished screen asks
  for: a tooltip on an icon button, tabular figures on a number that ticks, thousands separators,
  a hover and a press state, a skeleton while it loads, the back gesture on a floating element,
  and the wording rules on every label written.
- Drive the screen, do not only read its source: hover, a press state, a scrollbar's edge, a drawer
  closing on Back and a mobile width are answered only by the screen running.
- Fix what the sweep finds in the same pass. A rule that can be met is met here, never left.
- Write the sweep down: which screens were swept, which sections they were held to, what changed
  because of it, and every place a rule could not be met, with why. A place left unmet is named,
  never passed over in silence.
- In a job, the sweep goes into the report under Checked, naming the design rules by name. An unmet
  place that is real work somebody has to pick up goes into `--unfinished` or up as its own row.

## Stack and component choice

- The stack is Nuxt, Nuxt UI and Tailwind CSS.
- A product named as a reference is followed for how it behaves and is laid out, never for how it
  looks: corners, colours, buttons, fields and bars keep the shared style and its tokens.
- Components must be chosen in this order, a later step only when the earlier has nothing that
  fits:
  1. A Nuxt UI component, looked up through the Nuxt UI MCP server where it is available, and
     through the `ahas-ux-nuxt-ui` skill otherwise.
  2. A new component composed of Nuxt UI components.
  3. A component written from scratch.
- A Reka UI component must not be used where Nuxt UI has one for the same purpose.
- Page switching, in-page view tabs and every other navigation control must be Nuxt UI components.
- Nuxt UI components must keep their default spacing: never override a component's padding or gap
  unless a rule below demands it.
- A screen must never build its own version of a general component the interface already has — an
  image viewer, a table, a menu, a dialog, a properties list, a progress bar.
  1. It uses that component, so the same thing behaves the same on every screen: a picture opened
     anywhere zooms, moves and steps to its neighbours the same way.
  2. Where the component falls short of what the screen needs, the component is extended, never
     copied and changed.

## Typography and icons

- Korean and general UI text must use **Apple SD Gothic Neo**, self-hosted, with Pretendard behind it
  for the letters it lacks.
  1. Only the weights the screens use are carried, each with tabular figures, and the page preloads
     them, so the first screen is drawn in them rather than swapping into them.
- English system messages and similar machine text must use **CaskaydiaCove Nerd Font**.
- Icons must come from **Material Symbols**; file-type icons in a file explorer use the Material
  Icon Theme set.
- Code shown in a code viewer must be syntax-highlighted for every language the viewer can open.
- Font sizes must follow this table:

  | Use                            | Tailwind    | Size |
  | ------------------------------ | ----------- | ---: |
  | Badge / Caption / Metadata     | `text-sm`   | 14px |
  | Small Body / Table / Secondary | `text-sm`   | 14px |
  | Normal Body                    | `text-base` | 16px |
  | Subtitle                       | `text-lg`   | 18px |
  | Title                          | `text-xl`   | 20px |
  | Section Title                  | `text-2xl`  | 24px |
  | Page Title                     | `text-3xl`  | 30px |
  | Large Page Title               | `text-4xl`  | 36px |

- `text-xs` must never be used: it is too small.

## Numbers

- Every numeric value must carry thousands separators, whatever its size.
- Numbers that keep changing — uptime, counters, timers — must use tabular figures, so a chip or a
  button keeps its width as the value ticks.
- A total shown split into parts must name the parts with short labels joined by a middle dot
  (`R · W · CR · CW`), and must also show the ratio as a proportional bar.
- Several shares of a whole shown together must stand as one group of bars on shared columns.
  1. The names stand in one column as wide as the longest, the bars start on one line and the
     figures end on one line, so the group is read down as well as across.
  2. A figure with no whole in the same group keeps the bar's place empty rather than closing it up.
- A count shown as a badge beside a label — unread items, a tab's rows — is one small pill everywhere.
  1. Caption size in normal weight and tabular figures, on a line no taller than the label's, centred
     on the label, so it never makes its row taller or louder than the words beside it.
  2. A tint of its colour under the colour's own words, never the accent filled solid: the accent where
     the count is news to the reader, neutral where it only says how many.
  3. Rounded whole at its ends, at least as wide as it is tall, and declared once for every such badge.
- A measured value — of hardware, an instrument, a sensor — must be written in its physical unit.
  1. The figure is the quantity the thing has: a capacity in GB or TB, power in W, temperature in
     °C, a clock in MHz, a rotation in RPM, a voltage in V.
  2. A share of a whole never stands in its place. Where the value has a whole, the share may stand
     after it: `12.3 / 32.0 GB 38%`.
  3. Only a value with no physical unit of its own, such as a utilisation, is written as a share.
  4. It is exact: one unit and one precision for the same kind of value wherever it stands, and a
     figure that could not be read says so rather than standing as nought.
- A time in the future — a reset, a deadline — must be shown as the time remaining ("3시간 남음").
- Progress must be shown as `n/n n%`, with no words around it.
  1. Either part may stand alone.
  2. When the target is fixed, the percentage is always shown.
  3. Exception: where each finished piece of the work appears in the content as it arrives, the
     content is the progress; show only a spinning mark, with no count.

## Color and contrast

- Muted text must still contrast clearly with its background; a secondary color so dark it sinks
  into the background must not be used.
- A coloured badge's words must read at 4.5:1 or better against every surface it is read on, in
  every theme.
  1. The shade a theme fills with is not a text colour. Where a badge's words are drawn in it, or a
     light word is laid on it, the badge takes a darker step of the same colour in that theme.
  2. The step is set once, on what every badge carries, and every colour and shape follows it —
     never badge by badge or colour by colour.
  3. The badge keeps its shape: a filled one darkens its fill under its word, a tinted or outlined
     one darkens its word.
  4. A theme whose colours already read is left as it is.
  5. The surfaces are those a badge rests on or is hovered over — the page, a muted panel, a raised
     card. A press is a flash and is not one of them.
  6. The ratio is measured in the browser, on every colour, shape, surface and theme, and the
     measurement is kept as a test.
- Every clickable element must change color on hover and on press, each differing from its resting
  color.
  1. The rule holds the difference from rest; hover and press need not differ from each other.
  2. One color for both keeps the rule as long as it is not the resting color. A Nuxt UI ghost
     button — hover and press paint one token over a rest that paints none — is correct as it
     stands, never a rule to be worked around.
  3. An element whose hover color is its resting color has no hover state, however the style is
     written, and the same holds for press.
  4. The rule does not reach a ready-made component an external UI library supplies, such as a Nuxt
     UI USwitch. Its hover and press states are left as the library ships them: no color is painted
     over them in the shared theme or on a screen, and no browser check of its colors is written.
  5. The exception covers the library's own states and nothing built around them. A clickable
     element this interface draws itself, or a library component whose resting color a screen
     sets, is held to the rule.
- A waveform, a visualiser or any progress fill is drawn in solid colours with no gradient: the done
  part in the accent, the rest in one dimmed colour, meeting at a hard edge at the exact place.
- Where people are told apart by colour — the names in a conversation — the colours come from one
  fixed palette of the theme's own colour tokens, never a hex value.
  1. Eight colours at most, each at the step that reads at 4.5:1 or better on every ground the name
     stands on, in every theme.
  2. The colour is chosen by the person's lasting number, so one person — an agent included — wears
     one colour on every screen.
  3. The palette is declared once as tokens.
- A region that carries its own background must pair its own text color with it.
  1. Content that arrives from outside the interface, and any region given a background of its own,
     is read against that background, not the screen's.
  2. The theme's default text color must not reach inside such a region; where the region's
     background is fixed whatever the theme, its text color is fixed with it.
  3. A region that sets no background keeps the screen's own surface and colors, and reads in every
     theme.
- A background color must fill its region from edge to edge, with the padding inside the color.
  1. The color reaches the edges of the screen or of the container it stands in.
  2. Padding must never cut the color into a band with a strip of another color outside it. A
     region inset by its parent's padding is pulled back out to the parent's edges and given that
     padding inside itself instead.

## Focus

- Focus must be shown only where the keyboard put it.
  1. A control focused by a mouse press or a finger draws no ring; one reached by Tab, an arrow key
     or focus handed back after a key closed something draws it.
  2. The ring is drawn on `:focus-visible`, never on `:focus` alone.
- The focus ring must be one shared style, declared once as tokens: its colour, its width and its
  gap from the element.
  1. The colour is the theme's accent, at the step that reads at 3:1 or better on every surface it
     is drawn over, in every theme. Never white, never the control's own colour.
  2. It is thin — 2px — and stands a small gap outside the element, as an outline, so it follows the
     element's own corner radius.
  3. A row inside a box that clips what stands past its edge — a scrolling list, a tab bar, a table
     — draws it inside its own edge instead, at the same width.
  4. Where the control a row is pressed by is smaller than the row, the ring goes around the whole
     row, in the row's corners.
  5. A component never names a ring colour or width of its own, and the component library's own
     focus colour and width are set aside for the shared ones. Only whether the ring is drawn, and
     inside or outside, is the component's.
- A field — a place a value is typed or chosen in — must show focus on its own edge, with no ring.
  1. Its edge takes the accent while it holds focus; a field drawn with no edge takes a background
     one step up instead.
  2. A bar written in, such as a message input with buttons inside it, shows its text's focus on the
     bar, with no ring around the bar or the text: its background steps up however it was focused,
     and its edge takes the accent only where the keyboard brought the focus. Its buttons ring as
     every button does.

## Corners

- Every corner takes one of a few shared tiers, all read off the component library's one radius
  token; no arbitrary radius and no radius written inline.
  1. Small, the radius token itself: a chip and a badge, as the library draws its small badge, and
     what concentric corners call for inside a large box (below).
  2. Medium, the library's own default: a button, a field, a highlighted line.
  3. Large: a box that holds others — a card, a panel, a message bubble, an input box, a list item
     drawn as a card.
  4. Round: a face and a dot, nothing else.
- A rounded box makes everything near its edge follow its corner, or it is not rounded.
  1. An element inside it closer to its edge than its radius takes its radius less that inset, so the
     two curves are concentric.
  2. One standing the radius or further in is clear of the corner and keeps its own tier.
  3. A picture running to the edge takes no corners of its own and is cut by the box's.
  4. A larger radius on the outer box is never given without measuring every element inside it this
     way.
- Chips and badges standing side by side are one tier, one height and one padding.

## Spacing

- Containers and the elements inside them should use spacing between 8px and 16px.
- Every spacing value must be a multiple of 4px.
- Within one unit — a page, a modal — the padding must be the same everywhere.
- Spacing must be even, and must only differ where it separates.
  1. Inside one group of related elements — a label, its content and what hangs under it — every
     gap is one value.
  2. Between groups the gap is one larger value, the same between every pair of groups.
  3. A component's own default spacing that breaks either rule is overridden.
- A group of small elements placed among larger content must sit in a container of its own, which
  keeps their own tight gap inside and is spaced from the content around it by that content's own
  spacing.
- An element attached to a row's leading or trailing edge and held to the row's top — a checkbox, a
  mark, an icon, an avatar beside text of several lines — must be inset from that edge by the same
  value it is inset from the top.
  1. Both insets are measured from the box the row draws when it is hovered, chosen or highlighted,
     not from the column the row's content stands in. A row whose box reaches outside that column
     takes the difference off the element's own inset, so the two insets are equal on the box.
  2. An element centred in a row of one line is not covered: its vertical place is the row's centre.
  3. A header is not covered: what stands in it is centred, as the next rule says.
- Everything standing side by side in a header must be centred on the header's vertical axis.
  1. A header is any row that heads a region: a page's or a panel's top bar, a dialog's or a
     card's head, a section's title row, and a band directly under one that states a state and
     offers its action.
  2. The title, a mark or a count beside it, an icon, an avatar, a badge and every button share one
     centre line. None is held to the top or to a text baseline, or nudged onto the line with a
     margin.
  3. A title of two lines or more is centred as one block, and what stands beside it is centred
     against that block.
  4. It is set once on the header's row and on every group inside it, never item by item.

## Borders and dividers

- A containing box is one layer deep.
  1. A box that encloses a region with a border, a background and inner padding must not hold
     another box of the same kind.
  2. The groups inside such a box are parted by titles and by spacing.
  3. Only where each item repeated inside the box is a target of its own to press are the items
     told apart, and then by one step of background with no border.
- A drawing whose cell boundaries are its content is not built out of stacked boxes. Where the
  shape, size and place of each cell are the information it carries, it is drawn with lines and
  filled areas, and its cells carry none of a containing box's padding, shadow or rounded corners.
- A divider stands only where a different group stands on each side of it. A line that lands on
  the edge of a box or a header, or stands alone at the very start or end of a region, must not be
  drawn.
- Borders of neighbouring layout regions must line up. When a content header and a side-panel
  header both have a bottom border and the same height, the two borders must form one straight
  line.
- A divider inside a bordered container must reach the container's border; the container's inner
  padding must never leave a gap between them.
- Lines meet with no gap. A horizontal line that runs towards a vertical one — a header's or a
  bar's edge reaching the line between two panels — ends on it.
  1. Nothing shortens the line's box: no padding, margin or inset around it, and no scrollbar's
     gutter beside it.
  2. Room the box's contents must keep clear — a scrollbar's width, a column to line up with — is
     kept as padding inside the box, so the line runs on past it.
- Two adjoining panels must not both draw the shared edge: only one draws it, so the boundary is
  never 2px thick.
- A hairline must land on whole device pixels. It is never moved into place with a transform, which
  leaves it between two pixels as a blurred double line.
- These rules govern layout dividers — between regions, between the parts of a panel, between a
  form and its actions. A divider inside a flow of content, such as between messages or runs of a
  log, belongs to that content and is not covered.

## The rules by area

- The sections above and below this one bind every screen. The rest stand in files beside this
  one, one area each, and bind the same way.
- Before the first edit of a screen, read every area file the work reaches as well as this file;
  the sweep opens the same files again at the end.
- `layout.md` — Page layout and content width; Scrolling; Tab bar, view switch, toolbar and top
  bar. Read it when a page or a panel is laid out, a region scrolls, or a tab bar, view switch,
  toolbar or top bar stands on it.
- `controls.md` — Forms; Buttons and menus; Search. Read it when the screen has a field, a button,
  a menu or a search.
- `notices.md` — Messages and notices; Always-visible indicators. Read it when the screen tells the
  user something — a result, an error, a warning, a confirmation — or keeps an indicator in view.
- `lists.md` — Tables; List items; Reordering and choosing; Navigators; Conversations. Read it when
  the screen shows a table or a list of items, lets items be reordered or several be chosen, walks
  a hierarchy one level at a time, or holds a conversation.
- `floating.md` — Modals, drawers and floating elements; Tooltips. Read it when anything opens over the
  screen — a modal, a drawer, a popover, a sheet, a tooltip — or closes on Back.
- `motion.md` — Motion. Read it when anything on the screen moves, comes, goes, grows or changes
  size, or a list or a tab changes what it shows.
- `loading.md` — Skeletons. Read it when a region of the screen fetches what it shows.

## Interaction and state

- A button that refreshes data or changes a state — a refresh button, a state toggle — must show a
  spinner on itself until the work completes.
- An icon that marks work in progress must spin; a still progress mark reads as a static circle.
- An element that already shimmers or glows while its work goes on carries no spinning mark: the
  shimmer or glow is the whole sign, and the same holds for what stands beside or under it as part
  of the same group.
- Status must be shown by where it stands: in a list as a compact icon, on a detail page as a chip
  with text.
- A transient element — a notice, a toast, a one-off message — must never appear beneath a
  persistent element, but above it. A number alone does not lift it where the interface's root is
  an isolated stacking context: it is drawn into the page's body, beside whatever the component
  library floats there.

## Mobile

- Every screen must be designed for mobile as well, and designed and checked against the viewport
  of a Galaxy S25 in portrait.
- At that width, toolbars and tab bars must fit within a single row's height and must not cover the
  body. Wrong: two view-switch rows stacked on mobile, swelling the toolbar into a block that
  squeezes the body.
- A screen made of a body and the regions around it must keep the body on screen.
  1. A region that chooses what the body shows — a list, a tree, a table — opens as a bottom drawer
     from one button, as the drawer rules in `floating.md` allow.
  2. An auxiliary region pulled up to consult — sources, references, properties — opens the same
     way.
  3. These regions must not be switched with the body by a toggle row, which costs the top of the
     screen.
  4. Only peer views of the same content are switched with a view switch.
- A long press — a finger resting on one point, a touch screen's right press — must be cancelled,
  with nothing opened and nothing chosen, the moment the finger slides beyond the platform's own
  touch distance from where it landed (about ten pixels) or what it stands on starts to scroll, so
  it is never mistaken for a scroll or a drag. Lifting the finger, a second finger landing and the
  browser cancelling the touch end it the same way, and a scroll that stops with the finger still
  down never becomes one. Once fired, what it opened stays — a menu while the finger moves on or
  lifts, a drag a press on a list began — and every long press of the interface is read by one
  shared mechanism, never by a timer of its own in each screen.
- On a phone, Enter must insert a line break; sending happens only through the button.
- An input or a bottom bar must never be hidden behind the virtual keyboard.

## UI wording

- Descriptions of an item must be short.
- A complete sentence ending in `.` must be followed by a line break, so the text reads easily.
- Labels must be terse: every extra character costs horizontal space.
  1. Drop particles such as '~에', '~의'. Right: '파일 리뷰'; wrong: '파일에 리뷰'.
  2. Do not repeat what the screen already shows, such as the name of the open tab. Right: '리뷰';
     wrong: 'notes.md의 리뷰'.
  3. An action button names the action alone, not its object. Right: '저장', '전송'; wrong:
     '설정 저장', '메시지 전송'.
  4. Do not over-explain. Wrong: '메모 (비워 두고 저장하면 지워집니다)'; right: '메모',
     '사유 (선택)'.
- Excessively purified Korean must not be used in place of established terms.
  1. The word people commonly say is written, whatever its origin: a Sino-Korean word or a loanword
     in its Hangul spelling is never swapped for a native word or a coinage. Right: '위치', '툴바',
     '빌트인'; wrong: '자리', '도구띠', '붙박이'.
  2. English must not be dropped into Korean text. Right: '툴바를 눌러'; wrong: 'toolbar를 눌러'.
  3. English stays only where no Hangul spelling is in common use — code identifiers, commands,
     file names, product and library names.
  4. Read the UI wording guide through the `bmo-ux-wording` skill before writing Korean text: the one
     word each console concept takes in every inflection, one style per screen, no dash. That skill says
     where the guide lives, how a term is added, and how a use is reported and fixed.
- A page, section or panel header must not carry a sentence introducing what the screen is.
  1. The title and the layout alone must make clear what the screen is for.
  2. Sentences that describe the screen or explain how to use it — "~한 기록입니다", "~를 봅니다",
     "여기서 ~할 수 있습니다" — must not be written.
  3. Guidance is allowed only in an empty state or an error state, and then in one line.
- A screen must not explain what it did internally or why it looks the way it does. A fallback —
  opening at the first line when a target is not found — must happen silently; the user is told
  only when they have to do something.
