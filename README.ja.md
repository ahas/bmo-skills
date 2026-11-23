[English](README.md) | [한국어](README.ko.md) | **日本語** | [简体中文](README.zh-CN.md)

# bmo-skills

ジョブワーカーに渡すスキル群と、それを配置するプレーサー(placer)です。スキルは一つにつき一フォルダで、フォルダ名は
スコープ(`bmo` または `ahas`)と分類で始まります。各スキルは `SKILL.md` とそこから参照されるファイルで構成され、
front matter にどのバンドルに属するかを記します。バンドルは、一般のジョブワーカー全員が受け取る `worker`、コーダーの
コーディング・設計の実行が受け取る `coder`、プロデューサーの映像制作の実行が受け取る `producer`、そして一つの
リポジトリのチェックアウトが独自に持つスキルである `project` です。このリポジトリ自体は何も実行しません。利用側の
デプロイが `bmo-skills place` を実行すると、ワーカープラグインを Claude Code の標準プラグイン構成で、あるいは
スキル一つを専用フォルダとして、利用側のツリーに書き出します。

## 構成

| フォルダ | 内容 |
|---|---|
| `.claude-plugin/` | 配置されるすべてのバンドルの書き出し先となる `bmo-worker` プラグインのマニフェスト |
| `ahas-code-boundaries/` | バリエーションをどこに置くか、人が編集するリストをどこに置くか、基盤ライブラリへの貢献のしかた |
| `ahas-code-clean/` | リファクタリングの行単位のパス。原則と Rust・TypeScript のノート付き |
| `ahas-code-design-patterns/` | リファクタリングの構造のパス。パターンカタログ付き |
| `ahas-code-refactoring/` | リファクタリングのモジュール・関数のパス。カタログとコードの臭い付き |
| `ahas-code-test-writing/` | テストを書く際のルール |
| `ahas-docs-git/` | git のワークフロー、ドキュメント、変更履歴のルール |
| `ahas-tools-find-skills/` | 公開スキルの中からタスクに合うスキルを探す |
| `ahas-tools-playwright-cli/` | ブラウザの操作と Playwright テストの作成。リファレンス付き |
| `ahas-ux-design-rules/` | あらゆるユーザーインターフェースのための、領域別のデザインルール |
| `ahas-ux-nuxt-style/` | Node.js、TypeScript、Vue または Nuxt プロジェクトのコーディングスタイル |
| `ahas-ux-nuxt-ui/` | Nuxt UI でのインターフェース構築。コンポーネントとレイアウトのリファレンス付き |
| `ahas-video-code-rendered/` | すべてのフレームを時間の関数とするコードで映像を作る。トリートメント、計測したタイミング、シーンモジュール、オフラインレンダー付き |
| `ahas-video-shotcraft/` | video-shotcraft スタジオ(Remotion)で映像を作る。セットアップからレンダー、ライセンス表記まで |
| `bmo-asking/` | ジョブが自分で決められないことをスーパーバイザーに尋ね、オーナーにドキュメントを提出する |
| `bmo-beyond-the-checkout/` | 作業が自分のチェックアウトの外に及ぶとき、ジョブが責任を持つこと |
| `bmo-closing-a-job/` | ジョブの終え方: クローズ、残してよいもの、レポート |
| `bmo-code-korean-out/` | コードファイル一式から韓国語を取り除く |
| `bmo-docs-conventions/` | リポジトリに新しい情報をどこに、どう記録するか |
| `bmo-docs-eli5/` | トピックを図で、できるかぎり簡単に説明する |
| `bmo-docs-plain-writing/` | 人が読むあらゆる文章を、機械が書いたように読まれないように書く |
| `bmo-open-source-audit/` | 公開リポジトリに決して含めてはならないものと、コミット・公開前のチェック |
| `bmo-research-deep-dives/` | ディープダイブが問いを立て、作り手の資料を先に読み、ページのレビューを受ける方法 |
| `bmo-research-economy-brief/` | 一日の経済ニュースと数値のブリーフを書く |
| `bmo-research-investment/` | お金をどこに置くかに答える：戦略、銘柄、エントリーとエグジット |
| `bmo-research-kinds/` | あらゆる検索が質問の種類ごとに確かめ添えるものと、結果の形 |
| `bmo-research-products/` | 買う物を探す：販売先の順序、製品ラインのニュース、在庫、本当の割引、海外の実質価格 |
| `bmo-research-references/` | 描画のための参考画像を資料集にまとめる |
| `bmo-research-restaurants/` | 食べる場所や出前を勧める。尋ねた人自身のルール、なければ既定のルールで |
| `bmo-research-routine/` | 誰かが作ったものを調べ、測り、報告する |
| `bmo-research-security-advisories/` | 脆弱性やセキュリティ勧告に答える：影響、修正、パッチ手順 |
| `bmo-research-study-material/` | 主題を教える学習資料を書く |
| `bmo-tools-researching-and-benching/` | ラボで動かしているものの調査、ベンチマーク、チューニング |
| `bmo-ux-wording/` | コンソールの UI 用語ガイドの置き場所、用語の追加方法、使用箇所の直し方 |
| `bmo-waiting-on-long-work/` | 数分を超える作業をバックグラウンドで始め、上限付きの一度の待機で待つ |
| `bmo-warm-prefix/` | 実行中のセッションがキャッシュ済みプレフィックスとして保持する内容を変える |
| `src/` | `bmo-skills` プレーサー: スキルの読み込み、一覧、配置 |
| `tests/` | プレーサーのテストと、スキルが形式を守っているかのチェック |
| `docs/` | 形式、境界、インベントリについての正式なドキュメント |

ルートには Cargo マニフェスト、バージョン、韓国語の ledger を伴う `testbed.toml`、`rustfmt.toml`、`LICENSE`、
`CLAUDE.md` があります。このリポジトリは MIT ライセンスです。

## ドキュメント

| ドキュメント | 内容 |
|---|---|
| `docs/format.md` | スキルフォルダの形、front matter とバンドル、スキルが含めてよいもの |
| `docs/boundary.md` | プレーサーの動詞、マニフェスト、どのユニットがどこからどのスキルを受け取るか、各スキルが他のユニットに何を呼び出すか、バージョンのルール |
| `docs/inventory.md` | すべてのファイルの出所と置き場所、他のユニットに任せたもの、移植したテスト、置き換えたすべての名前 |
| `docs/PERSONAL-DATA.md` | リポジトリに決して入れない個人データ、代わりに値を置く場所、許可された二つの値、そしてそれを防ぐチェック |

## バージョン

- **リポジトリ全体で一つのバージョン**を semver で管理します。`VERSION` に記載され、パッケージとマニフェストの
  バージョンもこれと同じです。
- **変更によってバージョンがどう上がるか**は `docs/boundary.md` の「Versions」に記載しています。

## コマンド

```sh
cargo run -- list
cargo run -- place --bundle worker --into <dir>
cargo run -- place --skill <name> --into <dir>
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
