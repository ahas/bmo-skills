---
name: "ahas-video-code-rendered"
bundle: producer
version: "1.0.0"
description: "How to make a video as code, in which every frame is a pure function of time: a treatment and style bible first, timing data measured from the audio, one module per scene, an agent that looks at its own frames, a contact-sheet review loop, parallel scene authors under a written guide, and an offline render with motion blur. Read beside ahas-video-shotcraft for any video that should look authored rather than templated."
---

# Making a video as code

The method drawn from the open source of a widely seen music video made by a coding agent in conversation with its owner (`mexicat/pdoom-video`, MIT), its fork in a painted style (`JohnHeibel/PDoomVideo`) and an agent kit built on that (`Kreycin/pdoom-mv-kit`, MIT). This is the method only. Their song belongs to its authors and is not ours to use; their scenes, characters and designs are not to be copied. Take the discipline, make your own picture.

## What made it work

1. **Every frame is a pure function of time.** Any second can be drawn at once, a fix is re-rendered alone, frames render in parallel and out of order, and the preview equals the export. No state carried between frames, no `Math.random()`, no clock. Randomness is seeded; per-frame jitter is keyed to the frame's index.
2. **The agent looks at what it renders.** Animation cannot be judged from code. After every change it renders stills or a contact sheet and opens the images. A worker that cannot view images says so at once.
3. **A written treatment before any scene:** one paragraph of idea, the tone, what is forbidden, the palette, the type, the recurring motifs, then the plates one by one. Every later decision answers to it.
4. **Timing is measured, not guessed.** Beats, downbeats, sections, onsets and envelopes come from analysing the actual audio into a JSON file; word timings come from alignment against the actual narration. Scenes read that data, and find a line by its content, never by a hard-coded second.
5. **One scene, one module, one idea.** A scene owns its look; a timeline file owns the edit. Scene authors never touch the timeline or the engine.
6. **Revisions are named and specific,** each listing what changed and why, from watching: "this flash is too evident, carry the hit with a zoom punch", "this transition is a cut, make the line continue across it".
7. **The work is finished before the owner hears of it.** Anything that needs the owner's decision is raised at the end, in the report, never as a question in a pane or a stop in the middle of the run.

## The treatment (write it first, in the project folder)

- **The idea in one paragraph:** one conceit that holds the whole film together. A film of disconnected effects has none.
- **Tone, in words you can test a frame against:** what it is and what it is not.
- **The forbidden list:** the generic looks, by name. For anything about AI: no neon purple and cyan, no glowing brains, no code rain, no lens-flare soup, no particle nebulae, no stock imagery, no mascots unless the brief asks. If a frame could be anyone's, it fails.
- **Palette:** a few named values and one signal colour that alone may glow. One rare accent owned by one moment. Light and dark plates alternate for rhythm.
- **Type:** two or three families with a stated role each. Proper kerning and typographic punctuation. Swiss-grid discipline, asymmetry, negative space, hairlines and small annotations beside big display type. Type is part of a composed picture: never one giant word filling the frame, and never a run of big words one after another.
- **Motifs:** a few things that recur and change, so the film builds: one number that climbs, one mark that returns, one template that escalates at each return.
- **Humour or gravity by detail:** deadpan captions, tiny footnotes, a form stamped. Detail is what makes it look authored.

## Motion and editing

- Something always moves, and big changes land on the beat: cuts on downbeats, hits on kicks and snares, camera moves that ease into a downbeat.
- Strong easing, anticipation and overshoot. No floating screensaver motion. Inside a scene there are sub-cuts, reframings and snaps.
- **Every scene hands on to the next.** The action carries across the cut: a line that continues, a zoom through an object, a page that opens onto the next picture. A plain cut or crossfade is the exception and is chosen.
- One clear focal action per shot and a silhouette that reads at thumbnail size. Shots of 1.4 to 4 seconds, unless holding is the point. A gesture needs 0.4 s to read, a reaction 0.3 s, a reveal 0.8 s.
- Something happens in every shot, and something answers every action.
- Flashes are rare: carry most hits with a zoom punch or a colour fringe and keep the frame's blacks.
- **Light moves only with what is lit.** No full-screen colour flips and no flash on the beat. Measure the mean light of consecutive frames and keep the steps at the cuts small: a cut that jumps the picture's light hurts the eye more than a hard cut.

## Words on screen

- Each line is integrated graphically and differently from the last: typed, stamped, plotted, drawn along a curve. Words are part of the image, not subtitles laid over it.
- Synced per word to its measured start. Anticipation is allowed; highlighting never runs ahead of the voice.
- Inside the title-safe area, and readable.
- A foreign proper name is written in the Latin spelling the brief gives, while the voice says the language's own reading. Check every caption and every painted name.

## Sound and voice

- **The narration is bright, not gloomy.** Audition the voice for mood before the first take: measure a few sentences' median pitch, pitch range, level, voiced share and pace, and choose the voice, style and seed first. A gloomy read is not fixed after the fact.
- **A tone change on the same sentences keeps each take's length.** Choose the seed closest to the old take's length, then fit it with a pitch-preserving speed change inside about eight percent, so the measured timing and the picture are reused. A fitted take that breaks off between sentences is a failed take.
- Keep the narration's level under control and the music ducked under it. A sound change re-mixes the sound file, never the picture.

## The review loop

Keep a render tool with these modes, and use them in this order:

| Mode | Use |
|---|---|
| stills at given times | the first and last frame of a shot, and a moment |
| contact sheet of a range, about a frame per beat | does each shot read, does the whole move |
| strip of every frame of a key move or seam | popping, double exposure, a seam that does not carry |
| crop at full size | faces, type, hairlines |
| short clip with sound | do hits land against the audio |
| one frame every 4 s of the whole film | the shape of the film, its light and dark rhythm |

Check: is any line unreadable, anything cropped, frozen, popping or washed out by glow, does every seam carry, do hits land on beats. Fix, re-render only what changed, look again.

## Working with parallel authors

A lead writes the treatment, the engine and a guide for scene authors, then gives each subagent one scene. The guide states the API, the frame's coordinate rules, the data available, the check commands, and: "only edit your own scene file; report what you need from the engine". Keep a progress file with one line per scene (written, checked, fixed) so a new session can continue.

## The render

- Offline, in headless Chrome, with the app taking a time and returning a frame; raw frames streamed to one ffmpeg, in order, by several browsers rendering chunks.
- **Motion blur:** average sub-frames over a short shutter (a fifth of the frame time works), adaptively, more where the picture is still changing. Then a fast whip leaves a streak rather than stepped copies. This alone lifts the look far above a plain Remotion render. Grain and post come after the average.
- Tag the stream BT.709 and name the colour range. Very fine grain is expensive to encode; set the rate by looking at the file size.
- **A change renders in whole pieces.** Render again only the pieces that changed, copy every untouched stretch from the master without encoding, join without encoding and lay the film's sound under it. Check the frame count, each seam and the untouched packets' hashes. Never render the whole film again for a sound change or a few shots.
- Rendering on this platform takes a card ticket where the producer's rule says so; see `ahas-video-shotcraft`.

## The way of working, in order

1. Lead writes the treatment, then measures the audio.
2. Lead builds the engine and `docs/ENGINE.md`; the timeline stays the lead's.
3. Subagents take one scene each, in parallel, and check by opening their own stills and sheets.
4. Lead watches the whole film by sheets and clips, and writes a named revision: what changed, why.
5. At least two such revisions before the final render, with motion blur.
6. Fix what the check found, render only the changed pieces, and raise anything for the owner in the final report.

`mexicat/pdoom-video` is MIT: its engine and tools may start a project, with its licence kept in that project. Its song, lyrics, scenes and designs are not part of that.

## The pipeline

Quality comes from this pipeline as much as from the way of working, and Remotion does not give it:

- **Timing:** forced alignment of the known text against the voice, cross-checked with Whisper; stems, beats, downbeats, onsets and envelopes from the track (`analysis/`, run with uv).
- **Picture:** a WebGL app in TypeScript and three.js, run with bun and Vite, drawing into linear HDR targets, through shaders, GPU line batches and real kerned type. Screens are textures on objects, not DOM slides.
- **Post:** bloom on the signal colour alone, halation, grain, vignette, after the sub-frame average.
- **Render:** 60 fps, adaptive sub-frames per tile over a 0.2 shutter, x264 CRF 16, BT.709, several headless Chromes in chunks to one ffmpeg, on the GPU. Port the Chrome flags to the machine.

## The pipeline as a command

The stages should be a reusable command, not a memory of one film: one subcommand per stage, each runnable alone and resumed from what it produced; the stage prompts as files that name no film; every film's settings in its own project folder, so a new film touches no tool file. The producer's repository keeps one (`tools/produce.py` in the video producer's repository, stated in its `docs/production.md`): treatment, timing, voice audition, takes and fitting, one author per scene, contact-sheet review, revisions, the sentence check, the brightness measurement, the render, the partial re-render and join, and filing. A stage that needs an agent opens a role's session through the agency's own road rather than starting a program by hand; where no such road is open it hands the filled prompt to the running agent as a step.

## Rights

Music, footage and images need a licence you can name, recorded in `NOTES.md`. Do not download music from a video site; an original track made here (or one the owner licensed) is the road. Do not draw a character from a show, a meme's drawing or another video's design, even from memory: design your own. Name a product only where the brief does.

## Checking the result

Duration equals the audio's, the streams are as asked, and a one-frame-per-4-s sheet of the whole film has been looked at. Measure the mean light of consecutive frames and report the largest step and the counts over 8, 12, 20 and 40; a jump at a cut is a fault. Report where the file is, how long the render took, and whatever of the treatment you could not do.
