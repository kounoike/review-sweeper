---
id: doc-8
title: UI Design System and Review Workbench Guidelines
type: specification
created_date: '2026-08-29 03:41'
updated_date: '2026-08-29 03:44'
---
# UI Design System and Review Workbench Guidelines

## 1. 目的と位置づけ

この文書は Review Inbox、PR review workbench、Full Source に共通する UI token、状態表現、keyboard 操作、代表レイアウトを定義する。実装時は [Product & Design Principles](doc-2%20-%20Product-Design-Principles.md)、[Scope and Roadmap](doc-3%20-%20Scope-and-Roadmap.md)、ADR-0010 を併読する。

PR review workbench は、次の左右 3-pane を **MVP で試す暫定レイアウト** とする。これは最終仕様ではない。

- 左: file navigation
- 中央: Diff / Full Source
- 右: Checks / AI findings / Review Draft

後続実装と使用感の評価後に再決定する。別案として「左 navigation + 上部 summary + 折り畳み可能な下 pane」と、必要な情報を画面遷移で開く 2-pane を比較候補に残す。

試用可能な wireframe は [`prototypes/review-workbench.html`](../../../prototypes/review-workbench.html) に置く。静的な設計画像ではなく、theme、viewport、状態、右 pane tab、Command palette、代表画面を切り替えられる self-contained HTML である。

## 2. GPUI への適用境界

domain/application 層は `ReviewState`、`CheckState`、`FindingState`、command ID、selection/focus intent のような UI 非依存値を公開する。GPUI adapter は semantic token から `Hsla`、寸法、font、icon asset、animation を解決し、`uniform_list` 等の描画機構へ渡す。domain/application の公開 API に GPUI 固有型を出さない。

theme、font role、density は app-level settings として解決し、component が raw RGB、任意の font family、任意の pixel 値を直接持つことを避ける。大量 diff の virtualization、grapheme-aware selection/copy、IME、AccessKit metadata/action は GPUI adapter と custom diff surface の実装責務であり、この文書だけで実現済みとは扱わない。

## 3. UI token

### Color

component は palette 値ではなく semantic token を参照する。

| Token | Light | Dark | 用途 |
| --- | --- | --- | --- |
| `color.surface.canvas` | `#F4F6F8` | `#111418` | window 背景 |
| `color.surface.panel` | `#FFFFFF` | `#181D23` | pane / card |
| `color.surface.raised` | `#FFFFFF` | `#20262E` | menu / dialog |
| `color.text.primary` | `#1B2430` | `#EDF2F7` | 主本文 |
| `color.text.muted` | `#5E6B7A` | `#AAB4C0` | 補助情報 |
| `color.border.default` | `#D7DDE5` | `#343C46` | 境界 |
| `color.focus.ring` | `#0969DA` | `#58A6FF` | keyboard focus |
| `color.accent.primary` | `#0969DA` | `#58A6FF` | primary action |
| `color.diff.added.bg` | `#DAFBE1` | `#193B2A` | added line |
| `color.diff.deleted.bg` | `#FFEBE9` | `#462329` | deleted line |
| `color.state.success` | `#1A7F37` | `#3FB950` | success |
| `color.state.warning` | `#9A6700` | `#D29922` | warning / stale |
| `color.state.danger` | `#CF222E` | `#F85149` | failure / blocking |
| `color.state.info` | `#0969DA` | `#58A6FF` | active / running |

diff 背景上の text と syntax color は contrast を個別に検証する。色は意味の唯一の伝達手段にしない。

### Typography と font role

| Token | 初期値 | 用途 |
| --- | --- | --- |
| `font.role.ui` | Windows system UI (`Segoe UI` fallback) | chrome、label、本文 |
| `font.role.diff` | installed monospace (`Cascadia Mono` fallback) | diff / Full Source |
| `font.role.terminal` | installed monospace | terminal grid（将来） |
| `type.label.sm` | 12/16, 600 | badge、metadata |
| `type.body.md` | 14/20, 400 | 標準本文 |
| `type.body.strong` | 14/20, 600 | row title |
| `type.code.md` | 13/20, 400 | diff / source |
| `type.heading.lg` | 20/28, 650 | screen heading |

font role は ADR-0010 の installed system font、validation、last-known-good fallback に従う。

### Spacing、shape、border、elevation、motion

| 種別 | Token |
| --- | --- |
| spacing | `space.1=4`, `space.2=8`, `space.3=12`, `space.4=16`, `space.5=24`, `space.6=32` px |
| radius | `radius.sm=4`, `radius.md=6`, `radius.lg=10`, `radius.pill=999` px |
| border | `border.hairline=1`, `border.strong=2` px |
| elevation | `elevation.1=0 1px 2px / 12%`, `elevation.2=0 8px 24px / 18%` |
| motion | `motion.fast=100ms`, `motion.standard=160ms`, `motion.slow=240ms`; easing `cubic-bezier(.2,0,0,1)` |

OS の reduced-motion 指定時は位置移動と pulse を無効にし、opacity の即時切替を基本とする。レビューの進捗や error を animation だけで知らせない。

### Icon

`icon.xs=12`、`icon.sm=16`、`icon.md=20`、`icon.lg=24` px。icon は stroke 1.5–2 px の同一 family を使用し、意味のある icon には accessible name または隣接 label を持たせる。状態 icon は下表に従い、装飾 icon は accessibility tree から除外する。

## 4. Theme 規約

初期値は `system`。設定値は `system | light | dark` の3値を保存し、`system` の resolved theme は Windows の app theme 変更を監視して即時更新する。ユーザーが明示選択した `light` / `dark` は OS 変更で上書きしない。起動時は保存設定を先に読み、失敗時は `system` へ戻す。切替時に window を再生成せず、semantic token を app 全体へ再解決する。

theme にかかわらず text、focus ring、selected row、diff syntax、semantic state の contrast を確認する。high-contrast mode は system color を優先する別 adapter を後続実装で検証し、light/dark palette の単純反転で代用しない。

## 5. Semantic state 表

状態は **色 + icon + label + shape** を併用する。shape は badge の outline/filled/striped とし、同色や grayscale でも区別できるようにする。

| 対象 / 状態 | 色 | Icon | Label | Shape / 補助表現 |
| --- | --- | --- | --- | --- |
| remote review: needs review | info | `eye` | Needs review | filled pill |
| remote review: reviewed | success | `check` | Reviewed | outline pill |
| remote review: updated | warning | `history` | Updated since review | striped pill |
| local workspace: clean | success | `check-circle` | Local clean | outline rounded rect |
| local workspace: modified | warning | `file-edit` | Local modified | left-bar rounded rect + count |
| local workspace: conflict/diverged | danger | `split` | Conflict / HEAD diverged | strong outline + count |
| background work: queued | muted | `clock` | Queued | outline pill |
| background work: running | info | `spinner` | Preparing | filled pill + optional motion |
| background work: partial | warning | `circle-dashed` | Partial | striped pill + detail |
| background work: failed | danger | `x-circle` | Failed | strong outline + Retry |
| check: success | success | `check-circle` | Passed | outline row marker |
| check: pending | info | `clock` | Pending | filled row marker |
| check: skipped/cancelled | muted | `minus-circle` | Skipped / Cancelled | dashed row marker |
| check: failure | danger | `x-circle` | Failed | strong row marker + Open |
| finding: open | warning | `sparkle-alert` | AI finding | filled corner marker |
| finding: dismissed | muted | `eye-off` | Dismissed | dashed card |
| finding: draft | info | `message-plus` | Added to draft | left-bar card |
| finding: resolved | success | `check` | Resolved | outline card |

remote PR state と local workspace state は別の group/heading に置き、合成した単一の「clean」表示を作らない。AI finding は AI 由来である label を保持し、人間の review result と誤認させない。

## 6. Keyboard、Command、focus、selection

すべての代表操作を安定した command ID にし、shortcut、Command palette、menu、context menu、button は同じ command を発火する。MVP shortcut は Windows 標準に合わせる。

| Command | Shortcut | 挙動 |
| --- | --- | --- |
| `app.commandPalette` | `Ctrl+Shift+P` | command 検索 |
| `app.find` | `Ctrl+F` | active Diff / Source 内検索 |
| `app.settings` | `Ctrl+,` | settings |
| `nav.nextFile` / `nav.previousFile` | `Alt+Down` / `Alt+Up` | file 間移動 |
| `nav.nextHunk` / `nav.previousHunk` | `F8` / `Shift+F8` | hunk 間移動 |
| `view.toggleDiffSource` | `Ctrl+Enter` | Diff / Full Source 切替 |
| `review.toggleReviewed` | `Alt+R` | active file の reviewed 切替 |
| `review.addComment` | `Ctrl+Alt+M` | selection / line に draft comment |
| `view.toggleSidePanel` | `Ctrl+J` | 補助 pane の開閉 |
| `nav.back` | `Alt+Left` | Review Inbox / 直前位置へ戻る |
| `app.close` | `Alt+F4` | Windows 標準の終了 |

`Tab` / `Shift+Tab` は focus order、arrow key は現在の composite widget 内移動に使う。pane 間移動の追加 shortcut は使用テスト後に決める。focus ring は `color.focus.ring` の 2 px outer ring + 2 px offset とし、keyboard focus で常に表示する。mouse click 後も高コントラスト設定や入力対象では消さない。

row selection、text selection、diff range selection を混同しない。row は `Space` で選択、`Enter` で open。code surface は通常の text drag と `Shift+Arrow` を提供し、line gutter で review range を選ぶ。selection は copy 可能で、選択された line/range を icon または gutter shape でも示す。focus 移動で draft selection を暗黙破棄せず、破棄が必要なら確認する。

accessibility では pane/region heading、row position、selected/expanded/current、state label、shortcut、loading progress、error と retry action を expose する。focus order は視覚順と一致させ、virtualized row が画面外へ移動しても論理 focus と読み上げ位置を維持する。GPUI の AccessKit 制約は ADR-0010 の撤回条件として実機検証する。

## 7. Loading / empty / stale / partial / error

| 状態 | 表示 | 操作 |
| --- | --- | --- |
| loading | 対象領域内 skeleton + `Loading …` label。既存 content があれば保持 | Cancel が安全な処理のみ表示 |
| empty | 理由と次の一手を1文で表示 | Refresh / filter reset 等 |
| stale | 最終更新時刻と stale label。cached content は閲覧可能 | Refresh |
| partial | 利用可能な content を表示し、欠落領域と原因を明記 | Retry failed part |
| error | 失敗した範囲内に error summary、detail、再試行 | Retry / Copy details / Open GitHub 等 |
| cancelling | 現在処理と `Cancelling…` を表示 | 多重 cancel を無効化 |

一部障害で screen 全体を置換しない。たとえば Checks 取得失敗時も Diff と draft を操作可能にする。retry は同じ focus context を保ち、成功・再失敗を live region で通知する。

## 8. 代表画面

### Review Inbox

高密度 row を基本に repository、title、author、更新時刻、remote review、preparation、Checks、local state を列または折返し group で示す。remote と local を視覚的に分離する。loading、empty、partial/error は list 全体ではなく該当 group へ適用する。

### PR review workbench（MVP 暫定案 A）

左 240 px（最小 184）、中央 `minmax(480 px, 1fr)`、右 320 px（最小 280）を初期値とする。中央を最優先し、resize handle を keyboard でも操作できるようにする。右 pane は Checks / AI findings / Review Draft の tabs。中央は Diff / Full Source を切り替えて同じ file/line context を維持する。

狭幅では 3-pane を圧縮し続けない。960 px 未満で左・右を overlay/drawer 化し、中央を全幅にする。720 px 未満では補助 pane を一度に1つだけ表示し、toolbar は overflow menu へ送る。現在開いている pane、未読/失敗件数、閉じ方を label 付き control で示す。

### Full Source

中央 surface を再利用し、breadcrumb、現在 revision（PR HEAD / local）、Diff 位置へ戻る command、find、selection/copy、external editor 操作を持つ。local source を表示するときは persistent banner と icon/label/shape を使い、remote PR source と混同させない。

## 9. 未確定事項、代替案、再評価条件

承認済みなのは、(1) A の左右 3-pane を MVP の暫定レイアウトとして試すこと、(2) Windows 標準 shortcut + Command palette、(3) system theme 初期値、(4) 色 + icon + label + shape の状態表現である。3-pane の最終採用、pane 幅、breakpoint、tab 構成、pane 間 shortcut は未確定である。

| 候補 | 長所 | Tradeoff |
| --- | --- | --- |
| A: 左 file / 中央 Diff・Source / 右 Checks・AI・Draft | context を同時に比較でき、遷移が少ない | 中央 diff 幅を圧迫し、狭幅で密度が高い |
| D: 左 navigation / 上 summary / 折り畳み下 pane | 横幅を diff に割り当てやすい | 縦方向の code 表示量と上下移動にコスト |
| B: 左 file / 中央 Diff + 補助画面へ遷移 | diff の可読幅を最大化しやすい | Checks/finding/draft の往復と文脈復帰が増える |

後続実装では同一の代表レビューを各候補で試し、次を記録する。

- diff 可読性: 典型幅 1440/1280/1024/768 px で code 列が horizontal scroll なしに読める割合、wrap/side-by-side の破綻、中央 pane の実効幅
- 画面遷移量: 1 file の diff を読み、check failure と finding を確認し、draft comment を追加するまでの pane/tab/screen 切替回数、context 復帰失敗
- 狭幅時の操作性: 1024/768 px で target 到達までの操作数、overlay の閉じ込め、keyboard focus の復帰、主要操作の overflow 化
- qualitative: 初見ユーザーの pane 意味理解、見落とし、疲労、Diff と補助情報の比較しやすさ

再評価は TASK-14〜18 の代表データが接続され、少なくとも keyboard-only と mouse の両方で上記シナリオを試せる時点、または中央 pane が 640 px 未満になる主要環境で diff 読解を阻害した時点で行う。A を維持する場合も計測結果と tradeoff を TASK-6 の後続記録または TASK-26 の MVP 評価へ残す。数値 threshold の最終値や別案への変更は、使用感の証拠なしにこの文書だけで固定しない。

## 10. 実装参照 checklist

- raw color / spacing ではなく token を使用する。
- remote、local、background、finding/check の state group を混在させない。
- icon 単独または色単独で状態を伝えない。
- command は複数 surface から同じ ID を発火する。
- focus、selection、loading/error、partial failure を happy path と同時に実装・検証する。
- 3-pane は暫定案として instrumentation と responsive fallback を持たせ、再評価可能にする。
