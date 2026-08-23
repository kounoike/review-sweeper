---
id: TASK-21
title: PR Preparation状態とパイプラインを実装する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:50'
labels:
  - preparation
  - mvp
milestone: m-1
dependencies:
  - TASK-9
  - TASK-19
  - TASK-14
  - TASK-13
references:
  - adrs/0004-PR準備とレビューセッションを分離する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 21
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
レビューセッションとは独立したPreparation状態をモデル化し、Minimal、Workspace、AI Review、Fullの各モードを同じパイプラインで追跡する。信頼が必要なworkspace setup、scheduler、AI実行は別タスクから接続する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PRごとにレビュー進捗とは独立したPreparation状態、段階、進捗、失敗、再試行を保持する
- [ ] #2 Minimal、Workspace、AI Review、Fullの各モードが必要な段階を宣言できる
- [ ] #3 メタデータ・GitHub差分取得とworktree作成・更新を、準備段階として実行・再開できる
- [ ] #4 深い準備が失敗または進行中でもGitHubデータだけでレビューを開始できる
- [ ] #5 アプリ再起動、キャンセル、段階失敗、再試行でレビューセッション状態を破壊しないことをテストする
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
