---
id: task-1
title: プロジェクトのセットアップ
status: In Progress
assignee: []
created_date: ''
updated_date: '2026-08-20 17:37'
labels:
  - project-setup
milestone: m-0
dependencies: []
type: chore
---

## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
Review SweeperのRustアプリケーション基盤、開発ツール、CI、ローカルデータ配置を整備する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] 初期のRustアプリケーションスタックとリポジトリ構成を決める。
- [x] Backlog.mdを含め、`mise`をプロジェクトのツールマネージャーとして設定する。
- [ ] フォーマット、lint、テストのコマンドを`mise`タスクに追加する。
- [ ] Rustネイティブアプリの最初のUIフレームワーク連携方針を決める。候補の第一はGPUIとする。
- [ ] フォーマット、lint、テスト、基本的なビルド検証のCIを追加する。
- [ ] 共有リポジトリ、PR worktree、キャッシュ、ログ、設定用のローカルアプリデータディレクトリを定義する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
