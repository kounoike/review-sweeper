---
id: TASK-27
title: Windows向けWSL2実行バックエンドを追加する
status: To Do
assignee: []
created_date: '2026-08-20 19:06'
updated_date: '2026-08-23 00:51'
labels:
  - later
  - external-tools
  - worktree
milestone: m-3
dependencies:
  - TASK-26
  - TASK-13
  - TASK-20
  - TASK-22
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 27
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0002に基づき、MVP後のできるだけ早い段階で、WindowsネイティブGUIから任意のWSL2ディストリビューションを選択可能な実行バックエンドとして追加する。WSLは必須依存にせず、Windowsネイティブ経路を維持する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 WSL2と利用可能なLinuxディストリビューションを検出し、利用可否とエラー理由を表示する
- [ ] #2 Worktree作成時にWindowsネイティブまたはWSL2ディストリビューションを選び、そのWorktreeへ固定・表示する
- [ ] #3 既存Worktreeのbackend変更は暗黙に行わず、必要な再作成または明示的移行を安全に扱う
- [ ] #4 git、Worktree操作、workspace setupを選択したbackendだけで実行し、型付きパス・環境変数・Git状態をWindows側と混在させない
- [ ] #5 WSL未導入でもReview Inbox、差分確認、コメント下書き・送信を利用できる
- [ ] #6 ディストリビューション未選択、パス変換、権限、プロセス失敗をユーザーが解決できる形で通知する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
