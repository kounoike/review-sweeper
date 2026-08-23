---
id: TASK-13
title: Windowsネイティブ実行バックエンド基盤を実装する
status: To Do
assignee: []
created_date: '2026-08-23 00:49'
labels:
  - worktree
  - mvp
milestone: m-1
dependencies:
  - TASK-10
  - TASK-8
  - TASK-11
  - TASK-7
references:
  - adrs/0002-WindowsネイティブGUIとWSL実行バックエンドを分離する.md
documentation:
  - backlog/docs/product/doc-3 - Scope-and-Roadmap.md
type: feature
ordinal: 13
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
ADR-0002と技術検証結果に基づき、WindowsネイティブGUIからgit、Worktree操作、workspace setupを一貫して実行する共通バックエンド基盤とWindowsネイティブ実装を構築する。WSLは含めず、WSL未導入のMVP経路を成立させる。
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GUI・ドメイン層がOSコマンドを直接起動せず、共通実行バックエンド境界を経由する
- [ ] #2 Windowsネイティブ実装で作業ディレクトリ、環境変数、出力、終了状態、キャンセルを扱える
- [ ] #3 WorktreeにWindowsネイティブのバックエンド識別子を固定し、状態として取得できる
- [ ] #4 パス、環境変数、git状態が別バックエンドと暗黙に混在しない不変条件をテストする
- [ ] #5 WSLなしのWindows環境で基本実行経路が動作し、解決可能なエラー情報をUI層へ返せる
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 テストと該当するチェックが通る
- [ ] #2 文書が更新されている
- [ ] #3 リグレッションがない
<!-- DOD:END -->
