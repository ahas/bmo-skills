# UI Design Rules — Motion

Part of the `ahas-ux-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Motion

- A fade is for a thing that comes and goes on its own over the screen — a toast, a notice's
  content, a popover, a menu, a modal, a tooltip — and for nothing else.
  1. An item of a list never fades in or out, whatever brought it or took it away.
  2. The content of a page, a panel or a tab never fades as it is replaced.
- One screen moves along one axis at a time.
  1. A fade and a movement — a slide, a scroll, items changing places — never run on the same
     screen at the same moment.
  2. Nothing moves sideways while something beside it moves up or down, and the reverse.
  3. The parts of one change of mode — a selection mode's bar growing in and its checkboxes sliding
     in beside the rows — are one movement, whatever way each goes: they start in the same frame,
     take one length and one easing declared once for them all, and end in the same frame. Never
     one part first and the other once it has stopped.
- How a thing comes and goes must say where it came from.
  1. A thing that comes out of a particular place fades and scales together, growing out of that
     place and shrinking back into it, the way a window opens out of its dock icon: a modal opened
     by pressing a button grows out of that button, a menu or a popover out of what it stands beside.
  2. A thing that appears from nowhere in particular — a notice at the top of the screen — must never
     fade and scale together. It unfolds sideways from no width to its full width, its axis at its
     exact horizontal and vertical centre, and only its content fades, never its box. It folds back
     the same way as it goes.
  3. A thing that opens with no press behind it only fades, even where it is a kind that would grow
     out of its control.
  4. A sheet or a drawer slides in from the edge it belongs to and is neither.
- Content must never reflow while its box unfolds.
  1. The content is laid out at its final width from the first frame, and the box hides what lies
     past its edge while it is narrower.
  2. It is never cut short with an ellipsis, never given a scrollbar and never wrapped again during
     the movement.
- A list whose items come, go or change places while it is read — filtered, searched, added to,
  taken from, reordered — must show it by position alone.
  1. The items that stay slide from where they stood to their new places rather than jumping.
  2. An item that comes unfolds its height from nothing where it lands, and one that goes folds its
     height to nothing where it stood, the items around it closing up as it does. Its content is
     laid out at its full size throughout and only clipped.
  3. Every item moves at once and for the same length: no item waits for the one before it.
  4. Its items are keyed by their own lasting id, never by their position, or the wrong item moves.
  5. This is one shared list component, not a transition written per list.
- Switching tabs that show one list in different ways moves the list as one panel sideways.
  1. The new panel comes in from the side its tab stands on in the tab order, and the old one leaves
     to the other side, both together: a tab to the right comes in from the right.
  2. The items inside the panel move with it and have no movement of their own.
  3. A tab bar whose tabs are screens of their own, each a step of the address, changes page and
     moves nothing.
- A number that changes while it is read turns only the characters that changed.
  1. Where it grew, each changed character comes up from below and the one it replaces leaves
     upward; where it shrank, it comes down from above and the old one leaves downward.
  2. Each character turns in a window of its own, clipped to one character and one line, the places
     counted from the right; the characters that did not change, the words and unit round it and
     the rest of the line stand still.
  3. Its figures are tabular, so its width does not flutter as they turn. A number as first drawn
     stands still.
  4. It is one shared component, used by every number that changes on a screen standing open.
- A list replaced by a different one — the same panel read for another place, another account, a
  page change — stands at once, and none of its items moves.
- A long scrolled body, such as a conversation, opened or switched to stands at its opening place
  from its first frame.
  1. Where it opens — its end, or a place inside it it was asked to open at — is decided and scrolled
     to before it is first drawn. It is never drawn at one place and then moved to another, by a
     jump or a glide.
  2. What stands for it while it is fetched is its skeleton, and the content replaces it at once.
- Every movement must take its length and its easing from one shared set, declared once, never a
  number written beside a component. A test fails on a length written anywhere else.

  | Token | Value | For, and why this number |
  | --- | --- | --- |
  | quick | 150ms | A state changing in place — a colour, a mark turning, a tab marker — and a menu, popover or tooltip out of its control. Felt but never waited on; under 100ms reads as no change. |
  | pop | 200ms | A modal out of its control, its backdrop, a sheet sliding in. The reader is waiting to read it, so the shortest that still shows where it came from. |
  | unfold | 320ms | A box changing size — a notice unfolding or morphing, a region folding open, a list item unfolding or folding — list items sliding to new places, and a tab's panel sliding sideways. Edges travel furthest and the eye follows them; shorter reads as a jump. |
  | enter | `cubic-bezier(0.2, 0.8, 0.2, 1)` | Arriving: at speed, then settling, so it can be read before it stops. |
  | exit | `cubic-bezier(0.4, 0, 1, 1)` | Leaving: slow where the eye already is, then gone at speed; the enter run backwards. |
  | move | `cubic-bezier(0.4, 0, 0.2, 1)` | A thing that stays on screen while it changes; both ends eased. |

  1. The CSS framework's default transition length and easing point at quick and move.
  2. A component library's own overlay movements are kept and retimed onto the set, not replaced.
  3. A drawer a finger drags keeps its library's own curve, which also settles a throw.
- Each kind of movement has one tool, and one list is never moved by two.
  1. A list: the framework's own keyed list transition with its move step.
  2. A tab's panel: the framework's own transition of one keyed element, which moves the whole
     panel and nothing inside it.
  3. One element becoming another on the same page: a shared-layout animation library, always told
     to follow the reader's motion setting, since such libraries ignore it by default — but never
     for a box holding text or pictures that changes size on the way.
  4. Text does not scale, only the background transforms: where a box holding words or pictures
     changes size as it moves, its ground alone changes box — position, width and height, corners
     and tail kept — while its contents, laid out at their final size from the first frame, only
     move. A scale on the whole element stretches the letters and is never used.
  5. One element becoming another across two pages: the browser's view transitions.
- A view whose final state fills the whole screen — a fullscreen viewer, a dialog drawn fullscreen —
  stands and goes at once, with no change of size on the way.
  1. Its final shape is the screen itself, so any size it passes through is a shape that fits nothing.
  2. Where one is given a growing movement after all, the shape on the way is a window — an edge and
     rounded corners — which both go smoothly in the last moment, as it comes to fill the screen.
- A reader who asked for less movement sees nothing move, grow or unfold. A notice stands at once;
  a modal, a menu or a folding region at most fades where it stands.

