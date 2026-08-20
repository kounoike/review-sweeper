---
id: task-15
title: 読み取り専用AIレビューを追加する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-20 17:37'
labels:
  - ai-review
milestone: m-1
dependencies:
  - TASK-13
  - TASK-14
type: feature
---

## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
GitHub差分と準備済みリポジトリの両方から構造化された読み取り専用のAI指摘を提供し、そこから作られるレビューコメントをユーザーが明示的に管理できるようにする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] ファイル、行、重要度、カテゴリ、タイトル、説明、提案コメントを持つ構造化AI指摘を定義する。
- [ ] GitHub差分データから差分だけを対象とした高速なAI事前レビューを実行する。
- [ ] worktree準備後にリポジトリを考慮した深いAIレビューを実行する。
- [ ] 差分・ソースビューの隣にAI指摘を表示する。
- [ ] 指摘の破棄と、レビューコメント下書きへの変換に対応する。
- [ ] MVPではAIレビューを読み取り専用にする。
- [ ] 呼び出し元の検索、関連テストの検索、baseとの比較、チェック失敗の確認、リスク説明のフォローアップ操作を追加する。
- [ ] ユーザーの明示的な操作なしにAIがコメントを投稿したりレビューを送信したりしないようにする。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
