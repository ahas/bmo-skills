---
name: bmo-research-restaurants
description: "Read when someone asks where or what to eat, for a restaurant or a delivery recommendation, or for good places to eat nearby: whose rule filters the places (the asker's own restaurant rule, else the default rule), how many places and how they are picked, the delivery checks, and the menu, pictures and reviews each place carries."
bundle: worker
---

# Restaurant and delivery search

- This rulebook sets how a search filters places to eat when someone asks for a restaurant or a
  delivery recommendation. Its deciding section stands in the search runner's prompt and the run
  reads the rest once that section says it applies; it is the one place the default rule, how a
  person's own rule takes its place, and the delivery checks are defined.
- Nobody writes a rule into the query. The run is told who asked, and this rulebook decides.

## Whether this rulebook applies at all

This is the one place that decides it, and nothing else in the prompt does. Read it first; where
the answer is no, stop here: none of what follows applies and who asked decides nothing.

- **It applies** to a recommendation of places to eat or food to order: what should I eat, where should I eat,
  what should I order, good places nearby, a delivery recommendation, anything asking for a set of places or dishes to pick
  between.
- **It does not apply** to anything else, and most searches are anything else: a version, a price,
  a product, news, a law, research, a design, a place that is not somewhere to eat.
- **It does not apply** to a question about one named restaurant: its menu, its hours, whether it
  is open. That is answered as asked, with no rule either.
- **Who asked never brings it into play.** Anyone asking which GPU to buy is asking a product search, and
  a product search has no rule. The asker is read only once this rulebook already applies, and only
  to pick which rule is the one: the asker's own, or the default rule (「Whose rule」 below).
- **The `RULE:` line is this rulebook's alone.** A search it applies to writes `RULE:` with the name of
  the rule it applied in its head. Every other search writes no `RULE:` line; writing one tells the room
  its answer was filtered by a restaurant rule when it was not.

## Whose rule

- **A person may keep a restaurant rule of their own:** the checks every place recommended to them
  must pass, in their own words, kept among the private settings of the deployment that answers them
  and never in a skill or a prompt.
- **The asker's own rule where they keep one, else the default rule.** The asker is the person whose
  message the search was launched for, as the run is told it; an ask that names nobody is the
  owner's. Asking on another person's behalf does not change the rule; the asker's own does.
- **Where the asker's rule is read.** A research run is handed it with this rulebook: reading it whole
  answers the asker's own rule after it, under the line `THE ASKER'S OWN RULE:`. Any other session
  reads this skill with the research unit's skill reader, `research/skill`, naming the asker, and is
  answered the same way. Where no such rule comes after it, the asker keeps none and the default rule
  stands. A session that holds no skill reader applies the default rule and says so.
- **A person's own rule stands in place of the default rule whole,** under the name its text gives
  it, which the `RULE:` line names. Everything else here holds around it as it holds around the
  default rule: how many places and how they are picked, the delivery checks, what the result
  carries, the menu and the pictures.

## The default rule

Every place recommended has passed all four checks.

1. **The shop name.** Check from the name that it is a real food specialist.
   - Names ending in a generic pattern like "-food" are kitchens heating frozen goods, not
     restaurants. Exclude them.
   - This one check removes most bad candidates.
2. **Bad reviews first.** Judge a place by its negative reviews, not its positive ones: read what
   the bad reviews actually say (food poisoning, smell, portion, a sudden drop in quality) and drop
   the place where they point at the food.
3. **No frozen one-click menus.** Leave out menus easily made by heating a frozen product. The name
   check above is the first pass of this.
4. **A real storefront.** A place with no ground-floor shop is likely to fail. Check, down to the
   street view, that the restaurant is really at its registered address; one with no real shop
   there is excluded.

The default rule sets no price limit.

## How many, and how they are picked

- Every restaurant or delivery recommendation gives **at least three places and at most five**,
  counting only the places that passed.
- Pick them across food themes where the area allows (Korean food, Chinese food, Japanese food,
  fried chicken, snack food and the like) one or two per theme rather than five of one kind, and name
  each place's theme in its entry. A question naming one kind of food keeps to that kind.
- Where fewer than three pass, give those and say plainly that no more passed, rather than filling
  the count with a place that failed a check.

## Delivery

These checks apply on top of the rule applied whenever the recommendation is for delivery.

1. **Coupang Eats first.** Candidates are found on Coupang Eats: its listing for the address is the
   source, and each place's Coupang Eats store page is where its hours, delivery area and menu prices
   are read.
2. **Every shop that delivers there.** The candidates are every shop whose delivery area covers the
   address asked about, not only those in its own block: a better place across the district that
   delivers there is a candidate.
3. **The address is inside the delivery area.** Confirm for each place that it delivers to the
   address asked about; one that does not is dropped. Where the question names only an area, that
   area stands for the address and the result says so.
4. **Open at the time asked.** Judge each place at the search's "Asked at" time: its opening hours,
   its closing days and its break time. A place closed then (outside its hours, on a closing day,
   in its break, or past its last order) is dropped, with that reason.
   - A place whose hours could not be confirmed is not dropped for that alone; its entry says
     plainly that the hours were not confirmed.
5. **Baemin price beside it.** For each place, look up the same shop on Baemin and compare the
   recommended dish's price there, and the delivery fee, with its Coupang Eats price. Say which app is
   cheaper and by how much; where the shop is not on Baemin or the price could not be read, say
   so.
6. **Is the discount real.** Where a price is shown as discounted, check it against the same dish
   on the other app and the shop's own usual price. A discount no lower than, or about the same
   as, the undiscounted price elsewhere is a padded list price, and the entry says so.

## What the result carries

- A `RULE:` line naming the rule applied (the default rule, or the asker's own by its name), in the
  result's head.
- One entry per place that passed, each with:
  - `ADDRESS:` the registered address, `MAP:` its Naver Map link, and `ROADVIEW:` a street-view
    link where one was found.
  - Its menu as a `MENU:` block (「The menu」 below), which the document sets out as its menu.
  - Pictures, as `IMAGE:` lines written together right under its `MAP:` line, which the document
    shows as the place's one slide gallery (「The pictures」 below).
  - Reviews, as `REVIEW:` lines: three to five where that many exist, each with the reviewer's own
    words, the rating as that source writes it and the page it is on. The bad ones go in: check
    2 turns on them, and a reader seeing only praise cannot see the check was made. A review's
    photos join the place's pictures.
  - The price one person pays for a meal, where the rule applied checks a price.
  - Its food theme.
  - For delivery, also: its opening hours and that it is open at the time asked, or that the hours
    were not confirmed; that it delivers to the address; the Coupang Eats and Baemin prices of the
    recommended dish with the delivery fees; and whether a discount shown is real.
- The candidates that failed, one line each with the check that dropped it, so the reader sees the
  rule was applied rather than taking it on trust.

## The menu

- **Read the menu before the pictures.** It is what the pictures are matched to, and what a
  rule's price check reads.
- **The whole menu, as the place gives it:** every dish with its price, from the place's own page:
  its Naver place page, Dining Code, and for delivery its Coupang Eats store page.
- **No menu in text: look for a menu board first.** Where no page gives the menu as text, go
  through the place's pictures (its place page's photos, the pictures in its reviews and blog
  posts) for a menu board or a printed menu, look at each likely one with `eye/see`, and read
  the dishes and prices off it. The `MENU:` line then names the picture it was read from. Only
  where no menu board turns up either is the menu written as not confirmed.
- **A price read off a board is a price as of that picture.** Where the picture is dated, and a
  newer page gives a different price, the newer one stands.

## The pictures

- **Several, from more than one page, grouped by shot.** Gather candidates from the place page's
  photos, reviews and blog posts, and write them grouped as `bmo-research-kinds` (「Pictures」) sets,
  each line carrying the entry, the shot and the source: a group to each dish worth showing,
  named for the dish, one for the storefront, and one for the inside where one turns up; three to
  six pictures in a group where that many pass, at least three of the food and one of the
  storefront.
- **Look at every one with `eye/see` before it goes in.** A picture goes in only where it shows
  this place's food, its storefront or its inside. Leave out a picture of none of these, a menu
  board (read for the menu, not shown as food) a stock photo, another shop's or another branch's
  picture, and one too blurred to make out.
- **Match each food picture to the menu.** Its caption names the dish as the menu names it, with
  its price, such as bean sprout rice soup 9,000 won. A picture whose dish is not on the menu that was read is
  left out; where no menu could be read, the caption names what it shows and says the menu was not
  confirmed.
- **Where fewer pass than asked**, the entry says how many were found, rather than filling the
  gallery with pictures that failed.
