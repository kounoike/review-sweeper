---
id: TASK-23
title: Preparation schedulerと手動操作を実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - preparation
  - mvp
milestone: m-1
dependencies:
  - TASK-21
  - TASK-22
references:
  - adrs/0004-PR準備とレビューセッションを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 23
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
複数PRのPreparationをリソース制限付きで実行し、ユーザーが必要な段階を手動で開始・再試行・優先できるようにする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 worktree準備とAIレビューに独立した同時実行上限を設定できる
- [ ] #2 ユーザーが開いたPRまたは明示的に準備したPRを待機中ジョブより優先する
- [ ] #3 差分取得、worktree作成、setup、AIレビュー、全件準備、次PR準備を個別に開始・再試行できる
- [ ] #4 キャンセル、アプリ再起動、キュー重複、部分失敗でも同じ処理を無秩序に多重実行しない
- [ ] #5 キュー、実行中段階、待機理由、失敗理由をユーザーへ表示する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
