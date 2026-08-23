---
id: TASK-28
title: 外部AI coding agent連携を設計・実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - later
  - external-tools
  - ai-review
milestone: m-3
dependencies:
  - TASK-24
  - TASK-26
  - TASK-13
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
  - adrs/0009-AIレビューは読み取り専用とし人間が送信する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: enhancement
ordinal: 28
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MVPの読み取り専用AIレビューとは分離し、OpenAI Codex、Claude Code、OpenCodeなどの外部coding agentへレビューworkspaceを渡す連携を、権限・Worktree・実行バックエンドの境界を明示して追加する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 対象ツールの起動方式、入力、結果受け渡し、対応OS・backend、失敗時の代替を比較して対応範囲を決める
- [ ] #2 起動対象Worktreeと実行バックエンドを表示し、別Worktreeや別backendを暗黙に操作しない
- [ ] #3 読み取り専用、コマンド実行、ソース変更の権限を区別し、ユーザーが起動前に確認できる
- [ ] #4 AI出力を自動でGitHubへ投稿・送信せず、Review Sweeper側で確認できる形へ受け渡す
- [ ] #5 各対応ツールの未導入、認証不足、起動失敗、キャンセルをWindows初期ターゲットで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
