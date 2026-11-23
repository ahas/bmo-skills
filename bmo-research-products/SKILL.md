---
name: bmo-research-products
description: "Read when someone asks what to buy, which of several products is better, where a thing is cheapest, or what it costs now: the order the sellers are searched in, the product line's news, the selling state, whether a discount is real, the landed price from overseas, and the shape of a product answer."
bundle: worker
---

# Product search order

- This document sets the order and the places searched when someone asks to find something to buy.
- The order binds the actual search: the search runner's prompt carries this rulebook's deciding
  section, and the run reads the whole of it once that section says it applies. This document
  holds the grounds for that order.
- What the chat agent says in the room is shaped by the chat agent's answer-shape rulebook and worded by `bmo-docs-plain-writing`. The
  shape of the search's document in the archive, and what every candidate carries in it, are set here.
- **Why the order is fixed:** an answer after only one or two places cannot say whether a cheaper
  place existed. With a fixed order, the answer never says a place is best without saying how much
  was checked.

## Whether this rulebook applies at all

This is the one place that decides whether a search is a product search, and nothing else in the
prompt does. Read it first; where the answer is no, none of what follows applies.

- **It applies** where what is asked for is a thing to buy: what should I buy, which one should I
  buy, where is the cheapest, which is better between X and Y, how much is this now, a price check,
  a price comparison, anything whose answer
  is a product to buy, a place to buy it, or what it costs to have it in hand.
- **Where it applies, the product line's news is searched beside the sellers**, never left out:
  what was just announced, a successor, a release coming, a price raised or cut. A seller's listing
  shows yesterday's lineup at yesterday's price, so a price search of listings alone misses the
  product announced last week. 「The product line's news」 below sets it.
- **It does not apply** to a question about a product that is not about buying one: how a thing
  works, what a specification says, whether a library supports something, why a machine broke.
- **The query's words do not decide it**; what a finished answer would be does. A question naming
  no product at all, "how do I get rid of the room smell", is a product search the moment the answer is a
  thing to buy.

## The shape of a product answer

- **What a product result is made of**, in this order:
  1. the head, as every result has it;
  2. `VERDICT:`: the pick and the candidates' criteria, set out below, before anything else;
  3. the comparison table, directly under the verdict with no section line of its own;
  4. `SECTION: New products and prices`: the product line's news, set out in 「The product line's news」;
  5. one entry per candidate, between `===ITEM===` lines, each opening with `NAME:`;
  6. `UNCONFIRMED:`: what could not be checked.
- **The verdict, the news section and what could not be confirmed are never left out**, whatever
  else is.
- A one-product question (what does this cost, is this still sold) is a brief result like any
  one-value question, with no entries, no table and no section. Its line's news is searched all the
  same, and a successor or an announced price change found is a line of its own in the verdict.

## What every candidate's entry carries

Beside its name, picture and reviews, every entry carries these four lines: what products are compared
on. A value the search could not check is written `unchecked` in its place, not left out, so
an unchecked value is told from one that does not exist.

- `RELEASED: <when it came out>`: the release date as the source writes it. Where the only date
  is the month a listing first carried the product, write that month and say what it is:
  `2025.11. Danawa registration`.
- `NEWER: <the successor or the newest model>`: whether the same maker has since put out a
  successor or newer model of the line, named, with when it came out and what changed: a new
  variant of the same model (another memory size, another edition) and a model only announced so
  far count too, with their announced price and date. `none` where the search looked and there is none, which is
  itself worth knowing. It is filled from the line's news search, never from the seller pages
  alone, which list only what is already in stock; `unchecked` is written only where that news search
  ran and still could not settle it.
- `STOCK: <the state> :: <when it was read> :: <the page it was read on>`: the selling state in
  the product page's own words.
- `DEAL: <the price now> :: <the usual price> :: <whether the discount is real>`: only where the
  page is actually running an offer, left out where it is not.

## The product line's news

- **A listing knows only what is already on sale.** A product announced last week, a successor
  shipping next month, a new memory size of a candidate or a price raise the maker announced
  yesterday is on no seller's page yet, and an answer built on listings alone recommends an old
  lineup at an old price. So the news is searched on every product search, beside the sellers and
  never instead of them, whatever the depth.
- **What is looked for**, in the last three months up to the moment asked, and the latest
  successor whenever it came:
  - a new product or a new variant of a candidate announced or released;
  - a successor or the next generation of a candidate's line;
  - a release coming: a launch date, a pre-order, a product shown but not yet on sale;
  - a price raised or cut, by the maker or across the market.
- **Every line the question names or a candidate belongs to is searched**, and the category as a
  whole where the question is one (128GB local AI mini PC, cordless vacuum cleaner): the line's name with
  new product, release, announcement, successor, price increase, price cut, and the same in English (announce, launch,
  successor, price), at least one query each in Korean and in English, kept to the period. The
  maker's own newsroom, blog or forum is opened where it has one, since a price change is
  announced there first.
- **The news rules of `bmo-research-kinds` hold for each item**: its date, the maker's announcement
  marked apart from a rumour or a leak, and as fact only what the maker says or two outlets report.
- **It is written in `SECTION: New products and prices`**, after the comparison table and before the
  entries, so the news stands apart from the prices: one numbered line per item, newest first,
  `<date> · <the product> · <what happened> · <the price announced>` with its footnote, and a
  rumour marked rumour. Where the period held nothing, the section says so in one line, naming the
  period and the lines searched, which is itself worth knowing.
- **What the news changes in the answer**:
  - a product announced that can be bought or pre-ordered now is a candidate like any other, with
    its own entry;
  - one not yet on sale stays in the section with its date, and the verdict says where waiting for
    it is worth more than buying now;
  - an announced price change is the price in effect from its date, and a listing still showing
    the old one is said to be behind it;
  - every candidate's `NEWER:` line is written from what this search found.

## Reading the selling state

- **A price is not stock.** A comparison site lists a price some seller published; whether that
  seller ships today is on the seller's own page. Recommending an out of stock product wastes the reader's
  whole errand: the one failure a price comparison cannot excuse.
- **The state is read on the page the recommendation sends the reader to.** Open that product page
  with `web_fetch` and read the words by its buy button.
- **Write the page's own words, never a verdict of your own.** on sale, out of stock, temporarily out of stock, sales discontinued,
  discontinued, 3 left in stock, expected in stock in November: each says something different, and "cannot be bought" in their
  place throws away what the reader would have decided on.
- **The time is part of the state.** Stock turns over by the hour, so every `STOCK:` line says
  when it was read.
- **Coupang cannot be opened from this machine**, so a Coupang product's state is whatever its snippet
  showed, marked `[n†]`, or `unchecked`. A Coupang listing with no state read is never the pick on its
  own; check the same product at a seller whose page does open.
- **What Danawa answers about it.** The listing gives the month it registered the product, how
  many shops carry it now, and the launch price where the category has one. Danawa stops pricing a
  product no shop carries, so a mall count fallen to nothing, or a product gone from a listing it
  used to be in, is the first sign of a discontinued product: to confirm on a seller's page, never a verdict on
  its own.
- **A product that cannot be bought is not a candidate.** Leave it out: unless it is the product
  asked about, or every candidate is in that state. Where it stands anyway, its `STOCK:` line says
  so and it is never the pick.

## Whether a discount is real

- A discount badge is a selling device before it is a fact: a markdown from a list price nobody has
  charged in a year is no discount, and an answer reporting one as a reason to buy now has been
  used.
- **Compare the offer against what the thing has actually been selling for**: Danawa's lowest
  across the shops today, the price-history chart where the comparison page offers one, and the
  launch price where the listing carries it.
- **Say which it is in the third field of the `DEAL:` line** (same as the usual price, lowest in the last three months,
  a card discount on top) and `authenticity unchecked` where it could not be told.
- A coupon, a card discount or a first-purchase price counts in what the thing costs only when the
  answer says what has to be true for it to apply.

## Order

- **The line's news is searched first** (「The product line's news」), so the sellers are then
  searched for the lineup and the prices as they stand now, a product just announced among them.
- **The price comparison starts at Danawa**, which sweeps domestic sellers' prices at
  once: the default place to find the product and its lowest price.
- **A site the task's catalogue lists a tool for is read with that tool**, Danawa, Coupang, Naver
  Price Comparison and the exchange rate among them where it lists one; a site with none is read
  with the web tools.
- The sellers are then checked in this order:
  1. **Coupang** (its pages refuse this machine, so its prices are snippet-only and a price Danawa
     lists for Coupang is the checked one)
  2. **Domestic alternatives**: Naver Shopping (Smart Store, with Naver Price Comparison's lowest
     price and its malls), then Gmarket, and a category's specialist leader where it has one.
  3. **Google Shopping**: for sellers the two above miss.
  4. **Overseas**: at least AliExpress and Amazon, compared by landed price, not list price, at
     today's exchange rate.
- Comparing the domestic price with the Ali and Amazon prices is always worth giving when the item
  is sold overseas.

Before writing the answer, check two things:

- whether a domestic product is an Ali item resold with a margin
- whether the item is sold only on the manufacturer's own store

## Why Coupang comes first

- It has the widest range, so for most items its price, stock and delivery date are the baseline.
- Every other place is measured as "cheaper or faster than Coupang", so taking the baseline first keeps the
  search short.
- An item missing from Coupang altogether is information itself: a special item or one with no
  official domestic distribution, so overseas and the manufacturer's store weigh more at once.

## Among domestic alternatives, Naver comes before Gmarket

- **After Coupang comes Naver Shopping (Smart Store), and Gmarket after that.** Naver doubles as price
  comparison, which suits the second place to look; Gmarket is a valid seller, but not a leader.
  - Auction is in Gmarket's group, so it is looked at separately only when its curation is noticeably
    different.
- **11st, SSG.com and Lotte ON are left out of the default order.** They are rarely the only cheap
  place and yield little for the cost of looking.
  - The exception is a sign that one has exclusive distribution of an item or is noticeably
    cheaper for it.
- Where a category has a specialist leader, it takes a place here:
  - fashion: Musinsa, Ably
  - groceries: Kurly, Daiso Mall
  - beauty: Olive Young
  - electronics, appliances, PC: Danawa (above) weighs most
- **A platform's standing is never a value to copy into an answer.** The room gets the item's
  price, not a platform's share or earnings.

## Where Danawa fits

- **Danawa is where the price comparison starts**, once the line's news is in hand.
  - It is the standard of price comparison for electronics, appliances and PCs.
  - Danawa cross-checked with Naver Price Comparison sweeps practically every domestic lowest price.
  - Enuri is the other price-comparison leader; use it as a third check when prices disagree.
- Naver Price Comparison is Naver Shopping's price-comparison feature under that name, not a discontinued
  service. Never skip it as if it were gone.
- Danawa is a comparison tool, not a seller: the recommendation names the seller offering the
  lowest price found there, not Danawa.

## Overseas is compared by landed price

- Overseas, look at least at AliExpress and Amazon.
- **The value compared is not the list price but what it costs until the item is in hand.** An
  answer comparing list prices only is wrong, and buying on it costs the room money.
- The landed price has four parts:
  - **Exchange rate.**
    - Only converted to won can it be compared with domestic prices.
    - Read the rate every time and use that moment's value, naming its time. Never reuse a
      remembered value or one from an earlier answer.
  - **Shipping.**
    - Free shipping is common on Ali but varies by item.
    - US Amazon ships only some items directly to Korea, and expensively, so consolidating through
      a forwarding address is 30–50% cheaper on average.
    - When pricing assumes consolidation, state that assumption in the answer.
    - Japanese Amazon ships directly to Korea, so no forwarder is needed.
  - **Delivery time.**
    - Domestic is a day or two; overseas usually one to two weeks or more.
    - With a small price gap and a two-week wait, the cheaper option is not the better choice.
      State the time with the price.
  - **Duty and tax thresholds.**
    - The list-clearance duty-free threshold is USD 200 for shipments from the US, and USD 150 for other
      countries including China and for goods subject to import declaration.
    - The goods value includes the sending country's taxes and local shipping. International
      freight and insurance to Korea do not count toward the duty-free test, but are added to the
      taxable value once the limit is exceeded.
    - For an item near the threshold, compare with tax added.
    - Several shipments arriving the same day are taxed together, so an answer buying several
      items points that out.

## Checking for Ali resale

- A domestic open-market product is sometimes an item sourced from Ali or the like and resold with
  a margin.
- Overseas purchasing agency is a lawful, common way to sell, but knowing it changes
  the recommendation: the same item bought directly on Ali is cheaper and slower, so the question
  becomes price against waiting.
- **So when the domestic lowest price is noticeably above the Ali price, check whether it is the
  same item, and when it looks resold, state that and put the two prices side by side.**
- Signs to look at:
  - a "overseas purchasing agency" or "purchasing agency" label in the product name or detail page
  - an estimated delivery date at the usual one to two weeks or more
  - whether the seller information shows registration as an overseas purchasing agency
- These are signs, not a sure test. **When unsure, don't assert it; go no further than "it appears
  to be reselling Ali goods".**

## Checking for manufacturer direct sales

- Small brands and new products are sometimes absent from open markets and sold only on the
  manufacturer's own store, which is then the answer.
- When the open-market search comes back empty or full of other items, look for the official store
  by the brand name.
- Answering "it cannot be bought in Korea" about an item sold only on its own store is a wrong answer.
- When the manufacturer's store is the answer, confirm it is the official store and give its
  address: the manufacturer's own sales channel, not a resale page carrying the same name.

## Pictures, tables and reviews

- **A comparison is a table**, standing under the verdict with no section line of its own: the
  candidates as rows; as columns the price, the landed price, the delivery time, the selling
  state, the release date and the deciding specs, each price and state with its as-of time and an
  unchecked cell reading unchecked.
- **List the reviews** that decide the choice, as `REVIEW:` lines: the reviewer's own words, the
  rating as that seller or site writes it, and the page the review is on. Never a summary of
  reviews: a summary is where a known defect goes missing. The bad ones first, and a review's
  photos as `IMAGE:` lines under it.
- **Attach pictures for every product mentioned.**
  - How a thing looks is half of whether to buy it, and look-alike Chinese product lines cannot
    even be told apart without pictures, so bare names tell the reader nothing.
  - Name a product, give its pictures too: a candidate gets them in plenty, grouped by shot (front,
    side, detail, worn shot or in-use shot), several to a shot, the maker's own photos and buyers' photos
    both where they exist, since a maker's photo shows the product at its best and a buyer's shows
    what arrives.
  - The image line's format, its fields and how many go in are set by `bmo-research-kinds`
    (「Pictures」).
- Actually look at reviews and product photos before writing the answer; a recommendation written
  after skimming prices and specs ends up recommending items with known defects.

## The shape of the final recommendation

- **The verdict sets the candidates on different criteria** after the pick, one line each, as
  `bmo-research-kinds` sets it for every comparison: each line names the candidate best on one criterion, and
  no two lines share a criterion. The criteria are those that actually divide these candidates:
  what they cost, what they do, what buyers actually chose, or whatever else splits them. First, second
  and third place on one criterion is not written.
- A product search answer still chooses one. An answer listing candidates without choosing has not
  answered the question: the pick stands first, under `VERDICT:` above those lines, and holds
  three things:
  - **Where to buy**: the seller's name: where the payment is actually made, not a comparison site.
  - **What it costs**: the price until it is in hand. Overseas, the won price with rate, shipping
    and tax added, stating any assumed consolidated shipping or coupon.
  - **Why it beats the other candidates**: one line: cheapest, much faster for a small difference,
    or the only one available in Korea.
- **The pick is a product that can actually be bought.** A recommendation whose `STOCK:` line says
  out of stock or discontinued is not a recommendation, whatever its price.
- Never write the unchecked as checked. Stock not seen is said to be not seen; a rate is said to
  be as of its day. A recommendation carrying false certainty is worse than none.
