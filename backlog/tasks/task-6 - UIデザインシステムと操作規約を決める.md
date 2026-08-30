---
id: TASK-6
title: UIデザインシステムと操作規約を決める
status: Done
assignee:
  - '@kounoike'
created_date: '2026-08-20 18:06'
updated_date: '2026-08-29 03:47'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-1
documentation:
  - doc-3
  - doc-8
modified_files:
  - >-
    backlog/docs/product/doc-8 -
    UI-Design-System-and-Review-Workbench-Guidelines.md
  - prototypes/review-workbench.html
  - backlog/tasks/task-6 - UIデザインシステムと操作規約を決める.md
type: spike
ordinal: 6
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
レビュー画面全体で視覚と操作を統一するため、色、タイポグラフィ、余白、状態表示、コンポーネント、キーボード操作、テーマ切り替えのルールを定義する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Review Inbox、PR概要、Diff、Full Source、Checks、AI findingで共通利用する命名済みUI token（color、typography、spacing、radius、border、elevation、motion、icon、font role）とGPUIへの適用境界を文書化する。
- [x] #2 light・dark・system themeの解決・切替規約と、remote review・local workspace・background work・finding/checkの各状態を色だけに依存せずicon・label・shapeを併用して識別するsemantic state表を文書化する。
- [x] #3 keyboard-firstのcommand/shortcut体系、focus移動とfocus ring、text/row/range selection、loading/empty/stale/partial/error/retry/cancelの共通操作規約を、代表操作とaccessibility要件を含めて文書化する。
- [x] #4 Review Inbox、PR review workbench（Diff・file navigation・Checks・AI finding・Review Draftを含む）、Full Sourceの代表画面をrepo-nativeなwireframe/prototypeで作成し、light/dark、keyboard focus、loading/error、local state分離の例を実装タスクから参照できる状態にする。
- [x] #5 重要UX判断について選択肢・推奨案・tradeoffを提示してユーザー承認を得て、承認結果と未確定事項をTASK-6および設計文書へ記録する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 テストと該当するチェックが通る
- [x] #2 文書が更新されている
- [x] #3 リグレッションがない
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Product Vision、Design Principles、Scope/Roadmap、Accepted ADR-0010とGPUI技術検証から、MVP画面・状態・framework制約を抽出する。
2. 情報密度、ナビゲーション構造、shortcut方針、状態表現の重要UX選択肢を比較し、推奨案とtradeoffをcoordinator経由でユーザー承認にかける。
3. 承認結果に沿って、共通token、theme、semantic state、keyboard/focus/selection/loading/error規約をBacklogの製品設計文書として作成する。
4. 同じ規約を使うself-contained HTML/CSSの代表画面wireframeを作成し、light/darkおよび主要状態を静的に確認できるようにする。
5. 文書・prototypeの内部整合とリンクを確認し、git diff --check、backlog-check、adr-doctorおよびHTMLの構文確認を実行する。
6. task-finalization手順に従い、客観的証拠をnotes/final summaryへ記録し、AC/DoDを更新してTASK-6関連変更だけをcommitする。
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-08-29 着手確認: git status --shortはclean。TASK-1はDone、ADR-0010はAcceptedでGPUI採用済み。adr-list/adr-doctorは成功。Product Vision、Product & Design Principles、Scope and Roadmapを確認し、TASK-6以外へスコープを拡張しない。

2026-08-29 UX判断待ち: orchestration question msg_038f652d639d で、A（推奨: Review Inbox独立＋高密度3-pane workbench、Windows標準shortcut＋Command palette、system theme初期値、色+icon+label+shapeの状態表現）、B（2-pane＋画面遷移）、C（低密度document中心）を提示した。初回10分とresume後10分の計20分で回答がなく、重要UXを未承認で確定しないため実装を停止した。

2026-08-29 ユーザー承認: A（左=file navigation、中央=Diff/Full Source、右=Checks/AI findings/Review Draft）をMVPで試す暫定レイアウトとして採用するが、最終仕様には固定しない。Windows標準shortcut＋Command palette、system theme初期値、色＋icon＋label＋shapeの状態表現も承認。代替案として左navigation＋上summary＋折り畳み可能な下pane、および2-pane＋補助画面遷移を残し、後続実装でdiff可読性、画面遷移量、狭幅時の操作性を評価して再決定する。

2026-08-29 検証: git diff --check、mise run backlog-check、mise run adr-doctor、NodeによるHTML structure/JavaScript構文確認、設計文書からprototypeへの参照ファイル確認がすべて成功。Orca内蔵browser（http://127.0.0.1:4176/prototypes/review-workbench.html）でtheme light→dark、Ctrl+Shift+P palette open/Escape close、compact幅＋右pane、AI finding error、Full Source＋local bannerをDOM結果で確認し、screenshot取得にも成功した。accessibility snapshotは2回runtime_unavailableだったため、後続GPUI実装でのAccessKit/keyboard実機検証は設計文書記載どおり必要。
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
UI tokenとGPUI適用境界、system/light/dark theme、色＋icon＋label＋shapeのsemantic state、Windows標準shortcut＋Command palette、focus/selection/loading/error/accessibility規約をdoc-8に定義した。Review Inbox、MVP暫定3-pane workbench、Full Sourceを操作可能なself-contained HTML prototypeとして追加し、代替layoutとdiff可読性・遷移量・狭幅操作性による再評価条件を明記した。git diff --check、backlog-check、adr-doctor、HTML/JavaScript構文・参照確認、Orca browserでの代表操作とscreenshot取得により検証した。push/PR/mergeは依頼どおり未実施であり、タスク全体はPR作成待ち。
<!-- SECTION:FINAL_SUMMARY:END -->
