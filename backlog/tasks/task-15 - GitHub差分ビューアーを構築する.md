---
id: TASK-15
title: GitHub差分ビューアーを構築する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - mvp
milestone: m-1
dependencies:
  - TASK-4
  - TASK-14
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0007-構造化された差分モデルを使う.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 15
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub由来のPR差分を構造化モデルへ変換し、WindowsネイティブUIで高速に閲覧できる差分ビューアーを構築する。フルソース閲覧、レビュー送信、ローカル差分は別タスクとする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 差分の出所とGitHubコメント位置メタデータを保持したファイル・ハンク・行モデルへ変換する
- [ ] #2 統合表示と分割表示、旧新行番号、追加・削除、構文ハイライトを表示する
- [ ] #3 ファイル・ハンク移動、空白非表示、変更のない領域の折りたたみ、ファイル移動表示に対応する
- [ ] #4 大規模差分、binary、truncated、rename、空ファイル、取得失敗をfixtureで検証する
- [ ] #5 キーボード操作、フォーカス、選択、コピー、スクロールの基本操作をWindows上で検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
