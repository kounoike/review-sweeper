---
id: task-14
title: PR workspaceを安全に準備する
status: To Do
assignee: []
created_date: ''
updated_date: '2026-08-20 17:37'
labels:
  - preparation
milestone: m-1
dependencies:
  - TASK-12
  - TASK-13
type: feature
---

## Description
<!-- SECTION:DESCRIPTION:BEGIN -->
PR準備をレビューセッションから分離し、バックグラウンドのworkspaceセットアップを予測可能で、信頼済みかつリソース制限付きにする。

準備の段階は機能仕様として管理し、Minimal、Workspace、AI Review、Fullを提供する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] 各PRにレビューセッションとは別の準備状態を持たせる。
- [ ] Minimal、Workspace、AI Review、Fullの準備モードを追加する。
- [ ] PRがキューに入ったらメタデータと差分をすぐに取得する。
- [ ] バックグラウンドの準備手順としてworktreeを作成・更新する。
- [ ] `mise.toml`、`.tool-versions`、`package.json`、ロックファイル、`Cargo.toml`、`uv.lock`、`poetry.lock`、`go.mod`などからリポジトリセットアップの必要性を検出する。
- [ ] リポジトリが信頼されるまで実行せず、セットアップコマンドを提案する。
- [ ] `mise install`や`pnpm install --frozen-lockfile`など、承認済みのセットアップコマンドを実行する。
- [ ] セットアップコマンド実行にリポジトリ単位の信頼を追加する。
- [ ] バックグラウンドのworktree準備とAIレビュー処理にリソース制限を追加する。
- [ ] ユーザーが開いた、または明示的に準備したPRを優先する。
- [ ] 差分取得、workspace作成、セットアップ実行、AIレビュー実行、全件準備、次のPR準備の手動操作を追加する。
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
