# UI Design Rules — Messages, notices and indicators

Part of the `ahas-ux-design-rules` skill, bound the same way as its `SKILL.md`, which holds the rules
every screen answers to and the sweep.

## Messages and notices

- An interface must say everything it has to say through one mechanism.
  1. There is one message component, and the component library's own toaster is either that
     component or is not stood up at all: a message sent down a second road nobody watches is lost.
  2. The browser's own dialogs must never be used to tell or to ask: they are unstyled, outside
     every rule the interface is held to, and on a phone say nothing of where they came from.
  3. It is swept for rather than trusted: a test reads the interface's own files and fails on a
     second road.
- A transient message must stand at the top of the screen, centred, as a pill.
  1. It carries one radius at every size: a line on its own stands as a pill, and grown for a
     description or something to press it is a card of the same rounding.
  2. It is measured and given that size rather than told to fit its content, so it moves between one
     shape and the next instead of jumping.
  3. It unfolds sideways from its own centre and folds back into it, only its content fading, as
     「Motion」 in `motion.md` says of what appears from nowhere.
  4. Its surface carries its own text and mark colours, fixed whatever the theme. The tone is
     carried by the mark alone, never by the whole surface.
  5. It keeps clear of whatever the device puts at the top of its own screen, and on a narrow screen
     it stands inside the same gutter the content does.
- A notification is pressed in its whole.
  1. Wherever it leads, its title, description and mark all lead there; a button inside it is never
     the only way in.
  2. One that leads nowhere is dismissed by that same press.
  3. This holds equally for one the operating system draws and one the interface draws itself.
  4. A button inside it is only for a second thing it offers, such as taking back what was just
     done.
- Several messages arriving at once must not stack into a column, which covers the screen they are
  reporting on.
  1. One stands and the rest wait in a line.
  2. The one standing says how many are behind it.
  3. The next takes its place as the one before it goes, so the pill changes shape rather than going
     away and coming back.
  4. A message that may be said again before the first has gone is named, and the second replaces
     the first where it stands.
  5. The line is held to a length, and what is dropped is its oldest end.
- The pill tells and never asks. It carries a notice of something that arrived and the word that an
  action went through or did not, and nothing else.
- A question the reader answers before an action — whether to throw changes away included — is a
  small dialog in the middle of the screen, the interface's one question dialog, never the pill.
  1. Its title is the question, naming what the action is taken on or throws away. The item stands
     under it where the question does not name it, and what going through does stands in its body
     where that is more than the question says.
  2. Its foot holds the way out first, and the way through last, in the error colour, under the
     action's own verb rather than a bare yes.
  3. Only the way through answers yes. The way out, its close, Esc, a press outside and the back
     gesture close it and answer no (「Modals, drawers and floating elements」 in `floating.md`).
  4. A question asked while the page is already being left takes no history entry of its own: it is
     holding that navigation, and the navigation moves the history itself. Every other way out still
     answers it.
  5. A confirmation that carries a field, a choice or a form is a centred modal of its own.
  6. One shared question module opens it; no screen draws its own. In bmo-ui that is `askQuestion`,
     opening `QuestionDialog`.
  - Wrong: "삭제할까요?" standing in the message pill with 예 and 아니오 buttons. Right: the question
    dialog titled with what is deleted, 취소 first and 삭제 last in the error colour.
- A confirmation about one row or one item stays where that item is: a step that row grows, not a
  question dialog over the screen.
- A form's own refusal stays in the form, beside the field that was refused: a state of the screen
  rather than news.
- What has not been confirmed yet is not a failure.
  1. Checking and failing are told apart by their words and by their colour.
  2. While it is still being checked, a short calm line in the quiet colour says so.
  3. Only a failure that was actually met wears the error colour, in a short line.
  4. Once it is confirmed, nothing is said.
  5. A count or a state is not shown as nought or none before it has been read.

## Always-visible indicators

- An indicator that stays on screen — a status, a date, a reference time, a counter — must lead to
  what it refers to.
  1. It must respond when pressed: a persistent indicator that does nothing when pressed must not
     exist.
  2. A status leads to a page showing that status in detail, or to a modal showing it.
  3. A date or a time opens a date picker and moves to the chosen point.
  4. Any other value leads to where it is changed or seen in detail.
  5. A modal or popover opened this way must hold a button that goes to the target's own page.
  6. A pressable indicator must carry the hover and press states required of every clickable
     element.
