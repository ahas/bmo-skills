[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文**

# bmo-skills

分发给作业工作者（job worker）的技能，以及负责交付它们的放置器（placer）。每个技能一个目录，目录名以作用域（`bmo`
或 `ahas`）和类别开头。每个技能由一个 `SKILL.md` 及其引用的文件组成，front matter 中注明它属于哪个包：所有通用
作业工作者都会拿到的 `worker`，编码者的编码与设计运行拿到的 `coder`，制作者的视频运行拿到的 `producer`，以及某个
仓库的检出自行保留的技能 `project`。本仓库本身不运行任何东西：由使用方的部署执行 `bmo-skills place`，把工作者插件
按 Claude Code 的标准插件布局，或者把单个技能放进它自己的目录，写入使用方的目录树。

## 布局

| 目录 | 内容 |
|---|---|
| `.claude-plugin/` | `bmo-worker` 插件的清单，所有放置的包都写成这个插件 |
| `ahas-code-boundaries/` | 一个变化点应归属何处、人工编辑的列表放在哪里，以及如何向基础库贡献 |
| `ahas-code-clean/` | 重构中逐行处理的一轮，附原则以及 Rust 和 TypeScript 笔记 |
| `ahas-code-design-patterns/` | 重构中结构层面的一轮，附模式目录 |
| `ahas-code-refactoring/` | 重构中模块与函数层面的一轮，附重构目录和代码坏味道 |
| `ahas-code-test-writing/` | 编写测试的规则 |
| `ahas-docs-git/` | git 工作流、文档和变更日志的规则 |
| `ahas-tools-find-skills/` | 在公开技能中为任务寻找合适的技能 |
| `ahas-tools-playwright-cli/` | 操作浏览器并编写 Playwright 测试，附参考资料 |
| `ahas-ux-design-rules/` | 适用于所有用户界面的设计规则，按领域划分 |
| `ahas-ux-nuxt-style/` | Node.js、TypeScript 以及 Vue 或 Nuxt 项目的编码风格 |
| `ahas-ux-nuxt-ui/` | 用 Nuxt UI 构建界面，附组件和布局参考 |
| `ahas-video-code-rendered/` | 用代码制作视频，每一帧都是时间的函数，包括创意方案、实测时间、场景模块和离线渲染 |
| `ahas-video-shotcraft/` | 用 video-shotcraft 工作室（Remotion）制作视频，从搭建到渲染以及许可声明 |
| `bmo-asking/` | 向主管询问作业自己无法决定的事项，以及为所有者提交文档 |
| `bmo-beyond-the-checkout/` | 当工作超出自身检出范围时，作业需要负责的事项 |
| `bmo-closing-a-job/` | 作业如何结束：收尾、可以留下什么，以及报告 |
| `bmo-code-korean-out/` | 从一批代码文件中移除韩语 |
| `bmo-docs-conventions/` | 新信息应在仓库中记录在哪里、如何记录 |
| `bmo-docs-eli5/` | 用图示把一个主题讲得尽可能简单 |
| `bmo-docs-plain-writing/` | 写任何给人读的文字，不让它读起来像机器写的 |
| `bmo-open-source-audit/` | 公开仓库绝不能包含的内容，以及提交或发布前的检查 |
| `bmo-research-deep-dives/` | 深度调研如何提出问题、先读制作方的资料，以及页面如何被审阅 |
| `bmo-research-economy-brief/` | 撰写一天的经济新闻与数据简报 |
| `bmo-research-investment/` | 回答钱该放在哪里：策略、选股、进场与离场 |
| `bmo-research-kinds/` | 任何搜索按问题类型核查和附上的内容，以及结果的形状 |
| `bmo-research-products/` | 找要买的东西：卖家顺序、产品线动态、库存、真实折扣与海外到手价 |
| `bmo-research-references/` | 为一幅画收集参考图片，整理成资料集 |
| `bmo-research-restaurants/` | 推荐吃饭的地方或外卖，按提问者自己的规则，没有就按默认规则 |
| `bmo-research-routine/` | 调研、测量并报告别人做出的东西 |
| `bmo-research-security-advisories/` | 回答漏洞或安全公告：暴露情况、修复与补丁步骤 |
| `bmo-research-study-material/` | 撰写教授一个主题的学习资料 |
| `bmo-song-cover/` | 用歌曲工作台以聊天智能体的声音制作歌曲；一个 project 技能 |
| `bmo-tools-researching-and-benching/` | 调研、基准测试或调优实验室运行的东西 |
| `bmo-ux-wording/` | 控制台 UI 用语指南放在哪里、如何添加术语以及如何修正用法 |
| `bmo-waiting-on-long-work/` | 在后台启动超过几分钟的工作，并用一次有上限的等待来等它完成 |
| `bmo-warm-prefix/` | 修改运行中的会话作为缓存前缀保留的内容 |
| `src/` | `bmo-skills` 放置器：读取、列出和放置技能 |
| `tests/` | 放置器的测试，以及检查技能是否符合格式的测试 |
| `docs/` | 关于格式、边界和清单的权威文档 |

根目录下有 Cargo 清单、版本文件、附带韩语 ledger 的 `testbed.toml`、`rustfmt.toml`、`LICENSE` 和 `CLAUDE.md`。
本仓库采用 MIT 许可证。

## 文档

| 文档 | 说明 |
|---|---|
| `docs/format.md` | 技能目录的结构、front matter 与所属包，以及技能可以包含的内容 |
| `docs/boundary.md` | 放置器的动词、清单、哪个单元从哪里接收哪些技能、每个技能会调用其他单元的什么，以及版本规则 |
| `docs/inventory.md` | 每个文件的来源和位置、交给其他单元的内容、移植的测试，以及所有被替换的名称 |
| `docs/PERSONAL-DATA.md` | 哪些个人数据绝不进入仓库、相应的值改放在哪里、允许的两个值，以及阻止它们进入的检查 |

## 版本

- **整个仓库只有一个版本号，** 遵循 semver。它写在 `VERSION` 中，包的版本和清单的版本与之相同。
- **一次改动如何推进版本**，见 `docs/boundary.md` 的「Versions」一节。

## 命令

```sh
cargo run -- list
cargo run -- place --bundle worker --into <dir>
cargo run -- place --skill <name> --into <dir>
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
