[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

# bmo-skills

잡 워커에게 주어지는 스킬들과, 이를 넘겨주는 배치기(placer)입니다. 스킬은 하나에 폴더 하나이며, 폴더 이름은 범위
(`bmo` 또는 `ahas`)와 분류로 시작합니다. 각 스킬은 `SKILL.md`와 그것이 가리키는 파일들로 이루어지고, front matter에
어느 번들에 속하는지 적습니다. 번들은 일반 잡 워커가 모두 받는 `worker`, 코더의 코딩·설계 실행이 받는 `coder`,
프로듀서의 영상 실행이 받는 `producer`, 그리고 한 저장소의 체크아웃이 자체적으로 갖는 스킬인 `project`가 있습니다.
이 저장소는 아무것도 실행하지 않습니다. 소비하는 쪽의 배포가 `bmo-skills place`를 실행하면, 워커 플러그인을 Claude
Code의 표준 플러그인 구조로, 또는 스킬 하나를 그 자체 폴더로 소비자의 트리에 기록합니다.

## 구조

| 폴더 | 내용 |
|---|---|
| `.claude-plugin/` | 배치되는 모든 번들이 기록되는 `bmo-worker` 플러그인의 매니페스트 |
| `ahas-code-boundaries/` | 변형이 어디에 속하는지, 사람이 편집하는 목록을 어디에 두는지, 기반 라이브러리에 어떻게 기여하는지 |
| `ahas-code-clean/` | 리팩터링의 줄 단위 패스. 원칙과 Rust·TypeScript 노트 포함 |
| `ahas-code-design-patterns/` | 리팩터링의 구조 패스. 패턴 카탈로그 포함 |
| `ahas-code-refactoring/` | 리팩터링의 모듈·함수 패스. 카탈로그와 냄새 목록 포함 |
| `ahas-code-test-writing/` | 테스트 작성 규칙 |
| `ahas-docs-git/` | git 작업 흐름, 문서화, 변경 기록 규칙 |
| `ahas-tools-find-skills/` | 공개 스킬 가운데 작업에 맞는 스킬 찾기 |
| `ahas-tools-playwright-cli/` | 브라우저 조작과 Playwright 테스트 작성. 레퍼런스 포함 |
| `ahas-ux-design-rules/` | 모든 사용자 인터페이스에 대한 영역별 디자인 규칙 |
| `ahas-ux-nuxt-style/` | Node.js, TypeScript, Vue 또는 Nuxt 프로젝트의 코딩 스타일 |
| `ahas-ux-nuxt-ui/` | Nuxt UI로 인터페이스 만들기. 컴포넌트와 레이아웃 레퍼런스 포함 |
| `ahas-video-code-rendered/` | 모든 프레임이 시간의 함수인 코드로 영상 만들기. 트리트먼트, 측정된 타이밍, 장면 모듈, 오프라인 렌더 포함 |
| `ahas-video-shotcraft/` | video-shotcraft 스튜디오(Remotion)로 영상 만들기. 설정부터 렌더, 라이선스 표기까지 |
| `bmo-asking/` | 잡이 스스로 정할 수 없는 것을 감독자에게 묻기, 오너에게 문서 제출하기 |
| `bmo-beyond-the-checkout/` | 작업이 자기 체크아웃 밖까지 닿을 때 잡이 책임지는 것 |
| `bmo-closing-a-job/` | 잡을 끝내는 방법: 마감, 남겨도 되는 것, 보고 |
| `bmo-code-korean-out/` | 코드 파일 묶음에서 한국어 빼내기 |
| `bmo-docs-conventions/` | 저장소에 새 정보를 어디에, 어떻게 기록하는지 |
| `bmo-docs-eli5/` | 주제를 그림으로, 최대한 쉽게 설명하기 |
| `bmo-open-source-audit/` | 공개 저장소에 절대 들어가면 안 되는 것과, 커밋·공개 전 검사 |
| `bmo-song-cover/` | 노래 벤치로 채팅 에이전트의 목소리로 노래 만들기. project 스킬 |
| `bmo-tools-researching-and-benching/` | 랩이 돌리는 것을 조사, 벤치마크, 튜닝하기 |
| `bmo-ux-wording/` | 콘솔 UI 용어 가이드의 위치, 용어 추가 방법, 사용처 고치는 방법 |
| `bmo-waiting-on-long-work/` | 몇 분 넘게 걸리는 작업을 백그라운드로 시작하고, 제한 있는 대기 한 번으로 기다리기 |
| `bmo-warm-prefix/` | 실행 중인 세션이 캐시된 프리픽스로 들고 있는 내용 바꾸기 |
| `src/` | `bmo-skills` 배치기: 스킬 읽기, 나열하기, 배치하기 |
| `tests/` | 배치기 테스트와, 스킬이 형식을 지키는지 보는 검사 |
| `docs/` | 형식, 경계, 인벤토리에 대한 정식 문서 |

루트에는 Cargo 매니페스트, 버전, 한국어 ledger를 곁들인 `testbed.toml`, `rustfmt.toml`, `LICENSE`, `CLAUDE.md`가
있습니다. 이 저장소는 MIT 라이선스입니다.

## 문서

| 문서 | 내용 |
|---|---|
| `docs/format.md` | 스킬 폴더의 형태, front matter와 번들, 스킬이 담을 수 있는 것 |
| `docs/boundary.md` | 배치기의 동사, 매니페스트, 어느 유닛이 어디서 어떤 스킬을 받는지, 각 스킬이 다른 유닛에 무엇을 호출하는지, 버전 규칙 |
| `docs/inventory.md` | 모든 파일의 출처와 위치, 다른 유닛에 넘긴 것, 이식한 테스트, 바뀐 모든 이름 |
| `docs/PERSONAL-DATA.md` | 저장소에 절대 넣지 않는 개인정보, 그 값을 대신 두는 곳, 허용되는 두 값, 그리고 이를 막는 검사 |

## 버전

- **저장소 전체에 버전 하나,** semver를 따릅니다. `VERSION`에 적혀 있고, 패키지 버전과 매니페스트 버전도 이와
  같습니다.
- **변경이 버전을 어떻게 올리는지**는 `docs/boundary.md`의 「Versions」에 있습니다.

## 명령

```sh
cargo run -- list
cargo run -- place --bundle worker --into <dir>
cargo run -- place --skill <name> --into <dir>
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
