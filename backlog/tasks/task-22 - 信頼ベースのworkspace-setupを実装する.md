---
id: TASK-22
title: 信頼ベースのworkspace setupを実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - preparation
  - mvp
milestone: m-1
dependencies:
  - TASK-21
  - TASK-13
references:
  - adrs/0005-リポジトリセットアップ前に信頼を要求する.md
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 22
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
リポジトリのセットアップ要件を自動検出し、提案内容を表示したうえで、リポジトリ単位の明示的な信頼がある場合だけ固定済みの実行バックエンドでsetupを実行する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 mise.toml、.tool-versions、主要manifest・lockfileからセットアップ候補を副作用なしで検出する
- [ ] #2 検出したコマンド、実行理由、対象worktree、実行バックエンド、信頼状態を実行前に表示する
- [ ] #3 信頼されていないリポジトリではコードやsetupコマンドを一切実行しない
- [ ] #4 信頼済みリポジトリで承認したsetupを固定バックエンドから実行し、ログ、失敗、キャンセル、再試行を扱う
- [ ] #5 信頼の付与・取り消しと、PR更新・別worktree・別backendでの境界をテストする
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
