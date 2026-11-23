---
name: bmo-research-investment
description: "Read when someone asks where to put money: an investment strategy, what to hold now, whether to buy bonds, gold or dollars, which stocks will rise, or a market forecast: the strategy frames, the short and long views, the market figures, company analysis, whether news is already priced in, entry and exit, and the answer's shape."
bundle: worker
---

# Investment strategy search

- This rulebook sets how a search answers someone asking where to put money: a stable investment
  strategy, what to hold now, or which stocks are likely to rise. Its deciding section stands in the
  search runner's prompt and the run reads the rest once that section says it applies.
- **Why it exists:** most AI answers to an investment question retreat into "predictions cannot be made"
  and a list of what already moved, which answers nothing. A search here gives a concrete judgement
  built from the figures of the day it ran, with what would prove it wrong beside it.

## Whether this rulebook applies at all

This is the one place that decides whether a search is an investment search, and nothing else in
the prompt does. Read it first; where the answer is no, none of what follows applies.

- **It applies** where the answer is what to invest in, how, or what is likely to rise:
  - a strategy: a stable investment strategy, what to put money in now, whether to buy bonds, whether
    now is the time to get into gold, what beats a savings deposit, whether to buy dollars;
  - a forecast or a pick: find a stock about to jump, recommend one that will rise, make a forecast,
    will this stock rise, how far will the KOSPI go.
- **It does not apply** to a question after one market value: an index close, a rate, a gold
  price, a stock's price today. That is a brief answer under 「Economy and markets」 in
  `bmo-research-kinds`.
- **It does not apply** to news about a company or a market that does not ask what to do with it,
  or to how a financial product or a tax works in itself.

## Answering instead of refusing

- **A forecast request is answered, never turned away.** No line saying a forecast cannot be made,
  and no answer made only of what already rose today: a stock that has already jumped is a pick only
  where its reason still holds, with how far it has already moved said beside it.
- **The judgement is the answer.** From the figures of the day the search ran, say what to hold,
  what is likely to rise and why. What stands in for certainty is the dated grounds, the risk and
  the case against, not a hedge.
- **No return is promised as a certainty.** An expected return is a range tied to what it rests on
  (a bond's yield to maturity, a dividend yield, an earnings estimate, a historical average over a
  named period), never a bare figure.
- **No disclaimer paragraph.** The risk column and the case against are the caution; a closing
  "investment is at your own risk" block adds nothing to them.

## The order the search works in

A pick comes out of a strategy's rules and a company's figures, never out of the day's headlines.
The steps run in this order, and a later one never stands in for an earlier one.

1. **The market figures** (「The market figures the answer stands on」).
2. **The strategy frames,** researched from their sources (「The strategy frames」).
3. **The candidates,** screened by the rules the frames set: not taken from a news list.
4. **The company analysis** of each candidate (「Company analysis」).
5. **The news check** of each candidate (「Whether the news is already in the price」).
6. **The entry and the exit** of each pick that is left (「Entry and exit」).
7. **The answer,** conclusion first (「The shape of the answer」).

## The strategy frames

- **Every answer stands on named trading strategies,** each with rules a reader could follow: what
  qualifies an asset, when to buy, when to rebalance or add, when to sell. A strategy the asker
  names (Shannon's demon, Buffett-style value investing, or any other) is always one of them; otherwise the search
  finds the strategies that fit the question and the horizon.
- **A strategy is researched every time, never written from memory.** What a strategy is, what it
  rests on and what its rules are come from sources the search read in this run and cites; nothing
  in this rulebook and nothing the run already believes stands in for that reading.
- **For each strategy the search looks for four kinds of source,** and reads what it finds:
  - **the mathematics**: the theory or formula the strategy rests on, from a paper, a textbook
    page or a reference article that states it;
  - **the research**: academic papers and studies that tested it, with what they found and over
    which market and period;
  - **the practitioners**: interviews, shareholder letters, books or talks by the investors
    known for it, quoted where they state a rule;
  - **the outlook**: current house views, analyst forecasts and market speculation on whether
    today's conditions suit it.
- **A kind of source that could not be found is said,** in `UNCONFIRMED:`, never filled in.
- **Each rule is written with the source it came from.** The buying, selling and rebalancing rules
  and any threshold (a valuation level, a rebalancing band, a position size) carry the source
  mark of the page that states them; a rule no source states is the search's own judgement and is
  marked so.
- **The frames are their own section,** 「trading strategy frames」: one row per strategy with its rules,
  the theory it rests on, what the research found, the practitioner's words, and whether today's
  figures suit it, each cell with its source mark.

## The two views

Every investment answer is given from both views, each its own set of strategies, unless the
question itself names one horizon.

- **Short term (short-term trading): days to a few months.** What moves prices inside that span: the rate
  decisions ahead (FOMC, the Bank of Korea's Monetary Policy Board) and their dates, the event calendar (earnings, CPI and employment
  figures, policy announcements), supply and demand (foreign and institutional net buying and selling), the
  exchange rate's direction and momentum.
- **Long term (long-term investing): years.** What decides returns over years: valuation (PER, PBR, dividend
  yield against its own history), earnings growth, where the rate cycle stands, allocation across
  asset classes, and what holding costs: fees, currency hedging, and the tax accounts (ISA,
  pension savings, IRP) where they change which product to hold.
- **Stability first.** In each view the strategy with the smallest likely loss for its return
  stands first; one that is plainly aggressive is marked so in its risk, never left out where the
  asker wants a pick.

## The asset classes

- **Always looked at:** Korean stocks (the domestic market), US stocks (the US market), bonds (Korean and US government,
  short and long duration) and gold.
- **Added where the day's figures make them matter,** at the run's judgement: the dollar and other
  currencies, REITs, dividend stocks, cash-like parking (MMF, CMA, short-term bond ETFs),
  commodities. Crypto only where asked.
- **A question naming one class keeps to it,** still from both views.

## The market figures the answer stands on

- **Read before any strategy is written,** each with its date and page:
  - the policy rates, the Bank of Korea base rate and the Fed funds target, and the next decision date of each;
  - government bond yields: Korean 3-year and 10-year, US 2-year and 10-year;
  - the won-dollar rate;
  - the index levels of KOSPI, KOSDAQ, S&P 500 and NASDAQ, and the forward PER of KOSPI and the
    S&P 500 where a page gives it;
  - the gold price.
- **The site tools read what they cover:** where the task's catalogue lists a tool for an index
  close, a government bond yield, the exchange rate, the gold price, or a stock's or an ETF's price
  and valuation, that tool reads the value and no search is run for it.
- **A stock's or an ETF's price is read from its own item page or a tool that covers it**: a pick's
  price at the time read and what it has already moved, a product's price and whether it still
  trades. A value marked intraday is written as the in-session price with its time, never as the
  day's close.
- **Every figure carries its date** as its page gives it. A figure found only in a snippet is
  marked as the task says; one that could not be read is unchecked, never remembered.

## Company analysis

Every stock that is a candidate is analysed before it can be a pick; an ETF or a fund is judged by
its product page instead.

- **The figures are read from the company's item page or a tool that covers it,** never off a blog
  or a snippet: PER, PBR and ROE, with EV/EBITDA, debt ratio and FCF, the consensus, and each
  investor group's net buying.
- **Each ratio is read against two yardsticks:** the industry's (the industry PER the page gives,
  or the peers' figures where it gives none) and the company's own range over its reported years.
  A PER is high or low only against one of them, and the answer says which.
- **The ratios are read together, by the frame's own criteria:** what the researched sources count
  as cheap, as quality or as a warning sign decides the reading, with their source mark, not a
  threshold the run carries in its head. An earnings estimate that makes the forward PER low is
  weighed against how far the reported years swing.
- **EV/EBITDA, debt ratio and FCF are read where they decide the case** (a company carrying debt, one
  whose profit is not turning into cash, one valued on its cash flow) and unchecked where they could
  not be read.
- **The figures stand in a table of their own,** 「company analysis」: one row per company, with PER,
  industry PER, the company's own PER range, PBR, ROE, and EV/EBITDA, debt ratio and FCF where read, the
  period each figure stands for, and a verdict of a few words (undervalued, fairly valued, overvalued) with what
  it rests on.

## Whether the news is already in the price

- **Good news is not a reason on its own.** By the time an article runs, buyers who read it first
  have often already bought, so a headline is a reason to buy only where the price has not yet
  moved on it.
- **Every candidate whose case rests on news or a published revision is checked:**
  - the article's own date, from the article;
  - the daily prices across the sessions before that date and since it: how far the price moved
    before, on and after it;
  - the volume on those days against the sessions before them, and, from each investor group's
    net buying, who was buying: individuals buying heavily while foreigners and institutions sell
    is the late crowd arriving.
- **The verdict is one of three,** written in its own column priced in with the figures behind it:
  - **not yet priced in**: the price and the volume have not yet answered the news;
  - **partly priced in**: they have moved, but less than the news would justify by the frame's own
    reading; the pick stands with its score reduced and the move already made said;
  - **fully priced in**: most of the move came before or on the article, on a jump in volume; the stock
    is dropped, or kept only where a later catalyst with its date still lies ahead, and marked so.
- **A candidate with no news to test** says so in the column rather than leaving it empty.

## Entry and exit

- **Every pick carries its trading plan,** in prices, not adjectives:
  - **entry**: the price or the range to buy in, and the condition that makes it a buy;
  - **stop-loss**: the price at which the pick is sold at a loss, with the loss in percent from entry;
  - **take-profit**: the price or prices at which profit is taken, with the gain in percent from entry;
  - **period**: how long the plan holds before it is looked at again.
- **Each level has its grounds:** the frame's own rule where it sets one, with its source mark, or
  a level the stock's own figures give (a recent low, the 52-week range, a moving average read
  from its prices, the consensus target) named beside it.

## What every strategy carries

A value the search could not check is written `unchecked` in its place, not left out.

- **grounds**: the actual figures it rests on, from the market figures above or its own (a company's
  earnings and revisions, a fund's flows), each with its date and a source mark.
- **expected return**: a range and what it is drawn from, as 「Answering instead of refusing」 sets.
- **risk**: what would lose money and how much it has lost before: the fall it took in a named
  stress period, or the condition that breaks it (rates rising, the won strengthening, an earnings
  miss).
- **product examples**: the named products to hold it through: ETF names with their code or ticker
  (KODEX 200 069500, SPY, TLT, and the like), a stock's name and code, a bond by its issuer and
  maturity. Each is checked to exist and still trade on its own item page (Naver Finance's page
  for its code, or the issuer's fund page) with its total fee where the page gives it; a
  blog or a snippet naming a code is not that check.
- **case against** (the strongest case against it, taken from a source that actually makes it)
  a house view, an analyst, a column: and cited, never invented to fill the column.

## Stock picks

Where the question asks which stocks will rise, or the answer is a set of stocks, the picks are
their own section on top of the views.

- **Three to five names**, each tagged short term or long term and named for the strategy it comes out of,
  picked for a reason that still lies ahead: an earnings revision upward, a catalyst with its date
  (an earnings release, a product launch, an index inclusion, a policy decision), supply and demand turning, or a
  valuation below its peers with a reason it should close.
- **Each pick carries** its price at the time read, what has already moved (the change over the
  last week or month), the reason, the catalyst's date, its company analysis verdict, its priced-in verdict,
  its entry, stop-loss, take-profit and period, the risk, and the condition that would say the pick was wrong: a
  price level or an event outcome.
- **Where the day's screen is only yesterday's jumpers,** say so and pick from the reasons ahead,
  not from the list of what rose.

## The shape of the answer

- **The recommendation stands above everything.** It is the result's `VERDICT:` part, the answer
  the task puts first, and it is what was asked for:
  - **short-term trading**: what to trade short term: the name or product with its code, buy or sell, entry, stop-loss,
    take-profit;
  - **long-term investing**: what to hold long term, the same way;
  - **by strategy**: for each strategy frame, what it trades now.
  It is one table, a row to a trade, with no grounds in it: everything after it is why.
- **Under the table, still in the verdict,** one short paragraph: the pick's strongest ground and
  the one figure whose change would turn the judgement.
- **Then the grounds, one table to a section,** each under a `SECTION:` line named for what it
  holds:
  - the market figures, as rows of figure, value, date;
  - the strategy frames, as 「The strategy frames」 sets;
  - the short-term strategies and the long-term strategies, a table each, one row per strategy with
    the columns asset class, strategy, product examples, grounds, expected return, risk, case against;
  - the company analysis, as 「Company analysis」 sets;
  - the stock picks, where there are any, one row per stock, with the priced-in column and the trading
    plan.
- **A rise and a fall carry their arrow:** every change, return and expected direction is written
  with `▲` in front of a rise and `▼` in front of a fall (▲ 3.2%, ▼ 1,200 won (-4.9%), outlook ▲) so
  the document shows them in red and blue. A figure that did not move carries no arrow.
- **A cell stays short** (a figure with its date, a few words) and its source mark stands in it.
  A strategy needing more than a table cell can hold gets one short line under its table.
- **A chart stands beside the table where the figures are a shape rather than a lookup,** drawn as
  the task draws a chart, from the figures the search read:
  - a price trend (a pick's closes across the sessions around its news date, a strategy's
    asset over its period) as a line;
  - a valuation comparison, the candidates' PER or PBR against their industry and their own
    range, as bars;
  - a strategy's weights (a rebalancing split, an allocation across asset classes) as a pie.
  A table that is read one cell at a time needs no chart, and a chart never stands without the
  table its figures come from.
- **An index or price chart image** goes in as an `IMAGE:` line only where it shows what a pick
  turns on.
- **`UNCONFIRMED:` closes it,** as every result does: every figure not read, every product not
  checked, every kind of strategy source not found, every case against not found.
