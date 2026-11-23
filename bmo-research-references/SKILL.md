---
name: bmo-research-references
description: "Read when gathering reference pictures for a drawing of a subject (a character, a person, a place, an object, a style or a work) into a reference dossier: the dossier's JSON fields, which sources and candidates count, and how each licence is recorded."
bundle: worker
---

# References for a drawing

A reference dossier says what a subject looks like, which of its features a drawing must keep, and which
pictures of it a drawing may be built from. The drawing service reads the dossier, fetches each candidate
itself, and checks every address, dimension and media type again before it uses one. The dossier is a
lead.

## The dossier

The answer is one JSON object, in a single fenced code block and nothing else:

```json
{
  "subject": {"name": "the subject's name", "aliases": ["the other names it answers to"], "kind": "character"},
  "description": "a short, factual account of its appearance",
  "features": ["a feature a drawing must keep"],
  "constraints": ["what must not be drawn, and why"],
  "candidates": [
    {
      "url": "https://.../picture.png",
      "pageUrl": "https://.../page",
      "title": "the title the source gives",
      "creator": "the author the source names",
      "publisher": "the site or collection that published it",
      "retrievedAt": 0,
      "licence": {"id": "CC-BY-4.0", "url": "https://...", "attribution": "the attribution line"},
      "mediaType": "image/png",
      "width": 1200,
      "height": 1200,
      "note": "what this picture contributes"
    }
  ],
  "sources": [{"url": "https://...", "what": "what it supports"}],
  "unresolved": ["what could not be found, and why"]
}
```

Write every key the object names and no other. `kind` is the kind the request names. `retrievedAt` is when
you read the page in epoch milliseconds; write 0 where you cannot tell. A field the source does not state is
not invented: a candidate without a direct address, a media type or both pixel dimensions is left out of
`candidates`, and the reason is said under `unresolved`. A licence the source does not state is not guessed
either: the candidate stays, its `id` and `attribution` empty and its `url` null.

The description is a short, factual account of the subject's appearance. The features are the ones a drawing
must keep: the silhouette, the markings, the proportions, the clothing, whatever makes the subject itself.
The constraints are what a drawing must not do and why, such as a feature the sources disagree on or a
detail that belongs to another version of the subject. Nothing in the description, the features or the
constraints may rest on a page that is not named under `sources`.

## The candidate

- **`url`** is the picture file's own address, never the page it stands on. It is an `http` or `https`
  address that answers a picture.
- **`pageUrl`** is the page the picture came from, where one stands; null otherwise.
- **`title`** and **`creator`** are what the source calls the picture and its author. **`publisher`** is the
  site or collection that published it, such as Wikimedia Commons, Flickr or Unsplash.
- **`mediaType`** is the picture's own type, such as `image/png` or `image/jpeg`, as the source states it.
- **`width`** and **`height`** are the pixel dimensions the source states.
- **`licence`** is the licence line the source itself gives: `id` its own identifier, `url` the licence's own
  page where the source names one, and `attribution` the attribution the licence requires.
- **`note`** says in a few words what this picture contributes, such as a clear view of the face.

A candidate smaller than 512 pixels on the short edge does not carry the subject and is not listed. A
candidate larger than 4096 pixels on the long edge is listed: the drawing service downscales it for its own
check.

## The licence

A picture's licence decides nothing here. No picture is left out of the candidate list, or kept from a
drawing, because of its licence: public domain, any Creative Commons terms, the Unsplash licence, all rights
reserved, an unknown licence and no licence line alike. This dossier controls nothing by a licence.

The licence is metadata. Write it as the source states it, its `id`, its `url` and its `attribution`, so the
dossier reports it beside the picture's creator, publisher and page. An official character's art is listed
like any other picture of the subject, with the licence line its source gives.

## The sources

Ask only the image engines the request names, from the web namespace's images category.

- **Openverse** reads its image API. A result's `url` is the picture, `foreign_landing_url` its page,
  `creator` its author, `provider` or `source` its publisher, `license` and `license_version` its licence,
  `license_url` the licence's page, and `width`, `height` and `filetype` its dimensions and type. Ask with
  the subject's names in the dossier's language and in English.
- **Wikimedia Commons** reads the MediaWiki query API with `prop=imageinfo` and
  `iiprop=url|size|mime|extmetadata`: `url` is the picture, `descriptionurl` its page, `width` and `height`
  its dimensions, `mime` its type, and the extended metadata carries the licence's short name and page, and
  the author. Its publisher is Wikimedia Commons.
- **Unsplash** reads its public search and the photo page it links. The licence is the Unsplash licence, the
  creator the photographer, and the dimensions and type are on the photo page and the picture itself.

A source outside these is not asked, and no picture is downloaded to guess a field: the source's own record
answers it. A search engine or a source that will not answer is not worked around with another image source.

## The written account

Read the subject's own material where one stands: an official page, a character reference, a maker's own
description. Where the sources disagree, say so under `constraints` or `unresolved` rather than choosing
silently. A description that is guessed from a name alone is not a description; leave it thin and say what
stays open.

## The language

The request names the dossier's language, the language of the person who asked for the draw. Write the
description, the features, the constraints, the notes, the sources' words and the unresolved lines in that
language. The keys stay English. Names, aliases, licence ids, addresses, media types and figures stand as
the source writes them, in whatever script they are.
