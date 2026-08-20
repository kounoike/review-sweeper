---
id: TASK-18
title: Windows向けWSL実行バックエンドを追加する
status: To Do
assignee: []
created_date: '2026-08-20 19:06'
labels:
  - later
  - external-tools
  - worktree
dependencies:
  - TASK-17
references:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
  - backlog/docs/product/doc-2 - Product-Design-Principles.md
type: feature
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MVP後のできるだけ早い段階で、WindowsネイティブGUIからWSLを実行バックエンドとして利用できるようにする。WSLは必須依存にはせず、Windowsネイティブ実行を基本経路として残す。Worktree単位で実行バックエンドを選択・表示でき、gitやworkspace setupなどのコマンド実行を一貫した環境で行える状態を目指す。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 WSL2と利用可能なLinuxディストリビューションを検出し、利用可否とエラー理由をユーザーに表示できる
- [ ] #2 Windowsネイティブ実行とWSL実行をWorktree単位のバックエンドとして選択できる
- [ ] #3 git、Worktree操作、workspace setupの実行経路とパス・環境変数の扱いが明示され、Windows側とWSL側の状態を暗黙に混在させない
- [ ] #4 WSL未導入でも、Review Inbox、Diff確認、レビューコメント作成・送信などの基本レビュー機能を利用できる
- [ ] #5 WSL実行の失敗、ディストリビューション未選択、パス変換や権限の問題をユーザーが解決できる形で通知する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
