---
name: bmo-song-cover
description: Making a song sung in the chat agent's voice from its title alone, on a GPU node with the song bench of the bmo-song repository — by default a cover, YuE2 singing it afresh from the original's melody and lyrics in the chat agent's range over the original backing, or a remix over a new backing in a given style — every candidate video handed to the owner and a chosen take kept as a preset. Use when the owner or the chat agent asks for the chat agent to sing a song, for a cover, a remix or replay of a song already made, when `make` stops asking for lyrics to be put into YuE2's form, or for any change to a song's sound, tempo, style or video.
bundle: project
---

# A song in the chat agent's voice

The song bench is the bmo-song repository: every relative path below is inside its checkout,
and every command runs there. Its `docs/song.md` is the canonical account — 「Made from a title」 above
all: the command, its output, the style, the lyrics search, the backing finder and the YuE2 form. Read it before the first run.
This skill is how a job carries a song request through it.

## What is asked, and what is not

- **A song is asked for by its title,** with the singer where the asker named one, `--remix` where
  a remix was asked for, `--voice natural` where the song was asked for without the chat agent's voice or
  timbre, and `--style` only where the asker named a style for that remix — written
  as `docs/song.md` says: one line of comma-separated English tags, a tempo as `<n> BPM`.
- **Everything else the tool finds and sets by rule:** the recording (the backing finder), the
  lyric page (the lyrics search), the style a remix falls back on, the candidates and their voice
  levels (the settings' `make` section). A job never picks a recording, looks up or types lyrics,
  or sets a pitch move, a take count or a mix level for a song.
- **Never the singer's own voice converted.** `make` makes no original: a request whose `original`
  converts the recording's vocals carries the singer's timing, phrasing and delivery into the chat agent's
  voice, and is not started for a song request. A value the owner wants changed goes
  into `scripts/song/settings.json` or the stage, and `docs/song.md` and this skill with it.

## Running it

1. `bun scripts/song.ts check` — the bench on the node is whole. A `MISSING` line is set up as
   `docs/song.md` says before going on; an `absent` deno or `yt-dlp-ejs` does not stop a song, and
   is named in the report as host work for the chat agent where deno is the one missing.
2. `bun scripts/song.ts make "<title>" [--artist "<singer>"] [--remix [--style "<style>"] | --transpose <n>] [--voice natural] [--seed <n>] --chat <his chat id>`
   — it finds the recording, starts the run, follows it to its end and posts every video into his
   chat as it is made. A run takes tens of minutes: start it as a background task, and read its output file, one read to a
   call, until it ends.
   Run again with the same arguments, it takes up where it stopped.
3. Read its output lines (`docs/song.md`, 「Its output」): `UNAVAILABLE` a candidate recording yt-dlp
   could not download, passed over for the next, `RECORDING` the recording chosen and downloaded,
   `LYRICS` the lyric page, `FORMAT` how its lines were given, `KEY` a cover's measured pitch,
   the chat agent's range and the semitones the melody and backing were moved, `LISTEN` the lines of a take with lyrics heard sung otherwise than
   written and when, `VIDEO` and `POSTED` each candidate with its take and grid combination,
   `BROKEN` a take dropped and why, `DONE` or `FAILED` the end. Status 3 with `FORMAT NEEDED` is the
   form step below.
4. **Hand the owner every candidate:** each video not broken is posted to his chat as a file for him to
   hear; say which take and grid combination each is, and which were dropped as broken. He chooses
   by ear: never choose a take or a level for him, and never judge one by a number, which only
   drops a broken one.
5. **When the owner picks one,** keep it: `bun scripts/song.ts keep <name> <take> <preset> [--vocal <L>]`,
   the preset named for the song, the mode and the take; `bun scripts/song.ts replay <preset>`
   makes it again.

## Putting lyrics into YuE2's form

- **Only when `make` ends with `FORMAT NEEDED <lyrics file> <formatted file>`,** and only then, the
  worker reads the lyrics file and writes the formatted file. This is the one step where anybody
  reads the lyrics. A cover or a remix needs it only where the page marks sections or leaves a
  repeat as an instruction, and the tool then drops the tags, since the sections YuE2 is given are
  set by where the recording sings each line.
- **Form alone:** open each section with `[verse]`, `[chorus]`, `[bridge]` or `[outro]` on a line of
  its own, one sung line to a line, a blank line between sections, and a repeat the source only
  marks written out in full. Sections follow the lyrics' own shape: a stanza sung again with the
  same words is a chorus, the others verses, a single differing stanza late in the song a bridge.
- **Never** change, drop, move, shorten or translate a line, and never add one the source does
  not have. The tool checks the file against the source before the song starts, names a fault by
  its line numbers, and asks again.
- **The words go nowhere else:** never outside the song's work folder — not into a report, a
  chat, a commit, a document, a subtitle or a tool's output. The report names the counts `make`
  prints (`FORMAT worker <sections> sections, <lines> lines`) and that the worker formed them.
- Then run `make` again with the same arguments.

## Changing a song after listening

- **Rerun from the first stage the change touches** with `bun scripts/song.ts start
  ~/.local/share/bmo-script/songs/<name>/request.json --from <stage>`; the earlier stages' files are
  reused, and from a take stage on every take already made is run again, or only the one `--take
  <k>` names; from hear, the takes it judged broken are heard again too:

  | Change | From |
  | --- | --- |
  | the lyrics or their form | `--from lyrics` |
  | how a line is sung: a vowel drawn out, a word split into beats | `--from lyrics` |
  | tempo | `--from key` |
  | style, seed, takes count or tries, YuE2's sampling or guidance | `--from generate` |
  | the transcriber, its language or the broken floors | `--from hear` |
  | a cover's key or the chat agent's range | `--from key` |
  | the fit onto the recording's beats | `--from fit` |
  | voice clip, voice pitch, seed-vc settings or how level its notes are held | `--from convert` |
  | the listening check's share | `--from listen` |
  | mix levels, voice steps or room | `--from mix` |
  | the video's spectrum, plasma, title, colours, bitrate or margin | `--from render` |

- **How a line is sung,** where the owner asks for a vowel drawn out or a word broken into beats, goes
  into the request's `sung` form (`docs/song.md`, 「The request」), never into `lyrics`: the subtitle
  keeps the line as written. Whether it was sung so is read from the take's hearing and alignment,
  and said in the report.
- Then `bun scripts/song.ts fetch <name>` copies the new videos into the work folder, and each is
  posted to the owner as a file.
- **Every adjustment the owner makes to the sound or the video is carried into the tool, not left in a
  song:** a value into `settings.json`, a method into its stage, and `docs/song.md` and this skill
  brought to match in the same change.

## Rules the owner set

- **The lyrics are the original's, whole and in their order.** Shortening them, or dropping a verse,
  is asked of the owner first (`bmo-asking`). A chorus the recording sings more often than the
  page writes it is written out again by the lyrics stage, from the page's own lines; nobody types
  it.
- **Only the lyrics given are sung.** The trim stage silences the chat agent's voice outside the stretches the
  lyric lines are heard sung in, so an intro, the melody of a line left out and the bars after the
  last line carry the backing alone; the song is not shortened to the lyrics.
- **Videos are rendered on the song node alone.** A render tried by hand, a new look's trial
  included, runs there too, never on example-host.
- **The whole song in one pass.** Never cut a song into spans, make them apart and join them.
- **The chat agent's voice directly:** YuE2's separated vocals are converted to the chat agent's voice by seed-vc; the
  unconverted YuE2 render is handed over only where the song was asked for without the chat agent's voice
  (`--voice natural`), and the original singer's voice is neither handed over nor converted. A
  style's words never take the chat agent's voice out.
- **One take for a cover,** the settings' `make.cover`: a broken one is replaced by the next seed up
  to `takes.tries`, only the whole one is posted, and more are made only when he asks.
- **Three seconds of margin at each end of every video** (`video.margin_s`): silence over the bare
  black background, no spectrum, no title and no subtitle.
- **No fixed seed.** Every run takes a new random seed and every take its own, and the request in
  the song's work folder records it; `--seed` is given only to make an earlier run's takes again.
- **One voice level, -12 LUFS,** the settings' `mix.vocal_lufs`, which the owner chose for every song:
  each take is mixed and sent at it alone, and no other level is made unless he asks.
- **No mix level is carried over from an earlier song.** A level the owner liked on one song is never
  the basis of another's.
- **The backing follows the voice, never the voice the backing:** a move of the voice by
  semitones moves the backing by as much; only an octave move leaves the backing where it is.
- **A cover's key is measured:** the key stage sets the recording's median pitch in the chat agent's
  natural range, moving the score YuE2 sings and the backing with it; a song already sung there
  moves nothing. Nobody types a key; `--transpose <n>` replaces the measured one only when the
  asker wants the song higher or lower, and the owner may be offered the semitone either side of the
  measured key as candidates, a grid over `convert.semitone_shift` of -1, 0 and 1.
- **The backing keeps time with the voice:** the original's own under a cover's voice once the fit
  has moved it onto the recording's beats, or YuE2's own under a remix's voice.
- **Lyrics under copyright** are kept as the form step above says, and such a song's videos carry
  no subtitle.
- **One model on the GPU at a time,** under the song node's one GPU lock (`flock` on `~/.gpu.lock`),
  which Laya and the voice server take too where they share that node; the hosts keep the models in
  pinned host memory and move each up for its turn.
- **Out of GPU memory,** only these: park the language model in host memory between stages
  (`yue2.offload_lm`), offload its layers, hold its KV cache in FP8. Never quantised weights, fewer
  steps, or a shorter song.
- **The video is a circular spectrum of the backing and the chat agent's voice with the song's title in its
  middle,** the title `make` was asked by, and no picture of the chat agent; it stays near 900 kb/s so it fits
  Telegram's file limit.

## Known traps

- **The GPU is shared:** each piece of GPU work waits for the node's GPU lock, saying so in the
  status as `waiting for the GPU lock`. Wait; never stop another service's or job's process to get
  the GPU.
- **One command on a node ends after five minutes;** the tool starts the run detached for that
  reason. Never run a stage by hand in the foreground of a node command.
- **Work by hand on the node does not take the GPU lock** unless it is started under it:
  `flock ~/.gpu.lock <command>`.
- **보카로 가사 위키's index is built on the first search that reaches it** and takes several
  minutes; a build cut short goes on from where it stopped on the next search.
- **가사집 is read with the chat agent's own reader from the system's deploy tree;** a change the chat agent has not yet put
  live is not what the search reads.
- **The aligner's weights are named `model.pt`,** the same name as another project's large file in
  the shared torch cache; the voice host always loads them from the bench's own `torchhome/`.
- **seed-vc's requirements** are installed with the environment's own pip; `uv` cannot read the
  pip option on their first line. `uv` itself needs `--system-certs` on the node.
