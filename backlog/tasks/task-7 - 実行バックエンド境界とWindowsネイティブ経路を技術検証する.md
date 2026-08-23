---
id: TASK-7
title: 実行バックエンド境界とWindowsネイティブ経路を技術検証する
status: To Do
assignee: []
created_date: '2026-08-23 00:49'
labels:
  - project-setup
  - worktree
milestone: m-0
dependencies: []
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: spike
ordinal: 7
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0002に基づき、GUIからコマンド実行環境を分離する共通境界と、初期ターゲットであるWindowsネイティブ実行経路を実装前に検証する。WSL固有機能は実装せず、後から同じ境界へ追加できることを確認する。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 git、Worktree操作、workspace setupを表現する入力、出力、作業ディレクトリ、環境変数、キャンセル、ログ、エラーの境界を定義する
- [ ] #2 Worktreeと実行バックエンド種別・識別子を固定して保存・表示するモデルを定義する
- [ ] #3 Windowsネイティブでプロセス起動、終了、失敗、キャンセル、標準出力・標準エラー取得を最小プロトタイプで検証する
- [ ] #4 Windowsパスとバックエンド内パスを暗黙に混在させない型または変換境界を定義する
- [ ] #5 WSL2および将来のmacOS/Linuxネイティブ実装を追加できる拡張点と、再検討条件を技術メモに記録する
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
