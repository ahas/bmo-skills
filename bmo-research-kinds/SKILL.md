---
name: bmo-research-kinds
description: "Read before researching any question on the web for someone, or before writing a research result: what each kind of question (news, development and technology, AI models and hardware, prices, places, economy and markets and the rest) checks and attaches, how pictures and reviews are written, the shape of the body, the narration every result carries, and the options a search may carry."
bundle: worker
---

# Search kinds and options

- This rulebook sets what a search checks and attaches by kind of question, how it writes its
  pictures and reviews, the shape of its body, the narration every result carries, and the options
  any search may carry. It is appended to the search runner's prompt.
- A question of more than one kind takes the rules of each.
- Products are set by `bmo-research-products`, restaurants and delivery by `bmo-research-restaurants`,
  investment strategies and stock picks by `bmo-research-investment`, study material by
  `bmo-research-study-material` and a deep dive by `bmo-research-deep-dives`.
- **No kind here sets a `RULE:` line.** `bmo-research-restaurants` alone names a filtering rule and
  decides which searches carry one.

## By kind

1. **News and current affairs.**
   - Fix a period (the last 24 hours, the last week) and keep to it.
   - Write as fact only what at least two different outlets report.
   - Put the article date on every item.
2. **Development and technology.**
   - Look at the official documentation and GitHub releases first; a deep dive reads its maker as
     `bmo-research-deep-dives` sets.
   - For what is new (a release, a tool, IT news) also skim `news.hada.io` (GeekNews) to see what
     has come up. It is where to find out what happened, never the ground for saying it did:
     open the original the item points at and write as fact only what the original says.
   - Always give the version and its release date.
   - Blog posts only supplement.
3. **AI models and hardware.**
   - Copy benchmark figures exactly as the original gives them.
   - Mark official announcements apart from rumours and leaks.
   - **A model is its exact weights, never its family name.** One name covers many weights:
     sizes, dated releases, base and instruct, thinking and non-thinking, quantised and pruned
     derivatives, and a hosted API model beside the open weights it is said to rest on. The same
     name also scores differently from one dated release to the next, so figures read off the
     name alone fold different models into one.
   - **Pin every model a figure belongs to**, as far as its source lets:
     - the full repository or API identifier as its publisher writes it, owner and date suffix
       included (`Qwen/Qwen3.8-Flash-Next`, `DeepSeek-V4-Flash-0731`);
     - its release or snapshot date;
     - its size, total and active parameters;
     - its variant: the quantisation with its format and bit width, any pruning or fine-tune, and
       who made the derivative where not the original publisher;
     - whether the figure is the open weights' or a hosted API's, and for an API the date it was
       measured, since a hosted name can change the weights under it.
   - **Search by the exact identifier, not the family name**, once the question's model is pinned:
     a name the question gives loosely (`Qwen3.8 Flash low-bit version`) is first resolved to the
     identifiers it can mean, each named, and the answer either covers each or says which one it
     took and why.
   - **Every figure carries its exact weights** in its own row, cell or sentence, never once at the
     top of a section for all that follow.
   - **A comparison sets one exact version against another, never a family.** Figures of different
     weights (two dated releases, an original and its quantised derivative, an API and the open
     weights) are never one row or set against each other as one model's; where the question turns
     on them, each is a row of its own. A derivative's card repeating its original's table is the
     original's figure and is labelled so.
   - **A figure whose weights cannot be pinned is marked `version unchecked`** beside it, is kept out of any
     comparison the answer rests on, and is named in UNCONFIRMED with what was missing. A
     leaderboard, an index or an aggregator naming a model by its family is no exception: the
     model is pinned from that site's own model page or the figure is marked so, and this holds in
     the VERDICT as much as in a table.
   - **Whether a later dated release of the same name exists is checked**, as for a product, and the
     newest is named with its date.
4. **Law, regulation and tax.**
   - The original text first: the National Law Information Center and the like.
   - Give the effective date and whether it was amended recently.
5. **Health and medicine.**
   - Only official sources are grounds: the Ministry of Food and Drug Safety, papers.
   - Community reviews are marked apart.
6. **Real estate and second-hand prices.**
   - Actual transaction records are the basis: the Ministry of Land's actual transaction prices, Karrot and Bungaejangter prices.
7. **Travel and places.**
   - Opening hours, closing days and the booking link.
   - The weather for the day of the visit.
   - Location: the address, a map link and a street-view link.
8. **Economy and markets.**
   - A chart or indicator image for the price asked about (an IMAGE line), with the as-of time.
9. **Products and comparisons.**
   - A comparison table, product images, and the reviews as REVIEW lines.
   - A price-history chart as an IMAGE line where a price-comparison page offers one.
   - Every candidate carries its release date, whether a newer model replaced it, the selling
     state read off its own product page with the time read, and the discount where on one.
   - The news of every product line covered (new products, successors, releases coming, prices
     raised or cut) is searched beside the sellers and written in a section of its own.
   - The order of places, the answer's shape and each candidate's entry are `bmo-research-products`,
     which also alone decides whether a search is one of these.
10. **Local information and restaurants.**
    - Check the reviews for photos of the actual food and attach them.
    - The reviews themselves go in as REVIEW lines.
    - Attach the street view, the map and the address.
    - The filter, the count of places and the delivery checks are `bmo-research-restaurants`.
11. **Design, screens and anything looked at.**
    - The picture is the material, not decoration: a question about how something looks, how a
      screen is laid out, what a style is, is not answered by prose alone.
    - Attach the actual screens, works and pieces as IMAGE lines in the body, each captioned with
      what it shows and where it is from.
    - A description with no picture beside it is an unfinished answer to such a question.
12. **Study material.**
    - A request to be taught a subject, not a question about it, and a document kind of its own:
      `bmo-research-study-material` sets the whole of it and alone decides whether a search is one.
13. **Investment strategy.**
    - Where to put money, a stable strategy, a forecast or which stocks will rise: answered from a
      short-term and a long-term view across Korean and US stocks, bonds and gold, each strategy
      with its dated grounds, expected return, risk, named products and the case against.
    - A forecast request is answered with a concrete judgement, never turned away.
    - The whole of it is `bmo-research-investment`, which also alone decides whether a search is one
      of these; a question after one market value stays under 「Economy and markets」.

## Pictures

- **Attach a picture wherever the thing itself is worth seeing**: a dish, a product, a place, a
  chart, a screen, a design. The line is "IMAGE: <url> :: <caption>", the caption saying in a few
  words of the question's language what the picture shows and what it belongs to, so it reads on its
  own.
- **The line goes where what it shows is written about**: inside the entry it belongs to where
  the result is a set of things, never loose after it, and in the body where the answer itself
  is the thing looked at.
  - A question about a design, a screen, a layout, a style or a piece of work is not answered by
    prose alone: pictures of the actual designs or screens are the material, not decoration.
  - A market question gets a chart.
  - At most six IMAGE lines for the body.
- **The address serves the picture itself**: a direct link to an image file, one that itself
  serves the image (typically ending in .jpg, .jpeg, .png, .webp or .gif), never a page, a
  thumbnail wrapper or a search-result link. A page's pictures are not in the text it returns:
  open the review, place or article page with `web_fetch` and raw set to true, and take the
  addresses from its `og:image` and `img` src attributes.
- **Name a picture only once you have seen it.** Neither a page's text nor an address says what
  a picture shows, so look at every picture you mean to name with the eye/see tool, and name
  only one that shows what its caption says, its address written exactly as you gave it to the
  tool. A logo, a banner, a stock photo, a map, a price table or any picture that is mostly text, a
  picture of another branch or another business, another thing's picture and one too small or
  blurred to make out are left out.
- **An entry's pictures are written together**, one line after another:
  the document shows lines written together as one slide gallery, and the chat agent sends them as albums
  in the order written, one to each entry and shot.
- **Choosing among places, businesses or products takes pictures in plenty.** One picture per
  entry decides nothing: a reader judges a studio, a hall, a shop or a product by setting like against
  like.
  - Gather each entry's pictures grouped by composition or kind of shot (a studio's outdoor, indoor,
    dress full-body; a hall's hall view, aisle, banquet hall; a product's front, side, worn shot), two to four
    groups an entry where that many exist, and three to six similar pictures in each group.
  - Take them from more than one page: the business's own site and channels for its samples, and
    reviews and blog posts for what customers actually got.
  - Each such picture's line carries three more fields:
    "IMAGE: <url> :: <caption> :: <entry> :: <shot> :: <source>". The entry is the entry's
    `NAME:`, the shot is the group's name, written the same way for every picture of the group,
    and the source is `official` for the business's or maker's own sample or `customer` for a customer's
    photo.
  - Write an entry's pictures group by group, in the order the groups are best compared in.
  - Where fewer pass than that, the entry says how many were found, rather than filling a group
    with pictures that failed.
  - A place to eat gets the groups `bmo-research-restaurants` sets.
- **Never force one.** Where nothing visual turned up, the result carries no IMAGE line; and
  where something did, a price, a rate, a date or a version still takes none, since
  a question after one value or one fact is answered by that value, and a picture nobody asked for is noise.
- **The deployment keeps its own copy of every picture you name**, stored beside the search's document after
  the run, so an address that later stops working costs the record nothing: give the address you
  actually saw the picture at.

## Reviews

- **A review goes in as the review**, never as a sentence summarising reviews. One line per
  review: "REVIEW: <the rating> :: <the review's own words> :: <the page it is on>".
- **The rating is the source's own** (4.5, ★★★★☆, 4-star rating) as the source writes it, and `none`
  where it gives none.
- **The words are the reviewer's own**, quoted rather than rewritten; from a long review, quote
  only the part that decides something.
- **Put the bad reviews in first.** Praise decides nothing: a reader's pick is decided by what
  went wrong for somebody, and a summary is exactly where a known defect goes missing.
- **A review's own pictures** are looked at as any picture. Of them, you
  write those worth showing among the entry's other pictures, in the group of their shot with
  `customer` as their source where the entry's pictures carry one, and
  otherwise the caption saying they came from a review, so the entry's pictures stay one gallery.
- **Three to five reviews per thing** where that many exist, and plainly fewer where fewer.

## Common options

The request's options may give any of these; one not given is decided from the question.

1. **Period**: only material from the given span (the last few days, a date range).
2. **Region and language**: domestic material first, or overseas material too.
3. **Excluded sources**: advertising blogs or named sites left out.
4. **Result shape**: one of four:
   - **brief**: the head, the verdict with the value, its grounds and what could not be confirmed,
     nothing else.
   - **table**: the full document with the comparison as a table.
   - **full**: the whole document: the verdict, the body in the shape the question asks for, and
     what could not be confirmed.
   - **learning**: study material, in the shape `bmo-research-study-material` sets.

## The shape of the body

- **Every document has the contents that fit it**; none borrows another's outline. This section
  is the shape of a search's answer; study material is its own kind, shaped wholly by
  `bmo-research-study-material`, and nothing here binds it.
- **Two places are fixed and the middle is not.** Every result with a body opens with the
  verdict and closes with what could not be confirmed. What stands between is the question's,
  and no part is required: no introduction and no conclusion, since what was asked is the head's `TOPIC:` and
  the answer is the verdict.

### The verdict (core answer)

- **The answer comes first.** What was asked for is read before any document, so it stands alone
  right under the head, under `VERDICT:`, and the sections follow it. Nothing restates it in a list of
  its own.
- **It is the answer itself:** what to buy, where and at what cost, which one, the pick for each
  case the question splits into, the conclusion, the value.
  - One line per pick, numbered, or one small table where several picks share the same few
    columns.
  - No grounds beyond the one line a kind's rulebook asks of a pick: why a pick wins belongs
    to the sections.
- **It carries what the answer cannot be read without**, in its own lines: the one thing to know
  first where the answer is unreadable without its setting, the conditions that narrow it where it
  holds only under them (when, for whom, at what cost).
- **A comparison's verdict sets the candidates on different criteria** after its pick, one line
  each: the best on one criterion, then the best on another (what it costs, what it does, what the
  market actually chose, whatever really divides that search). The search names its own criteria,
  and no two lines share a criterion. First, second and third place on one criterion is one line
  split three ways, and is not written.
- **A kind's own rulebook fills it** where it sets what the answer is: the product pick, the
  places to go, the trades to make.
- **It is what the room is answered from first:** the chat agent opens its answer with it, under the
  document's link where it gives one.
- **Study material has none;** it answers nothing. A brief result's verdict is the value with its
  as-of time.

### The middle

- **The document's nature decides the order**, never a template:
  - **Background first** where the facts cannot be read without it: a dispute, a change, a
    claim whose meaning depends on how it came about.
  - **Importance first** where the facts are themselves the answer: a comparison, a catalogue,
    the kinds of a thing, options to pick between: what matters most stands first.
  - Neither is a set of parts to fill; a section with nothing of its own is not written.
- **A section is named for what it holds.** Each opens with `SECTION: <its name>`, the name saying
  what stands in it: the dispute so far, what the maker says, how the options divide. Never a
  generic word like body, background or conclusion standing for any content.
- **A single table or a single set needs no section line**; it stands under the verdict as is.
- **Figures that are compared or that move are drawn as a chart** as well as tabled: values set
  against each other across the candidates, a price, rate or count over time, the shares of a
  whole.
  - A fenced `mermaid` block: `xychart-beta` for bars or a line, `pie` for shares, eight series
    at most, no colour named, since the console gives each series its hue.
  - The chart shows the shape; the table keeps the exact values and their dates.
  - A single value, or figures nobody compares, gets none.
- **A product search** keeps this frame, its table and entries as the middle. `bmo-research-products`
  sets the whole of it and decides which searches are these.

## A brief result

- **The result's size follows the question.** A question after one value or one fact (a
  price, a rate, a date, a version, a yes or no) is brief unless the launch said otherwise: the
  value, its as-of time, the grounds it rests on, and what could not be confirmed. Nothing the
  question did not ask for is added: no trend, background, comparison or history unless asked.
- **Every result leaves its document, a brief one too.** The room hears the answer in a line or two
  and is handed the document's link; the grounds and the sources stand in the document, where anyone
  can later ask which page a value came from, and never in the chat.
- **Its shape:** the head, `VERDICT:` with the value and its as-of time, `SECTION: Grounds` with a
  few lines on what the value rests on and how it was checked against a second source, the
  `SOURCE` lines that close it, and `UNCONFIRMED:`.
- **It is the same cut as the brief shape.** Options asking for the brief shape ask for this result
  whatever the question; a question with more in it than one value takes the body the rest of this
  rulebook sets.

## The narration

- **Every result carries one:** a script for the ear, written apart from the document, which the
  chat agent's voice speaks and the document keeps at its top, so its reader can listen while
  looking at the page. A brief result's is as short as the result, a deep dive's as long as the
  rule below allows.
- **It is not the document read out.** It tells a listener what the thing is, the verdict and the
  two or three things deciding it, what is still uncertain and what to try next, in sentences of
  its own.
- **It is written to be heard,** in the question's language: plain sentences, no persona, no list,
  table, footnote mark, address, code or structure word; only the few figures that matter, rounded
  where the exact value adds nothing; a long identifier said by its short name.
- **It is written words, kept to be read as well as heard.** The script is the narration's transcript:
  names, acronyms, model and product names and the figures it keeps stand exactly as they are written,
  never spelled as they sound. The voice unit derives the spoken words from it, the same words as a
  Korean reader says them aloud, every name and acronym in its Korean reading and every figure in the
  words a reader says, with no word added, dropped or moved, and checks them before it speaks.
- **At most three to four minutes aloud,** a deep dive's some 1,500 to 2,500 characters of Korean or
  400 to 600 words of English, a brief result's a few sentences, and never more: the voice speaks at
  most 4,000 characters.

## The tools before the search

- **Look at the site tools before the first query.** The task lists them: one per site the same
  kind of value is read from again and again, a rate, a spot price, a listing. Where one covers
  what is asked, it is the answer and no search is run for that value.
- **A broken tool is repaired, not abandoned.** Sites change shape under their readings. Get that
  one value with the web tools for this answer, write nothing of what the tool answered into the
  result (the task's rule on a tool's answers about itself), and never take a broken tool as a
  reason to go back to searching for that value every time.
- **A search asks for no new tool.** A site no tool covers is read with the web tools, and what
  could not be read is said in `UNCONFIRMED:`; a tool is made only where the owner asks for one.
