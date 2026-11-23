---
name: bmo-research-economy-brief
description: "Read when asked for a brief of a day's or a span's economic or market news, or for the day's economic figures gathered into one document: what is read, how each item and its figures are written, and the table a later system reads."
bundle: worker
---

# Economy brief

- This rulebook sets how a search writes a brief of the day's economic news and figures for the items it
  is handed: one document for a day, each item in a few lines with the figures it carries and their
  sources, and the figures written again as a table a later system reads. Its deciding section stands in
  the search runner's prompt and the run reads the rest once that section says it applies.
- **Why it exists:** the platform's research unit gathers economic news for a later portfolio and trading
  system. A brief is read twice: by a person, as news, and by a store, as facts; so every figure
  stands in both the prose and the table, dated and sourced.

## Whether this rulebook applies at all

- **It applies** where the request asks for a brief of economic or market news for a day or a span,
  handing the items to cover (addresses or titles), or asks for the day's economic figures gathered: an
  economy brief, a market brief, the day's economic news in one document.
- **It does not apply** to a question after one market value (a brief answer under 「Economy and markets」
  in `bmo-research-kinds`), to what to invest in (`bmo-research-investment`), or to the news of one company for
  its own sake.

## What is read

- **Each item handed is read on its own page,** and its figures are taken from the page or the primary
  source the page cites: a statistics office's release, a central bank's statement, an exchange's close,
  a company's filing. A figure a news page gives without a source is marked as the page's.
- **The figures of the day the request names** are read from their primary sources where the items
  do not carry them: the indices the request names, the policy rate, the exchange rate, the yields; none
  beyond what the request asks.

## The shape of the body

- **The verdict is the day in one short paragraph,** in the owner's language: what moved and why,
  then the three things that matter most, each with its figure.
- **The middle:**
  1. `오늘의 항목` — one short section per item handed: what happened in two to four lines, the figure or
     figures it carries with their as-of date, and what it may mean, said as a reading and marked as one.
     An item that turned out to be no news (a repeat, an opinion piece with no figure, an advertisement)
     is listed in one line as such.
  2. `수치표` — every figure the brief carries, one row each: `항목 | 값 | 단위 | 기준일 | 출처`, the
     source a page the run read, the as-of date the source's own. A figure appears here exactly as the
     prose gives it. The table is what the research unit keeps as facts, so its first column names the
     figure as a series name (`KOSPI 종가`, `원/달러 환율`, `한국은행 기준금리`), the same name every
     day.
  3. `읽은 자료 목록 · 못 읽은 것` as `bmo-research-routine` keeps it.
- **No forecast and no advice:** the brief says what the figures are and what the sources say they mean;
  what to do with money is `bmo-research-investment`'s and is never written here.
- **Plain prose, one figure per sentence,** as `bmo-docs-plain-writing` asks; a figure's unit and currency stand
  beside it.

## The narration

- A brief's narration (`bmo-research-kinds`, 「The narration」) is the day in a minute: the verdict and the three
  figures that matter, nothing of the table.
