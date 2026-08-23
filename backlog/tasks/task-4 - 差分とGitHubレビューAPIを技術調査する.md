---
id: TASK-4
title: 差分とGitHubレビューAPIを技術調査する
status: To Do
assignee: []
created_date: '2026-08-20 18:06'
updated_date: '2026-08-23 00:49'
labels:
  - project-setup
milestone: m-0
dependencies:
  - TASK-2
references:
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0007-構造化された差分モデルを使う.md
type: spike
ordinal: 4
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub差分とローカル差分を扱い、インラインコメントを正しい位置へ対応付けるための差分形式、行番号、レビューAPIの制約を検証する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GitHubの差分形式とローカルgit diffの差異をfixtureで確認する。
- [ ] #2 ファイル、ハンク、行、旧新行番号、コメント位置のモデルを定義する。
- [ ] #3 統合表示・分割表示・空白除外・前回レビューとの差分に必要な変換を検証する。
- [ ] #4 レビューコメントの下書きから送信までの位置情報とエラー処理を決める。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
