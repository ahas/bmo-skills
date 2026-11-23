---
name: bmo-research-routine
description: "Read when researching, measuring or reporting on a thing somebody made (a model, an engine, a library, a tool, a dataset, a service, a recipe for running one) or a claim about one, a job re-measuring a maker's figures on the lab's own machines among them: the three shapes a report takes, its opening, its skeleton, the line saying when and from what it was researched, the mark every figure carries, how a measurement is run and kept, and how a report ends."
bundle: worker
---

# The research routine

- This rulebook sets how a research about a thing somebody made is carried out, measured and
  reported: the three shapes a report takes, the opening every one of them starts with, the fixed
  skeleton of each, the line that says when and from what it was researched, the mark every figure
  carries, how a measurement is run, written and kept, and how a report ends. Its deciding section stands
  in the search runner's prompt and the run reads the rest once that section says it applies; a job that
  measures something the lab runs is bound to it by its researching-and-benching skill.
- It stands on `bmo-research-deep-dives`, which says how the maker is read before anyone writing about it
  and how its figures become the baseline; this rulebook says what the document is then made of.
  Where it applies it sets the middle's sections, as `bmo-research-study-material` sets its own kind's, and the
  order `bmo-research-kinds` leaves to the question is this rulebook's.

## Whether this rulebook applies at all

- **It applies** to every deep dive, a run at the thorough or exhaustive depth, whose subject is a
  thing somebody made — a model, an engine, a library, a tool, a dataset, a service, a recipe for
  running one — or a claim or an event about such a thing: what it is, what its maker says it does,
  whether it holds, what it is worth to us. It also binds a job that re-measures a maker's figures on
  the lab's own machines.
- **It does not apply** to a run at the basic depth, to a product to buy, a place to go, an
  investment or study material, which their own rulebooks shape, nor to a question that only touches
  a made thing on its way to something else.

## The three shapes

- **One shape per report, chosen before the first query** from what the question asks:
  - **the investigation** (조사 리포트) where the subject's identity or a claim about it has to be
    settled: what was announced, what is really there, what others measured, what it is worth to us;
  - **the manual** (사용설명서) where the subject is a made thing we may run or use: how it was made,
    how its maker runs it, what the maker claims under which conditions, whether it fits our
    machines, and how its figures would be re-measured;
  - **the verification** (검수 보고서) where the maker's figures are re-measured on our hardware. A
    search run measures nothing, so this shape is a job's; a manual's reproduction plan is what the
    job starts from.
- **The shape's name closes the title:** `<name> 사용설명서`, `<name> 검수 보고서`, and an
  investigation's title names the thing and the shape, `<name> 조사`. In an English question the
  same in English: `<name> manual`, `<name> verification report`, `<name> investigation`.
- **A title is a noun phrase, never a sentence:** the thing's name and the shape, no predicate, no
  verdict and no clause after a comma or a dash; the verdict stands in the body and the narration. It
  holds at most 15 Korean syllables, spaces not counted, or 30 characters in English, as the search
  task sets for every result; a long name is said by its short name.

## The opening

- **Every report opens with two things, before any figure or configuration:** where the research came
  from, under its title, and what its subject is and what it is for, opening its body. They stand in every
  shape, the manual, the investigation and the verification alike, and are unnumbered: the manual's
  numbers count from its verdict.
- **Where this research came from** (`어디서 나온 조사인가`, English: `Where this research came from`) is
  one line: the source that found the subject, by name; the owner's ask, his words quoted; or, for a
  follow-up, the earlier document it follows from — `문서 N 「제목」의 후속 조사` with that document's
  share link — and why this one was asked, quoted. The question hands the run this line, and a deep
  dive's page prints it under the title from the item itself, so the run writes no section of it. A
  document no daemon draws, a verification job's among them, writes it itself as the first line under its
  title; a verification follows from the manual whose reproduction plan it starts from.
- **What it is and what it is for** (`무엇이고 어디에 쓰나`, English: `What it is and what it is for`) is
  the body's first section, written as the first `SECTION:` after the verdict; the page sets it above the
  verdict, so it is the first thing read. Two to five plain sentences, for a reader who has never heard
  the name:
  - the kind of thing it is — a model, an engine, a library, a tool, a dataset, a paper, a service, or a
    claim or an event about one;
  - the task it does, said in words a newcomer follows, a term the reader may not know explained rather
    than only named;
  - who would use it and for what;
  - what it replaces or competes with, each named the same way.
- **No figure stands in it:** no size, no version, no benchmark, no setting, no command and no price; those
  are the sections' after it. A thing whose use is not yet known says so in a sentence rather than
  guessing one.
- **A follow-up says one sentence more there:** how this subject bears on the document it follows from —
  what that document left open that this one answers.

## The line that says when and from what

- **The first section after the opening is the research's own record,** named `조사 시각` (English:
  `When and from what`), one line: the span of the research in KST with the minute's last digit
  blurred (`2026-10-21 03:26~05:1x KST`), the primary sources swept and what they were checked
  against (`1차 출처(공식 저장소 · 모델카드 · PyPI API) 전수 + 한국·영어 보도 대조`), and what the
  research did not do, each counted at zero: `설치 0 · 빌드 0 · 실행 0 · 가중치 다운로드 0`. What it
  did do is counted the same way: `우리 카드는 읽기만 했습니다`, `등록 정보와 258개 파일 목록`.
- **The narration's own line is not the run's to write.** The document keeps the narration at its
  top with its length, and `bmo-research-kinds` 「The narration」 sets the script; the run writes `LISTEN:`
  and nothing about listening in the body.

## The manual

- **Eleven sections, numbered 0 to 10, in this order and under these names,** after the opening
  (「The opening」), in the question's language (the Korean names are the ones the owner reads; an
  English question takes their plain English):
  0. `한 줄 정체` — the verdict: who made it, how big, what it does and what it claims, in one
     paragraph. It stands under `VERDICT:`, not as a section.
  1. `만든 곳 · 공개일 · 크기 · 라이선스` — the registry's own figures (the byte total, the file
     count, the licence file's words), and what the licence allows and forbids in one paragraph.
  2. `어떻게 만들어졌나 (논문 요약)` — the paper's method and evaluation, read whole; where there
     is none, the section opens `논문 없음 —` and names what stands in for it (the card, the blog,
     the repository).
  3. `제조사는 어떻게 구동하나` — the maker's own way of running it, the engine and the versions it
     names, the commands as the maker writes them. A dataset's reads `제조사는 어떻게 불러오나`.
  4. `제조사는 어떻게 쓰라고 하나` — the inputs, the settings, the prompt or template, and what the
     maker says not to do.
  5. `제조사가 밝힌 성능과 그 조건` — every figure with its benchmark, split, settings, hardware and
     who measured it; the comparison models the maker set beside it, whole; and one line saying
     whether the maker publishes the means of reproduction (the script, the data, the prompts).
  6. `한계와 주의` — in three parts, each under its own label: `제조사가 스스로 쓴 것.`,
     `공개 토론에서 확인된 것.` (issues, discussions, forum threads, each followed to whether it was
     fixed), `이번 조사에서 찾은 것.` (what the research itself found, a contradiction between the
     maker's documents among them, as such).
  7. `우리 랩에서 구동하려면` — the arithmetic (「Calculations shown whole」), then one label a
     line: `들어가는 카드:`, `안 들어가는 카드:`, `들어가려면:` (what would make it fit: a
     quantisation, a shorter context, an offload), `엔진:` (whether the maker's engine is one we
     run), `라이선스:`, `막히는 점:`.
  8. `우리 랩의 어떤 일과 닿나` — the place in our own routing it would take and what holds that
     place now, read from the brain, never from memory; it ends with `써 보면 알 수 있는 것`, three
     to five numbered questions. A manual decides nothing: whether to take the thing in is the
     owner's, so the section asks, never judges.
  9. `재현 계획 초안` — one label a line: `무엇을 다시 재나.`, `어떤 공개 세트 몇 문항으로.`,
     `어떤 설정으로.` (the maker's own conditions, as section 5 names them), `대략 걸릴 시간.`
     (always closed with `이것은 추정입니다.`), `재현이 아예 안 되는 것.` (a figure whose set,
     script or scoring the maker keeps). It is what a verification job starts from.
  10. `읽은 자료 목록 · 못 읽은 것` — every primary material read, each with its size (`카드 원문
      (26,990자)`, `논문 PDF 13쪽 전문`) and how far it was read (`전문`, `구조만 확인`, `요약만`);
      then `못 읽은 것.`, numbered, each with why (a gate, a login, a 404, a page drawn by script,
      a file not downloaded by the order's rule) and whether its absence moves the conclusion; `없음`
      where nothing was missed. What a source does not say is not among them: that is
      `적혀 있지 않은 것`, said where the value would have stood.
- **A thing with no weights** (a skill bundle, a recipe, a test bed) keeps the skeleton and says in
  section 7 why the arithmetic does not apply, rather than dropping the section.

## The investigation

- **Its sections, in this order,** the verdict above them as every result has:
  - `지금 결론` — the verdict, one paragraph, under `VERDICT:`.
  - `무엇이고 어디에 쓰나` — the opening (「The opening」).
  - `조사 시각` — the line above.
  - `처음 → 무엇이 틀려서 → 다음 → 지금` — how the matter came about, dated: what was first said,
    what proved wrong, what came next, where it stands now.
  - `사실 전수표` — the facts as numbered tables (`표 1 — …`), every table carrying a `출처` column;
    the first table rules the look-alikes in or out (`혼동 후보 전수`) so the reader knows which
    thing is meant.
  - `주장 심사` — the maker's claims judged in layers, with the table `제조사 주장 vs 제3자 측정 —
    대조표`: columns `수치·주장 | 조건 | 잰 사람 | 날짜 | 출처 | 판정`, a row `제3자 측정 | 없음`
    where nobody outside the maker measured, and `반대편 논거` set beside it.
  - `같은 범주 앞선 조사와 견주기` (English: `How it stands against earlier research`) — the earlier
    reports the question brought, each by its document number and title: what this subject changes from
    them, what it supersedes, what still holds, and which of their documents this research updates.
    Where the question says none stands, one line says the category held no earlier report and nothing
    is compared; never a comparison invented.
  - `우리 카드 산수` — 「Calculations shown whole」.
  - `쓸모 판정` — one of three, written with its mark: `ⓐ 시험하자`, `ⓑ 조각만 가져오자`,
    `ⓒ 안 씀, 계속 지켜봄`; ⓐ and ⓑ carry a `발주서 초안` for the job that follows: the grounds,
    the measurement plan, the rollback and what the job will not do.
  - `연표` — one table, dated.
  - `사장님이 정하실 것` — opens with how urgent the decision is (`지금 급한 결정은 없다` where none),
    then the options as a table `선택지 | 무엇 | 사람 시간 | 레인 점유 | 되돌림`, and one line
    `권하는 순서`.
  - `아직 모르는 것` — the `UNCONFIRMED:` list, under that name.

## The verification

- **Its sections, in this order,** under the origin line, which names the manual it follows from:
  - `결론` — the verdict, one paragraph: whether the maker's figures were reproduced, what was kept,
    and what the owner has to do, `사장님이 하실 일은 없습니다` where nothing.
  - `무엇이고 어디에 쓰나` — the opening (「The opening」).
  - `이걸로 정하실 것` — numbered options, each with `비용` and `되돌림`; `지금은 보관만(비용 0)`
    is always one of them, and the one recommended says `(권장)`.
  - `모델 한 줄`.
  - `제조사 주장 vs 우리 측정` — the table whose title carries every condition of the measurement
    (`모두 2026-10-06 KST · rtx6000 · bf16 · 1024² · 25스텝 · CFG 3.0`), with the columns
    `벤치 | 제조사 값 (출처) | 우리 값 (<card> · 3회 중앙값) | 판정 | 조건 차이`.
    - **The pass line is set before the first run** and written beside the claim: `합격선 −5p`,
      `제조사 주장 «약 5배» — 합격선 4.75배 이상`.
    - **The verdict column's words:** `✅ 합격 (차이 +2.2p)`, `✅ 우위 (+31%)`, `동급`, `재현 안 됨
      (제조사 91.1 · 우리 65.6~75.4)`, `미검증 (우리 검출 도구 없음)`, `축 상이 — 직접 비교 불가`;
      `—` where the maker published no figure and ours stands alone. A speed that reproduced beside
      a quality that did not is two rows, never one word.
    - **Each side's denominator stands with it:** `376/420` against `11/12`, and a failure's kinds
      set against the weaknesses the maker admits.
  - `어떻게 돌렸나 (제조사 조건 그대로)` — what was checked to be the maker's own: the data (rows
    and tokens), the engine's release or commit against the one the maker requires, the weights'
    sha256 against the maker's checksum, the prompt or template byte for byte, the sampling, and
    the hardware by name (the card, the site, the arbiter's hold). The same script measures both
    sides. Runs are repeated, three at least, the median given with its spread (`49.8 토큰/초 (21회 ·
    49.4~50.0)`) and a deterministic result said to be one; where a maker's condition cannot be had,
    the pair measured by the same road stands in (`BF16↔Orca`) and the absolute comparison is
    withheld, saying so; where a claim's own measure is unpublished, the verification redesigns
    around what can be measured (`채점 조건 미공개 → 생성 성공·거부 없음·속도를 재는 검수`) and says
    so. An ablation that removes the thing claimed (`LoRA 제거`) is run beside it.
  - `보관 (nastank)` — the path (`/mnt/tank/Backup/models/<name>/`), what was kept (the weights with
    a sha256 a file, the runner's commit and versions, the scripts, the test data, every round's
    results, `RESTORE.md`), and the restore proved: fetched back, every hash matched to the original
    and the maker's, served again and a regeneration byte-identical to the original; then what was
    cleaned up and the space left.
  - `우리 적용 방안` — as a table `키 | 값`: the slot, the consumer, who owns the wiring, the
    rollback.
  - `열린 질문` — the `UNCONFIRMED:` list, under that name: what was not measured and why, each
    with what would measure it.
  - `근거·원문 링크` — the sources, with the evidence folder the raw results stand in.
- **A verification that could not run** writes its verdict as `판정: 실행 대기 — <why>`, the table
  as `항목 | 값` with what was already checked (the archive, the hashes, the install), and ends with
  `다음에 정하실 것`.

## Every figure carries its mark

- **Every figure carries where it came from,** in the search's own footnote `[n]` and, in a table,
  a `출처` column naming the source's part in words: `카드 §Usage`, `등록 정보`, `논문 표 2`,
  `저장소 config.json`, `토론 #25`, `릴리스 노트 「Known issues」`. Words taken from a source stand
  in `«…»`; a document's title in `「…」`.
- **Who measured it stands with it:** `제조사 자체 발표`, `자기 벤치`, `커뮤니티 실측 (한 사람의
  측정 · 제조사 확인 없음)`, `우리 실측 (2026-10-03 14:47 · lab-live)`. The maker's figure is never
  written as ours, and `제3자 재현 없음 (확인 시각)` is said where nobody else measured.
- **A value not verified says so beside it,** its ground in brackets: `(추정 — 카드의 비율을
  가정)`, `(미검증)`, `(미측정)`, `(카드 주장)`, `(확인 필요)`, `«측정 전»`; `(직접 계산)` on a value
  the run derived. A value seen in a search result alone is `«스니펫»`. Nothing is guessed to fill a
  cell: an empty cell means `모른다`, and is written `카드에 없음` or `공개된 것 없음`.
- **Two sources that disagree stand side by side** with the difference said, and the one trusted
  named with why (the repository's file over the card's prose; the newer page over the older
  picture).
- **The maker's superlative is narrowed to its scope:** `«모두 가장 높다» 는 같은 크기 안에서의
  이야기입니다`.

## Calculations shown whole

- **The cards and their totals are read from the brain's knowledge of the compute nodes,** never
  from memory, and the arithmetic is on an empty card's total: what is on it now is the arbiter's
  business at the time.
- **A calculation is written so the reader can redo it:** the input with its source (`248,937,280,400
  바이트 ÷ 10억 [등록 정보]`), the formula, the result, and the verdict a card: `510.3 GB ÷ 121 GB =
  4.2 → 노드 5대 (가중치만 · KV 전)`; `키밸류 2헤드 × 헤드 차원 64 × 층 8 × 토큰 수 × K·V 2벌 ×
  2바이트 ≈ 0.5기가`. Decimal and binary units are told apart and the conversion shown.
- **What the maker left unstated is its own line,** marked: the activation and cache share
  `(추정 — 재 봐야 아는 값)`, a vocoder's share `(카드의 0.10을 가정하면)`.
- **A time is always an estimate,** `(추정)`, and a cost's exchange rate is stated with it.
- **A claim is checked by arithmetic where it can be:** a card's parameter count against the file's
  bytes at the stated precision.

## The pictures

- **A picture is taken only from material this research read.** It comes from a page the run or one of its
  branches opened, or from a source the report lists, and it shows the subject itself: a model's interface,
  a chart of its figures, a written specification, a screen, a design, the thing a product shot or a
  diagram shows. The `IMAGE:` line stands where the text writes about what it shows, inside the section or
  entry it belongs to, and the block closes with the `SOURCE` lines that name where it came from. A picture
  whose block names no source a reader can open is left out, and so is one from a page nobody here read.
- **At most six pictures in the body**, and fewer where fewer carry the subject: one to three for a thing,
  and more only where the question is how something looks or what a screen or a figure actually says. Never
  one for a value, a rate, a date or a version.
- **The caption says what the picture shows and where it came from:** the thing, the part, the view or the
  screen in a few words of the question's language, then the source entry's own title. It reads on its own,
  so a reader who does not open the source still knows what they are looking at. The report keeps its own
  copy of every picture it carries and draws it beside that caption.
- **Leave out decoration and anything not this subject:** a logo, a banner, an icon, a stock photo, a map, a
  price table or any picture that is mostly text, a screenshot's browser chrome, another thing's picture,
  another branch's or another business's picture, and one too small or blurred to make out. Where nothing
  visual was read, the result carries no `IMAGE:` line at all.

## The voice

- **The question's language,** plain sentences in the register the owner's documents use (Korean:
  합쇼체); `사장님` only where a decision is put to him. A term is written in its usual Hangul
  spelling with the original in parentheses on its first use (`추측 디코딩(speculative decoding)`),
  and a reading spelled for the ear (`브이엘엘엠`) stays in the narration.
- **Nothing is dressed up:** no run says it installed, built or downloaded what it did not; no
  figure stands without its measurer; no option is argued past what was measured; and a manual
  leaves the decision to the owner as questions.
