---
id: TASK-29
title: ローカル修正をSuggested Changes下書きへ変換する
status: To Do
assignee: []
created_date: '2026-08-29 03:45'
labels:
  - mvp
  - worktree
milestone: m-1
dependencies:
  - TASK-17
  - TASK-20
references:
  - adrs/0006-GitHub差分とローカル差分を別のソースとして扱う.md
  - adrs/0007-構造化された差分モデルを使う.md
  - adrs/0008-ソースworktreeを観測可能なローカル状態として扱う.md
  - adrs/0009-AIレビューは読み取り専用とし人間が送信する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 1028
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
PR HEADに紐づくworktree上のユーザーまたはエージェントによる微修正を、GitHubのcanonical diff上で安全に位置付けられる場合だけSuggested Changesのレビュー下書きへ変換する。ローカル差分をGitHub差分の正本として扱わず、由来、revision、置換内容を保持し、ユーザー確認なしに送信しない。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 PR HEADに紐づくローカル変更から、source snapshot、expected head SHA、path、対象行範囲、元の内容、置換後内容、worktreeとbackendの由来を保持したSuggested Changes下書きを作成できる
- [ ] #2 変換対象の変更とGitHubへ送信される提案内容をプレビューし、ユーザーが明示的に選択・確認したものだけをTASK-17のレビュー下書きへ追加できる
- [ ] #3 GitHub canonical diff上に安全な位置を解決できない変更は近傍行へ推測配置せず、通常コメント、file-levelコメント、または送信不可として理由を表示する
- [ ] #4 PR head更新、force-push、ローカルHEAD divergence、元内容の不一致では下書きをNeedsRemapにし、ユーザーの再確認なしに再配置または送信しない
- [ ] #5 単一行・複数行、rename、空白変更、差分外変更、partial・binary、競合、複数のローカル変更をfixtureで検証し、未選択のローカル変更を変更・破棄しない
- [ ] #6 送信は既存の明示的レビュー送信フローを利用し、失敗、再試行、結果不明時に重複したSuggested Changesを作成しない
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
