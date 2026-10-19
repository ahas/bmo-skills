# Wording

## How this document stands

- This is the one place that says how the chat agent and the workers spell words in Korean text, and the one
  list of purified words that must not be used.
- The banned-term list and the allowed uses below are read by the wording check, a test in the
  repository suite (`repo/checks/wording.test.ts`) that also runs among the quick checks ahead
  of it (`bmo check quick`); a line added here is enforced from the next run.
- A term the owner points out as over-purified is added here as one more line, together with any use
  already in the tree that the check then reports.
- `bmo check wording` hands this list to a program outside the bot: `table` prints the banned
  terms, `scan` names the terms each text on its input uses, matched as the check matches them, and
  `add` writes new banned lines at the end of the list.
- The same list is read again where words are written rather than committed, as
  「Where it is held as words are written」 says.
- A chat turn is given this document by the system-documents gate (`docs/reference-index.md`); a
  worker reads this list through `AGENTS.md` and the principle in its own trailer, and the
  `ui-design-rules` skill carries it as `wording.md`
  beside itself.

## The principle

- A Korean sentence uses the word people most commonly say for the thing, whatever its origin: a
  Sino-Korean word, a loanword in its usual Hangul spelling, or a native word, whichever is the one
  in everyday use.
  - Right: 위치, 질문, 단어, 이유, 툴바, 빌트인, 편집, 수정.
  - Wrong: a native word or coinage put in place of the common one — "자리" for 위치, "물음" for
    질문, "낱말" for 단어, "까닭" for 이유, "도구 띠" for 툴바, "붙박이" for 빌트인, "손대기" for 편집.
- A native word is written where it is the one people say — 파일을 열어, 줄을 바꿔 — and never
  because it is native. Plain writing (`prompts/rules/plain-writing.md`) is about how sentences
  read; it is not a preference for native words.
- The English documents a writer reads are written in English idiom, not in terms: a thing
  "stands" somewhere, holds "a place", is reached by "a road", leaves "a strand". A Korean text
  renders the meaning in the common Korean word, never the English word's native Korean twin.
  - A place is 위치, a queue's 순서 or a pool's 슬롯; to stand somewhere is 있다 or 표시되다; a
    road is 경로 or 방법; a strand is one 건 of a list.
- English is not dropped into the middle of a Korean sentence.
  - Right: "툴바를 눌러". Wrong: "toolbar를 눌러".
- English stays English only where no Hangul spelling is in common use:
  - Code identifiers, commands, file and path names — `max_tokens`, `bun run test`, `WORDING.md`.
  - Names of products, libraries and protocols — Nuxt UI, SSE, herdr.
- **A job is Job or 작업 in Korean text, never 잡.** This is the owner's own choice of the word, and it
  holds wherever the chat agent or a worker writes: an answer, a job's report, a notice, a console label, a
  document's Korean example.
  - Job where the text names this system's unit of work — Job #123, Job 워커, Job 보고, 다음
    Job, 실행 중인 Job — and wherever 작업 would read as the TODO item (항목) or as a work order
    (작업 지시서). A particle follows it as it followed 잡: Job이, Job을, Job은, Job으로.
  - 작업 where the text means the work itself rather than the unit that runs it.
  - Spoken, Job is 잡: the voice name table reads it so in a briefing (`docs/voice-plan.md`,
    「How names are said」), so a line written with Job is heard the way the owner says it.
- The list below names the swaps already caught; a word missing from it is held to the same
  principle.

## Console text

- The web console, its language pack and every message a control route hands it to show are
  written by this glossary as well as by the principle above.
- Each line is `<English concept> → <the Korean UI word>`, with the forms it takes and the forms it
  replaces. The word stands in every inflection: where the glossary says 삭제, a past tense is
  삭제했습니다 and a failure 삭제하지 못했습니다.
- delete → 삭제: 삭제, 삭제했습니다, 삭제하지 못했습니다, 삭제된, 삭제할까요. Never 지우다, 지우기, 지운,
  지워진, 지움; taking one entry out of a list is 삭제 or 제거, never 빼기.
- retire, take down → 보관; bringing one back → 복원. Switching a piece in or out of use → 켜기, 끄기.
  Never 내리기, 올리기.
- fold (a work order) → 종료. 접기 is only ever a panel folding away, and is written as below.
- file, stand up (an item) → 만들기: 항목 만들기, 만든 항목, 만들지 못했습니다. Never 세우기, 올리기.
- ask (a job's question) → 질문, 질문하다: Job의 질문. Never 물다, 물어본 것, 묻기.
- machine → 기기: 이 기기, 기기 여유. Never 기계.
- render → 렌더링: 렌더링하지 않는 기간. Never 그리다.
- message, words → 메시지: 메시지를 보내다, 보낼 메시지. A conversation is 대화, an instruction added
  to an item is 지시. Never 말, 할 말, 이야기, 말을 걸다.
- filter, sift → 필터. Never 거르기.
- page → 페이지. Never 쪽.
- table → 테이블; row → 행; query → 쿼리, wherever the text is about a database. Never 표, 줄, 질의.
- default → 기본값. Never 처음 값.
- section → 섹션. Never 구역.
- health check → 헬스 체크; the one run before anything changes → 사전 헬스 체크. Never 생존 확인.
- rollback → 롤백. Never 되돌림.
- template → 템플릿; coverage → 커버리지; cap → 상한; cooldown → 쿨다운. Never 틀, 덮음, 천장, 휴식.
- output tokens → 출력; context window → 컨텍스트. Never 답, 창.
- collapse, expand a panel → 닫기, 열기; collapse, expand in place → 축소, 확장. Never 접기, 펴기, 펼치기.
- reach a limit → 도달; a match → 일치, 해당; tied to → 연결된. Never 걸리다, 걸린 곳.
- reach a service → 연결되다: 봇에 연결되지 않아. Never 닿다.
- request → 요청하다. Never 청하다.
- choose, pick → 선택: 선택해 주세요, 선택 취소. Never 고르기, 골라 주세요, 물리기.
- enter text → 입력: 입력해 주세요, {n}자 이내로 입력. Never 적어 주세요.
- launch, start a run → 실행. Never 띄우다, 뜨다, 발사.
- stop a run → 중지; pause and resume a timer → 정지, 재개; run out of time → 시간 초과. Never 멈추기,
  다시 흐르게, 때를 놓침.
- leave the queue → 대기열에서 제외. Never 대기열에서 빠지다.
- set by hand → 수동 지정. Never 손으로 정함.
- **One screen speaks in one style.** A notice, a guidance line and an error end in 합니다 and
  했습니다 throughout; 어요 is never mixed into them.
- **No dash between words.** A range is written `{min}~{max}`; a caveat on a title stands on its
  own smaller line, or after a ` · `.

## Banned terms

- Each line is `<language code> · <banned term> → <what to write>`, optionally followed by
  ` — <when it applies>`.
- The language code is the language of the text the entry governs (`ko`, `ja`, `en`); an entry is
  checked only against text written in that language.
- The banned term is matched as written, spaces included, so a spacing variant is a line of its
  own.
- A Korean term is matched only where it stands at the head of its word: a syllable running into
  it from the left makes it part of another word, so 되돌리기 is not 돌리기 and 한도는 is not 도는.
  What follows it is not asked about, since a particle or an ending there leaves the word its own.
- One line per ending in use, rather than a stem standing for a family, so that the line can say
  what to write in that ending's place.
- 돌려 is not listed: 돌려주다 and 돌려보내다 are their own verbs and share the spelling. Where it
  means running something, write 실행해 — 실행해 둔 채, 두 번 실행해도, 한 줄만 실행해 주면.
- 돈 and 돌아 are not listed either: 돈 is money and 돌아오다 and 돌아가다 are their own verbs. Where
  either means running something, write 실행 — 실행된 Job, 뜨지 못한 실행.
- 나무 alone is not listed: 나무위키 and 나무집 are names. Where it stands for a tree of files, folders
  or nodes, write 트리, and each such phrase is a line below.
- 마디 is listed alone, since 한마디 is its own word and the word-head rule leaves it be; it is
  written without a space.
- 자리 is listed alone and banned where it stands for a position, a slot or a part of a text or of
  code; the phrases in which it keeps its own sense, a seat or a digit, are listed under
  「Uses in the word's own sense」. 제자리, 빈자리 and 일자리 are words of their own, and the
  word-head rule leaves them be.
- 길 is not listed alone: 길이 and 길다 share it. Where it means a route or a way of doing
  something, write 경로 or 방법, and each phrase in use is a line below.
- 잡 is listed alone, since a job written 잡 takes the same particles the verb 잡다 takes. The
  verb's own endings, the words that open with it (잡음, 잡담, 잡지) and 잡 following an object
  (을 잡, 를 잡) are listed under 「Uses in the word's own sense」; 손잡이 and 붙잡다 are left be by
  the word-head rule. 잡은 and 잡을 with no object before them read as a job, so the verb is
  written there with its object.
- 판 is not listed alone: 판정, 판단 and 판매 open with it, and a round of a game is 한 판. Where it
  means a version, write 버전 — the console labels an installed commit 버전 — and each phrase in
  use is a line below. A phrase line matches 판정, 판단 and 판매 after it too, so each of those it
  meets in use is listed under 「Uses in the word's own sense」.

- ko · 도구 띠 → 툴바
- ko · 도구띠 → 툴바
- ko · 붙박이 → 빌트인
- ko · 손대기 → 수정, 편집, 정정
- ko · 고치기 → 수정, 편집, 정정 — when it means editing; where it means a repair, the use is written under the allowed uses
- ko · 지시서 접기 → 지시서 종료 — 접기 means folding a panel away, never ending a thing
- ko · 건너뛰고 접기 → 건너뛰고 종료
- ko · 접는 중인 지시서 → 종료 중인 지시서
- ko · 접는 중이라 → 종료 중이라
- ko · 있어 접지 않습니다 → 있어 종료하지 않습니다
- ko · 다시 읽기 → 새로고침 — a control or notice that fetches a screen's data again
- ko · 다시 읽는 중 → 새로고침 중
- ko · 돌리기 → 실행 — running a thing is 실행: 협업으로 돌리기 is 협업으로 실행, 다시 돌리기 is 다시 실행
- ko · 돌기 → 실행 — when it means running something
- ko · 돌리는 → 실행하는
- ko · 돌리지 → 실행하지
- ko · 돌리고 → 실행하고
- ko · 돌리며 → 실행하며
- ko · 돌리게 → 실행하게
- ko · 돌리거나 → 실행하거나
- ko · 돌리려면 → 실행하려면
- ko · 돌린 → 실행한 — 돌린다 with it
- ko · 돌릴 → 실행할
- ko · 돌립니다 → 실행합니다
- ko · 돌렸 → 실행했
- ko · 돎 → 실행됨 — 협업으로 돎 is 협업 실행됨
- ko · 도는 → 실행 중인, 실행되는
- ko · 돌고 → 실행 중이고, 실행되고 — 돌고 있습니다 is 실행 중입니다
- ko · 돈다 → 실행된다
- ko · 돌았 → 실행됐
- ko · 돌면 → 실행되면
- ko · 검색 문서 → 아카이브 — the space every kept document stands in, whoever wrote it; one document in it is 아카이브 문서, or 문서
- ko · 까닭 → 이유, 사유, 원인 — 이유 for why a thing is or was so; 사유 for the reason a person gives for stopping, skipping, ending or taking down a thing, so 접는 까닭 is 종료 사유 and 중지하는 까닭 is 중지 사유; 원인 for what made a thing fail
- ko · 파일 나무 → 파일 트리
- ko · 폴더 나무 → 폴더 트리
- ko · 나무 구조 → 트리 구조
- ko · 마디 → 노드, 턴 — 노드 for a node of a graph or a tree, 턴 for a turn of a conversation: {n}마디 is {n}턴
- ko · 갈래 → 브랜치, 종류, 분류, 번호 — 브랜치 for a branch of a search, a co-work or a workflow; 종류 for a kind of thing, as in <종류>:<id>; 분류 for a group things are counted under; 번호 for one numbered part of a 미결, as in 미결 2번; where it means splitting, 둘로 나누기
- ko · 곁가지 → 코워커 — a co-worker's run out of the pool is a co-worker's, whichever road it came by: a co-work task, a search's branch or a job's own call
- ko · 자리 → 위치, 부분, 지점, 슬롯, 순서, 바로 — 위치 for where a thing is or goes, so 이 자리 is 이 위치 and 원래 자리 is 원래 위치; 부분 or 코드 for the part of a text or of code that does something, so 읽는 자리 is 읽는 부분; 지점 for where a run stopped; 슬롯 for a place a pool or a worker holds, so 코워커 자리 is 코워커 슬롯; 순서 for an item's place in a queue; 바로 or 즉시 for 그 자리에서 meaning at once; where it means a matter that is someone's, as in 오너가 정할 자리다, 일
- ko · 낱말 → 단어
- ko · 물음 → 질문
- ko · 머리말 → 헤더, 서두
- ko · 꼬리말 → 푸터, 끝부분
- ko · 토막 → 부분, 단락, 구간 — 세 토막 양식 is 세 부분 양식
- ko · 갈무리 → 캡처, 저장
- ko · 길목 → 경로, 지점
- ko · 같은 길로 → 같은 경로로, 같은 방법으로
- ko · 같은 길을 → 같은 경로를, 같은 방법을
- ko · 다른 길로 → 다른 경로로, 다른 방법으로
- ko · 다른 길을 → 다른 경로를, 다른 방법을
- ko · 문턱 → 임계값, 기준 — 95% 문턱 is 95% 임계값
- ko · 값어치 → 가치
- ko · 짜임 → 구조, 구성
- ko · 가닥 → 건, 항목 — 미결 3가닥 is 미결 3건
- ko · 콘솔 글자 → 콘솔 문구
- ko · 화면 글자 → 화면 문구
- ko · 잡 → Job, 작업 — a job: Job where the text names the unit of work, so 잡 #123 is Job #123, 잡 워커 is Job 워커 and 실행 중인 잡 is 실행 중인 Job; 작업 where it means the work itself
- ko · 데이터 집합 → 데이터셋, 데이터 세트 — a set of data
- ko · 대체 불가능한 토큰 → NFT — a non-fungible token
- ko · 상향 판매 → 업셀링, 업셀 — selling a customer a costlier or added product
- ko · 위도 풀다운 → 랫풀다운, 랫 풀다운 — the gym exercise, lat read as latitude
- ko · 하위 레딧 → 서브레딧 — a community on Reddit
- ko · 새 판 올리 → 새 버전 올리 — a version: 새 판 올리느라 is 새 버전 올리느라
- ko · 판을 올리 → 버전을 올리
- ko · 판이 올라 → 버전이 올라
- ko · 판으로 올 → 버전으로 올
- ko · 같은 판 → 같은 버전
- ko · 이전 판 → 이전 버전
- ko · 커밋의 판 → 커밋의 버전
- ko · 받은 판 → 받은 버전
- ko · 판 번호 → 버전 번호
- ko · 발사 → 실행, 시작 — launch read word for word: launching a unit, a run, a search or a session is 실행, and its first moment 시작

## Uses in the word's own sense

- A term banned in one sense keeps its own sense in the phrases listed here, and a use inside one
  of them passes wherever it is written: in a file, an answer, a report or a row.
- Each line is `<language code> · <banned term> · <a phrase holding the term>`, followed by
  ` — <the sense it keeps>`. The phrase is matched as written, spaces included, around the term.
- A phrase that does not hold its term, or names a term not banned above, is refused.

- ko · 자리 · 자리를 비 — a seat left empty: 자리를 비우다, 자리를 비웠어
- ko · 자리 · 자리 비 — a seat left empty: 자리 비움
- ko · 자리 · 자리에 앉 — a seat
- ko · 자리 · 앉을 자리 — a seat
- ko · 자리 · 앉은 자리 — a seat
- ko · 자리 · 앉는 자리 — a seat
- ko · 자리 · 자리에서 일어나 — a seat
- ko · 자리 · 자리를 뜨 — leaving where one sat
- ko · 자리 · 자리를 지키 — staying where one is
- ko · 자리 · 자리를 지켜 — staying where one is
- ko · 자리 · 자리 지켜 — staying where one is
- ko · 자리 · 자리 예약 — a seat booked
- ko · 자리 · 창가 자리 — a seat
- ko · 자리 · 식사 자리 — a gathering
- ko · 자리 · 회식 자리 — a gathering
- ko · 자리 · 모임 자리 — a gathering
- ko · 자리 · 자리 숫자 — a digit
- ko · 자리 · 자리 수 — a count of digits
- ko · 자리 · 자리 코드 — a code of so many digits
- ko · 자리 · 자리 번호 — a number of so many digits
- ko · 자리 · 째 자리 — a decimal place
- ko · 자리 · 자리매김 — its own word
- ko · 물음 · 물음표 — the question mark
- ko · 토막 · 토막잠 — its own word
- ko · 문턱 · 문턱이 낮 — a barrier to entry
- ko · 문턱 · 문턱이 높 — a barrier to entry
- ko · 문턱 · 문턱을 넘 — a threshold crossed
- ko · 가닥 · 가닥을 잡 — getting the gist
- ko · 가닥 · 가닥이 잡 — getting the gist
- ko · 잡 · 을 잡 — the verb 잡다 after its object
- ko · 잡 · 를 잡 — the verb 잡다 after its object
- ko · 잡 · 잡아 — the verb 잡다
- ko · 잡 · 잡았 — the verb 잡다
- ko · 잡 · 잡고 — the verb 잡다
- ko · 잡 · 잡는 — the verb 잡다
- ko · 잡 · 잡느라 — the verb 잡다
- ko · 잡 · 잡다 — the verb 잡다, and 잡다한
- ko · 잡 · 잡게 — the verb 잡다
- ko · 잡 · 잡기 — the verb 잡다
- ko · 잡 · 잡자 — the verb 잡다
- ko · 잡 · 잡던 — the verb 잡다
- ko · 잡 · 잡습니 — the verb 잡다
- ko · 잡 · 잡으면 — the verb 잡다
- ko · 잡 · 잡으려 — the verb 잡다
- ko · 잡 · 잡으며 — the verb 잡다
- ko · 잡 · 잡으므로 — the verb 잡다
- ko · 잡 · 잡으니 — the verb 잡다
- ko · 잡 · 잡으세요 — the verb 잡다
- ko · 잡 · 잡을 수 — the verb 잡다
- ko · 잡 · 잡을게 — the verb 잡다
- ko · 잡 · 잡을까 — the verb 잡다
- ko · 잡 · 잡히 — the verb 잡히다
- ko · 잡 · 잡혀 — the verb 잡히다
- ko · 잡 · 잡힌 — the verb 잡히다
- ko · 잡 · 잡힐 — the verb 잡히다
- ko · 잡 · 잡혔 — the verb 잡히다
- ko · 잡 · 잡힘 — the verb 잡히다
- ko · 잡 · 잡지 — the verb 잡다, and a magazine
- ko · 잡 · 잡음 — noise, and a slot held
- ko · 잡 · 잡담 — its own word
- ko · 잡 · 잡일 — its own word
- ko · 잡 · 잡무 — its own word
- ko · 잡 · 잡념 — its own word
- ko · 잡 · 잡초 — its own word
- ko · 잡 · 잡화 — its own word
- ko · 잡 · 잡동사니 — its own word
- ko · 같은 판 · 같은 판정 — a judgement
- ko · 같은 판 · 같은 판단 — a judgement
- ko · 같은 판 · 같은 판매 — a sale
- ko · 이전 판 · 이전 판정 — a judgement
- ko · 이전 판 · 이전 판단 — a judgement
- ko · 이전 판 · 이전 판매 — a sale
- ko · 받은 판 · 받은 판정 — a judgement
- ko · 받은 판 · 받은 판단 — a judgement
- ko · 커밋의 판 · 커밋의 판단 — a judgement

## Allowed uses

- A use that is right in its context is written here, so the check passes it and nothing else.
- Each line is `<language code> · <banned term> · <file path> · <an exact excerpt of the line>`,
  followed by ` — <why it is right there>`.
- An allowed use that no longer matches any line is itself reported, so the list does not outlive
  the text it excused.

- ko · 도구 띠 · prompts/agents/worker/trailer.md · "도구 띠"; 빌트인 — the worker's statement of this principle, naming the wrong form
- ko · 붙박이 · prompts/agents/worker/trailer.md · 빌트인, not "붙박이" — the worker's statement of this principle, naming the wrong form
- ko · 손대기 · prompts/agents/worker/trailer.md · not "손대기" — the worker's statement of this principle, naming the wrong form
- ko · 도구 띠 · prompts/agents/chat/manual.md · 툴바, not "도구 띠" — the chat's statement of this principle, naming the wrong form
- ko · 붙박이 · prompts/agents/chat/manual.md · 빌트인, not "붙박이" — the chat's statement of this principle, naming the wrong form
- ko · 손대기 · prompts/agents/chat/manual.md · not "손대기" or "고치기" — the chat's statement of this principle, naming the wrong form
- ko · 고치기 · prompts/agents/chat/manual.md · not "손대기" or "고치기" — the chat's statement of this principle, naming the wrong form
- ko · 도구띠 · prompts/agents/worker/plugin/skills/ui-design-rules/SKILL.md · '도구띠', — the skill's statement of this principle, naming the wrong form
- ko · 자리 · prompts/agents/worker/plugin/skills/ui-design-rules/SKILL.md · wrong: '자리', — the skill's statement of this principle, naming the wrong form
- ko · 자리 · prompts/agents/worker/trailer.md · 위치, not "자리" — the worker's statement of this principle, naming the wrong form
- ko · 물음 · prompts/agents/worker/trailer.md · 질문, not "물음" — the worker's statement of this principle, naming the wrong form
- ko · 자리 · prompts/agents/chat/manual.md · 위치, not "자리" — the chat's statement of this principle, naming the wrong form
- ko · 물음 · prompts/agents/chat/manual.md · 질문, not "물음" — the chat's statement of this principle, naming the wrong form
- ko · 자리 · prompts/rules/plain-writing.md · "자리" for 위치 — the rulebook's statement of this principle, naming the wrong form
- ko · 물음 · prompts/rules/plain-writing.md · "물음" for 질문 — the rulebook's statement of this principle, naming the wrong form
- ko · 낱말 · prompts/rules/plain-writing.md · "낱말" for 단어 — the rulebook's statement of this principle, naming the wrong form
- ko · 까닭 · prompts/rules/plain-writing.md · "까닭" for 이유 — the rulebook's statement of this principle, naming the wrong form
- ko · 도구 띠 · prompts/rules/plain-writing.md · "도구 띠" for 툴바 — the rulebook's statement of this principle, naming the wrong form
- ko · 붙박이 · prompts/agents/worker/plugin/skills/ui-design-rules/SKILL.md · '붙박이'. — the skill's statement of this principle, naming the wrong form
- ko · 돌리기 · markup/markup/wording.ts · 되돌리기 is not 돌리기 — the check's own statement of the word-head rule, naming the two forms it tells apart
- ko · 검색 문서 · graph/bot.json · "검색 문서" — the old name kept as a graph alias of the archive document, so a search in the owner's earlier word still lands on it
- ko · 검색 문서 · graph/console.json · "검색 문서" — the old name kept as a graph alias of the archive screen, so a search in the owner's earlier word still lands on it
- ko · 곁가지 · graph/bot.json · "곁가지" — the old name kept as a graph alias of the co-work runner, so a search in the owner's earlier word still lands on it
- ko · 갈래 · agents/agent/job-spans.ts · { 갈래: branchWord } — the word spans already kept in an archive open with, named so the archive can rewrite them to the branch word
- ko · 잡 · prompts/rules/plain-writing.md · Job or 작업, never 잡 — the rulebook's statement of this rule, naming the wrong form
- ko · 잡 · prompts/rules/plain-writing.md · says Job as 잡 — the rulebook's statement of how a briefing says the word
- ko · 잡 · docs/voice-plan.md · | `Job` | 잡 | — the voice name table's row saying Job as 잡
- ko · 잡 · docs/voice-plan.md · the owner says it 잡 — the reason for that row
- ko · 잡 · bot/bot/voice.ts · the owner says it 잡 — the reason for the seed row saying Job as 잡
- ko · 잡 · bot/bot/voice.ts · ko: "잡" — the seed row saying Job as 잡
- ko · 잡 · docs/voice-lines.md · (잡 for Job) — a voice script's reading may say Job as the voice name table does
- ko · 잡 · tools/voice/bmo/voice-lines.ts · Job is read 잡 — why a voice script's reading may say Job as the voice name table does
- ko · 잡 · lang/bmo-lang/lang/ko/agent-catalogue.yaml · "잡 워커", "워커", "워커잡" — the names the owner calls the worker by, kept so his words still find it
- ko · 잡 · policy/bmo-policy/policy/common-words.toml · "job", "잡", — a word every turn carries, listed so a description leaning on it is told from one saying what its piece is about; the older word is still in what is written
- ko · 잡 · policy/bmo-policy/policy/closing.toml · (Job|잡|항목 — a report's deferral is read in the older word as much as the written one

## Where it is held as words are written

- **A TODO row:** filing one and rewriting one refuse a title, and the parts the chat agent writes around the
  conversation — what the work is, what to do, the conclusion, a judgement — that use a banned term,
  and name each term beside what to write. This holds for `bmo todo --add` and `--edit`
  and for the chat's `todo_add` alike. The conversation copied into a row is what was said and is
  not read.
- **The chat agent's answer,** in every room and the owner's own chat with them: an answer that uses a banned
  term is asked for once more, with the terms and what to write named, and the answer that comes
  back goes out in its place where it is an answer in the room's language.
- **A worker's record put into Korean:** a rendering of what a job wrote onto an item that uses a
  banned term is asked for again, up to twice, with the terms named, before it is written on the row
  (`docs/worker-language.md`, 「What a worker writes onto an item」).
- **A job's report:** the worker's turn is held at its end while its final answer uses a banned term,
  within the holds a stop is given, with the terms named. A report that still uses one goes out
  with the terms named under it, where the owner and the next job both see it.
- **A job's question to the owner:** `bmo job ask` refuses a question whose question, options or why use
  a banned term, and a talk whose answer or changed question does, naming each term beside what to
  write, before its row is written or the chat is told. A job writes its question in Korean itself,
  with no rendering between it and the chat, so this is the one place its words are held
  (`docs/jobs.md`, 「A job's question to the owner」).
- **Quoted words are not read there.** A span inside backticks or quotation marks is how a text
  names a wrong form — 툴바, not "도구 띠" — or quotes a command or somebody's words, and it passes.
  The file check has no such exception: a quoted use in the tree is written under the allowed uses.

## Where it is checked

- **Files:** every tracked file of these kinds, test files and this document excepted.
  - The web console's screens and code under `client/app`.
  - The bot's code under `server` and `scripts`, whose strings reach the chat.
  - **Source is source whatever it is written in.** Under those three roots the check reads
    TypeScript and Vue, and equally a python script, a shell script and a stylesheet: Korean
    reaches a person out of any of them, and a rule naming only the two languages the bot happens
    to be written in let a script's own printed messages past.
  - The documents: `docs/`, `README.md`, `AGENTS.md`, `STRUCTURE.md`, `prompts/rules/`, `prompts/agents/`,
    `prompts/persona/`, and the dependency graph under `graph/`.
- **Language of a line:** a line holding Hangul is Korean (`ko`); a line holding kana and no
  Hangul is Japanese (`ja`); any other line is English (`en`). An entry is matched only against
  lines of its own language, so a mixed English document is checked for a Korean term only on the
  lines that are Korean text.
- **Test files are left out** because they carry made-up titles and quoted input, which are not
  anybody's wording.
- **Data is left out for the same reason, however much Korean it holds.** The sentences a voice
  measurement scores itself against, a recorded output kept under a `fixtures/` directory so a
  test can compare against it, a third-party font's licence, a list of package versions — none of
  it is anybody's wording to fix, and a check that reported it would teach its readers to ignore
  it. A fixture is left out by the directory it stands in, whatever it is written in.
