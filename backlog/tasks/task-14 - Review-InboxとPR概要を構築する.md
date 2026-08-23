---
id: TASK-14
title: Review InboxとPR概要を構築する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-23 00:50'
labels:
  - mvp
milestone: m-1
dependencies:
  - TASK-10
  - TASK-2
  - TASK-6
  - TASK-8
  - TASK-11
references:
  - adrs/0001-Rustネイティブアプリケーションを採用する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 14
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
WindowsネイティブアプリでGitHub認証を行い、自分がレビューすべきPRをReview Inboxで把握してPR概要を開ける、MVPのオンライン専用経路を構築する。差分表示、コメント送信、worktree、AIは別タスクとする。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 安全に保管した認証情報でGitHubへ接続し、失効・権限不足・レート制限をユーザーへ通知できる
- [ ] #2 直接・チームのレビュー依頼と割り当てられたPRを取得し、Needs Review、Updated Since Review、Reviewedで分類できる
- [ ] #3 受信箱にリポジトリ、番号、タイトル、作成者、更新時刻、変更量、レビュー状態、Checks概要、Preparation状態を表示する
- [ ] #4 PR概要に説明、base/head、コミット、変更ファイル、reviewer、現在のレビュー状態、Checksを表示する
- [ ] #5 ページング、再取得、通信失敗、空状態をfixtureを使ったテストで検証する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
