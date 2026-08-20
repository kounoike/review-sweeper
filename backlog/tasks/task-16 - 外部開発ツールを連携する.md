---
id: task-16
title: 外部開発ツールとAIエージェントを連携する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-20 20:57'
labels:
  - external-tools
milestone: m-1
dependencies:
  - TASK-5
  - TASK-12
type: feature
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ユーザーが好みのエディター、シェル、AIエージェントでレビューworkspaceとソース位置を開けるようにし、起動動作と連携範囲を設定可能にする。外部ツール連携の詳細仕様はこのタスクで定義する。AIエージェントによるレビュー連携の初期候補はOpenAI Codex、Claude Code、OpenCodeとし、各ツールを同じ抽象化で扱えるか、必要な起動方法・権限・結果の受け渡し・Worktree境界を調査して決める。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 VS Code、VS Code Insiders、Cursor、Zed、JetBrains IDE、ファイルマネージャー、シェル、OpenAI Codex、Claude Code、OpenCode、カスタムコマンドのランチャープロファイル候補と対応範囲を定義する。
- [ ] #2 AIエージェントによるレビューについて、起動対象Worktree、読み取り専用・コマンド実行の権限境界、入力情報、結果の受け渡し、ユーザーによる確認範囲を定義する。
- [ ] #3 カスタムランチャーコマンドで`{workspace}`、`{file}`、`{line}`、`{column}`などのプレースホルダーに対応する。
- [ ] #4 worktreeパスと`cd`コマンドをコピーする操作を提供する。
- [ ] #5 選定した連携方式をWindows初期ターゲットで検証し、未対応または実行できない場合のエラーと代替手段を定義する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
