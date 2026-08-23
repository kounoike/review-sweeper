---
id: TASK-25
title: 読み取り専用AIレビューとFinding表示を実装する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:51'
labels:
  - ai-review
  - mvp
milestone: m-1
dependencies:
  - TASK-9
  - TASK-21
  - TASK-15
  - TASK-16
  - TASK-22
references:
  - adrs/0009-AIレビューは読み取り専用とし人間が送信する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 25
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub差分と準備済みworktreeを読み取り専用で解析し、構造化Findingとして差分・ソース閲覧へ表示する。AIはGitHub投稿やレビュー送信を行わず、ユーザーがFindingを破棄またはコメント下書きへ変換する。高度なcoding agent連携はMVP後とする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ファイル、行、重要度、カテゴリ、タイトル、説明、提案コメントを持つ構造化Findingを定義する
- [ ] #2 GitHub差分のみの事前レビューと、準備済みworktreeを読む深いレビューを実行できる
- [ ] #3 Findingを差分・ソース位置へ対応付けて表示し、破棄またはレビューコメント下書きへ変換できる
- [ ] #4 AIに渡すデータ、読み取り権限、失敗・キャンセル・再試行、保存期間を明示する
- [ ] #5 AIがGitHubコメント投稿、レビュー送信、ソース変更、任意コマンド実行を行えないことを境界テストで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
