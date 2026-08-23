---
id: TASK-16
title: フルソース閲覧と差分位置の相互移動を実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - mvp
milestone: m-1
dependencies:
  - TASK-14
  - TASK-15
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 16
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
PR HEADのファイル全体をWindowsネイティブUIで閲覧し、差分位置とソース位置を往復できるようにする。ソース編集は対象外とする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PR HEADのファイル全体を行番号と構文ハイライト付きで表示する
- [ ] #2 ファイル内検索、テキスト選択、コピーに対応する
- [ ] #3 差分の行から対応するフルソース位置へ移動し、元の差分位置へ戻れる
- [ ] #4 大規模ファイル、binary、削除済みファイル、取得失敗を扱う
- [ ] #5 ソース編集、LSP、デバッガーを実装範囲へ含めない
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
