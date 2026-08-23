---
id: TASK-24
title: Windows向け外部エディター連携を実装する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:51'
labels:
  - external-tools
  - mvp
milestone: m-1
dependencies:
  - TASK-6
  - TASK-19
  - TASK-13
  - TASK-16
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 24
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
WindowsのReview Sweeperから選択中worktree、ファイル、行をVS Codeまたは設定済みカスタムコマンドで開けるようにする。AI coding agent連携と高度なlauncher profileはMVP後の別タスクとする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 VS Codeとカスタムコマンドのlauncher profileを設定・検証できる
- [ ] #2 Worktree、ファイル、行をWindowsネイティブ実行経路で開ける
- [ ] #3 カスタムコマンドでworkspace、file、line、columnのプレースホルダーを安全に展開できる
- [ ] #4 worktreeパスとcdコマンドをコピーできる
- [ ] #5 未対応、実行ファイル不在、不正なテンプレート、起動失敗をユーザーが解決できる形で通知する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
