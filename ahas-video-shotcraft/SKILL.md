---
name: "ahas-video-shotcraft"
bundle: producer
version: "1.0.0"
description: "How a job makes a video with the video-shotcraft studio (Remotion): where the studio stands, how it is set up on a machine, how a render takes the card, what is checked before the file goes out, and the licence lines the studio's own files draw. Use before the first step of any item that produces a video or edits one."
---

# Making a video with the studio

## Where the studio stands

- The studio is the video-shotcraft skill bundle, a clone of its own repository under the worker's own
  repository at `studio/video-shotcraft`. Its `SKILL.md` is the method and binds the work as if it stood in
  this skill; read it whole before the first shot. Its `references/shots/` are the shot recipe cards, its
  `template/` a runnable Remotion project, its `workbench/` the editor served after delivery, and its
  `assets/audio/` the licensed music.
- Where the folder is missing, make it: clone `https://github.com/Vincentwei1021/video-shotcraft.git` into
  `studio/video-shotcraft` and install the template's packages with `npm install` in `template/`. Remotion
  fetches its headless Chrome on the first render; where the machine's network refuses that download, name
  the machine's own Chrome with `--browser-executable`.
- The machine needs Node 22 or newer, npm, ffmpeg and ffprobe; the beat analysis of a music track needs `uv`.
  Check each before the first render and report what is missing rather than working around it.

## The render

- A project is one folder an item under the worker's repository: the brief, the beat sheet, the captured
  pages and pictures, the composition, the renders and `NOTES.md`.
- Render the first scenes first as a proof at full quality, check its frames, then render the whole. A render
  of any length runs from the project's folder through the repository's `tools/render-on-card.py`:
  `python3 <repository>/tools/render-on-card.py <entry> <composition> <out.mp4>`, with `--frames <first>-<last>` for a
  proof, `--props <file>`, `--muted`, `--crf <n>`, `--audio-bitrate <rate>`, and any further Remotion flag after `--`.
- That command takes a ticket from this machine's card arbiter under the occupant `render`, with no key, and waits for
  its turn however long that takes: the model services give way, and a claimant that takes the card stops the render,
  which goes on from its last finished piece once the card is back. It renders with Chrome on the card, gives the ticket
  back whatever ends it, and renders on CPU only where the machine has no card. A card that is taken is waited for, never
  rendered around. Never run `remotion render` yourself: the studio's template and a project's config may set ANGLE, which
  stands on the card with no ticket beside the model services.
- Its exit code says how it ended: `0` the file is written, `1` a step failed, `3` the node has no place for the occupant,
  which is reported and never worked around, 128 plus the signal for a stop. The repository's `docs/rendering.md` is the
  whole of it.
- After the whole render, run the studio's frame checks and its parity script, and look at the frames the
  checks flag with the platform's eye. A video that fails a check is not finished.
- A finished video is filed in the theater with `bmo/theater_file` at the quality the item asks, with no size
  limit of its own, and the room gets the theater's link. Only where an item asks for the file itself in a room is
  an MP4 under 50 MiB sent, since the rooms' chat services refuse more.

## What the studio's own files say of licences

- The code and the shot cards are Apache-2.0. The music under `assets/audio/` is Mixkit's free licence,
  except any track the attribution file marks as untraceable, which is not used. The template's sound effects
  are Mixkit's where the attribution file names them so, and not used where it marks the origin unknown.
  Sound effects fetched from Freesound carry their own licence each, credited in `NOTES.md`.
- Remotion's licence is free for a person and a small team; a company's use may need a paid licence. Say so
  in the report where the video is for a company's use, and leave the decision to the owner.
- The editor export for another tool writes device identifiers into its draft; strip them before such a
  draft leaves the machine.

## The record

- `NOTES.md` in the project keeps what was made, the sources of every picture and sound with their
  licences, the render settings, and the render time measured with the machine it ran on.
- The report names the file sent, its length and size, the render time, and anything the checks flagged
  and how it was fixed.
