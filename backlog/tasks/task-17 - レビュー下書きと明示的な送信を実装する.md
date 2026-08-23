---
id: TASK-17
title: レビュー下書きと明示的な送信を実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:51'
labels:
  - mvp
milestone: m-1
dependencies:
  - TASK-2
  - TASK-15
references:
  - adrs/0007-構造化された差分モデルを使う.md
  - adrs/0009-AIレビューは読み取り専用とし人間が送信する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 17
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHubのpending review modelに沿ってインラインコメントとレビュー本文を下書きし、ユーザー確認後にComment、Approve、Request Changesとして送信できるようにする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GitHub差分の有効な位置へインラインコメント下書きを作成・編集・削除できる
- [ ] #2 レビュー本文とインライン下書きをローカルに自動保存し、再起動後に復元できる
- [ ] #3 送信前に全下書きとレビュー種別を確認し、ユーザーの明示操作でのみ送信する
- [ ] #4 Comment、Approve、Request ChangesをGitHubへ送信し、成功後の状態を再取得する
- [ ] #5 outdated位置、権限不足、競合、部分失敗、再試行で重複送信しないことをfixtureで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
